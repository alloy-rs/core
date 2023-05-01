extern crate alloc;

use alloc::string::String;
use core::{fmt, result::Result};
use serde::{de, Deserializer, Serializer};

use super::FixedBytes;

/// Serializes a slice of bytes.
pub(crate) fn serialize_raw<S>(
    slice: &mut [u8],
    bytes: &[u8],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if bytes.is_empty() {
        serializer.serialize_str("0x")
    } else {
        serializer.serialize_str(super::to_hex_raw(slice, bytes, false, true))
    }
}

/// Expected length of bytes vector.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpectedLen<'a> {
    /// Exact length in bytes.
    Exact(&'a mut [u8]),
    /// A bytes length between (min; slice.len()].
    #[allow(dead_code)]
    Between(usize, &'a mut [u8]),
}

impl<'a> fmt::Display for ExpectedLen<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExpectedLen::Exact(ref v) => write!(fmt, "length of {}", v.len() * 2),
            ExpectedLen::Between(min, ref v) => {
                write!(fmt, "length between ({}; {}]", min * 2, v.len() * 2)
            }
        }
    }
}

/// Deserialize into vector of bytes with additional size check.
/// Returns number of bytes written.
pub(crate) fn deserialize_check_len<'a, 'de, D>(
    deserializer: D,
    len: ExpectedLen<'a>,
) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor<'a> {
        len: ExpectedLen<'a>,
    }

    impl<'a, 'b> de::Visitor<'b> for Visitor<'a> {
        type Value = usize;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                formatter,
                "a hex string (with or wthout 0x prefix) with {} bytes",
                self.len
            )
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            let len = v.len();
            let is_len_valid = match self.len {
                ExpectedLen::Exact(ref slice) => len == 2 * slice.len(),
                ExpectedLen::Between(min, ref slice) => len <= 2 * slice.len() && len > 2 * min,
            };

            if !is_len_valid {
                return Err(E::invalid_length(v.len(), &self));
            }

            let bytes = match self.len {
                ExpectedLen::Exact(slice) => slice,
                ExpectedLen::Between(_, slice) => slice,
            };

            super::from_hex_raw(v, bytes).map_err(E::custom)
        }

        fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
            self.visit_str(&v)
        }
    }

    deserializer.deserialize_str(Visitor { len })
}

impl<const N: usize> serde::Serialize for FixedBytes<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut slice = [0u8; 2 + 2 * 32];
        serialize_raw(&mut slice, &self.0, serializer)
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for FixedBytes<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut bytes = [0u8; N];
        deserialize_check_len(deserializer, ExpectedLen::Exact(&mut bytes))?;
        Ok(Self(bytes))
    }
}
