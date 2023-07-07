use super::FixedBytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<const N: usize> Serialize for FixedBytes<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut buf = hex::Buffer::<N, true>::new();
        serializer.serialize_str(buf.format(&self.0))
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedBytes<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        hex::deserialize::<'de, D, [u8; N]>(deserializer).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let bytes = FixedBytes([0, 0, 0, 0, 1, 35, 69, 103, 137, 171, 205, 239]);
        let ser = serde_json::to_string(&bytes).unwrap();
        assert_eq!(ser, "\"0x000000000123456789abcdef\"");
        assert_eq!(serde_json::from_str::<FixedBytes<12>>(&ser).unwrap(), bytes);
    }
}
