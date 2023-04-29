extern crate alloc;

use alloc::string::String;
use core::{fmt, result::Result};
use serde::{de, Deserializer, Serializer};

use super::{Address, B160, B256, B512};

static CHARS: &[u8] = b"0123456789abcdef";

fn to_hex_raw<'a>(v: &'a mut [u8], bytes: &[u8], skip_leading_zero: bool) -> &'a str {
    assert!(v.len() > 1 + bytes.len() * 2);

    v[0] = b'0';
    v[1] = b'x';

    let mut idx = 2;
    let first_nibble = bytes[0] >> 4;
    if first_nibble != 0 || !skip_leading_zero {
        v[idx] = CHARS[first_nibble as usize];
        idx += 1;
    }
    v[idx] = CHARS[(bytes[0] & 0xf) as usize];
    idx += 1;

    for &byte in bytes.iter().skip(1) {
        v[idx] = CHARS[(byte >> 4) as usize];
        v[idx + 1] = CHARS[(byte & 0xf) as usize];
        idx += 2;
    }

    // SAFETY: all characters come either from CHARS or "0x", therefore valid UTF8
    unsafe { core::str::from_utf8_unchecked(&v[0..idx]) }
}

/// Decoding bytes from hex string error.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum FromHexError {
    /// Invalid (non-hex) character encountered.
    InvalidHex {
        /// The unexpected character.
        character: char,
        /// Index of that occurrence.
        index: usize,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for FromHexError {}

impl fmt::Display for FromHexError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidHex { character, index } => {
                write!(fmt, "invalid hex character: {character}, at {index}")
            }
        }
    }
}

/// Decode given 0x-prefix-stripped hex string into provided slice.
/// Used internally by `from_hex` and `deserialize_check_len`.
///
/// The method will panic if `bytes` have incorrect length (make sure to allocate enough
/// beforehand).
fn from_hex_raw(v: &str, bytes: &mut [u8], stripped: bool) -> Result<usize, FromHexError> {
    let bytes_len = v.len();
    let mut modulus = bytes_len % 2;
    let mut buf = 0;
    let mut pos = 0;
    for (index, byte) in v.bytes().enumerate() {
        buf <<= 4;

        match byte {
            b'A'..=b'F' => buf |= byte - b'A' + 10,
            b'a'..=b'f' => buf |= byte - b'a' + 10,
            b'0'..=b'9' => buf |= byte - b'0',
            b' ' | b'\r' | b'\n' | b'\t' => {
                buf >>= 4;
                continue;
            }
            b => {
                let character = char::from(b);
                return Err(FromHexError::InvalidHex {
                    character,
                    index: index + if stripped { 2 } else { 0 },
                });
            }
        }

        modulus += 1;
        if modulus == 2 {
            modulus = 0;
            bytes[pos] = buf;
            pos += 1;
        }
    }

    Ok(pos)
}

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
        serializer.serialize_str(to_hex_raw(slice, bytes, false))
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
                "a (both 0x-prefixed or not) hex string with {}",
                self.len
            )
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            let (v, stripped) = v.strip_prefix("0x").map_or((v, false), |v| (v, true));

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

            from_hex_raw(v, bytes, stripped).map_err(E::custom)
        }

        fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
            self.visit_str(&v)
        }
    }

    deserializer.deserialize_str(Visitor { len })
}

impl serde::Serialize for B256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut slice = [0u8; 2 + 2 * 32];
        serialize_raw(&mut slice, &self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for B256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut bytes = [0u8; 32];
        deserialize_check_len(deserializer, ExpectedLen::Exact(&mut bytes))?;
        Ok(B256(bytes))
    }
}

impl serde::Serialize for B160 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut slice = [0u8; 2 + 2 * 20];
        serialize_raw(&mut slice, &self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for B160 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut bytes = [0u8; 20];
        deserialize_check_len(deserializer, ExpectedLen::Exact(&mut bytes))?;
        Ok(B160(bytes))
    }
}

impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut slice = [0u8; 2 + 2 * 20];
        serialize_raw(&mut slice, &self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut bytes = [0u8; 20];
        deserialize_check_len(deserializer, ExpectedLen::Exact(&mut bytes))?;
        Ok(Address(bytes))
    }
}

impl serde::Serialize for B512 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut slice = [0u8; 2 + 2 * 64];
        serialize_raw(&mut slice, &self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for B512 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut bytes = [0u8; 64];
        deserialize_check_len(deserializer, ExpectedLen::Exact(&mut bytes))?;
        Ok(B512(bytes))
    }
}
