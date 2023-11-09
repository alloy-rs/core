use crate::{ty::as_tuple, DynSolType, DynSolValue, Result};
use alloc::vec::Vec;
use alloy_primitives::{Address, FixedBytes, Function, Sign, I256, U256};
use alloy_sol_type_parser::utils::{array_parser, char_parser, spanned};
use alloy_sol_types::Word;
use core::fmt;
use hex::FromHexError;
use winnow::{
    ascii::{alpha0, alpha1, digit1, hex_digit0, hex_digit1, space0},
    combinator::{cut_err, dispatch, fail, opt, preceded, success},
    error::{
        AddContext, ContextError, ErrMode, ErrorKind, FromExternalError, StrContext,
        StrContextValue,
    },
    stream::Stream,
    token::take_while,
    trace::trace,
    PResult, Parser,
};

impl DynSolType {
    /// Coerces a string into a [`DynSolValue`] via this type.
    ///
    /// # Syntax
    ///
    /// - [`Bool`](DynSolType::Bool): `true|false`
    /// - [`Int`](DynSolType::Int): `[+-]?{Uint}`
    /// - [`Uint`](DynSolType::Uint): `{literal}(\.[0-9]+)?(\s*{unit})?`
    ///   - literal: base 2, 8, 10, or 16 integer literal. If not in base 10, must be prefixed with
    ///     `0b`, `0o`, or `0x` respectively.
    ///   - unit: same as [Solidity ether units](https://docs.soliditylang.org/en/latest/units-and-global-variables.html#ether-units)
    ///   - decimals with more digits than the unit's exponent value are not allowed
    ///   - decimals are only allowed when the `std` feature is enabled due to floating point
    ///     operations; this may be relaxed in the future
    /// - [`FixedBytes`](DynSolType::FixedBytes): `(0x)?[0-9A-Fa-f]{$0*2}`
    /// - [`Address`](DynSolType::Address): `(0x)?[0-9A-Fa-f]{40}`
    /// - [`Function`](DynSolType::Function): `(0x)?[0-9A-Fa-f]{48}`
    /// - [`Bytes`](DynSolType::Bytes): `(0x)?[0-9A-Fa-f]+`
    /// - [`String`](DynSolType::String): `.*`
    ///   - can be surrounded by a pair of `"` or `'`
    ///   - trims whitespace if not surrounded
    /// - [`Array`](DynSolType::Array): any number of the inner type delimited by commas (`,`) and
    ///   surrounded by brackets (`[]`)
    /// - [`FixedArray`](DynSolType::FixedArray): exactly the given number of the inner type
    ///   delimited by commas (`,`) and surrounded by brackets (`[]`)
    /// - [`Tuple`](DynSolType::Tuple): the inner types delimited by commas (`,`) and surrounded by
    ///   parentheses (`()`)
    /// - [`CustomStruct`](DynSolType::CustomStruct): the same as `Tuple`
    ///
    /// # Examples
    ///
    /// ```
    /// use alloy_dyn_abi::{DynSolType, DynSolValue};
    /// use alloy_primitives::U256;
    ///
    /// let ty: DynSolType = "(uint256,string)[]".parse()?;
    /// let value = ty.coerce_str("[(0, \"hello\"), (42, \"world\")]")?;
    /// assert_eq!(
    ///     value,
    ///     DynSolValue::Array(vec![
    ///         DynSolValue::Tuple(vec![
    ///             DynSolValue::Uint(U256::from(0), 256),
    ///             DynSolValue::String(String::from("hello"))
    ///         ]),
    ///         DynSolValue::Tuple(vec![
    ///             DynSolValue::Uint(U256::from(42), 256),
    ///             DynSolValue::String(String::from("world"))
    ///         ]),
    ///     ])
    /// );
    /// assert!(value.matches(&ty));
    /// assert_eq!(value.as_type().unwrap(), ty);
    /// # Ok::<_, alloy_dyn_abi::Error>(())
    /// ```
    #[doc(alias = "tokenize")] // from ethabi
    pub fn coerce_str(&self, s: &str) -> Result<DynSolValue> {
        ValueParser::new(self)
            .parse(s)
            .map_err(|e| crate::Error::TypeParser(alloy_sol_type_parser::Error::parser(e)))
    }
}

struct ValueParser<'a> {
    ty: &'a DynSolType,
    list_end: Option<char>,
}

impl<'i> Parser<&'i str, DynSolValue, ContextError> for ValueParser<'_> {
    fn parse_next(&mut self, input: &mut &'i str) -> PResult<DynSolValue, ContextError> {
        #[cfg(feature = "debug")]
        let name = self.ty.sol_type_name();
        #[cfg(not(feature = "debug"))]
        let name = "value_parser";
        trace(name, move |input: &mut &str| match self.ty {
            DynSolType::Bool => bool(input).map(DynSolValue::Bool),
            &DynSolType::Int(size) => {
                int(size).parse_next(input).map(|int| DynSolValue::Int(int, size))
            }
            &DynSolType::Uint(size) => {
                uint(size).parse_next(input).map(|uint| DynSolValue::Uint(uint, size))
            }
            &DynSolType::FixedBytes(size) => {
                fixed_bytes(size).parse_next(input).map(|word| DynSolValue::FixedBytes(word, size))
            }
            DynSolType::Address => address(input).map(DynSolValue::Address),
            DynSolType::Function => function(input).map(DynSolValue::Function),
            DynSolType::Bytes => bytes(input).map(DynSolValue::Bytes),
            DynSolType::String => {
                self.string().parse_next(input).map(|s| DynSolValue::String(s.into()))
            }
            DynSolType::Array(ty) => self.in_list(']', |this| {
                this.with(ty).array().parse_next(input).map(DynSolValue::Array)
            }),
            DynSolType::FixedArray(ty, len) => self.in_list(']', |this| {
                this.with(ty).fixed_array(*len).parse_next(input).map(DynSolValue::Array)
            }),
            as_tuple!(DynSolType tys) => {
                self.in_list(')', |this| this.tuple(tys).parse_next(input).map(DynSolValue::Tuple))
            }
        })
        .parse_next(input)
    }
}

impl<'a> ValueParser<'a> {
    #[inline]
    const fn new(ty: &'a DynSolType) -> Self {
        Self { list_end: None, ty }
    }

    #[inline]
    fn in_list<F: FnOnce(&mut Self) -> R, R>(&mut self, list_end: char, f: F) -> R {
        let prev = core::mem::replace(&mut self.list_end, Some(list_end));
        let r = f(self);
        self.list_end = prev;
        r
    }

    #[inline]
    const fn with(&self, ty: &'a DynSolType) -> Self {
        Self { list_end: self.list_end, ty }
    }

    #[inline]
    fn string<'s, 'i: 's>(&'s self) -> impl Parser<&'i str, &'i str, ContextError> + 's {
        trace("string", |input: &mut &'i str| {
            let Some(delim) = input.chars().next() else {
                return Ok("");
            };
            let has_delim = matches!(delim, '"' | '\'');
            if has_delim {
                *input = &input[1..];
            }

            // TODO: escapes?
            let mut s = if has_delim || self.list_end.is_some() {
                let (chs, l) = if has_delim {
                    ([delim, '\0'], 1)
                } else if let Some(c) = self.list_end {
                    ([',', c], 2)
                } else {
                    unreachable!()
                };
                let min = if has_delim { 0 } else { 1 };
                take_while(min.., move |c: char| !unsafe { chs.get_unchecked(..l) }.contains(&c))
                    .parse_next(input)?
            } else {
                input.next_slice(input.len())
            };

            if has_delim {
                cut_err(char_parser(delim))
                    .context(StrContext::Label("string"))
                    .parse_next(input)?;
            } else {
                s = s.trim_end();
            }

            Ok(s)
        })
    }

    #[inline]
    fn array<'i: 'a>(self) -> impl Parser<&'i str, Vec<DynSolValue>, ContextError> + 'a {
        #[cfg(feature = "debug")]
        let name = format!("{}[]", self.ty);
        #[cfg(not(feature = "debug"))]
        let name = "array";
        trace(name, array_parser(self))
    }

    #[inline]
    fn fixed_array<'i: 'a>(
        self,
        len: usize,
    ) -> impl Parser<&'i str, Vec<DynSolValue>, ContextError> + 'a {
        #[cfg(feature = "debug")]
        let name = format!("{}[{len}]", self.ty);
        #[cfg(not(feature = "debug"))]
        let name = "fixed_array";
        trace(
            name,
            array_parser(self).try_map(move |values: Vec<DynSolValue>| {
                if values.len() == len {
                    Ok(values)
                } else {
                    Err(Error::FixedArrayLengthMismatch(len, values.len()))
                }
            }),
        )
    }

    #[inline]
    fn tuple<'i: 's, 't: 's, 's>(
        &'s self,
        tuple: &'t Vec<DynSolType>,
    ) -> impl Parser<&'i str, Vec<DynSolValue>, ContextError> + 's {
        #[cfg(feature = "debug")]
        let name = DynSolType::Tuple(tuple.clone()).to_string();
        #[cfg(not(feature = "debug"))]
        let name = "tuple";
        trace(name, move |input: &mut &'i str| {
            space0(input)?;
            char_parser('(').parse_next(input)?;

            let mut values = Vec::with_capacity(tuple.len());
            for (i, ty) in tuple.iter().enumerate() {
                if i > 0 {
                    space0(input)?;
                    char_parser(',').parse_next(input)?;
                }
                space0(input)?;
                values.push(self.with(ty).parse_next(input)?);
            }

            space0(input)?;
            char_parser(')').parse_next(input)?;

            Ok(values)
        })
    }
}

#[derive(Debug)]
enum Error {
    IntOverflow,
    #[cfg(not(feature = "std"))]
    FloatNoStd(f64),
    #[cfg(feature = "std")]
    FractionalNotAllowed(f64),
    #[cfg(feature = "std")]
    TooManyDecimals(usize, usize),
    InvalidFixedBytesLength(usize),
    FixedArrayLengthMismatch(usize, usize),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntOverflow => f.write_str("number too large to fit in target type"),
            #[cfg(not(feature = "std"))]
            Self::FloatNoStd(n) => {
                write!(f, "fractional numbers are not supported without `std`: {n}")
            }
            #[cfg(feature = "std")]
            Self::TooManyDecimals(expected, actual) => {
                write!(f, "expected at most {expected} decimals, got {actual}")
            }
            #[cfg(feature = "std")]
            Self::FractionalNotAllowed(n) => write!(
                f,
                "non-zero fraction {n} not allowed without specifying non-wei units (gwei, ether, etc.)"
            ),
            Self::InvalidFixedBytesLength(len) => {
                write!(f, "fixed bytes length {len} greater than 32")
            }
            Self::FixedArrayLengthMismatch(expected, actual) => write!(
                f,
                "fixed array length mismatch: expected {expected} elements, got {actual}"
            ),
        }
    }
}

#[inline]
fn bool(input: &mut &str) -> PResult<bool> {
    trace(
        "bool",
        dispatch! {alpha1.context(StrContext::Label("boolean"));
            "true" => success(true),
            "false" => success(false),
            _ => fail
        }
        .context(StrContext::Label("boolean")),
    )
    .parse_next(input)
}

#[inline]
fn int<'i>(size: usize) -> impl Parser<&'i str, I256, ContextError> {
    #[cfg(feature = "debug")]
    let name = format!("int{size}");
    #[cfg(not(feature = "debug"))]
    let name = "int";
    trace(
        name,
        (int_sign, uint(size)).try_map(|(sign, abs)| {
            I256::checked_from_sign_and_abs(sign, abs).ok_or(Error::IntOverflow)
        }),
    )
}

#[inline]
fn int_sign(input: &mut &str) -> PResult<Sign> {
    trace("int_sign", |input: &mut &str| match input.as_bytes().first() {
        Some(b'+') => {
            *input = &input[1..];
            Ok(Sign::Positive)
        }
        Some(b'-') => {
            *input = &input[1..];
            Ok(Sign::Negative)
        }
        Some(_) | None => Ok(Sign::Positive),
    })
    .parse_next(input)
}

#[inline]
fn uint<'i>(len: usize) -> impl Parser<&'i str, U256, ContextError> {
    #[cfg(feature = "debug")]
    let name = format!("uint{len}");
    #[cfg(not(feature = "debug"))]
    let name = "uint";
    trace(name, move |input: &mut &str| {
        let (s, (_, fract)) = spanned((
            prefixed_int,
            opt(preceded(
                '.',
                cut_err(digit1.context(StrContext::Expected(StrContextValue::Description(
                    "at least one digit",
                )))),
            )),
        ))
        .parse_next(input)?;

        let _ = space0(input)?;
        let units = int_units(input)?;

        let uint = if let Some(fract) = fract {
            let x = s
                .parse::<f64>()
                .map_err(|e| ErrMode::from_external_error(input, ErrorKind::Verify, e))?;

            // TODO: add num_traits with libm feature to support this
            #[cfg(not(feature = "std"))]
            {
                let _ = fract;
                return Err(ErrMode::from_external_error(
                    input,
                    ErrorKind::Verify,
                    Error::FloatNoStd(x),
                ));
            }

            #[cfg(feature = "std")]
            {
                if units == 0 && x.fract() != 0.0 {
                    return Err(ErrMode::from_external_error(
                        input,
                        ErrorKind::Verify,
                        Error::FractionalNotAllowed(x.fract()),
                    ));
                }

                if fract.len() > units {
                    return Err(ErrMode::from_external_error(
                        input,
                        ErrorKind::Verify,
                        Error::TooManyDecimals(units, fract.len()),
                    ));
                }

                U256::try_from(x * 10f64.powi(units as i32))
                    .map_err(|e| ErrMode::from_external_error(input, ErrorKind::Verify, e))
            }
        } else {
            s.parse::<U256>()
                .map_err(|e| ErrMode::from_external_error(input, ErrorKind::Verify, e))?
                .checked_mul(U256::from(10usize.pow(units as u32)))
                .ok_or_else(|| {
                    ErrMode::from_external_error(input, ErrorKind::Verify, Error::IntOverflow)
                })
        }?;

        if uint.bit_len() > len {
            return Err(ErrMode::from_external_error(input, ErrorKind::Verify, Error::IntOverflow));
        }

        Ok(uint)
    })
}

#[inline]
fn prefixed_int<'i>(input: &mut &'i str) -> PResult<&'i str> {
    trace("prefixed_int", |input: &mut &'i str| {
        let has_prefix = matches!(input.get(..2), Some("0b" | "0B" | "0o" | "0O" | "0x" | "0X"));
        if has_prefix {
            *input = &input[2..];
            // parse hex since it's the most general
            hex_digit1(input)
        } else {
            digit1(input)
        }
        .map_err(|e| {
            e.add_context(
                input,
                StrContext::Expected(StrContextValue::Description("at least one digit")),
            )
        })
    })
    .parse_next(input)
}

#[inline]
fn int_units(input: &mut &str) -> PResult<usize> {
    trace(
        "int_units",
        dispatch! {alpha0;
            "ether" => success(18),
            "gwei" | "nano" | "nanoether" => success(9),
            "" | "wei" => success(0),
            _ => fail,
        },
    )
    .parse_next(input)
}

#[inline]
fn fixed_bytes<'i>(len: usize) -> impl Parser<&'i str, Word, ContextError> {
    #[cfg(feature = "debug")]
    let name = format!("bytes{len}");
    #[cfg(not(feature = "debug"))]
    let name = "bytesN";
    trace(name, move |input: &mut &str| {
        if len > Word::len_bytes() {
            return Err(ErrMode::from_external_error(
                input,
                ErrorKind::Fail,
                Error::InvalidFixedBytesLength(len),
            ));
        }

        let hex = hex_str(input)?;
        let mut out = Word::ZERO;
        match hex::decode_to_slice(hex, &mut out[..len]) {
            Ok(()) => Ok(out),
            Err(e) => Err(hex_error(input, e)),
        }
    })
}

#[inline]
fn address(input: &mut &str) -> PResult<Address> {
    trace("address", fixed_bytes_inner).parse_next(input).map(Address::from)
}

#[inline]
fn function(input: &mut &str) -> PResult<Function> {
    trace("function", fixed_bytes_inner).parse_next(input).map(Function::from)
}

#[inline]
fn bytes(input: &mut &str) -> PResult<Vec<u8>> {
    trace("bytes", hex_str.try_map(hex::decode)).parse_next(input)
}

#[inline]
fn fixed_bytes_inner<const N: usize>(input: &mut &str) -> PResult<FixedBytes<N>> {
    hex_str
        .try_map(|s| {
            let mut out = FixedBytes::ZERO;
            hex::decode_to_slice(s, out.as_mut_slice()).map(|()| out)
        })
        .parse_next(input)
}

#[inline]
fn hex_str<'i>(input: &mut &'i str) -> PResult<&'i str> {
    trace("hex_str", preceded(opt("0x"), hex_digit0)).parse_next(input)
}

fn hex_error(input: &&str, e: FromHexError) -> ErrMode<ContextError> {
    let kind = match e {
        FromHexError::InvalidHexCharacter { .. } => unreachable!("{e:?}"),
        FromHexError::InvalidStringLength | FromHexError::OddLength => ErrorKind::Eof,
    };
    ErrMode::from_external_error(input, kind, e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{
        boxed::Box,
        string::{String, ToString},
    };
    use alloy_primitives::address;
    use core::str::FromStr;

    #[test]
    fn coerce_bool() {
        assert_eq!(DynSolType::Bool.coerce_str("true").unwrap(), DynSolValue::Bool(true));
        assert_eq!(DynSolType::Bool.coerce_str("false").unwrap(), DynSolValue::Bool(false));

        assert!(DynSolType::Bool.coerce_str("").is_err());
        assert!(DynSolType::Bool.coerce_str("0").is_err());
        assert!(DynSolType::Bool.coerce_str("1").is_err());
        assert!(DynSolType::Bool.coerce_str("tru").is_err());
    }

    #[test]
    fn coerce_int() {
        assert_eq!(
            DynSolType::Int(256)
                .coerce_str("0x1111111111111111111111111111111111111111111111111111111111111111")
                .unwrap(),
            DynSolValue::Int(I256::from_be_bytes([0x11; 32]), 256)
        );

        assert_eq!(
            DynSolType::Int(256)
                .coerce_str("0x2222222222222222222222222222222222222222222222222222222222222222")
                .unwrap(),
            DynSolValue::Int(I256::from_be_bytes([0x22; 32]), 256)
        );

        assert_eq!(
            DynSolType::Int(256)
                .coerce_str("0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
                .unwrap(),
            DynSolValue::Int(I256::MAX, 256)
        );
        assert!(DynSolType::Int(256)
            .coerce_str("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .is_err());

        assert_eq!(
            DynSolType::Int(256).coerce_str("0").unwrap(),
            DynSolValue::Int(I256::ZERO, 256)
        );

        assert_eq!(
            DynSolType::Int(256).coerce_str("-0").unwrap(),
            DynSolValue::Int(I256::ZERO, 256)
        );

        assert_eq!(
            DynSolType::Int(256).coerce_str("+0").unwrap(),
            DynSolValue::Int(I256::ZERO, 256)
        );

        assert_eq!(
            DynSolType::Int(256).coerce_str("-1").unwrap(),
            DynSolValue::Int(I256::MINUS_ONE, 256)
        );

        assert_eq!(
            DynSolType::Int(256)
                .coerce_str(
                    "57896044618658097711785492504343953926634992332820282019728792003956564819967"
                )
                .unwrap(),
            DynSolValue::Int(I256::MAX, 256)
        );
        assert_eq!(
            DynSolType::Int(256).coerce_str("-57896044618658097711785492504343953926634992332820282019728792003956564819968").unwrap(),
            DynSolValue::Int(I256::MIN, 256)
        );
    }

    #[test]
    fn coerce_uint() {
        assert_eq!(
            DynSolType::Uint(256)
                .coerce_str("0x1111111111111111111111111111111111111111111111111111111111111111")
                .unwrap(),
            DynSolValue::Uint(U256::from_be_bytes([0x11; 32]), 256)
        );

        assert_eq!(
            DynSolType::Uint(256)
                .coerce_str("0x2222222222222222222222222222222222222222222222222222222222222222")
                .unwrap(),
            DynSolValue::Uint(U256::from_be_bytes([0x22; 32]), 256)
        );

        assert_eq!(
            DynSolType::Uint(256)
                .coerce_str("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
                .unwrap(),
            DynSolValue::Uint(U256::from_be_bytes([0xff; 32]), 256)
        );

        // 255 bits fails
        assert!(DynSolType::Uint(255)
            .coerce_str("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .is_err());

        assert_eq!(
            DynSolType::Uint(256)
                .coerce_str("115792089237316195423570985008687907853269984665640564039457584007913129639935")
                .unwrap(),
            DynSolValue::Uint(U256::MAX, 256)
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("0").unwrap(),
            DynSolValue::Uint(U256::ZERO, 256)
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("1").unwrap(),
            DynSolValue::Uint(U256::from(1), 256)
        );
    }

    #[test]
    fn coerce_uint_wei() {
        assert_eq!(
            DynSolType::Uint(256).coerce_str("1wei").unwrap(),
            DynSolValue::Uint(U256::from(1), 256)
        );
        assert_eq!(
            DynSolType::Uint(256).coerce_str("1 wei").unwrap(),
            DynSolValue::Uint(U256::from(1), 256)
        );

        assert!(DynSolType::Uint(256).coerce_str("1").is_ok());
        assert!(DynSolType::Uint(256).coerce_str("1.").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1 .").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1 .0").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1.wei").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1. wei").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1.0wei").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1.0 wei").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1.00wei").is_err());
        assert!(DynSolType::Uint(256).coerce_str("1.00 wei").is_err());
    }

    #[test]
    fn coerce_uint_gwei() {
        assert_eq!(
            DynSolType::Uint(256).coerce_str("1nano").unwrap(),
            DynSolValue::Uint(U256::from_str("1000000000").unwrap(), 256)
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("1nanoether").unwrap(),
            DynSolValue::Uint(U256::from_str("1000000000").unwrap(), 256)
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("1gwei").unwrap(),
            DynSolValue::Uint(U256::from_str("1000000000").unwrap(), 256)
        );

        if cfg!(feature = "std") {
            assert_eq!(
                DynSolType::Uint(256).coerce_str("0.1 gwei").unwrap(),
                DynSolValue::Uint(U256::from_str("100000000").unwrap(), 256)
            );
        }
    }

    #[test]
    fn coerce_uint_ether() {
        assert_eq!(
            DynSolType::Uint(256).coerce_str("10000000000ether").unwrap(),
            DynSolValue::Uint(U256::from_str("10000000000000000000000000000").unwrap(), 256)
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("1ether").unwrap(),
            DynSolValue::Uint(U256::from_str("1000000000000000000").unwrap(), 256)
        );

        if cfg!(feature = "std") {
            assert_eq!(
                DynSolType::Uint(256).coerce_str("0.01 ether").unwrap(),
                DynSolValue::Uint(U256::from_str("10000000000000000").unwrap(), 256)
            );

            assert_eq!(
                DynSolType::Uint(256).coerce_str("0.000000000000000001ether").unwrap(),
                DynSolValue::Uint(U256::from(1), 256)
            );

            assert_eq!(
                DynSolType::Uint(256).coerce_str("0.000000000000000001ether"),
                DynSolType::Uint(256).coerce_str("1wei"),
            );
        }
    }

    #[test]
    fn coerce_uint_array_ether() {
        assert_eq!(
            DynSolType::Array(Box::new(DynSolType::Uint(256)))
                .coerce_str("[ 1   ether,  10 ether ]")
                .unwrap(),
            DynSolValue::Array(vec![
                DynSolValue::Uint(U256::from_str("1000000000000000000").unwrap(), 256),
                DynSolValue::Uint(U256::from_str("10000000000000000000").unwrap(), 256),
            ])
        );
    }

    #[test]
    fn coerce_uint_invalid_units() {
        // 0.1 wei
        assert!(DynSolType::Uint(256).coerce_str("0.1 wei").is_err());
        assert!(DynSolType::Uint(256).coerce_str("0.0000000000000000001ether").is_err());

        // 1 ether + 0.1 wei
        assert!(DynSolType::Uint(256).coerce_str("1.0000000000000000001ether").is_err());

        // 1_000_000_000 ether + 0.1 wei
        assert!(DynSolType::Uint(256).coerce_str("1000000000.0000000000000000001ether").is_err());

        assert!(DynSolType::Uint(256).coerce_str("0..1 gwei").is_err());

        assert!(DynSolType::Uint(256).coerce_str("..1 gwei").is_err());

        assert!(DynSolType::Uint(256).coerce_str("1. gwei").is_err());

        assert!(DynSolType::Uint(256).coerce_str(".1 gwei").is_err());

        assert!(DynSolType::Uint(256).coerce_str("2.1.1 gwei").is_err());

        assert!(DynSolType::Uint(256).coerce_str(".1.1 gwei").is_err());

        assert!(DynSolType::Uint(256).coerce_str("1abc").is_err());

        assert!(DynSolType::Uint(256).coerce_str("1 gwei ").is_err());

        assert!(DynSolType::Uint(256).coerce_str("g 1 gwei").is_err());

        assert!(DynSolType::Uint(256).coerce_str("1gwei 1 gwei").is_err());
    }

    #[test]
    fn coerce_fixed_bytes() {
        let mk_word = |sl: &[u8]| {
            let mut out = Word::ZERO;
            out[..sl.len()].copy_from_slice(sl);
            out
        };

        // not actually valid, but we don't care here
        assert_eq!(
            DynSolType::FixedBytes(0).coerce_str("0x").unwrap(),
            DynSolValue::FixedBytes(mk_word(&[]), 0)
        );

        assert_eq!(
            DynSolType::FixedBytes(1).coerce_str("0x00").unwrap(),
            DynSolValue::FixedBytes(mk_word(&[0x00]), 1)
        );
        assert_eq!(
            DynSolType::FixedBytes(1).coerce_str("0x00").unwrap(),
            DynSolValue::FixedBytes(mk_word(&[0x00]), 1)
        );
        assert_eq!(
            DynSolType::FixedBytes(2).coerce_str("0017").unwrap(),
            DynSolValue::FixedBytes(mk_word(&[0x00, 0x17]), 2)
        );
        assert_eq!(
            DynSolType::FixedBytes(3).coerce_str("123456").unwrap(),
            DynSolValue::FixedBytes(mk_word(&[0x12, 0x34, 0x56]), 3)
        );

        assert!(DynSolType::FixedBytes(1).coerce_str("").is_err());
        assert!(DynSolType::FixedBytes(1).coerce_str("0").is_err());
        assert!(DynSolType::FixedBytes(1).coerce_str("0x").is_err());
        assert!(DynSolType::FixedBytes(1).coerce_str("0x0").is_err());
    }

    #[test]
    fn coerce_address() {
        // 38
        assert!(DynSolType::Address.coerce_str("00000000000000000000000000000000000000").is_err());
        // 39
        assert!(DynSolType::Address.coerce_str("000000000000000000000000000000000000000").is_err());
        // 40
        assert_eq!(
            DynSolType::Address.coerce_str("0000000000000000000000000000000000000000").unwrap(),
            DynSolValue::Address(Address::ZERO)
        );
        assert_eq!(
            DynSolType::Address.coerce_str("0x1111111111111111111111111111111111111111").unwrap(),
            DynSolValue::Address(Address::new([0x11; 20]))
        );
        assert_eq!(
            DynSolType::Address.coerce_str("2222222222222222222222222222222222222222").unwrap(),
            DynSolValue::Address(Address::new([0x22; 20]))
        );
    }

    #[test]
    fn coerce_function() {
        assert_eq!(
            DynSolType::Function
                .coerce_str("000000000000000000000000000000000000000000000000")
                .unwrap(),
            DynSolValue::Function(Function::ZERO)
        );
        assert_eq!(
            DynSolType::Function
                .coerce_str("0x111111111111111111111111111111111111111111111111")
                .unwrap(),
            DynSolValue::Function(Function::new([0x11; 24]))
        );
        assert_eq!(
            DynSolType::Function
                .coerce_str("222222222222222222222222222222222222222222222222")
                .unwrap(),
            DynSolValue::Function(Function::new([0x22; 24]))
        );
    }

    #[test]
    fn coerce_bytes() {
        assert_eq!(DynSolType::Bytes.coerce_str("").unwrap(), DynSolValue::Bytes(vec![]));
        assert_eq!(DynSolType::Bytes.coerce_str("0x").unwrap(), DynSolValue::Bytes(vec![]));
        assert!(DynSolType::Bytes.coerce_str("0x0").is_err());
        assert!(DynSolType::Bytes.coerce_str("0").is_err());
        assert_eq!(DynSolType::Bytes.coerce_str("00").unwrap(), DynSolValue::Bytes(vec![0]));
        assert_eq!(DynSolType::Bytes.coerce_str("0x00").unwrap(), DynSolValue::Bytes(vec![0]));

        assert_eq!(
            DynSolType::Bytes.coerce_str("123456").unwrap(),
            DynSolValue::Bytes(vec![0x12, 0x34, 0x56])
        );
        assert_eq!(
            DynSolType::Bytes.coerce_str("0x0017").unwrap(),
            DynSolValue::Bytes(vec![0x00, 0x17])
        );
    }

    #[test]
    fn coerce_string() {
        assert_eq!(
            DynSolType::String.coerce_str("gavofyork").unwrap(),
            DynSolValue::String("gavofyork".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("gav of york").unwrap(),
            DynSolValue::String("gav of york".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("\"hello world\"").unwrap(),
            DynSolValue::String("hello world".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("'hello world'").unwrap(),
            DynSolValue::String("hello world".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("'\"hello world\"'").unwrap(),
            DynSolValue::String("\"hello world\"".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("'   hello world '").unwrap(),
            DynSolValue::String("   hello world ".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("'\"hello world'").unwrap(),
            DynSolValue::String("\"hello world".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("a, b").unwrap(),
            DynSolValue::String("a, b".into())
        );
        assert_eq!(
            DynSolType::String.coerce_str("hello (world)").unwrap(),
            DynSolValue::String("hello (world)".into())
        );

        assert!(DynSolType::String.coerce_str("\"hello world").is_err());
        assert!(DynSolType::String.coerce_str("\"hello world'").is_err());
        assert!(DynSolType::String.coerce_str("'hello world").is_err());
        assert!(DynSolType::String.coerce_str("'hello world\"").is_err());

        assert_eq!(
            DynSolType::String.coerce_str("Hello, world!").unwrap(),
            DynSolValue::String("Hello, world!".into())
        );
        let s = "$$g]a\"v/of;[()];2,yo\r)k_";
        assert_eq!(DynSolType::String.coerce_str(s).unwrap(), DynSolValue::String(s.into()));
    }

    #[test]
    fn coerce_strings() {
        let arr = DynSolType::Array(Box::new(DynSolType::String));
        let mk_arr = |s: &[&str]| {
            DynSolValue::Array(s.iter().map(|s| DynSolValue::String(s.to_string())).collect())
        };

        assert_eq!(arr.coerce_str("[]").unwrap(), mk_arr(&[]));
        assert_eq!(arr.coerce_str("[    ]").unwrap(), mk_arr(&[]));

        // TODO: should this be an error?
        // assert!(arr.coerce_str("[,]").is_err());
        // assert!(arr.coerce_str("[ , ]").is_err());

        assert_eq!(arr.coerce_str("[ foo bar ]").unwrap(), mk_arr(&["foo bar"]));
        assert_eq!(arr.coerce_str("[foo bar,]").unwrap(), mk_arr(&["foo bar"]));
        assert_eq!(arr.coerce_str("[  foo bar,  ]").unwrap(), mk_arr(&["foo bar"]));
        assert_eq!(arr.coerce_str("[ foo , bar ]").unwrap(), mk_arr(&["foo", "bar"]));

        assert_eq!(arr.coerce_str("[\"foo\",\"bar\"]").unwrap(), mk_arr(&["foo", "bar"]));

        assert_eq!(arr.coerce_str("['']").unwrap(), mk_arr(&[""]));
        assert_eq!(arr.coerce_str("[\"\"]").unwrap(), mk_arr(&[""]));
        assert_eq!(arr.coerce_str("['', '']").unwrap(), mk_arr(&["", ""]));
        assert_eq!(arr.coerce_str("['', \"\"]").unwrap(), mk_arr(&["", ""]));
        assert_eq!(arr.coerce_str("[\"\", '']").unwrap(), mk_arr(&["", ""]));
        assert_eq!(arr.coerce_str("[\"\", \"\"]").unwrap(), mk_arr(&["", ""]));
    }

    #[test]
    fn coerce_empty_array() {
        assert_eq!(
            DynSolType::Array(Box::new(DynSolType::Bool)).coerce_str("[]").unwrap(),
            DynSolValue::Array(vec![])
        );
    }

    #[test]
    fn coerce_bool_array() {
        assert_eq!(
            DynSolType::coerce_str(&DynSolType::Array(Box::new(DynSolType::Bool)), "[true, false]")
                .unwrap(),
            DynSolValue::Array(vec![DynSolValue::Bool(true), DynSolValue::Bool(false)])
        );
    }

    #[test]
    fn coerce_bool_array_of_arrays() {
        assert_eq!(
            DynSolType::coerce_str(
                &DynSolType::Array(Box::new(DynSolType::Array(Box::new(DynSolType::Bool)))),
                "[ [ true, true, false ], [ false]]"
            )
            .unwrap(),
            DynSolValue::Array(vec![
                DynSolValue::Array(vec![
                    DynSolValue::Bool(true),
                    DynSolValue::Bool(true),
                    DynSolValue::Bool(false)
                ]),
                DynSolValue::Array(vec![DynSolValue::Bool(false)])
            ])
        );
    }

    #[test]
    fn single_quoted_in_array_must_error() {
        assert!(DynSolType::Array(Box::new(DynSolType::Bool))
            .coerce_str("[true,\"false,false]")
            .is_err());
        assert!(DynSolType::Array(Box::new(DynSolType::Bool)).coerce_str("[false\"]").is_err());
        assert!(DynSolType::Array(Box::new(DynSolType::Bool))
            .coerce_str("[true,false\"]")
            .is_err());
        assert!(DynSolType::Array(Box::new(DynSolType::Bool))
            .coerce_str("[true,\"false\",false]")
            .is_err());
        assert!(DynSolType::Array(Box::new(DynSolType::Bool)).coerce_str("[true,false]").is_ok());
    }

    #[test]
    fn tuples() {
        let ty = DynSolType::Tuple(vec![DynSolType::String, DynSolType::Bool, DynSolType::String]);
        assert_eq!(
            ty.coerce_str("(\"a,]) b\", true, true? ]and] false!)").unwrap(),
            DynSolValue::Tuple(vec![
                DynSolValue::String("a,]) b".into()),
                DynSolValue::Bool(true),
                DynSolValue::String("true? ]and] false!".into()),
            ])
        );
        assert!(ty.coerce_str("(\"\", true, a, b)").is_err());
        assert!(ty.coerce_str("(a, b, true, a)").is_err());
    }

    #[test]
    fn tuples_arrays_mixed() {
        assert_eq!(
            DynSolType::Array(Box::new(DynSolType::Tuple(vec![
                DynSolType::Array(Box::new(DynSolType::Tuple(vec![DynSolType::Bool]))),
                DynSolType::Array(Box::new(DynSolType::Tuple(vec![
                    DynSolType::Bool,
                    DynSolType::Bool
                ]))),
            ])))
            .coerce_str("[([(true)],[(false,true)])]")
            .unwrap(),
            DynSolValue::Array(vec![DynSolValue::Tuple(vec![
                DynSolValue::Array(vec![DynSolValue::Tuple(vec![DynSolValue::Bool(true)])]),
                DynSolValue::Array(vec![DynSolValue::Tuple(vec![
                    DynSolValue::Bool(false),
                    DynSolValue::Bool(true)
                ])]),
            ])])
        );

        assert_eq!(
            DynSolType::Tuple(vec![
                DynSolType::Array(Box::new(DynSolType::Tuple(vec![DynSolType::Bool]))),
                DynSolType::Array(Box::new(DynSolType::Tuple(vec![
                    DynSolType::Bool,
                    DynSolType::Bool
                ]))),
            ])
            .coerce_str("([(true)],[(false,true)])")
            .unwrap(),
            DynSolValue::Tuple(vec![
                DynSolValue::Array(vec![DynSolValue::Tuple(vec![DynSolValue::Bool(true)])]),
                DynSolValue::Array(vec![DynSolValue::Tuple(vec![
                    DynSolValue::Bool(false),
                    DynSolValue::Bool(true)
                ])]),
            ])
        );
    }

    #[test]
    fn tuple_array_nested() {
        assert_eq!(
            DynSolType::Tuple(vec![
                DynSolType::Array(Box::new(DynSolType::Tuple(vec![DynSolType::Address]))),
                DynSolType::Uint(256),
            ])
            .coerce_str("([(5c9d55b78febcc2061715ba4f57ecf8ea2711f2c)],2)")
            .unwrap(),
            DynSolValue::Tuple(vec![
                DynSolValue::Array(vec![DynSolValue::Tuple(vec![DynSolValue::Address(address!(
                    "5c9d55b78febcc2061715ba4f57ecf8ea2711f2c"
                ))])]),
                DynSolValue::Uint(U256::from(2), 256),
            ])
        );
    }

    // keep `n` low to avoid stack overflows (debug mode)
    #[test]
    fn lotsa_array_nesting() {
        let n = 10;

        let mut ty = DynSolType::Bool;
        for _ in 0..n {
            ty = DynSolType::Array(Box::new(ty));
        }
        let mut value_str = String::new();
        value_str.push_str(&"[".repeat(n));
        value_str.push_str("true");
        value_str.push_str(&"]".repeat(n));

        let mut value = ty.coerce_str(&value_str).unwrap();
        for _ in 0..n {
            let DynSolValue::Array(arr) = value else { panic!("{value:?}") };
            assert_eq!(arr.len(), 1);
            value = arr.into_iter().next().unwrap();
        }
        assert_eq!(value, DynSolValue::Bool(true));
    }

    #[test]
    fn lotsa_tuple_nesting() {
        let n = 10;

        let mut ty = DynSolType::Bool;
        for _ in 0..n {
            ty = DynSolType::Tuple(vec![ty]);
        }
        let mut value_str = String::new();
        value_str.push_str(&"(".repeat(n));
        value_str.push_str("true");
        value_str.push_str(&")".repeat(n));

        let mut value = ty.coerce_str(&value_str).unwrap();
        for _ in 0..n {
            let DynSolValue::Tuple(tuple) = value else { panic!("{value:?}") };
            assert_eq!(tuple.len(), 1);
            value = tuple.into_iter().next().unwrap();
        }
        assert_eq!(value, DynSolValue::Bool(true));
    }
}
