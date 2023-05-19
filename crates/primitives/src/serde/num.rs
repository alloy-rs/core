// TODO: Remove once Uint supports serde with both numbers and strings.

use crate::U256;
use alloc::string::String;
use core::fmt;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

/// Helper type to parse a numeric value which can be stringified.
///
/// Use [`deserialize_stringified_numeric`] and
/// [`deserialize_stringified_numeric_opt`] instead.
struct StringifiedNumeric(U256);

impl<'de> Deserialize<'de> for StringifiedNumeric {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StringifiedNumericVisitor)
    }
}

struct StringifiedNumericVisitor;

impl Visitor<'_> for StringifiedNumericVisitor {
    type Value = StringifiedNumeric;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a stringified numeric value")
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(StringifiedNumeric(U256::from(v)))
    }

    fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
        Ok(StringifiedNumeric(U256::from(v)))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse()
            .map(StringifiedNumeric)
            .map_err(serde::de::Error::custom)
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(&v)
    }
}

/// Supports parsing numbers as strings.
///
/// Use with `#[serde(deserialize_with = "deserialize_stringified_numeric")]`.
pub fn deserialize_stringified_numeric<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    StringifiedNumeric::deserialize(deserializer).map(|x| x.0)
}

/// Supports parsing numbers as strings.
///
/// Use with
/// `#[serde(deserialize_with = "deserialize_stringified_numeric_opt")]`.
pub fn deserialize_stringified_numeric_opt<'de, D>(
    deserializer: D,
) -> Result<Option<U256>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Some(num) = Option::<StringifiedNumeric>::deserialize(deserializer)? {
        Ok(Some(num.0))
    } else {
        Ok(None)
    }
}

/// Deserializes the input into an `Option<U256>`, using [`from_int_or_hex`] to
/// deserialize the inner value.
///
/// Use with `#[serde(deserialize_with = "from_int_or_hex_opt")]`.
pub fn from_int_or_hex_opt<'de, D>(deserializer: D) -> Result<Option<U256>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<NumberOrHexU256>::deserialize(deserializer)?.map(Into::into))
}

/// Deserializes the input into a U256, accepting both 0x-prefixed hex and
/// decimal strings with arbitrary precision.
///
/// Use with `#[serde(deserialize_with = "from_int_or_hex")]`.
pub fn from_int_or_hex<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(NumberOrHexU256::deserialize(deserializer)?.into())
}

#[derive(Deserialize)]
#[serde(untagged)]
enum NumberOrHexU256 {
    Int(u64),
    Hex(U256),
}

impl From<NumberOrHexU256> for U256 {
    fn from(value: NumberOrHexU256) -> Self {
        match value {
            NumberOrHexU256::Int(num) => U256::from(num),
            NumberOrHexU256::Hex(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u256_int_or_hex() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct V(#[serde(deserialize_with = "from_int_or_hex")] U256);

        proptest::proptest!(|(value: u64)| {
            let u256_val = U256::from(value);

            let num_obj = serde_json::to_string(&value).unwrap();
            let hex_obj = serde_json::to_string(&u256_val).unwrap();

            let int_val: V = serde_json::from_str(&num_obj).unwrap();
            let hex_val = serde_json::from_str(&hex_obj).unwrap();
            assert_eq!(int_val, hex_val);
        });
    }

    #[test]
    fn test_u256_int_or_hex_opt() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct V(#[serde(deserialize_with = "from_int_or_hex_opt")] Option<U256>);

        let null = serde_json::to_string(&None::<U256>).unwrap();
        let val: V = serde_json::from_str(&null).unwrap();
        assert!(val.0.is_none());

        proptest::proptest!(|(value: u64)| {
            let u256_val = U256::from(value);

            let num_obj = serde_json::to_string(&value).unwrap();
            let hex_obj = serde_json::to_string(&u256_val).unwrap();

            let int_val:V = serde_json::from_str(&num_obj).unwrap();
            let hex_val =  serde_json::from_str(&hex_obj).unwrap();
            assert_eq!(int_val, hex_val);
            assert_eq!(int_val.0, Some(u256_val));
        });
    }

    // <https://github.com/gakonst/ethers-rs/issues/2353>
    #[test]
    fn deserialize_stringified() {
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct TestValues {
            #[serde(deserialize_with = "deserialize_stringified_numeric")]
            value_1: U256,
            #[serde(deserialize_with = "deserialize_stringified_numeric")]
            value_2: U256,
            #[serde(deserialize_with = "deserialize_stringified_numeric")]
            value_3: U256,
            #[serde(deserialize_with = "deserialize_stringified_numeric")]
            value_4: U256,
        }

        let data = r#"
            {
                "value_1": "750000000000000000",
                "value_2": "21000000000000000",
                "value_3": "0",
                "value_4": "1"
            }
        "#;

        let deserialized: TestValues = serde_json::from_str(data).unwrap();
        let expected = TestValues {
            value_1: U256::from(750_000_000_000_000_000u64),
            value_2: U256::from(21_000_000_000_000_000u64),
            value_3: U256::from(0u64),
            value_4: U256::from(1u64),
        };
        assert_eq!(deserialized, expected);
    }
}
