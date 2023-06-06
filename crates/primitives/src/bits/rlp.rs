use super::FixedBytes;
use alloy_rlp::{impl_max_encoded_len, length_of_length, Decodable, Encodable};

impl<const N: usize> Decodable for FixedBytes<N> {
    #[inline]
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::DecodeError> {
        Decodable::decode(buf).map(Self)
    }
}

impl<const N: usize> Encodable for FixedBytes<N> {
    #[inline]
    fn length(&self) -> usize {
        self.0.length()
    }

    #[inline]
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.0.encode(out)
    }
}

// cannot implement this with const generics due to Rust issue #76560:
// https://github.com/rust-lang/rust/issues/76560
macro_rules! fixed_bytes_max_encoded_len {
    ($($sz:literal),+) => {$(
        impl_max_encoded_len!(FixedBytes<$sz>, $sz + length_of_length($sz));
    )+};
}

fixed_bytes_max_encoded_len!(0, 1, 2, 4, 8, 16, 20, 32, 64, 128, 256, 512, 1024);
