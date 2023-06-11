use crate::{sol_data::*, token::WordToken, SolType};
use alloc::vec::Vec;
use alloy_primitives::keccak256;
use core::borrow::Borrow;

/// A Solidity event topic.
///
/// These types implement a special encoding used only in Solidity indexed event
/// parameters.
///
/// For more details, see the [Solidity reference][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#encoding-of-indexed-event-parameters
pub trait EventTopic: SolType {
    /// The length of the encoded topic, in bytes.
    fn topic_encoded_length(rust: &Self::RustType) -> usize;

    /// Encodes `self` as an element of an indexed event parameter.
    ///
    /// This is not the same as `encode_topic`.
    fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>);

    /// Encodes `self` as an indexed event parameter.
    fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken;
}

// Single word types: just encode the word (with padding)
macro_rules! word_impl {
    ($t:ty) => {
        #[inline]
        fn topic_encoded_length(_: &Self::RustType) -> usize {
            32
        }

        #[inline]
        fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
            out.extend(Self::encode_topic(rust).0 .0);
        }

        #[inline]
        fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
            <$t as SolType>::tokenize(rust)
        }
    };
}

impl EventTopic for Address {
    word_impl!(Address);
}

impl EventTopic for Bool {
    word_impl!(Bool);
}

impl<const BITS: usize> EventTopic for Int<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    word_impl!(Int<BITS>);
}

impl<const BITS: usize> EventTopic for Uint<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    word_impl!(Uint<BITS>);
}

// Bytes-like types: encode bytes padded to 32; hash the bytes
macro_rules! bytes_impl {
    ($t:ty) => {
        #[inline]
        fn topic_encoded_length(rust: &Self::RustType) -> usize {
            crate::util::round_up_nearest_multiple(rust.len(), 32)
        }

        #[inline]
        fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
            encode_topic_bytes(rust.borrow().as_ref(), out);
        }

        #[inline]
        fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
            WordToken(keccak256(rust.borrow()))
        }
    };
}

impl EventTopic for String {
    bytes_impl!(String);
}

impl EventTopic for Bytes {
    bytes_impl!(Bytes);
}

impl<const N: usize> EventTopic for FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    bytes_impl!(FixedBytes<N>);
}

// Array types: encode each element, then hash the result
macro_rules! array_impl {
    ($t:ty) => {
        #[inline]
        fn topic_encoded_length(rust: &Self::RustType) -> usize {
            rust.iter().map(T::topic_encoded_length).sum()
        }

        #[inline]
        fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
            let b = rust.borrow();
            out.reserve(Self::topic_encoded_length(b));
            for t in b {
                T::encode_topic_raw(t, out);
            }
        }

        #[inline]
        fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
            let r = rust.borrow();
            let mut out = Vec::with_capacity(r.len() * 32);
            Self::encode_topic_raw(r, &mut out);
            WordToken(keccak256(out))
        }
    };
}

impl<T: EventTopic> EventTopic for Array<T> {
    array_impl!(Array<T>);
}

impl<const N: usize, T: EventTopic> EventTopic for FixedArray<T, N> {
    array_impl!(FixedArray<T, N>);
}

macro_rules! tuple_impls {
    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: EventTopic,)+> EventTopic for ($($t,)+) {
            #[inline]
            fn topic_encoded_length(rust: &Self::RustType) -> usize {
                let ($($t,)+) = rust;
                0usize $( + <$t>::topic_encoded_length($t) )+
            }

            #[inline]
            fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
                let b @ ($($t,)+) = rust.borrow();
                out.reserve(Self::topic_encoded_length(b));
                $(
                    <$t>::encode_topic_raw($t, out);
                )+
            }

            #[inline]
            fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
                let b = rust.borrow();
                let mut out = Vec::with_capacity(Self::topic_encoded_length(b));
                Self::encode_topic_raw(b, &mut out);
                WordToken(keccak256(out))
            }
        }
    };
}

all_the_tuples!(tuple_impls);

fn encode_topic_bytes(sl: &[u8], out: &mut Vec<u8>) {
    let padding = 32 - sl.len() % 32;
    out.reserve(sl.len() + padding);

    static PAD: [u8; 32] = [0; 32];
    out.extend_from_slice(sl);
    out.extend_from_slice(&PAD[..padding]);
}
