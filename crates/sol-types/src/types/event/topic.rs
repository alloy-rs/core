use crate::{SolType, abi::token::WordToken, sol_data::*};
use alloc::vec::Vec;
use alloy_primitives::keccak256;

/// A Solidity event topic.
///
/// These types implement a special encoding used only in Solidity indexed event
/// parameters.
///
/// For more details, see the [Solidity reference][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#encoding-of-indexed-event-parameters
///
/// # Implementer's Guide
///
/// It should not be necessary to implement this trait manually. Instead, use
/// the [`sol!`](crate::sol!) procedural macro to parse Solidity syntax into
/// types that implement this trait.
pub trait EventTopic: SolType {
    /// The number of bytes this type occupies in another topic's preimage,
    /// usually a multiple of 32.
    ///
    /// This should be used in conjunction with [`encode_topic_preimage`] to
    /// construct the preimage of a complex topic.
    ///
    /// [`encode_topic_preimage`]: EventTopic::encode_topic_preimage
    fn topic_preimage_length(rust: &Self::RustType) -> usize;

    /// Encodes this type as preimage bytes which are then hashed in
    /// complex types' [`encode_topic`][EventTopic::encode_topic].
    ///
    /// See the [Solidity ABI spec][ref] for more details.
    ///
    /// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#encoding-of-indexed-event-parameters
    fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>);

    /// Indexed event parameter encoding.
    ///
    /// Note that this is different from [`encode_topic_preimage`] and
    /// [`SolType::abi_encode`]. See the [Solidity ABI spec][ref] for more
    /// details.
    ///
    /// [`encode_topic_preimage`]: EventTopic::encode_topic_preimage
    /// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#encoding-of-indexed-event-parameters
    fn encode_topic(rust: &Self::RustType) -> WordToken;
}

// Single word types: encoded as just the single word
macro_rules! word_impl {
    () => {
        #[inline]
        fn topic_preimage_length(_: &Self::RustType) -> usize {
            32
        }

        #[inline]
        fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>) {
            out.extend($crate::private::SolTypeValue::<Self>::stv_to_tokens(rust).0);
        }

        #[inline]
        fn encode_topic(rust: &Self::RustType) -> WordToken {
            $crate::private::SolTypeValue::<Self>::stv_to_tokens(rust)
        }
    };
}

impl EventTopic for Address {
    word_impl!();
}

impl EventTopic for Function {
    word_impl!();
}

impl EventTopic for Bool {
    word_impl!();
}

impl<const BITS: usize> EventTopic for Int<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    word_impl!();
}

impl<const BITS: usize> EventTopic for Uint<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    word_impl!();
}

impl<const N: usize> EventTopic for FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    word_impl!();
}

// Bytes-like types - preimage encoding: bytes padded to 32; hash: the bytes
macro_rules! bytes_impl {
    () => {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            crate::utils::next_multiple_of_32(rust.len())
        }

        #[inline]
        fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>) {
            encode_topic_bytes(rust.as_ref(), out);
        }

        #[inline]
        fn encode_topic(rust: &Self::RustType) -> WordToken {
            WordToken(keccak256(rust))
        }
    };
}

impl EventTopic for String {
    bytes_impl!();
}

impl EventTopic for Bytes {
    bytes_impl!();
}

// Complex types - preimage encoding and hash: iter each element
macro_rules! array_impl {
    ($ty:ident) => {
        #[inline]
        fn topic_preimage_length(rust: &Self::RustType) -> usize {
            rust.iter().map($ty::topic_preimage_length).sum()
        }

        #[inline]
        fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>) {
            out.reserve(Self::topic_preimage_length(rust));
            for t in rust {
                $ty::encode_topic_preimage(t, out);
            }
        }

        #[inline]
        fn encode_topic(rust: &Self::RustType) -> WordToken {
            let mut out = Vec::new();
            Self::encode_topic_preimage(rust, &mut out);
            WordToken(keccak256(out))
        }
    };
}

impl<T: EventTopic> EventTopic for Array<T> {
    array_impl!(T);
}

impl<T: EventTopic, const N: usize> EventTopic for FixedArray<T, N> {
    array_impl!(T);
}

macro_rules! tuple_impls {
    ($count:literal $($ty:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($ty: EventTopic,)+> EventTopic for ($($ty,)+) {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                let ($($ty,)+) = rust;
                0usize $( + <$ty>::topic_preimage_length($ty) )+
            }

            #[inline]
            fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>) {
                let b @ ($($ty,)+) = rust;
                out.reserve(Self::topic_preimage_length(b));
                $(
                    <$ty>::encode_topic_preimage($ty, out);
                )+
            }

            #[inline]
            fn encode_topic(rust: &Self::RustType) -> WordToken {
                let mut out = Vec::new();
                Self::encode_topic_preimage(rust, &mut out);
                WordToken(keccak256(out))
            }
        }
    };
}

impl EventTopic for () {
    #[inline]
    fn topic_preimage_length(_: &Self::RustType) -> usize {
        0
    }

    #[inline]
    fn encode_topic_preimage(_: &Self::RustType, _: &mut Vec<u8>) {}

    #[inline]
    fn encode_topic(_: &Self::RustType) -> WordToken {
        WordToken::default()
    }
}

all_the_tuples!(tuple_impls);

fn encode_topic_bytes(sl: &[u8], out: &mut Vec<u8>) {
    let padding = crate::utils::next_multiple_of_32(sl.len()) - sl.len();
    out.reserve(sl.len() + padding);
    out.extend_from_slice(sl);
    out.extend(core::iter::repeat_n(0, padding));
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;

    fn assert_composite_topic<T: EventTopic>(value: &T::RustType, expected: &[u8]) {
        let mut preimage = Vec::new();
        T::encode_topic_preimage(value, &mut preimage);
        assert_eq!(preimage, expected);
        assert_eq!(T::topic_preimage_length(value), expected.len());
        assert_eq!(T::encode_topic(value).0, keccak256(expected));
    }

    #[test]
    fn empty_members_in_composite_topics() {
        assert_composite_topic::<Array<String>>(&vec![Default::default()], &[]);
        assert_composite_topic::<Array<Bytes>>(&vec![Default::default()], &[]);

        let mut ab = [0; 32];
        ab[..2].copy_from_slice(b"ab");
        assert_composite_topic::<FixedArray<String, 2>>(&[Default::default(), "ab".into()], &ab);
        assert_composite_topic::<Array<Bytes>>(
            &vec![Default::default(), b"ab".to_vec().into()],
            &ab,
        );
        assert_composite_topic::<(String, Bytes)>(&("ab".into(), Default::default()), &ab);

        // A generated struct uses the same member preimages as a tuple.
        crate::sol! {
            struct S {
                uint256 n;
                string s;
                bytes b;
            }
        }
        let mut one = [0; 32];
        one[31] = 1;
        assert_composite_topic::<(Uint<256>, String)>(&(U256::ONE, Default::default()), &one);
        assert_composite_topic::<S>(
            &S { n: U256::ONE, s: Default::default(), b: Default::default() },
            &one,
        );
        assert_composite_topic::<Array<S>>(
            &vec![S { n: U256::ONE, s: Default::default(), b: Default::default() }],
            &one,
        );
    }

    #[test]
    fn bytes_and_string_topic_padding() {
        for (len, padded_len) in [(0, 0), (1, 32), (31, 32), (32, 32), (33, 64), (64, 64)] {
            let bytes = vec![b'a'; len];
            let string = "a".repeat(len);
            let mut expected = vec![0; padded_len];
            expected[..len].copy_from_slice(&bytes);
            assert_composite_topic::<Array<Bytes>>(&vec![bytes.clone().into()], &expected);
            assert_composite_topic::<Array<String>>(&vec![string.clone()], &expected);

            // Top-level bytes and strings hash their contents without padding.
            assert_eq!(Bytes::encode_topic(&bytes.clone().into()).0, keccak256(&bytes));
            assert_eq!(String::encode_topic(&string).0, keccak256(&bytes));
        }
    }

    #[test]
    fn encode_topic_bytes_empty() {
        let mut out = Vec::new();
        encode_topic_bytes(&[], &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn encode_topic_bytes_remainder_modulo_32() {
        let mut out = Vec::new();
        encode_topic_bytes(&[0u8; 11], &mut out);
        assert_eq!(out, vec![0; 32]);
    }

    #[test]
    fn encode_topic_bytes_non_zero_multiple() {
        let mut out = Vec::new();
        encode_topic_bytes(&[0u8; 32], &mut out);
        assert_eq!(out, vec![0; 32]);

        let mut out = Vec::new();
        encode_topic_bytes(&[0u8; 64], &mut out);
        assert_eq!(out, vec![0; 64]);
    }
}
