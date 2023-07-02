use crate::{
    no_std_prelude::{Cow, *},
    DynSolType, DynToken, Word,
};
use alloy_primitives::{Address, I256, U256};
use alloy_sol_types::Encoder;

/// This type represents a Solidity value that has been decoded into rust. It
/// is broadly similar to `serde_json::Value` in that it is an enum of possible
/// types, and the user must inspect and disambiguate.
#[derive(Debug, Clone, PartialEq)]
pub enum DynSolValue {
    /// An address.
    Address(Address),
    /// A boolean.
    Bool(bool),
    /// A dynamic-length byte array.
    Bytes(Vec<u8>),
    /// A fixed-length byte string.
    FixedBytes(Word, usize),
    /// A signed integer.
    Int(I256, usize),
    /// An unsigned integer.
    Uint(U256, usize),
    /// A string.
    String(String),
    /// A tuple of values.
    Tuple(Vec<DynSolValue>),
    /// A dynamically-sized array of values.
    Array(Vec<DynSolValue>),
    /// A fixed-size array of values.
    FixedArray(Vec<DynSolValue>),
    /// A named struct, treated as a tuple with a name parameter.
    CustomStruct {
        /// The name of the struct.
        name: String,
        /// The struct's prop names, in declaration order.
        prop_names: Vec<String>,
        /// The inner types.
        tuple: Vec<DynSolValue>,
    },
    /// A user-defined value type.
    CustomValue {
        /// The name of the custom value type.
        name: String,
        /// The value itself.
        inner: Word,
    },
}

impl DynSolValue {
    /// The Solidity type. This returns the solidity type corresponding to this
    /// value, if it is known. A type will not be known if the value contains
    /// an empty sequence, e.g. `T[0]`.
    pub fn sol_type(&self) -> Option<DynSolType> {
        let ty = match self {
            Self::Address(_) => DynSolType::Address,
            Self::Bool(_) => DynSolType::Bool,
            Self::Bytes(_) => DynSolType::Bytes,
            Self::FixedBytes(_, size) => DynSolType::FixedBytes(*size),
            Self::Int(_, size) => DynSolType::Int(*size),
            Self::Uint(_, size) => DynSolType::Uint(*size),
            Self::String(_) => DynSolType::String,
            Self::Tuple(inner) => DynSolType::Tuple(
                inner
                    .iter()
                    .map(Self::sol_type)
                    .collect::<Option<Vec<_>>>()?,
            ),
            Self::Array(inner) => DynSolType::Array(Box::new(Self::sol_type(inner.first()?)?)),
            Self::FixedArray(inner) => {
                DynSolType::FixedArray(Box::new(Self::sol_type(inner.first()?)?), inner.len())
            }
            Self::CustomStruct {
                name,
                prop_names,
                tuple,
            } => DynSolType::CustomStruct {
                name: name.clone(),
                prop_names: prop_names.clone(),
                tuple: tuple
                    .iter()
                    .map(Self::sol_type)
                    .collect::<Option<Vec<_>>>()?,
            },
            Self::CustomValue { name, .. } => DynSolType::CustomValue { name: name.clone() },
        };
        Some(ty)
    }

    /// The Solidity type name. This returns the solidity type corresponding to
    /// this value, if it is known. A type will not be known if the value
    /// contains an empty sequence, e.g. `T[0]`.
    pub fn sol_type_name(&self) -> Option<Cow<'_, str>> {
        let ty = match self {
            Self::Address(_) => Cow::Borrowed("address"),
            Self::Bool(_) => Cow::Borrowed("bool"),
            Self::Bytes(_) => Cow::Borrowed("bytes"),
            Self::FixedBytes(_, size) => Cow::Owned(format!("bytes{}", size)),
            Self::Int(_, size) => Cow::Owned(format!("int{}", size)),
            Self::Uint(_, size) => Cow::Owned(format!("uint{}", size)),
            Self::String(_) => Cow::Borrowed("string"),
            Self::Tuple(inner) => Cow::Owned(format!(
                "({})",
                inner
                    .iter()
                    .map(|v| v.sol_type_name())
                    .collect::<Option<Vec<_>>>()?
                    .join(", ")
            )),
            Self::Array(t) => Cow::Owned(format!("{}[]", t.first()?.sol_type_name()?)),
            Self::FixedArray(t) => {
                Cow::Owned(format!("{}[{}]", t.first()?.sol_type_name()?, t.len()))
            }
            Self::CustomStruct { name, .. } => Cow::Borrowed(name.as_str()),
            Self::CustomValue { name, .. } => Cow::Borrowed(name.as_str()),
        };
        Some(ty)
    }

    /// Trust if this value is encoded as a single word. False otherwise.
    pub const fn is_word(&self) -> bool {
        matches!(
            self,
            Self::Address(_)
                | Self::Bool(_)
                | Self::FixedBytes(_, _)
                | Self::Int(_, _)
                | Self::Uint(_, _)
                | Self::CustomValue { .. }
        )
    }

    /// Fallible cast to a single word. Will succeed for any single-word type.
    pub fn as_word(&self) -> Option<Word> {
        match self {
            Self::Address(a) => Some(a.into_word()),
            Self::Bool(b) => Some({
                let mut buf = [0u8; 32];
                if *b {
                    buf[31] = 1;
                }
                buf.into()
            }),
            Self::FixedBytes(w, _) => Some(*w),
            Self::Int(w, _) => Some(w.to_be_bytes().into()),
            Self::Uint(u, _) => Some(u.to_be_bytes::<32>().into()),
            Self::CustomValue { inner, .. } => Some(*inner),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant DynSolValue {.
    pub const fn as_address(&self) -> Option<Address> {
        match self {
            Self::Address(a) => Some(*a),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub const fn as_fixed_bytes(&self) -> Option<(&[u8], usize)> {
        match self {
            Self::FixedBytes(w, size) => Some((w.as_slice(), *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub const fn as_int(&self) -> Option<(I256, usize)> {
        match self {
            Self::Int(w, size) => Some((*w, *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub const fn as_uint(&self) -> Option<(U256, usize)> {
        match self {
            Self::Uint(u, size) => Some((*u, *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub fn as_tuple(&self) -> Option<&[DynSolValue]> {
        match self {
            Self::Tuple(t) => Some(t.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub fn as_array(&self) -> Option<&[DynSolValue]> {
        match self {
            Self::Array(a) => Some(a.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub fn as_fixed_array(&self) -> Option<&[DynSolValue]> {
        match self {
            Self::FixedArray(a) => Some(a.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub fn as_custom_struct(&self) -> Option<(&str, &[String], &[DynSolValue])> {
        match self {
            Self::CustomStruct {
                name,
                prop_names,
                tuple,
            } => Some((name.as_str(), prop_names.as_slice(), tuple.as_slice())),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    pub fn as_custom_value(&self) -> Option<(&str, Word)> {
        match self {
            Self::CustomValue { name, inner } => Some((name.as_str(), *inner)),
            _ => None,
        }
    }

    /// Returns true if the value is a sequence type.
    pub const fn is_sequence(&self) -> bool {
        matches!(
            self,
            Self::Array(_) | Self::FixedArray(_) | Self::Tuple(_) | Self::CustomStruct { .. }
        )
    }

    /// Fallible cast to a fixed-size array. Any of a `FixedArray`, a `Tuple`,
    /// or a `CustomStruct`.
    pub fn as_fixed_seq(&self) -> Option<&[DynSolValue]> {
        match self {
            Self::FixedArray(inner) => Some(inner.as_slice()),
            Self::Tuple(inner) => Some(inner.as_slice()),
            Self::CustomStruct { tuple, .. } => Some(tuple.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast to a packed sequence. Any of a String, or a Bytes.
    pub fn as_packed_seq(&self) -> Option<&[u8]> {
        match self {
            Self::String(s) => Some(s.as_bytes()),
            Self::Bytes(b) => Some(b.as_slice()),
            _ => None,
        }
    }

    /// Encodes the packed value and appends it to the end of a byte array.
    pub fn encode_packed_to(&self, buf: &mut Vec<u8>) {
        match self {
            Self::Address(addr) => buf.extend_from_slice(addr.as_slice()),
            Self::Bool(b) => buf.push(*b as u8),
            Self::Bytes(bytes) => buf.extend_from_slice(bytes),
            Self::FixedBytes(word, size) => buf.extend_from_slice(&word.as_slice()[..*size]),
            Self::Int(num, size) => {
                let mut bytes = num.to_be_bytes::<32>();
                let start = 32 - *size;
                if num.is_negative() {
                    bytes[start] |= 0x80;
                } else {
                    bytes[start] &= 0x7f;
                }
                buf.extend_from_slice(&bytes[start..])
            }
            Self::Uint(num, size) => {
                buf.extend_from_slice(&num.to_be_bytes::<32>().as_slice()[(32 - *size)..])
            }
            Self::String(s) => buf.extend_from_slice(s.as_bytes()),
            Self::Tuple(inner)
            | Self::Array(inner)
            | Self::FixedArray(inner)
            | Self::CustomStruct { tuple: inner, .. } => {
                inner.iter().for_each(|v| v.encode_packed_to(buf))
            }
            Self::CustomValue { inner, .. } => buf.extend_from_slice(inner.as_slice()),
        }
    }

    /// Encodes the value into a packed byte array.
    pub fn encode_packed(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode_packed_to(&mut buf);
        buf
    }

    /// Tokenize this value into a [`DynToken`].
    pub fn tokenize(&self) -> DynToken<'_> {
        match self {
            DynSolValue::Address(a) => a.into_word().into(),
            DynSolValue::Bool(b) => Word::with_last_byte(*b as u8).into(),
            DynSolValue::Bytes(buf) => DynToken::PackedSeq(buf),
            DynSolValue::FixedBytes(buf, _) => (*buf).into(),
            DynSolValue::Int(int, _) => int.to_be_bytes::<32>().into(),
            DynSolValue::Uint(uint, _) => uint.to_be_bytes::<32>().into(),
            DynSolValue::String(s) => DynToken::PackedSeq(s.as_bytes()),
            DynSolValue::Tuple(t) => DynToken::from_fixed_seq(t),
            DynSolValue::Array(t) => DynToken::from_dyn_seq(t),
            DynSolValue::FixedArray(t) => DynToken::from_fixed_seq(t),
            DynSolValue::CustomStruct { tuple, .. } => DynToken::from_fixed_seq(tuple),
            DynSolValue::CustomValue { inner, .. } => (*inner).into(),
        }
    }
}

impl From<Address> for DynSolValue {
    #[inline]
    fn from(value: Address) -> Self {
        Self::Address(value)
    }
}

impl From<bool> for DynSolValue {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<u8>> for DynSolValue {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<String> for DynSolValue {
    #[inline]
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<DynSolValue>> for DynSolValue {
    #[inline]
    fn from(value: Vec<DynSolValue>) -> Self {
        Self::Array(value)
    }
}

impl<const N: usize> From<[DynSolValue; N]> for DynSolValue {
    #[inline]
    fn from(value: [DynSolValue; N]) -> Self {
        Self::FixedArray(value.to_vec())
    }
}

macro_rules! impl_from_int {
    ($($t:ty),+) => {$(
        impl From<$t> for DynSolValue {
            #[inline]
            fn from(value: $t) -> Self {
                const BITS: usize = <$t>::BITS as usize;
                const BYTES: usize = BITS / 8;
                const _: () = assert!(BYTES <= 32);

                let mut word = if value.is_negative() {
                    alloy_primitives::B256::repeat_byte(0xff)
                } else {
                    alloy_primitives::B256::ZERO
                };
                word[32 - BYTES..].copy_from_slice(&value.to_be_bytes());

                Self::Int(I256::from_be_bytes(word.0), BITS)
            }
        }
    )+};
}

impl_from_int!(i8, i16, i32, i64, isize, i128);

impl From<I256> for DynSolValue {
    #[inline]
    fn from(value: I256) -> Self {
        Self::Int(value, 256)
    }
}

macro_rules! impl_from_uint {
    ($($t:ty),+) => {$(
        impl From<$t> for DynSolValue {
            #[inline]
            fn from(value: $t) -> Self {
                Self::Uint(U256::from(value), <$t>::BITS as usize)
            }
        }
    )+};
}

impl_from_uint!(u8, u16, u32, u64, usize, u128);

impl From<U256> for DynSolValue {
    #[inline]
    fn from(value: U256) -> Self {
        Self::Uint(value, 256)
    }
}

impl DynSolValue {
    /// Returns `true` if the value is an instance of a dynamically sized type.
    #[inline]
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Address(_) => false,
            Self::Bool(_) => false,
            Self::Bytes(_) => true,
            Self::FixedBytes(_, _) => false,
            Self::Int(_, _) => false,
            Self::Uint(_, _) => false,
            Self::String(_) => true,
            Self::Tuple(inner) => inner.iter().any(|v| v.is_dynamic()),
            Self::Array(_) => true,
            Self::FixedArray(inner) => inner.iter().any(|v| v.is_dynamic()),
            Self::CustomStruct { tuple, .. } => tuple.iter().any(|v| v.is_dynamic()),
            Self::CustomValue { .. } => false,
        }
    }

    /// Returns the number of words this type uses in the head of the ABI blob.
    #[inline]
    pub fn head_words(&self) -> usize {
        match self.as_fixed_seq() {
            Some(_) if self.is_dynamic() => 1,
            Some(inner) => inner.iter().map(Self::head_words).sum(),
            None => 1,
        }
    }

    /// Returns the number of words this type uses in the tail of the ABI blob.
    #[inline]
    pub fn tail_words(&self) -> usize {
        if self.is_word() {
            return 0
        }

        if let Some(buf) = self.as_packed_seq() {
            return 1 + (buf.len() + 31) / 32
        }

        if let Some(vals) = self.as_fixed_seq() {
            return self.is_dynamic() as usize * vals.len()
        }

        if let Some(vals) = self.as_array() {
            return 1 + vals.iter().map(Self::tail_words).sum::<usize>()
        }

        unreachable!()
    }

    /// Append this data to the head of an in-progress blob via the encoder.
    #[inline]
    pub fn head_append(&self, enc: &mut Encoder) {
        if let Some(word) = self.as_word() {
            return enc.append_word(word)
        }

        if self.is_dynamic() {
            return enc.append_indirection()
        }

        let seq = self
            .as_fixed_seq()
            .expect("is definitely a non-dynamic fixed sequence");
        seq.iter().for_each(|inner| inner.head_append(enc))
    }

    /// Append this data to the tail of an in-progress blob via the encoder.
    #[inline]
    pub fn tail_append(&self, enc: &mut Encoder) {
        if self.is_word() {
            return
        }

        if let Some(buf) = self.as_packed_seq() {
            return enc.append_packed_seq(buf)
        }

        if let Some(sli) = self.as_fixed_seq() {
            if self.is_dynamic() {
                Self::encode_sequence(sli, enc);
            }
            return
        }

        if let Some(sli) = self.as_array() {
            enc.append_seq_len(sli);
            Self::encode_sequence(sli, enc);
            return
        }

        unreachable!()
    }

    /// Encode this data, if it is a sequence. Error otherwise.
    pub(crate) fn encode_sequence(contents: &[Self], enc: &mut Encoder) {
        let head_words = contents.iter().map(Self::head_words).sum::<usize>();
        enc.push_offset(head_words as u32);

        contents.iter().for_each(|t| {
            t.head_append(enc);
            enc.bump_offset(t.tail_words() as u32);
        });
        contents.iter().for_each(|t| t.tail_append(enc));
        enc.pop_offset();
    }

    /// Encode this value into a byte array by wrapping it into a 1-element
    /// sequence.
    pub fn encode_single(self) -> Vec<u8> {
        let mut enc = Default::default();

        Self::encode_sequence(&[self], &mut enc);

        enc.into_bytes()
    }

    /// Encode this value into a byte array suitable for passing to a function.
    /// If this value is a tuple, it is encoded as is. Otherwise, it is wrapped
    /// into a 1-element sequence.
    ///
    /// ### Example
    ///
    /// ```ignore,pseudo-code
    /// 
    /// // Encoding for function foo(address)
    /// DynSolValue::Address(_).encode_params()
    ///
    /// // Encoding for function foo(address, uint256)
    /// DynSolValue::Tuple(
    ///     vec![
    ///         DynSolValue::Address(_),
    ///         DynSolValue::Uint(_, 256),
    ///     ]
    /// ).encode_params();
    /// ```
    pub fn encode_params(self) -> Vec<u8> {
        match self {
            Self::Tuple(_) => self.encode().expect("tuple is definitely a sequence"),
            _ => self.encode_single(),
        }
    }

    /// If this value is a fixed sequence, encode it into a byte array. If this
    /// value is not a fixed sequence, return `None`
    pub fn encode(&self) -> Option<Vec<u8>> {
        self.as_fixed_seq().map(|seq| {
            let mut encoder = Default::default();
            Self::encode_sequence(seq, &mut encoder);
            encoder.into_bytes()
        })
    }
}
