use super::Bytes;
use core::result::Result;

impl serde::Serialize for Bytes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        hex::serialize(self, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Bytes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        hex::deserialize::<'de, D, alloc::vec::Vec<u8>>(deserializer).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let bytes = Bytes::from_static(&[1, 35, 69, 103, 137, 171, 205, 239]);
        let ser = serde_json::to_string(&bytes).unwrap();
        assert_eq!(ser, "\"0x0123456789abcdef\"");
        assert_eq!(serde_json::from_str::<Bytes>(&ser).unwrap(), bytes);
    }
}
