use crate::{no_std_prelude::*, Word};
use alloy_primitives::{Address, I256, U256};

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
    /// The Solidity type name.
    pub fn sol_type_name(&self) -> String {
        match self {
            Self::Address(_) => "address".to_string(),
            Self::Bool(_) => "bool".to_string(),
            Self::Bytes(_) => "bytes".to_string(),
            Self::FixedBytes(_, size) => format!("bytes{}", size),
            Self::Int(_, size) => format!("int{}", size),
            Self::Uint(_, size) => format!("uint{}", size),
            Self::String(_) => "string".to_string(),
            Self::Tuple(_) => "tuple".to_string(),
            Self::Array(_) => "array".to_string(),
            Self::FixedArray(_) => "fixed_array".to_string(),
            Self::CustomStruct { name, .. } => name.clone(),
            Self::CustomValue { name, .. } => name.clone(),
        }
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
            Self::FixedBytes(w, size) => Some((w.as_bytes(), *size)),
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

    /// Encodes the packed value and appends it to the end of a byte array.
    pub fn encode_packed_to(&self, buf: &mut Vec<u8>) {
        match self {
            Self::Address(addr) => buf.extend_from_slice(addr.as_bytes()),
            Self::Bool(b) => buf.push(*b as u8),
            Self::Bytes(bytes) => buf.extend_from_slice(bytes),
            Self::FixedBytes(word, size) => buf.extend_from_slice(&word.as_bytes()[..*size]),
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
            Self::CustomValue { inner, .. } => buf.extend_from_slice(inner.as_bytes()),
        }
    }

    /// Encodes the value into a packed byte array.
    pub fn encode_packed(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode_packed_to(&mut buf);
        buf
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
                    alloy_primitives::B256::default()
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

// TODO: more?
impl_from_uint!(u8, u16, u32, u64, usize, u128);

impl From<U256> for DynSolValue {
    #[inline]
    fn from(value: U256) -> Self {
        Self::Uint(value, 256)
    }
}

impl From<String> for DynSolValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

macro_rules! impl_from_tuple {
    ($num:expr, $( $ty:ident : $no:tt ),+ $(,)?) => {
        impl<$($ty,)+> From<($( $ty, )+)> for DynSolValue
        where
            $(
                $ty: Into<DynSolValue>,
            )+
        {
            fn from(value: ($( $ty, )+)) -> Self {
                Self::Tuple(vec![$( value.$no.into(), )+])
            }
        }
    }
}

impl_from_tuple!(1, A:0, );
impl_from_tuple!(2, A:0, B:1, );
impl_from_tuple!(3, A:0, B:1, C:2, );
impl_from_tuple!(4, A:0, B:1, C:2, D:3, );
impl_from_tuple!(5, A:0, B:1, C:2, D:3, E:4, );
impl_from_tuple!(6, A:0, B:1, C:2, D:3, E:4, F:5, );
impl_from_tuple!(7, A:0, B:1, C:2, D:3, E:4, F:5, G:6, );
impl_from_tuple!(8, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, );
impl_from_tuple!(9, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, );
impl_from_tuple!(10, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, );
impl_from_tuple!(11, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, );
impl_from_tuple!(12, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, );
impl_from_tuple!(13, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, );
impl_from_tuple!(14, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, );
impl_from_tuple!(15, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, );
impl_from_tuple!(16, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, );
impl_from_tuple!(17, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16,);
impl_from_tuple!(18, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17,);
impl_from_tuple!(19, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18,);
impl_from_tuple!(20, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19,);
impl_from_tuple!(21, A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15, Q:16, R:17, S:18, T:19, U:20,);

impl From<Vec<DynSolValue>> for DynSolValue {
    fn from(value: Vec<DynSolValue>) -> Self {
        Self::Array(value)
    }
}

impl<T, const N: usize> From<[T; N]> for DynSolValue
where
    T: Into<DynSolValue>,
{
    fn from(value: [T; N]) -> Self {
        Self::Array(value.into_iter().map(|v| v.into()).collect())
    }
}
