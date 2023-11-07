use crate::{Address, Bloom, FixedBytes};
use alloc::vec::Vec;
use ssz::{Decode, DecodeError, Encode};

impl<const N: usize> Encode for FixedBytes<N> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_bytes_len(&self) -> usize {
        N
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.0);
    }
    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl<const N: usize> Decode for FixedBytes<N> {
    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        N
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() != N {
            return Err(DecodeError::InvalidByteLength { len: bytes.len(), expected: N });
        }

        let mut fixed_array = [0u8; N];
        fixed_array.copy_from_slice(bytes);

        Ok(FixedBytes::<N>(fixed_array))
    }
}

impl_ssz_fixed_len!(Address, 20);
impl_ssz_fixed_len!(Bloom, 256);

#[cfg(test)]
mod tests {
    use crate::{Address, Bloom, FixedBytes};

    test_encode_decode_ssz!(
        test_encode_decode_fixed_bytes32,
        FixedBytes<32>,
        [fixed_bytes!("a1de988600a42c4b4ab089b619297c17d53cffae5d5120d82d8a92d0bb3b78f2")]
    );

    test_encode_decode_ssz!(
        test_encode_decode_fixed_bytes4,
        FixedBytes<4>,
        [fixed_bytes!("01234567")]
    );

    test_encode_decode_ssz!(
        test_encode_decode_bloom,
        Bloom,
        [bloom!(
            "00000000000000000000000000000000
             00000000100000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000002020000000000000000000000
             00000000000000000000000800000000
             10000000000000000000000000000000
             00000000000000000000001000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000"
        )]
    );
    test_encode_decode_ssz!(
        test_encode_decode_address,
        Address,
        [
            address!("2222222222222222222222222222222222222222"),
            address!("0000000000000000000000000000000000012321"),
            address!("0000000000000000000000000000000000000000")
        ]
    );
}
