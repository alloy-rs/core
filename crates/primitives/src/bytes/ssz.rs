use crate::Bytes;
use alloc::vec::Vec;
use ssz::{Decode, DecodeError, Encode};

impl Encode for Bytes {
    #[inline]
    fn is_ssz_fixed_len() -> bool {
        false
    }

    #[inline]
    fn ssz_bytes_len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn ssz_append(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.0);
    }

    #[inline]
    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Decode for Bytes {
    #[inline]
    fn is_ssz_fixed_len() -> bool {
        false
    }

    #[inline]
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        Ok(bytes.to_vec().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_encode_decode_ssz {
        ($test_name:ident, $type:ty, [$( $value:expr ),*]) => {
            #[test]
            fn $test_name() {
                $(
                    let expected: $type = $value;
                    let encoded = ssz::Encode::as_ssz_bytes(&expected);
                    let actual: $type = ssz::Decode::from_ssz_bytes(&encoded).unwrap();
                    assert_eq!(expected, actual, "Failed for value: {:?}", $value);
                )*
            }
        };
    }

    test_encode_decode_ssz!(
        test_encode_decode_bytes,
        Bytes,
        [bytes!("01234567"), bytes!("6394198df16000526103ff60206004601c335afa6040516060f3")]
    );
}
