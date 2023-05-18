//! Various serde utilities

mod storage_key;
pub use storage_key::*;

mod jsonu256;
pub use jsonu256::*;

mod num;
pub use num::*;

/// serde functions for handling primitive `u64` as [U64](crate::U64)
pub mod u64_hex {
    use crate::U64;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Deserializes an `u64` from [U64] accepting a hex quantity string with
    /// optional 0x prefix
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        U64::deserialize(deserializer).map(|val| val.into_limbs()[0])
    }

    /// Serializes u64 as hex string
    pub fn serialize<S: Serializer>(value: &u64, s: S) -> Result<S::Ok, S::Error> {
        // TODO: Uint serde
        U64::from(*value).serialize(s)
    }
}

pub use hex::serde as hex_bytes;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    #[ignore = "TODO: Uint serde"]
    fn test_hex_u64() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Value {
            #[serde(with = "u64_hex")]
            inner: u64,
        }

        let val = Value { inner: 1000 };
        let s = serde_json::to_string(&val).unwrap();
        assert_eq!(s, "{\"inner\":\"0x3e8\"}");

        let deserialized: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(val, deserialized);
    }
}
