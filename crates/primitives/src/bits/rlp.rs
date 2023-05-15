use super::FixedBytes;
use ethers_rlp::{Decodable, Encodable};

impl<const N: usize> Decodable for FixedBytes<N> {
    #[inline]
    fn decode(buf: &mut &[u8]) -> Result<Self, ethers_rlp::DecodeError> {
        Decodable::decode(buf).map(Self)
    }
}

impl<const N: usize> Encodable for FixedBytes<N> {
    #[inline]
    fn length(&self) -> usize {
        N
    }

    #[inline]
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.0.encode(out)
    }
}
