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



