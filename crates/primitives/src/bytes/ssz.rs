use crate::Bytes;
use alloc::vec::Vec;
use ssz::{Decode, DecodeError, Encode};

impl Encode for Bytes {
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

impl Decode for Bytes {
    fn is_ssz_fixed_len() -> bool {
        false
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(bytes.to_vec().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_encode_decode_ssz;

    use super::*;

    test_encode_decode_ssz!(
        test_encode_decode_bytes,
        Bytes,
        [bytes!("01234567"), bytes!("6394198df16000526103ff60206004601c335afa6040516060f3")]
    );
}
