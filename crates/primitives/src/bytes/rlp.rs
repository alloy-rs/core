use super::Bytes;
use alloy_rlp::{Decodable, Encodable};

impl Encodable for Bytes {
    fn length(&self) -> usize {
        self.0.length()
    }

    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.0.encode(out)
    }
}

impl Decodable for Bytes {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::DecodeError> {
        Ok(Self(bytes::Bytes::decode(buf)?))
    }
}
