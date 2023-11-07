use crate::{resolve::ResolveSolType, DynSolValue, DynToken, Error, Result, SolType, Word};
use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
use alloy_sol_type_parser::TypeSpecifier;
use alloy_sol_types::{abi::Decoder, sol_data};
use core::{fmt, iter::zip, num::NonZeroUsize, str::FromStr};

#[cfg(feature = "eip712")]
macro_rules! as_tuple {
    ($ty:ident $t:tt) => {
        $ty::Tuple($t) | $ty::CustomStruct { tuple: $t, .. }
    };
}
#[cfg(not(feature = "eip712"))]
macro_rules! as_tuple {
    ($ty:ident $t:tt) => {
        $ty::Tuple($t)
    };
}
pub(crate) use as_tuple;

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructProp {
    name: String,
    ty: DynSolType,
}

/// A dynamic Solidity type.
///
/// Equivalent to an enum wrapper around all implementers of [`SolType`].
///
/// This is used to represent Solidity types that are not known at compile time.
/// It is used in conjunction with [`DynToken`] and [`DynSolValue`] to allow for
/// dynamic ABI encoding and decoding.
///
/// # Examples
///
/// Parsing Solidity type strings:
///
/// ```
/// use alloy_dyn_abi::DynSolType;
///
/// let type_name = "(bool,address)[]";
/// let ty = DynSolType::parse(type_name)?;
/// assert_eq!(
///     ty,
///     DynSolType::Array(Box::new(DynSolType::Tuple(
///         vec![DynSolType::Bool, DynSolType::Address,]
///     )))
/// );
/// assert_eq!(ty.sol_type_name(), type_name);
///
/// // alternatively, you can use the FromStr impl
/// let ty2 = type_name.parse::<DynSolType>()?;
/// assert_eq!(ty, ty2);
/// # Ok::<_, alloy_dyn_abi::Error>(())
/// ```
///
/// Decoding dynamic types:
///
/// ```
/// use alloy_dyn_abi::{DynSolType, DynSolValue};
/// use alloy_primitives::U256;
///
/// let my_type = DynSolType::Uint(256);
/// let my_data: DynSolValue = U256::from(183u64).into();
///
/// let encoded = my_data.abi_encode();
/// let decoded = my_type.abi_decode(&encoded)?;
///
/// assert_eq!(decoded, my_data);
///
/// let my_type = DynSolType::Array(Box::new(my_type));
/// let my_data = DynSolValue::Array(vec![my_data.clone()]);
///
/// let encoded = my_data.abi_encode();
/// let decoded = my_type.abi_decode(&encoded)?;
///
/// assert_eq!(decoded, my_data);
/// # Ok::<_, alloy_dyn_abi::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynSolType {
    /// Boolean.
    Bool,
    /// Signed Integer.
    Int(usize),
    /// Unsigned Integer.
    Uint(usize),
    /// Fixed-size bytes, up to 32.
    FixedBytes(usize),
    /// Address.
    Address,
    /// Function.
    Function,

    /// Dynamic bytes.
    Bytes,
    /// String.
    String,

    /// Dynamically sized array.
    Array(Box<DynSolType>),
    /// Fixed-sized array.
    FixedArray(Box<DynSolType>, usize),
    /// Tuple.
    Tuple(Vec<DynSolType>),

    /// User-defined struct.
    #[cfg(feature = "eip712")]
    CustomStruct {
        /// Name of the struct.
        name: String,
        /// Prop names.
        prop_names: Vec<String>,
        /// Inner types.
        tuple: Vec<DynSolType>,
    },
}

impl fmt::Display for DynSolType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.sol_type_name())
    }
}

impl FromStr for DynSolType {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl DynSolType {
    /// Parses a Solidity type name string into a [`DynSolType`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_dyn_abi::DynSolType;
    /// let type_name = "uint256";
    /// let ty = DynSolType::parse(type_name)?;
    /// assert_eq!(ty, DynSolType::Uint(256));
    /// assert_eq!(ty.sol_type_name(), type_name);
    /// assert_eq!(ty.to_string(), type_name);
    ///
    /// // alternatively, you can use the FromStr impl
    /// let ty2 = type_name.parse::<DynSolType>()?;
    /// assert_eq!(ty2, ty);
    /// # Ok::<_, alloy_dyn_abi::Error>(())
    /// ```
    #[inline]
    pub fn parse(s: &str) -> Result<Self> {
        TypeSpecifier::parse(s).map_err(Error::TypeParser).and_then(|t| t.resolve())
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub fn as_tuple(&self) -> Option<&[Self]> {
        match self {
            Self::Tuple(t) => Some(t),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_custom_struct(&self) -> Option<(&str, &[String], &[Self])> {
        match self {
            #[cfg(feature = "eip712")]
            Self::CustomStruct { name, prop_names, tuple } => Some((name, prop_names, tuple)),
            _ => None,
        }
    }

    /// Returns whether this type is contains a custom struct.
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn has_custom_struct(&self) -> bool {
        #[cfg(feature = "eip712")]
        {
            match self {
                Self::CustomStruct { .. } => true,
                Self::Array(t) => t.has_custom_struct(),
                Self::FixedArray(t, _) => t.has_custom_struct(),
                Self::Tuple(t) => t.iter().any(Self::has_custom_struct),
                _ => false,
            }
        }
        #[cfg(not(feature = "eip712"))]
        {
            false
        }
    }

    /// Check that the given [`DynSolValue`]s match these types.
    ///
    /// See [`matches`](Self::matches) for more information.
    #[inline]
    pub fn matches_many(types: &[Self], values: &[DynSolValue]) -> bool {
        types.len() == values.len() && zip(types, values).all(|(t, v)| t.matches(v))
    }

    /// Check that the given [`DynSolValue`] matches this type.
    ///
    /// Note: this will not check any names, but just the types; e.g for
    /// `CustomStruct`, when the "eip712" feature is enabled, this will only
    /// check equality between the lengths and types of the tuple.
    pub fn matches(&self, value: &DynSolValue) -> bool {
        match self {
            Self::Bool => matches!(value, DynSolValue::Bool(_)),
            Self::Int(size) => matches!(value, DynSolValue::Int(_, s) if s == size),
            Self::Uint(size) => matches!(value, DynSolValue::Uint(_, s) if s == size),
            Self::FixedBytes(size) => matches!(value, DynSolValue::FixedBytes(_, s) if s == size),
            Self::Address => matches!(value, DynSolValue::Address(_)),
            Self::Function => matches!(value, DynSolValue::Function(_)),
            Self::Bytes => matches!(value, DynSolValue::Bytes(_)),
            Self::String => matches!(value, DynSolValue::String(_)),
            Self::Array(t) => {
                matches!(value, DynSolValue::Array(v) if v.iter().all(|v| t.matches(v)))
            }
            Self::FixedArray(t, size) => matches!(
                value,
                DynSolValue::FixedArray(v) if v.len() == *size && v.iter().all(|v| t.matches(v))
            ),
            Self::Tuple(types) => {
                matches!(value, as_tuple!(DynSolValue tuple) if zip(types, tuple).all(|(t, v)| t.matches(v)))
            }
            #[cfg(feature = "eip712")]
            Self::CustomStruct { name: _, prop_names, tuple } => {
                if let DynSolValue::CustomStruct { name: _, prop_names: p, tuple: t } = value {
                    // check just types
                    prop_names.len() == tuple.len()
                        && prop_names.len() == p.len()
                        && tuple.len() == t.len()
                        && zip(tuple, t).all(|(a, b)| a.matches(b))
                } else if let DynSolValue::Tuple(v) = value {
                    zip(v, tuple).all(|(v, t)| t.matches(v))
                } else {
                    false
                }
            }
        }
    }

    /// Dynamic detokenization.
    // This should not fail when using a token created by `Self::empty_dyn_token`.
    #[allow(clippy::unnecessary_to_owned)] // https://github.com/rust-lang/rust-clippy/issues/8148
    pub fn detokenize(&self, token: DynToken<'_>) -> Result<DynSolValue> {
        match (self, token) {
            (Self::Bool, DynToken::Word(word)) => {
                Ok(DynSolValue::Bool(sol_data::Bool::detokenize(word.into())))
            }

            // cheating here, but it's ok
            (Self::Int(size), DynToken::Word(word)) => {
                Ok(DynSolValue::Int(sol_data::Int::<256>::detokenize(word.into()), *size))
            }

            (Self::Uint(size), DynToken::Word(word)) => {
                Ok(DynSolValue::Uint(sol_data::Uint::<256>::detokenize(word.into()), *size))
            }

            (Self::FixedBytes(size), DynToken::Word(word)) => Ok(DynSolValue::FixedBytes(
                sol_data::FixedBytes::<32>::detokenize(word.into()),
                *size,
            )),

            (Self::Address, DynToken::Word(word)) => {
                Ok(DynSolValue::Address(sol_data::Address::detokenize(word.into())))
            }

            (Self::Function, DynToken::Word(word)) => {
                Ok(DynSolValue::Function(sol_data::Function::detokenize(word.into())))
            }

            (Self::Bytes, DynToken::PackedSeq(buf)) => Ok(DynSolValue::Bytes(buf.to_vec())),

            (Self::String, DynToken::PackedSeq(buf)) => {
                Ok(DynSolValue::String(sol_data::String::detokenize(buf.into())))
            }

            (Self::Array(t), DynToken::DynSeq { contents, .. }) => {
                t.detokenize_array(contents.into_owned()).map(DynSolValue::Array)
            }

            (Self::FixedArray(t, size), DynToken::FixedSeq(tokens, _)) => {
                if *size != tokens.len() {
                    return Err(crate::Error::custom(
                        "array length mismatch on dynamic detokenization",
                    ));
                }
                t.detokenize_array(tokens.into_owned()).map(DynSolValue::FixedArray)
            }

            (Self::Tuple(types), DynToken::FixedSeq(tokens, _)) => {
                if types.len() != tokens.len() {
                    return Err(crate::Error::custom(
                        "tuple length mismatch on dynamic detokenization",
                    ));
                }
                Self::detokenize_many(types, tokens.into_owned()).map(DynSolValue::Tuple)
            }

            #[cfg(feature = "eip712")]
            (Self::CustomStruct { name, tuple, prop_names }, DynToken::FixedSeq(tokens, len)) => {
                if len != tokens.len() || len != tuple.len() {
                    return Err(crate::Error::custom(
                        "custom length mismatch on dynamic detokenization",
                    ));
                }
                Self::detokenize_many(tuple, tokens.into_owned()).map(|tuple| {
                    DynSolValue::CustomStruct {
                        name: name.clone(),
                        prop_names: prop_names.clone(),
                        tuple,
                    }
                })
            }

            _ => Err(crate::Error::custom("mismatched types on dynamic detokenization")),
        }
    }

    fn detokenize_array(&self, tokens: Vec<DynToken<'_>>) -> Result<Vec<DynSolValue>> {
        let mut values = Vec::with_capacity(tokens.len());
        for token in tokens {
            values.push(self.detokenize(token)?);
        }
        Ok(values)
    }

    fn detokenize_many(types: &[Self], tokens: Vec<DynToken<'_>>) -> Result<Vec<DynSolValue>> {
        assert_eq!(types.len(), tokens.len());
        let mut values = Vec::with_capacity(tokens.len());
        for (ty, token) in zip(types, tokens) {
            values.push(ty.detokenize(token)?);
        }
        Ok(values)
    }

    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    fn sol_type_name_simple(&self) -> Option<&'static str> {
        match self {
            Self::Address => Some("address"),
            Self::Function => Some("function"),
            Self::Bool => Some("bool"),
            Self::Bytes => Some("bytes"),
            Self::String => Some("string"),
            _ => None,
        }
    }

    #[inline]
    fn sol_type_name_raw(&self, out: &mut String) {
        match self {
            Self::Address | Self::Function | Self::Bool | Self::Bytes | Self::String => {
                out.push_str(unsafe { self.sol_type_name_simple().unwrap_unchecked() });
            }

            Self::FixedBytes(size) | Self::Int(size) | Self::Uint(size) => {
                let prefix = match self {
                    Self::FixedBytes(..) => "bytes",
                    Self::Int(..) => "int",
                    Self::Uint(..) => "uint",
                    _ => unreachable!(),
                };
                out.push_str(prefix);
                out.push_str(itoa::Buffer::new().format(*size));
            }

            as_tuple!(Self tuple) => {
                out.push('(');
                for (i, val) in tuple.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    val.sol_type_name_raw(out);
                }
                if tuple.len() == 1 {
                    out.push(',');
                }
                out.push(')');
            }
            Self::Array(t) => {
                t.sol_type_name_raw(out);
                out.push_str("[]");
            }
            Self::FixedArray(t, len) => {
                t.sol_type_name_raw(out);
                out.push('[');
                out.push_str(itoa::Buffer::new().format(*len));
                out.push(']');
            }
        }
    }

    /// Returns an estimate of the number of bytes needed to format this type.
    ///
    /// This calculation is meant to be an upper bound for valid types to avoid
    /// a second allocation in `sol_type_name_raw` and thus is almost never
    /// going to be exact.
    fn sol_type_name_capacity(&self) -> usize {
        match self {
            | Self::Address // 7
            | Self::Function // 8
            | Self::Bool // 4
            | Self::Bytes // 5
            | Self::String // 6
            | Self::FixedBytes(_) // 5 + 2
            | Self::Int(_) // 3 + 3
            | Self::Uint(_) // 4 + 3
            => 8,

            | Self::Array(t) // t + 2
            | Self::FixedArray(t, _) // t + 2 + log10(len)
            => t.sol_type_name_capacity() + 8,

            as_tuple!(Self tuple) // sum(tuple) + len(tuple) + 2
            => tuple.iter().map(Self::sol_type_name_capacity).sum::<usize>() + 8,
        }
    }

    /// The Solidity type name. This returns the Solidity type corresponding to
    /// this value, if it is known. A type will not be known if the value
    /// contains an empty sequence, e.g. `T[0]`.
    pub fn sol_type_name(&self) -> Cow<'static, str> {
        if let Some(s) = self.sol_type_name_simple() {
            Cow::Borrowed(s)
        } else {
            let mut s = String::with_capacity(self.sol_type_name_capacity());
            self.sol_type_name_raw(&mut s);
            Cow::Owned(s)
        }
    }

    /// The Solidity type name, as a `String`.
    ///
    /// Note: this shadows the inherent [`ToString`] implementation, derived
    /// from [`fmt::Display`], for performance reasons.
    #[inline]
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.sol_type_name().into_owned()
    }

    /// Instantiate an empty dyn token, to be decoded into.
    pub(crate) fn empty_dyn_token<'a>(&self) -> DynToken<'a> {
        match self {
            Self::Address
            | Self::Function
            | Self::Bool
            | Self::FixedBytes(_)
            | Self::Int(_)
            | Self::Uint(_) => DynToken::Word(Word::ZERO),

            Self::Bytes | Self::String => DynToken::PackedSeq(&[]),

            Self::Array(t) => DynToken::DynSeq {
                contents: Default::default(),
                template: Some(Box::new(t.empty_dyn_token())),
            },
            &Self::FixedArray(ref t, size) => {
                DynToken::FixedSeq(vec![t.empty_dyn_token(); size].into(), size)
            }
            as_tuple!(Self tuple) => DynToken::FixedSeq(
                tuple.iter().map(DynSolType::empty_dyn_token).collect(),
                tuple.len(),
            ),
        }
    }

    /// Decode an event topic into a [`DynSolValue`].
    pub(crate) fn decode_event_topic(&self, topic: Word) -> DynSolValue {
        match self {
            Self::Address
            | Self::Function
            | Self::Bool
            | Self::FixedBytes(_)
            | Self::Int(_)
            | Self::Uint(_) => self.detokenize(DynToken::Word(topic)).unwrap(),
            _ => DynSolValue::FixedBytes(topic, 32),
        }
    }

    /// Decode a [`DynSolValue`] from a byte slice. Fails if the value does not
    /// match this type.
    ///
    /// This method is used for decoding single values. It assumes the `data`
    /// argument is an encoded single-element sequence wrapping the `self` type.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn abi_decode(&self, data: &[u8]) -> Result<DynSolValue> {
        self.abi_decode_inner(&mut Decoder::new(data, false), DynToken::decode_single_populate)
    }

    /// Decode a [`DynSolValue`] from a byte slice. Fails if the value does not
    /// match this type.
    ///
    /// This method is used for decoding function arguments. It tries to
    /// determine whether the user intended to decode a sequence or an
    /// individual value. If the `self` type is a tuple, the `data` will be
    /// decoded as a sequence, otherwise it will be decoded as a single value.
    ///
    /// # Examples
    ///
    /// ```solidity
    /// // This function takes a single simple param:
    /// // DynSolType::Uint(256).decode_params(data)
    /// function myFunc(uint256 a) public;
    ///
    /// // This function takes 2 params:
    /// // DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Bool])
    /// //     .decode_params(data)
    /// function myFunc(uint256 b, bool c) public;
    /// ```
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn abi_decode_params(&self, data: &[u8]) -> Result<DynSolValue> {
        match self {
            Self::Tuple(_) => self.abi_decode_sequence(data),
            _ => self.abi_decode(data),
        }
    }

    /// Decode a [`DynSolValue`] from a byte slice. Fails if the value does not
    /// match this type.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn abi_decode_sequence(&self, data: &[u8]) -> Result<DynSolValue> {
        self.abi_decode_inner(&mut Decoder::new(data, false), DynToken::decode_sequence_populate)
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn abi_decode_inner<'d, F>(
        &self,
        decoder: &mut Decoder<'d>,
        f: F,
    ) -> Result<DynSolValue>
    where
        F: FnOnce(&mut DynToken<'d>, &mut Decoder<'d>) -> Result<()>,
    {
        let mut token = self.empty_dyn_token();
        f(&mut token, decoder)?;
        let value = self.detokenize(token).expect("invalid empty_dyn_token");
        debug_assert!(
            self.matches(&value),
            "decoded value does not match type:\n  type: {self:?}\n value: {value:?}"
        );
        Ok(value)
    }

    /// Wrap in an array of the specified size
    #[inline]
    pub(crate) fn array_wrap(self, size: Option<NonZeroUsize>) -> Self {
        match size {
            Some(size) => Self::FixedArray(Box::new(self), size.get()),
            None => Self::Array(Box::new(self)),
        }
    }

    /// Iteratively wrap in arrays.
    #[inline]
    pub(crate) fn array_wrap_from_iter(
        self,
        iter: impl IntoIterator<Item = Option<NonZeroUsize>>,
    ) -> Self {
        iter.into_iter().fold(self, Self::array_wrap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{hex, Address};

    #[test]
    fn dynamically_encodes() {
        let word1 =
            "0000000000000000000000000101010101010101010101010101010101010101".parse().unwrap();
        let word2 =
            "0000000000000000000000000202020202020202020202020202020202020202".parse().unwrap();

        let val = DynSolValue::Address(Address::repeat_byte(0x01));
        let token = val.tokenize();
        assert_eq!(token, DynToken::from(word1));

        let val = DynSolValue::FixedArray(vec![
            Address::repeat_byte(0x01).into(),
            Address::repeat_byte(0x02).into(),
        ]);

        let token = val.tokenize();
        assert_eq!(
            token,
            DynToken::FixedSeq(vec![DynToken::Word(word1), DynToken::Word(word2)].into(), 2)
        );
        let mut enc = crate::Encoder::default();
        DynSolValue::encode_seq_to(val.as_fixed_seq().unwrap(), &mut enc);
        assert_eq!(enc.finish(), vec![word1, word2]);
    }

    // also tests the type name parser
    macro_rules! encoder_tests {
        ($($name:ident($ty:literal, $encoded:literal)),* $(,)?) => {$(
            #[test]
            fn $name() {
                encoder_test($ty, &hex!($encoded));
            }
        )*};
    }

    fn encoder_test(s: &str, encoded: &[u8]) {
        let t: DynSolType = s.parse().expect("parsing failed");
        assert_eq!(t.sol_type_name(), s, "type names are not the same");

        let dec = t.abi_decode_params(encoded).expect("decoding failed");
        if let Some(value_name) = dec.sol_type_name() {
            assert_eq!(value_name, s, "value names are not the same");
        }

        // Tuples are treated as top-level lists. So if we encounter a
        // dynamic tuple, the total length of the encoded data will include
        // the offset, but the encoding/decoding process will not. To
        // account for this, we add 32 bytes to the expected length when
        // the type is a dynamic tuple.
        let mut len = encoded.len();
        if dec.as_tuple().is_some() && dec.is_dynamic() {
            len += 32;
        }
        assert_eq!(dec.total_words() * 32, len, "dyn_tuple={}", len != encoded.len());

        let re_encoded = dec.abi_encode_params();
        assert_eq!(re_encoded, encoded);
    }

    encoder_tests! {
        address("address", "0000000000000000000000001111111111111111111111111111111111111111"),

        dynamic_array_of_addresses("address[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        fixed_array_of_addresses("address[2]", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        two_addresses("(address,address)", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        fixed_array_of_dynamic_arrays_of_addresses("address[][2]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        dynamic_array_of_fixed_arrays_of_addresses("address[2][]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        dynamic_array_of_dynamic_arrays("address[][]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        dynamic_array_of_dynamic_arrays2("address[][]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        fixed_array_of_fixed_arrays("address[2][2]", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        fixed_array_of_static_tuples_followed_by_dynamic_type("((uint256,uint256,address)[2],string)", "
                0000000000000000000000000000000000000000000000000000000005930cc5
                0000000000000000000000000000000000000000000000000000000015002967
                0000000000000000000000004444444444444444444444444444444444444444
                000000000000000000000000000000000000000000000000000000000000307b
                00000000000000000000000000000000000000000000000000000000000001c3
                0000000000000000000000002222222222222222222222222222222222222222
                00000000000000000000000000000000000000000000000000000000000000e0
                0000000000000000000000000000000000000000000000000000000000000009
                6761766f66796f726b0000000000000000000000000000000000000000000000
            "),

        empty_array("address[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
        "),

        empty_array_2("(address[],address[])", "
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000060
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
        "),

        // Nested empty arrays
        empty_array_3("(address[][],address[][])", "
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
        "),

        fixed_bytes("bytes2", "1234000000000000000000000000000000000000000000000000000000000000"),

        string("string", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
        "),

        bytes("bytes", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            1234000000000000000000000000000000000000000000000000000000000000
        "),

        bytes_2("bytes", "
            0000000000000000000000000000000000000000000000000000000000000020
            000000000000000000000000000000000000000000000000000000000000001f
            1000000000000000000000000000000000000000000000000000000000000200
        "),

        bytes_3("bytes", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            1000000000000000000000000000000000000000000000000000000000000000
            1000000000000000000000000000000000000000000000000000000000000000
        "),

        two_bytes("(bytes,bytes)", "
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            000000000000000000000000000000000000000000000000000000000000001f
            1000000000000000000000000000000000000000000000000000000000000200
            0000000000000000000000000000000000000000000000000000000000000020
            0010000000000000000000000000000000000000000000000000000000000002
        "),

        uint("uint256", "0000000000000000000000000000000000000000000000000000000000000004"),

        int("int256", "0000000000000000000000000000000000000000000000000000000000000004"),

        bool("bool", "0000000000000000000000000000000000000000000000000000000000000001"),

        bool2("bool", "0000000000000000000000000000000000000000000000000000000000000000"),

        comprehensive_test("(uint8,bytes,uint8,bytes)", "
            0000000000000000000000000000000000000000000000000000000000000005
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000003
            00000000000000000000000000000000000000000000000000000000000000e0
            0000000000000000000000000000000000000000000000000000000000000040
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
            0000000000000000000000000000000000000000000000000000000000000040
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
            131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
        "),

        comprehensive_test2("(bool,string,uint8,uint8,uint8,uint8[])", "
            0000000000000000000000000000000000000000000000000000000000000001
            00000000000000000000000000000000000000000000000000000000000000c0
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000003
            0000000000000000000000000000000000000000000000000000000000000004
            0000000000000000000000000000000000000000000000000000000000000100
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000003
            0000000000000000000000000000000000000000000000000000000000000005
            0000000000000000000000000000000000000000000000000000000000000006
            0000000000000000000000000000000000000000000000000000000000000007
        "),

        dynamic_array_of_bytes("bytes[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000026
            019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a421
            8a274f00d6070000000000000000000000000000000000000000000000000000
        "),

        dynamic_array_of_bytes2("bytes[]", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000026
            4444444444444444444444444444444444444444444444444444444444444444
            4444444444440000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000026
            6666666666666666666666666666666666666666666666666666666666666666
            6666666666660000000000000000000000000000000000000000000000000000
        "),

        static_tuple_of_addresses("(address,address)", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
        "),

        dynamic_tuple("((string,string),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
        "),

        dynamic_tuple_of_bytes("((bytes,bytes),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000040
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000026
            4444444444444444444444444444444444444444444444444444444444444444
            4444444444440000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000026
            6666666666666666666666666666666666666666666666666666666666666666
            6666666666660000000000000000000000000000000000000000000000000000
        "),

        complex_tuple("((uint256,string,address,address),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            1111111111111111111111111111111111111111111111111111111111111111
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000009
            6761766f66796f726b0000000000000000000000000000000000000000000000
        "),

        nested_tuple("((string,bool,string,(string,string,(string,string))),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000001
            00000000000000000000000000000000000000000000000000000000000000c0
            0000000000000000000000000000000000000000000000000000000000000100
            0000000000000000000000000000000000000000000000000000000000000004
            7465737400000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000006
            6379626f72670000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000060
            00000000000000000000000000000000000000000000000000000000000000a0
            00000000000000000000000000000000000000000000000000000000000000e0
            0000000000000000000000000000000000000000000000000000000000000005
            6e69676874000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000003
            6461790000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000040
            0000000000000000000000000000000000000000000000000000000000000080
            0000000000000000000000000000000000000000000000000000000000000004
            7765656500000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000008
            66756e7465737473000000000000000000000000000000000000000000000000
        "),

        params_containing_dynamic_tuple("(address,(bool,string,string),address,address,bool)", "
            0000000000000000000000002222222222222222222222222222222222222222
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000060
            00000000000000000000000000000000000000000000000000000000000000a0
            0000000000000000000000000000000000000000000000000000000000000009
            7370616365736869700000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000006
            6379626f72670000000000000000000000000000000000000000000000000000
        "),

        params_containing_static_tuple("(address,(address,bool,bool),address,address)", "
            0000000000000000000000001111111111111111111111111111111111111111
            0000000000000000000000002222222222222222222222222222222222222222
            0000000000000000000000000000000000000000000000000000000000000001
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000003333333333333333333333333333333333333333
            0000000000000000000000004444444444444444444444444444444444444444
        "),

        dynamic_tuple_with_nested_static_tuples("((((bool,uint16),),uint16[]),)", "
            0000000000000000000000000000000000000000000000000000000000000020
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000777
            0000000000000000000000000000000000000000000000000000000000000060
            0000000000000000000000000000000000000000000000000000000000000002
            0000000000000000000000000000000000000000000000000000000000000042
            0000000000000000000000000000000000000000000000000000000000001337
        "),
    }
}
