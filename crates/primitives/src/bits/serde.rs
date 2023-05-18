use super::FixedBytes;
use alloc::string::String;
use core::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

impl<const N: usize> Serialize for FixedBytes<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut buf = hex::Buffer::<N, true>::new();
        serializer.serialize_str(buf.format(&self.0))
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedBytes<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(FixedBytesVisitor::<N>)
    }
}

struct FixedBytesVisitor<const N: usize>;

impl<const N: usize> de::Visitor<'_> for FixedBytesVisitor<N> {
    type Value = FixedBytes<N>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a {N} byte hex string")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let mut buffer = [0u8; N];
        match hex::decode_to_slice(v.as_bytes(), &mut buffer) {
            Ok(()) => Ok(FixedBytes(buffer)),
            Err(e) => Err(de::Error::custom(e)),
        }
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(v.as_str())
    }
}
