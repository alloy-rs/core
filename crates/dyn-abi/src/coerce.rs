use crate::{ty::as_tuple, DynSolType, DynSolValue, Result};
use alloy_primitives::{Address, FixedBytes, Function, Sign, B256, I256, U256};
use alloy_sol_type_parser::utils::{array_parser, char_parser, spanned};
use core::fmt;
use hex::FromHexError;
use winnow::{
    ascii::{alpha0, alpha1, hex_digit1, space0},
    combinator::{delimited, dispatch, fail, opt, preceded, success},
    error::{ContextError, ErrMode, ErrorKind, FromExternalError},
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
    FixedBytesLengthMismatch(usize),
    FixedArrayLengthMismatch(usize, usize),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntOverflow => f.write_str("number too large to fit in target type"),
            Self::FixedBytesLengthMismatch(expected) => write!(
                f,
                "fixed bytes length mismatch: got more than {expected} bytes"
            ),
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
    trace(
        name,
        spanned((uint_prefix, hex_digit1))
            .try_map(|(s, _)| s.parse::<U256>())
            .try_map(move |uint| {
                if uint.bit_len() <= len {
                    Ok(uint)
                } else {
                    Err(Error::IntOverflow)
                }
            }),
    )
}

#[inline]
fn uint_prefix<'i>(input: &mut &'i str) -> PResult<()> {
    if matches!(
        input.get(..2),
        Some("0b" | "0B" | "0o" | "0O" | "0x" | "0X")
    ) {
        *input = &input[2..];
    }
    Ok(())
}

#[inline]
fn fixed_bytes<'i>(size: usize) -> impl Parser<&'i str, B256, ContextError> {
    #[cfg(feature = "debug")]
    let name = format!("bytes{size}");
    #[cfg(not(feature = "debug"))]
    let name = "bytesN";
    trace(
        name,
        fixed_bytes_inner.try_map(move |word| {
            if let Some(sl) = word.get(size..) {
                if !sl.iter().all(|x| *x == 0) {
                    return Err(Error::FixedBytesLengthMismatch(size))
                }
            }
            Ok(word)
        }),
    )
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
    trace("string", delimited(opt('"'), alpha0, opt('"')))
        .parse_next(input)
        .map(String::from)
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
