use crate::FixedBytes;
use alloc::vec::Vec;
use ssz::{Decode, DecodeError, Encode};

impl<const N: usize> Encode for FixedBytes<N> {
    fn is_ssz_fixed_len() -> bool {
        false
    }

    fn ssz_bytes_len(&self) -> usize {
        self.0.len()
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
        false
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() != N {
            return Err(DecodeError::InvalidByteLength { len: bytes.len(), expected: N });
        }

        let fixed_array: [u8; N] = bytes
            .try_into()
            .map_err(|_| DecodeError::InvalidByteLength { len: bytes.len(), expected: N })?;

        Ok(FixedBytes::<N>::from(fixed_array))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        const EXPECTED: FixedBytes<4> = fixed_bytes!("01234567");
        let encoded = EXPECTED.as_ssz_bytes();
        let actual = FixedBytes::<4>::from_ssz_bytes(&encoded).unwrap();
        assert_eq!(EXPECTED, actual);
    }
}
