use super::Bytes;
use ethers_rlp::{Decodable, Encodable};

impl Encodable for Bytes {
    fn length(&self) -> usize {
        self.0.length()
    }

    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.0.encode(out)
    }
}

impl Decodable for Bytes {
    fn decode(buf: &mut &[u8]) -> Result<Self, ethers_rlp::DecodeError> {
        Ok(Self(bytes::Bytes::decode(buf)?))
    }
}
