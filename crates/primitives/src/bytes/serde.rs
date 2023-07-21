use serde::de::{self, Visitor};

use core::result::Result;

use crate::Bytes;
use alloc::vec::Vec;

impl serde::Serialize for Bytes {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            hex::serialize(self, serializer)
        } else {
            serializer.serialize_bytes(self.as_ref())
        }
    }
}

impl<'de> serde::Deserialize<'de> for Bytes {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct BytesVisitor;

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = Bytes;

            fn expecting(&self, formatter: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
                formatter.write_str("a variable number of bytes, represented as a hex string, an array of u8, or raw bytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Bytes::from(v.to_vec()))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Bytes::from(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = Vec::new();

                while let Some(byte) = seq.next_element()? {
                    bytes.push(byte);
                }

                Ok(Bytes::from(bytes))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                hex::decode(v)
                    .map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(v), &"a valid hex string")
                    })
                    .map(From::from)
            }
        }

        deserializer.deserialize_any(BytesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    struct TestCase {
        variable: Bytes,
    }

    #[test]
    fn serde() {
        let bytes = Bytes::from_static(&[1, 35, 69, 103, 137, 171, 205, 239]);
        let ser = serde_json::to_string(&bytes).unwrap();
        assert_eq!(ser, "\"0x0123456789abcdef\"");
        assert_eq!(serde_json::from_str::<Bytes>(&ser).unwrap(), bytes);
    }

    #[test]
    fn serde_num_array() {
        let json = serde_json::json! {{"variable": [0,1,2,3,4]}};

        assert_eq!(
            serde_json::from_value::<TestCase>(json.clone())
                .unwrap()
                .variable,
            Bytes::from(Vec::from([0, 1, 2, 3, 4]))
        );
    }
}
