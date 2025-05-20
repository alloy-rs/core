use super::Signed;
use alloc::string::String;
use core::{fmt, str};
use ruint::Uint;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<const BITS: usize, const LIMBS: usize> Serialize for Signed<BITS, LIMBS> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            serializer.serialize_bytes(&self.0.to_be_bytes_vec())
        }
    }
}

impl<'de, const BITS: usize, const LIMBS: usize> Deserialize<'de> for Signed<BITS, LIMBS> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserialize(deserializer)
    }
}

fn deserialize<'de, const BITS: usize, const LIMBS: usize, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Signed<BITS, LIMBS>, D::Error> {
    struct SignedVisitor<const BITS: usize, const LIMBS: usize>;

    impl<const BITS: usize, const LIMBS: usize> Visitor<'_> for SignedVisitor<BITS, LIMBS> {
        type Value = Signed<BITS, LIMBS>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "a {BITS} bit signed integer")
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Signed::try_from(v).map_err(de::Error::custom)
        }

        fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
            Signed::try_from(v).map_err(de::Error::custom)
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
            Signed::try_from(v).map_err(de::Error::custom)
        }

        fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
            Signed::try_from(v).map_err(de::Error::custom)
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse().map_err(serde::de::Error::custom)
        }

        fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
            self.visit_str(&v)
        }
        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(match Uint::try_from_be_slice(v) {
                Some(v) => Signed::from_raw(v),
                None => String::from_utf8(v.to_vec())
                    .map(|s| s.parse::<Self::Value>())
                    .ok()
                    .map(|v| v.ok())
                    .flatten()
                    .ok_or(de::Error::custom("Invalid bytes"))?,
            })
        }
    }
    if deserializer.is_human_readable() {
        deserializer.deserialize_any(SignedVisitor)
    } else {
        deserializer.deserialize_bytes(SignedVisitor)
    }
}

// TODO: Tests
