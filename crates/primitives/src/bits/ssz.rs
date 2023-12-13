use crate::FixedBytes;
use alloc::vec::Vec;
use ssz::{Decode, DecodeError, Encode};

impl<const N: usize> Encode for FixedBytes<N> {
    #[inline]
    fn is_ssz_fixed_len() -> bool {
        true
    }

    #[inline]
    fn ssz_bytes_len(&self) -> usize {
        N
    }

    #[inline]
    fn ssz_fixed_len() -> usize {
        N
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Address, Bloom};

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

    #[test]
    fn test_ssz_fixed_lengths() {
        assert_eq!(<u64 as Encode>::ssz_fixed_len(), 8);
        assert_eq!(<u64 as Decode>::ssz_fixed_len(), 8);

        assert_eq!(<[u8; 32] as Encode>::ssz_fixed_len(), 32);
        assert_eq!(<[u8; 32] as Decode>::ssz_fixed_len(), 32);

        assert_eq!(<Bloom as Encode>::ssz_fixed_len(), 256);
        assert_eq!(<Bloom as Decode>::ssz_fixed_len(), 256);

        assert_eq!(<Address as Encode>::ssz_fixed_len(), 20);
        assert_eq!(<Address as Decode>::ssz_fixed_len(), 20);

        assert_eq!(<FixedBytes<32> as Encode>::ssz_fixed_len(), 32);
        assert_eq!(<FixedBytes<32> as Decode>::ssz_fixed_len(), 32);

        assert_eq!(<FixedBytes<4> as Encode>::ssz_fixed_len(), 4);
        assert_eq!(<FixedBytes<4> as Decode>::ssz_fixed_len(), 4);
    }
}
