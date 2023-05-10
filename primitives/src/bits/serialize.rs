use super::FixedBytes;
use alloc::string::String;
use core::result::Result;

impl<const N: usize> serde::Serialize for FixedBytes<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format_args!("{}", self))
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for FixedBytes<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let expected = 2 * N + 2;
        let s = String::deserialize(deserializer)?;
        if s.len() != expected {
            return Err(serde::de::Error::custom(format!(
                "Expected exactly {expected} chars, including a 0x prefix. Got {}",
                s.len()
            )));
        }
        s.parse().map_err(serde::de::Error::custom)
    }
}
