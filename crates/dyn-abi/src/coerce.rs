use crate::{ty::as_tuple, DynSolType, DynSolValue, Result};
use alloy_primitives::{Address, FixedBytes, Function, Sign, I256, U256};
use alloy_sol_type_parser::utils::{array_parser, char_parser, spanned};
use alloy_sol_types::Word;
use core::fmt;
use hex::FromHexError;
use winnow::{
    ascii::{alpha0, alpha1, digit1, hex_digit1, space0},
    combinator::{cut_err, dispatch, fail, opt, preceded, success},
    error::{ContextError, ErrMode, ErrorKind, FromExternalError, ParserError, StrContext},
    token::take_while,
    trace::trace,
    PResult, Parser,
};

impl DynSolType {
    /// Coerces a string into a [`DynSolValue`] via this type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_dyn_abi::{DynSolType, DynSolValue};
    /// # use alloy_primitives::U256;
    /// let ty = "(uint256,string)[]".parse::<DynSolType>()?;
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
        self.value_parser()
            .parse(s)
            .map_err(|e| crate::Error::TypeParser(alloy_sol_type_parser::Error::parser(e)))
    }

    fn value_parser<'i: 't, 't>(&'t self) -> impl Parser<&'i str, DynSolValue, ContextError> + 't {
        #[cfg(feature = "debug")]
        let name = self.sol_type_name();
        #[cfg(not(feature = "debug"))]
        let name = "type";
        trace(name, move |input: &mut &str| match self {
            Self::Bool => bool(input).map(DynSolValue::Bool),
            &Self::Int(size) => int(size)
                .parse_next(input)
                .map(|int| DynSolValue::Int(int, size)),
            &Self::Uint(size) => uint(size)
                .parse_next(input)
                .map(|uint| DynSolValue::Uint(uint, size)),
            &Self::FixedBytes(size) => fixed_bytes(size)
                .parse_next(input)
                .map(|word| DynSolValue::FixedBytes(word, size)),
            Self::Address => address(input).map(DynSolValue::Address),
            Self::Function => function(input).map(DynSolValue::Function),
            Self::Bytes => bytes(input).map(DynSolValue::Bytes),
            Self::String => string(input).map(DynSolValue::String),
            Self::Array(ty) => array(ty).parse_next(input).map(DynSolValue::Array),
            Self::FixedArray(ty, len) => fixed_array(ty, *len)
                .parse_next(input)
                .map(DynSolValue::Array),
            as_tuple!(Self tys) => tuple(tys).parse_next(input).map(DynSolValue::Tuple),
        })
    }
}

#[derive(Debug)]
enum Error {
    IntOverflow,
    FractionalNotAllowed(f64),
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
            Self::TooManyDecimals(expected, actual) => {
                write!(f, "too many decimals: {actual} > {expected}")
            }
            Self::FractionalNotAllowed(n) => write!(
                f,
                "non-zero fraction {n} not allowed without specifying units (gwei, ether, etc.)"
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
        dispatch! {alpha1;
            "true" => success(true),
            "false" => success(false),
            _ => fail,
        },
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
        (sign, uint(size)).try_map(|(sign, abs)| {
            I256::checked_from_sign_and_abs(sign, abs).ok_or(Error::IntOverflow)
        }),
    )
}

#[inline]
fn sign(input: &mut &str) -> PResult<Sign> {
    trace("sign", |input: &mut &str| match input.as_bytes().first() {
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
        let (s, ((), fract)) =
            spanned((uint_integer, opt(preceded('.', cut_err(digit1))))).parse_next(input)?;

        let _ = space0(input)?;
        let units = uint_units(input)?;

        let uint = if let Some(fract) = fract {
            let x = s
                .parse::<f64>()
                .map_err(|e| ErrMode::from_external_error(input, ErrorKind::Verify, e))?;

            if units == 0 && x.fract() != 0.0 {
                return Err(ErrMode::from_external_error(
                    input,
                    ErrorKind::Verify,
                    Error::FractionalNotAllowed(x.fract()),
                ))
            }

            if fract.len() > units {
                return Err(ErrMode::from_external_error(
                    input,
                    ErrorKind::Verify,
                    Error::TooManyDecimals(units, fract.len()),
                ))
            }

            U256::try_from(x * 10f64.powi(units as i32))
                .map_err(|e| ErrMode::from_external_error(input, ErrorKind::Verify, e))
        } else {
            s.parse::<U256>()
                .map_err(|e| ErrMode::from_external_error(input, ErrorKind::Verify, e))?
                .checked_mul(U256::from(10usize.pow(units as u32)))
                .ok_or_else(|| {
                    ErrMode::from_external_error(input, ErrorKind::Verify, Error::IntOverflow)
                })
        }?;

        if uint.bit_len() > len {
            return Err(ErrMode::from_external_error(
                input,
                ErrorKind::Verify,
                Error::IntOverflow,
            ))
        }

        Ok(uint)
    })
}

#[inline]
fn uint_integer(input: &mut &str) -> PResult<()> {
    let has_prefix = matches!(
        input.get(..2),
        Some("0b" | "0B" | "0o" | "0O" | "0x" | "0X")
    );
    if has_prefix {
        *input = &input[2..];
        hex_digit1(input)
    } else {
        digit1(input)
    }
    .map(drop)
}

#[inline]
fn uint_units(input: &mut &str) -> PResult<usize> {
    alpha0(input).and_then(|s| {
        Ok(match s {
            "ether" => 18,
            "gwei" | "nano" | "nanoether" => 9,
            "" | "wei" => 0,
            _ => return Err(ErrMode::from_error_kind(input, ErrorKind::Fail)),
        })
    })
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
            ))
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
    trace("address", fixed_bytes_inner)
        .parse_next(input)
        .map(Address::from)
}

#[inline]
fn function(input: &mut &str) -> PResult<Function> {
    trace("function", fixed_bytes_inner)
        .parse_next(input)
        .map(Function::from)
}

#[inline]
fn bytes(input: &mut &str) -> PResult<Vec<u8>> {
    trace("bytes", hex_str)
        .parse_next(input)
        .map(|s| hex::decode(s).unwrap())
}

#[inline]
fn string(input: &mut &str) -> PResult<String> {
    trace("string", string_inner)
        .parse_next(input)
        .map(String::from)
}

#[inline]
fn string_inner<'i>(input: &mut &'i str) -> PResult<&'i str> {
    let Some(delim) = input.chars().next() else {
        return Ok("")
    };
    let has_delim = matches!(delim, '"' | '\'');
    if has_delim {
        *input = &input[1..];
    }

    // TODO: escapes?
    let min = if has_delim { 0 } else { 1 };
    let until_ch = if has_delim {
        core::slice::from_ref(&delim)
    } else {
        &[',', ')', ']']
    };
    let mut s = take_while(min.., |ch: char| !until_ch.contains(&ch)).parse_next(input)?;

    if has_delim {
        cut_err(char_parser(delim))
            .context(StrContext::Label("string"))
            .parse_next(input)?;
    } else {
        s = s.trim_end();
    }

    Ok(s)
}

#[inline]
fn array<'i: 't, 't>(
    ty: &'t DynSolType,
) -> impl Parser<&'i str, Vec<DynSolValue>, ContextError> + 't {
    trace("array", array_parser(ty.value_parser()))
}

#[inline]
fn fixed_array<'i: 't, 't>(
    ty: &'t DynSolType,
    len: usize,
) -> impl Parser<&'i str, Vec<DynSolValue>, ContextError> + 't {
    trace(
        "fixed_array",
        array(ty).try_map(move |values| {
            if values.len() == len {
                Ok(values)
            } else {
                Err(Error::FixedArrayLengthMismatch(len, values.len()))
            }
        }),
    )
}

#[inline]
fn tuple<'i: 't, 't>(
    tuple: &'t Vec<DynSolType>,
) -> impl Parser<&'i str, Vec<DynSolValue>, ContextError> + 't {
    trace("tuple", move |input: &mut &'i str| {
        space0(input)?;
        char_parser('(').parse_next(input)?;

        let mut values = Vec::with_capacity(tuple.len());
        for (i, ty) in tuple.iter().enumerate() {
            if i > 0 {
                space0(input)?;
                char_parser(',').parse_next(input)?;
            }
            space0(input)?;
            values.push(ty.value_parser().parse_next(input)?);
        }

        space0(input)?;
        char_parser(')').parse_next(input)?;

        Ok(values)
    })
}

#[inline]
fn fixed_bytes_inner<const N: usize>(input: &mut &str) -> PResult<FixedBytes<N>> {
    let s = hex_str(input)?;
    let mut out = FixedBytes::ZERO;
    match hex::decode_to_slice(s, out.as_mut_slice()) {
        Ok(()) => Ok(out),
        Err(e) => Err(hex_error(input, e)),
    }
}

#[inline]
fn hex_str<'i>(input: &mut &'i str) -> PResult<&'i str> {
    trace("hex_str", preceded(opt("0x"), hex_digit1)).parse_next(input)
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
    use core::str::FromStr;

    #[test]
    fn coerce_bool() {
        assert_eq!(
            DynSolType::Bool.coerce_str("true").unwrap(),
            DynSolValue::Bool(true)
        );
        assert_eq!(
            DynSolType::Bool.coerce_str("false").unwrap(),
            DynSolValue::Bool(false)
        );
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

        assert_eq!(
            DynSolType::Uint(256).coerce_str("0.1 gwei").unwrap(),
            DynSolValue::Uint(U256::from_str("100000000").unwrap(), 256)
        );
    }

    #[test]
    fn coerce_uint_ether() {
        assert_eq!(
            DynSolType::Uint(256)
                .coerce_str("10000000000ether")
                .unwrap(),
            DynSolValue::Uint(
                U256::from_str("10000000000000000000000000000").unwrap(),
                256
            )
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("1ether").unwrap(),
            DynSolValue::Uint(U256::from_str("1000000000000000000").unwrap(), 256)
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("0.01 ether").unwrap(),
            DynSolValue::Uint(U256::from_str("10000000000000000").unwrap(), 256)
        );

        assert_eq!(
            DynSolType::Uint(256)
                .coerce_str("0.000000000000000001ether")
                .unwrap(),
            DynSolValue::Uint(U256::from(1), 256)
        );

        assert_eq!(
            DynSolType::Uint(256).coerce_str("0.000000000000000001ether"),
            DynSolType::Uint(256).coerce_str("1wei"),
        );
    }

    #[test]
    fn coerce_uint_array_ether() {
        assert_eq!(
            DynSolType::Array(Box::new(DynSolType::Uint(256)))
                .coerce_str("[1ether,0.1 ether]")
                .unwrap(),
            DynSolValue::Array(vec![
                DynSolValue::Uint(U256::from_str("1000000000000000000").unwrap(), 256),
                DynSolValue::Uint(U256::from_str("100000000000000000").unwrap(), 256),
            ])
        );
    }

    #[test]
    fn coerce_uint_invalid_units() {
        assert!(DynSolType::Uint(256).coerce_str("0.1 wei").is_err());

        // 0.1 wei
        assert!(DynSolType::Uint(256)
            .coerce_str("0.0000000000000000001ether")
            .is_err());

        // 1 ether + 0.1 wei
        assert!(DynSolType::Uint(256)
            .coerce_str("1.0000000000000000001ether")
            .is_err());

        // 1_000_000_000 ether + 0.1 wei
        assert!(DynSolType::Uint(256)
            .coerce_str("1000000000.0000000000000000001ether")
            .is_err());

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
        assert_eq!(
            DynSolType::Address
                .coerce_str("0000000000000000000000000000000000000000")
                .unwrap(),
            DynSolValue::Address(Address::ZERO)
        );
        assert_eq!(
            DynSolType::Address
                .coerce_str("0x1111111111111111111111111111111111111111")
                .unwrap(),
            DynSolValue::Address(Address::new([0x11; 20]))
        );
        assert_eq!(
            DynSolType::Address
                .coerce_str("2222222222222222222222222222222222222222")
                .unwrap(),
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

        assert!(DynSolType::String.coerce_str("\"hello world").is_err());
        assert!(DynSolType::String.coerce_str("\"hello world'").is_err());
        assert!(DynSolType::String.coerce_str("'hello world").is_err());
        assert!(DynSolType::String.coerce_str("'hello world\"").is_err());
    }

    #[test]
    fn coerce_strings() {
        let arr = DynSolType::Array(Box::new(DynSolType::String));
        let mk_arr = |s: &[&str]| {
            DynSolValue::Array(
                s.iter()
                    .map(|s| DynSolValue::String(s.to_string()))
                    .collect(),
            )
        };

        assert_eq!(arr.coerce_str("[]").unwrap(), mk_arr(&[]));
        assert_eq!(arr.coerce_str("[    ]").unwrap(), mk_arr(&[]));

        // TODO: should this be an error?
        // assert!(arr.coerce_str("[,]").is_err());
        // assert!(arr.coerce_str("[ , ]").is_err());

        assert_eq!(arr.coerce_str("[ foo bar ]").unwrap(), mk_arr(&["foo bar"]));
        assert_eq!(arr.coerce_str("[foo bar,]").unwrap(), mk_arr(&["foo bar"]));
        assert_eq!(
            arr.coerce_str("[  foo bar,  ]").unwrap(),
            mk_arr(&["foo bar"])
        );
        assert_eq!(
            arr.coerce_str("[ foo , bar ]").unwrap(),
            mk_arr(&["foo", "bar"])
        )
    }

    #[test]
    fn coerce_empty_array() {
        assert_eq!(
            DynSolType::Array(Box::new(DynSolType::Bool))
                .coerce_str("[]")
                .unwrap(),
            DynSolValue::Array(vec![])
        );
    }

    #[test]
    fn coerce_bool_array() {
        assert_eq!(
            DynSolType::coerce_str(
                &DynSolType::Array(Box::new(DynSolType::Bool)),
                "[true, false]"
            )
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
}
