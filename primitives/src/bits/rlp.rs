use super::{Address, B256};

macro_rules! fixed_hash_impl {
    ($t:ty) => {
        impl ethers_rlp::Decodable for $t {
            fn decode(buf: &mut &[u8]) -> Result<Self, ethers_rlp::DecodeError> {
                ethers_rlp::Decodable::decode(buf).map(Self)
            }
        }
        impl ethers_rlp::Encodable for $t {
            fn length(&self) -> usize {
                self.0.length()
            }

            fn encode(&self, out: &mut dyn bytes::BufMut) {
                self.0.encode(out)
            }
        }
        ethers_rlp::impl_max_encoded_len!($t, {
            ethers_rlp::length_of_length(<$t>::len_bytes()) + <$t>::len_bytes()
        });
    };
}

fixed_hash_impl!(Address);
fixed_hash_impl!(B256);
