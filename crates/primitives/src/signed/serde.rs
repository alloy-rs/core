use crate::Sign;

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
            let (sign, abs) = self.into_sign_and_abs();
            let le_bytes = abs.as_le_bytes_trimmed();
            let mut buf = [0u8; 1 + 32];
            buf[0] = sign.is_positive() as u8;
            buf[1..1 + le_bytes.len()].copy_from_slice(&le_bytes);
            serializer.serialize_bytes(&buf[0..le_bytes.len() + 1])
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
            let sign = if v[0] == 1 { Sign::Positive } else { Sign::Negative };
            let u256 = Uint::<BITS, LIMBS>::try_from_le_slice(&v[1..])
                .ok_or(de::Error::custom("Expected a valid unsigned integer"))?;
            Ok(Signed::overflowing_from_sign_and_abs(sign, u256).0)
        }
    }
    if deserializer.is_human_readable() {
        deserializer.deserialize_any(SignedVisitor)
    } else {
        deserializer.deserialize_bytes(SignedVisitor)
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use crate::I256;

    const TEST_VALS: [&str; 5] = [
        "12345681238128738123",
        "-1239373781294184527124318238",
        "99999999999999999999999999999999999999999999999999999999999",
        "57896044618658097711785492504343953926634992332820282019728792003956564819967",
        "-57896044618658097711785492504343953926634992332820282019728792003956564819968",
    ];

    #[test]
    fn serde_json_roundtrip() {
        for val in TEST_VALS {
            let signed = I256::from_str(val).unwrap();
            let ser = serde_json::to_string(&signed).unwrap();
            assert_eq!(serde_json::from_str::<I256>(&ser).unwrap(), signed);
            assert_eq!(serde_json::from_str::<I256>(&format!("\"{}\"", val)).unwrap(), signed);
        }
    }

    #[test]
    fn serde_bincode_roundtrip() {
        for val in TEST_VALS {
            let signed = I256::from_str(val).unwrap();
            let ser = bincode::serialize(&signed).unwrap();
            assert!(ser.len() <= 64);
            assert_eq!(bincode::deserialize::<I256>(&ser).unwrap(), signed);
        }
    }
}
