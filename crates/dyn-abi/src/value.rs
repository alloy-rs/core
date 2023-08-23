use crate::{DynSolType, DynToken, Word};
use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
use alloy_primitives::{Address, Function, I256, U256};
use alloy_sol_types::{utils::words_for_len, Encoder};

#[cfg(feature = "eip712")]
macro_rules! as_fixed_seq {
    ($tuple:tt) => {
        Self::CustomStruct { tuple: $tuple, .. } | Self::FixedArray($tuple) | Self::Tuple($tuple)
    };
}
#[cfg(not(feature = "eip712"))]
macro_rules! as_fixed_seq {
    ($tuple:tt) => {
        Self::FixedArray($tuple) | Self::Tuple($tuple)
    };
}

/// A dynamic Solidity value.
///
/// It is broadly similar to `serde_json::Value` in that it is an enum of
/// possible types, and the user must inspect and disambiguate.
///
/// # Examples
///
/// ```
/// use alloy_dyn_abi::{DynSolType, DynSolValue};
///
/// let my_type: DynSolType = "uint64".parse().unwrap();
/// let my_data: DynSolValue = 183u64.into();
///
/// let encoded = my_data.encode_single();
/// let decoded = my_type.decode_single(&encoded)?;
///
/// assert_eq!(decoded, my_data);
/// # Ok::<(), alloy_dyn_abi::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DynSolValue {
    /// An address.
    Address(Address),
    /// A function pointer.
    Function(Function),
    /// A boolean.
    Bool(bool),
    /// A signed integer.
    Int(I256, usize),
    /// An unsigned integer.
    Uint(U256, usize),
    /// A fixed-length byte string.
    FixedBytes(Word, usize),

    /// A dynamic-length byte array.
    Bytes(Vec<u8>),
    /// A string.
    String(String),

    /// A dynamically-sized array of values.
    Array(Vec<DynSolValue>),
    /// A fixed-size array of values.
    FixedArray(Vec<DynSolValue>),
    /// A tuple of values.
    Tuple(Vec<DynSolValue>),

    /// A named struct, treated as a tuple with a name parameter.
    #[cfg(feature = "eip712")]
    CustomStruct {
        /// The name of the struct.
        name: String,
        /// The struct's prop names, in declaration order.
        prop_names: Vec<String>,
        /// The inner types.
        tuple: Vec<DynSolValue>,
    },
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

impl From<Vec<Self>> for DynSolValue {
    #[inline]
    fn from(value: Vec<Self>) -> Self {
        Self::Array(value)
    }
}

impl<const N: usize> From<[Self; N]> for DynSolValue {
    #[inline]
    fn from(value: [Self; N]) -> Self {
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
    /// The Solidity type. This returns the solidity type corresponding to this
    /// value, if it is known. A type will not be known if the value contains
    /// an empty sequence, e.g. `T[0]`.
    pub fn as_type(&self) -> Option<DynSolType> {
        let ty = match self {
            Self::Address(_) => DynSolType::Address,
            Self::Function(_) => DynSolType::Function,
            Self::Bool(_) => DynSolType::Bool,
            Self::Bytes(_) => DynSolType::Bytes,
            Self::FixedBytes(_, size) => DynSolType::FixedBytes(*size),
            Self::Int(_, size) => DynSolType::Int(*size),
            Self::Uint(_, size) => DynSolType::Uint(*size),
            Self::String(_) => DynSolType::String,
            Self::Tuple(inner) => {
                return inner
                    .iter()
                    .map(Self::as_type)
                    .collect::<Option<Vec<_>>>()
                    .map(DynSolType::Tuple)
            }
            Self::Array(inner) => DynSolType::Array(Box::new(Self::as_type(inner.first()?)?)),
            Self::FixedArray(inner) => {
                DynSolType::FixedArray(Box::new(Self::as_type(inner.first()?)?), inner.len())
            }
            #[cfg(feature = "eip712")]
            Self::CustomStruct {
                name,
                prop_names,
                tuple,
            } => DynSolType::CustomStruct {
                name: name.clone(),
                prop_names: prop_names.clone(),
                tuple: tuple
                    .iter()
                    .map(Self::as_type)
                    .collect::<Option<Vec<_>>>()?,
            },
        };
        Some(ty)
    }

    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    fn sol_type_name_simple(&self) -> Option<&str> {
        match self {
            Self::Address(_) => Some("address"),
            Self::Function(_) => Some("function"),
            Self::Bool(_) => Some("bool"),
            Self::Bytes(_) => Some("bytes"),
            Self::String(_) => Some("string"),
            #[cfg(feature = "eip712")]
            Self::CustomStruct { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }

    #[inline]
    fn sol_type_name_raw(&self, out: &mut String) -> bool {
        match self {
            #[cfg(not(feature = "eip712"))]
            Self::Address(_)
            | Self::Function(_)
            | Self::Bool(_)
            | Self::Bytes(_)
            | Self::String(_) => {
                out.push_str(unsafe { self.sol_type_name_simple().unwrap_unchecked() });
            }
            #[cfg(feature = "eip712")]
            Self::Address(_)
            | Self::Function(_)
            | Self::Bool(_)
            | Self::Bytes(_)
            | Self::String(_)
            | Self::CustomStruct { .. } => {
                out.push_str(unsafe { self.sol_type_name_simple().unwrap_unchecked() });
            }

            Self::FixedBytes(_, size) | Self::Int(_, size) | Self::Uint(_, size) => {
                let prefix = match self {
                    Self::FixedBytes(..) => "bytes",
                    Self::Int(..) => "int",
                    Self::Uint(..) => "uint",
                    _ => unreachable!(),
                };
                out.push_str(prefix);
                out.push_str(itoa::Buffer::new().format(*size));
            }

            Self::Tuple(inner) => {
                out.push('(');
                for (i, val) in inner.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    if !val.sol_type_name_raw(out) {
                        return false
                    }
                }
                if inner.len() == 1 {
                    out.push(',');
                }
                out.push(')');
            }
            Self::Array(t) => {
                if let Some(first) = t.first() {
                    if !first.sol_type_name_raw(out) {
                        return false
                    }
                    out.push_str("[]");
                } else {
                    return false
                }
            }
            Self::FixedArray(t) => {
                if let Some(first) = t.first() {
                    if !first.sol_type_name_raw(out) {
                        return false
                    }
                    out.push('[');
                    out.push_str(itoa::Buffer::new().format(t.len()));
                    out.push(']');
                } else {
                    return false
                }
            }
        }
        true
    }

    /// The Solidity type name. This returns the solidity type corresponding to
    /// this value, if it is known. A type will not be known if the value
    /// contains an empty sequence, e.g. `T[0]`.
    pub fn sol_type_name(&self) -> Option<Cow<'_, str>> {
        if let Some(s) = self.sol_type_name_simple() {
            Some(Cow::Borrowed(s))
        } else {
            let capacity = match self {
                Self::Tuple(_) => 256,
                _ => 16,
            };
            let mut s = String::with_capacity(capacity);
            if self.sol_type_name_raw(&mut s) {
                Some(Cow::Owned(s))
            } else {
                None
            }
        }
    }

    /// Trust if this value is encoded as a single word. False otherwise.
    #[inline]
    pub const fn is_word(&self) -> bool {
        matches!(
            self,
            Self::Address(_)
                | Self::Bool(_)
                | Self::FixedBytes(..)
                | Self::Int(..)
                | Self::Uint(..)
        )
    }

    /// Fallible cast to a single word. Will succeed for any single-word type.
    #[inline]
    pub fn as_word(&self) -> Option<Word> {
        match *self {
            Self::Address(a) => Some(a.into_word()),
            Self::Function(f) => Some(f.into_word()),
            Self::Bool(b) => Some(Word::with_last_byte(b as u8)),
            Self::FixedBytes(w, _) => Some(w),
            Self::Int(i, _) => Some(i.into()),
            Self::Uint(u, _) => Some(u.into()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant DynSolValue {.
    #[inline]
    pub const fn as_address(&self) -> Option<Address> {
        match self {
            Self::Address(a) => Some(*a),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub const fn as_fixed_bytes(&self) -> Option<(&[u8], usize)> {
        match self {
            Self::FixedBytes(w, size) => Some((w.as_slice(), *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub const fn as_int(&self) -> Option<(I256, usize)> {
        match self {
            Self::Int(w, size) => Some((*w, *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub const fn as_uint(&self) -> Option<(U256, usize)> {
        match self {
            Self::Uint(u, size) => Some((*u, *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
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
    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    pub fn as_fixed_array(&self) -> Option<&[Self]> {
        match self {
            Self::FixedArray(a) => Some(a),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant.
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_custom_struct(&self) -> Option<(&str, &[String], &[Self])> {
        match self {
            #[cfg(feature = "eip712")]
            Self::CustomStruct {
                name,
                prop_names,
                tuple,
            } => Some((name, prop_names, tuple)),
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
                Self::Array(t) | Self::FixedArray(t) | Self::Tuple(t) => {
                    t.iter().any(Self::has_custom_struct)
                }
                _ => false,
            }
        }
        #[cfg(not(feature = "eip712"))]
        {
            false
        }
    }

    /// Returns true if the value is a sequence type.
    #[inline]
    pub const fn is_sequence(&self) -> bool {
        matches!(self, as_fixed_seq!(_) | Self::Array(_))
    }

    /// Fallible cast to a fixed-size array. Any of a `FixedArray`, a `Tuple`,
    /// or a `CustomStruct`.
    #[inline]
    pub fn as_fixed_seq(&self) -> Option<&[Self]> {
        match self {
            as_fixed_seq!(tuple) => Some(tuple),
            _ => None,
        }
    }

    /// Fallible cast to a packed sequence. Any of a String, or a Bytes.
    #[inline]
    pub fn as_packed_seq(&self) -> Option<&[u8]> {
        match self {
            Self::String(s) => Some(s.as_bytes()),
            Self::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Returns `true` if the value is an instance of a dynamically sized type.
    #[inline]
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Address(_)
            | Self::Function(_)
            | Self::Bool(_)
            | Self::Int(..)
            | Self::Uint(..)
            | Self::FixedBytes(..) => false,
            Self::Bytes(_) | Self::String(_) | Self::Array(_) => true,
            as_fixed_seq!(tuple) => tuple.iter().any(Self::is_dynamic),
        }
    }

    /// Check that these values have the same type as the given [`DynSolType`]s.
    ///
    /// See [`DynSolType::matches`] for more information.
    #[doc(alias = "types_check")] // from ethabi
    #[inline(always)]
    pub fn matches_many(values: &[Self], types: &[DynSolType]) -> bool {
        DynSolType::matches_many(types, values)
    }

    /// Check that this value has the same type as the given [`DynSolType`].
    ///
    /// See [`DynSolType::matches`] for more information.
    #[doc(alias = "type_check")] // from ethabi
    #[inline(always)]
    pub fn matches(&self, ty: &DynSolType) -> bool {
        ty.matches(self)
    }

    /// Returns the number of words this type uses in the head of the ABI blob.
    #[inline]
    pub(crate) fn head_words(&self) -> usize {
        match self.as_fixed_seq() {
            // If dynamic 1 for the length, otherwise the sum of all head words.
            Some(vals) => {
                // `is_dynamic` iterates over all elements, and we need to sum all elements'
                // head words, so do both things at once
                let mut sum = 0;
                for val in vals {
                    if val.is_dynamic() {
                        return 1
                    }
                    sum += val.head_words();
                }
                sum
            }
            // Just a single word
            None => 1,
        }
    }

    /// Returns the number of words this type uses in the tail of the ABI blob.
    #[inline]
    pub(crate) fn tail_words(&self) -> usize {
        match self {
            // `self.is_word()`
            Self::Address(_)
            | Self::Function(_)
            | Self::Bool(_)
            | Self::FixedBytes(..)
            | Self::Int(..)
            | Self::Uint(..) => 0,

            // `self.as_packed_seq()`
            // 1 for the length, then the body padded to the next word.
            Self::String(s) => 1 + words_for_len(s.len()),
            Self::Bytes(b) => 1 + words_for_len(b.len()),

            // `self.as_fixed_seq()`
            // if static, 0.
            // If dynamic, all words for all elements.
            as_fixed_seq!(tuple) => {
                // `is_dynamic` iterates over all elements, and we need to sum all elements'
                // total words, so do both things at once
                let mut any_dynamic = false;
                let mut sum = 0;
                for val in tuple {
                    any_dynamic = any_dynamic || val.is_dynamic();
                    sum += val.total_words();
                }
                any_dynamic as usize * sum
            }

            // `self.as_array()`
            // 1 for the length. Then all words for all elements.
            Self::Array(vals) => 1 + vals.iter().map(Self::total_words).sum::<usize>(),
        }
    }

    /// Returns the total number of words this type uses in the ABI blob,
    /// assuming it is not the top-level
    #[inline]
    pub(crate) fn total_words(&self) -> usize {
        self.head_words() + self.tail_words()
    }

    /// Append this data to the head of an in-progress blob via the encoder.
    #[inline]
    pub fn head_append(&self, enc: &mut Encoder) {
        match self {
            Self::Address(_)
            | Self::Function(_)
            | Self::Bool(_)
            | Self::FixedBytes(..)
            | Self::Int(..)
            | Self::Uint(..) => enc.append_word(unsafe { self.as_word().unwrap_unchecked() }),

            Self::String(_) | Self::Bytes(_) | Self::Array(_) => enc.append_indirection(),

            as_fixed_seq!(s) => {
                if s.iter().any(Self::is_dynamic) {
                    enc.append_indirection();
                } else {
                    for inner in s {
                        inner.head_append(enc);
                    }
                }
            }
        }
    }

    /// Append this data to the tail of an in-progress blob via the encoder.
    #[inline]
    pub fn tail_append(&self, enc: &mut Encoder) {
        match self {
            Self::Address(_)
            | Self::Function(_)
            | Self::Bool(_)
            | Self::FixedBytes(..)
            | Self::Int(..)
            | Self::Uint(..) => {}

            Self::String(string) => enc.append_packed_seq(string.as_bytes()),
            Self::Bytes(bytes) => enc.append_packed_seq(bytes),

            as_fixed_seq!(s) => {
                if self.is_dynamic() {
                    Self::encode_sequence_to(s, enc);
                }
            }

            Self::Array(array) => {
                enc.append_seq_len(array.len());
                Self::encode_sequence_to(array, enc);
            }
        }
    }

    /// Encodes the packed value and appends it to the end of a byte array.
    pub fn encode_packed_to(&self, buf: &mut Vec<u8>) {
        match self {
            Self::Address(addr) => buf.extend_from_slice(addr.as_slice()),
            Self::Function(func) => buf.extend_from_slice(func.as_slice()),
            Self::Bool(b) => buf.push(*b as u8),
            Self::String(s) => buf.extend_from_slice(s.as_bytes()),
            Self::Bytes(bytes) => buf.extend_from_slice(bytes),
            Self::FixedBytes(word, size) => buf.extend_from_slice(&word[..*size]),
            Self::Int(num, size) => {
                let mut bytes = num.to_be_bytes::<32>();
                let start = 32 - *size;
                if num.is_negative() {
                    bytes[start] |= 0x80;
                } else {
                    bytes[start] &= 0x7f;
                }
                buf.extend_from_slice(&bytes[start..]);
            }
            Self::Uint(num, size) => {
                buf.extend_from_slice(&num.to_be_bytes::<32>()[(32 - *size)..]);
            }
            as_fixed_seq!(inner) | Self::Array(inner) => {
                for val in inner {
                    val.encode_packed_to(buf);
                }
            }
        }
    }

    /// Encodes the value into a packed byte array.
    #[inline]
    pub fn encode_packed(&self) -> Vec<u8> {
        // TODO: capacity
        let mut buf = Vec::new();
        self.encode_packed_to(&mut buf);
        buf
    }

    /// Tokenize this value into a [`DynToken`].
    pub fn tokenize(&self) -> DynToken<'_> {
        match self {
            Self::Address(a) => a.into_word().into(),
            Self::Function(f) => f.into_word().into(),
            Self::Bool(b) => Word::with_last_byte(*b as u8).into(),
            Self::Bytes(buf) => DynToken::PackedSeq(buf),
            Self::FixedBytes(buf, _) => (*buf).into(),
            Self::Int(int, _) => int.to_be_bytes::<32>().into(),
            Self::Uint(uint, _) => uint.to_be_bytes::<32>().into(),
            Self::String(s) => DynToken::PackedSeq(s.as_bytes()),
            Self::Array(t) => DynToken::from_dyn_seq(t),
            as_fixed_seq!(t) => DynToken::from_fixed_seq(t),
        }
    }

    /// Encode this data as a sequence.
    pub(crate) fn encode_sequence(seq: &[Self]) -> Vec<u8> {
        let sz = seq.iter().map(Self::total_words).sum();
        let mut encoder = Encoder::with_capacity(sz);
        Self::encode_sequence_to(seq, &mut encoder);
        encoder.into_bytes()
    }

    /// Encode this data as a sequence into the given encoder.
    pub(crate) fn encode_sequence_to(contents: &[Self], enc: &mut Encoder) {
        let head_words = contents.iter().map(Self::head_words).sum::<usize>();
        enc.push_offset(head_words as u32);

        for t in contents {
            t.head_append(enc);
            enc.bump_offset(t.tail_words() as u32);
        }

        for t in contents {
            t.tail_append(enc);
        }

        enc.pop_offset();
    }

    /// Encode this value into a byte array suitable for passing to a function.
    /// If this value is a tuple, it is encoded as is. Otherwise, it is wrapped
    /// into a 1-element sequence.
    ///
    /// # Examples
    ///
    /// ```ignore (pseudo-code)
    /// // Encoding for function foo(address)
    /// DynSolValue::Address(_).encode_params();
    ///
    /// // Encoding for function foo(address, uint256)
    /// DynSolValue::Tuple(vec![
    ///     DynSolValue::Address(_),
    ///     DynSolValue::Uint(_, 256),
    /// ]).encode_params();
    /// ```
    #[inline]
    pub fn encode_params(&self) -> Vec<u8> {
        match self {
            Self::Tuple(_) => self.encode().expect("tuple is definitely a sequence"),
            _ => self.encode_single(),
        }
    }

    /// Encode this value into a byte array by wrapping it into a 1-element
    /// sequence.
    #[inline]
    pub fn encode_single(&self) -> Vec<u8> {
        Self::encode_sequence(core::slice::from_ref(self))
    }

    /// If this value is a fixed sequence, encode it into a byte array. If this
    /// value is not a fixed sequence, return `None`.
    #[inline]
    pub fn encode(&self) -> Option<Vec<u8>> {
        self.as_fixed_seq().map(Self::encode_sequence)
    }
}
