use ethers_primitives::{B160, U256};

use crate::Word;

/// This type represents a solidity value that has been decoded into rust. It
/// is broadly similar to `serde_json::Value` in that it is an enum of possible
/// types, and the user must inspect and disambiguate
#[derive(Debug, Clone, PartialEq)]
pub enum DynSolValue {
    /// An address
    Address(B160),
    /// A boolean
    Bool(bool),
    /// A dynamic-length byte array
    Bytes(Vec<u8>),
    /// A fixed-length byte string
    FixedBytes(Word, usize),
    /// A signed integer
    Int(Word, usize),
    /// An unsigned integer
    Uint(U256, usize),
    /// A function
    Function(B160, [u8; 4]),
    /// A string
    String(String),
    /// A tuple of values
    Tuple(Vec<DynSolValue>),
    /// A dynamically-sized array of values
    Array(Vec<DynSolValue>),
    /// A fixed-size array of values
    FixedArray(Vec<DynSolValue>),
    /// A named struct, treated as a tuple with a name parameter
    CustomStruct {
        /// The name of the struct
        name: String,
        // TODO: names
        /// A inner types
        tuple: Vec<DynSolValue>,
    },
    /// A user-defined value type.
    CustomValue {
        /// The name of the custom value type
        name: String,
        /// The value itself
        inner: Word,
    },
}

impl DynSolValue {
    /// Fallible cast to the contents of a variant DynSolValue {
    pub fn as_address(&self) -> Option<B160> {
        match self {
            Self::Address(a) => Some(*a),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_fixed_bytes(&self) -> Option<(&[u8], usize)> {
        match self {
            Self::FixedBytes(w, size) => Some((w.as_bytes(), *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_int(&self) -> Option<(Word, usize)> {
        match self {
            Self::Int(w, size) => Some((*w, *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_uint(&self) -> Option<(U256, usize)> {
        match self {
            Self::Uint(u, size) => Some((*u, *size)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_function(&self) -> Option<(B160, [u8; 4])> {
        match self {
            Self::Function(a, f) => Some((*a, *f)),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_tuple(&self) -> Option<&[DynSolValue]> {
        match self {
            Self::Tuple(t) => Some(t.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_array(&self) -> Option<&[DynSolValue]> {
        match self {
            Self::Array(a) => Some(a.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_fixed_array(&self) -> Option<&[DynSolValue]> {
        match self {
            Self::FixedArray(a) => Some(a.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_custom_struct(&self) -> Option<(&str, &[DynSolValue])> {
        match self {
            Self::CustomStruct { name, tuple } => Some((name.as_str(), tuple.as_slice())),
            _ => None,
        }
    }

    /// Fallible cast to the contents of a variant
    pub fn as_custom_value(&self) -> Option<(&str, Word)> {
        match self {
            Self::CustomValue { name, inner } => Some((name.as_str(), *inner)),
            _ => None,
        }
    }

    /// Encodes the packed value and appends it to the end of a byte array
    pub fn encode_packed_to(&self, buf: &mut Vec<u8>) {
        match self {
            DynSolValue::Address(addr) => buf.extend_from_slice(addr.as_bytes()),
            DynSolValue::Bool(b) => buf.push(*b as u8),
            DynSolValue::Bytes(bytes) => buf.extend_from_slice(bytes),
            DynSolValue::FixedBytes(word, size) => buf.extend_from_slice(&word.as_bytes()[..*size]),
            DynSolValue::Int(num, size) => buf.extend_from_slice(&num[(32 - *size)..]),
            DynSolValue::Uint(num, size) => {
                buf.extend_from_slice(&num.to_be_bytes::<32>().as_slice()[(32 - *size)..])
            }
            DynSolValue::Function(addr, selector) => {
                buf.extend_from_slice(addr.as_bytes());
                buf.extend_from_slice(selector.as_slice());
            }
            DynSolValue::String(s) => buf.extend_from_slice(s.as_bytes()),
            DynSolValue::Tuple(inner)
            | DynSolValue::Array(inner)
            | DynSolValue::FixedArray(inner)
            | DynSolValue::CustomStruct { tuple: inner, .. } => {
                inner.iter().for_each(|v| v.encode_packed_to(buf));
            }
            DynSolValue::CustomValue { inner, .. } => buf.extend_from_slice(inner.as_bytes()),
        }
    }

    /// Encodes the value into a packed byte array
    pub fn encode_packed(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode_packed_to(&mut buf);
        buf
    }
}

impl From<B160> for DynSolValue {
    fn from(value: B160) -> Self {
        Self::Address(value)
    }
}

impl From<bool> for DynSolValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<u8>> for DynSolValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

macro_rules! impl_from_int {
    ($size:ty) => {
        impl From<$size> for DynSolValue {
            fn from(value: $size) -> Self {
                let bits = <$size>::BITS as usize;
                let bytes = bits / 8;
                let mut word = if value < 0 {
                    ethers_primitives::B256::repeat_byte(0xff)
                } else {
                    ethers_primitives::B256::default()
                };
                word[32 - bytes..].copy_from_slice(&value.to_be_bytes());

                Self::Int(word.into(), bits)
            }
        }
    };
}

impl_from_int!(i8);
impl_from_int!(i16);
impl_from_int!(i32);
impl_from_int!(i64);
impl_from_int!(isize);
// TODO: more?

macro_rules! impl_from_uint {
    ($size:ty) => {
        impl From<$size> for DynSolValue {
            fn from(value: $size) -> Self {
                Self::Uint(U256::from(value), <$size>::BITS as usize)
            }
        }
    };
}

impl_from_uint!(u8);
impl_from_uint!(u16);
impl_from_uint!(u32);
impl_from_uint!(u64);
impl_from_uint!(usize);
// TODO: more?
impl_from_uint!(U256);

impl From<(B160, [u8; 4])> for DynSolValue {
    fn from(value: (B160, [u8; 4])) -> Self {
        Self::Function(value.0, value.1)
    }
}

impl From<String> for DynSolValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
