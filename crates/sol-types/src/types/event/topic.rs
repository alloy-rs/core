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
    /// The number of bytes this type occupies in another topic's preimage,
    /// usually a multiple of 32.
    ///
    /// This should be used in conjunction with [`encode_topic_preimage`] to
    /// construct the preimage of a complex topic.
    ///
    /// [`encode_topic_preimage`]: EventTopic::encode_topic_preimage
    fn topic_preimage_length<B: Borrow<Self::RustType>>(rust: B) -> usize;

    /// Encodes this type as preimage bytes which are then hashed in
    /// complex types' [`encode_topic`][EventTopic::encode_topic].
    ///
    /// See the [Solidity ABI spec][ref] for more details.
    ///
    /// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#encoding-of-indexed-event-parameters
    fn encode_topic_preimage<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>);

    /// Indexed event parameter encoding.
    ///
    /// Note that this is different from [`encode_topic_preimage`] and
    /// [`SolType::encode`]. See the [Solidity ABI spec][ref] for more details.
    ///
    /// [`encode_topic_preimage`]: EventTopic::encode_topic_preimage
    /// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#encoding-of-indexed-event-parameters
    fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken;
}

// Single word types: encoded as just the single word
macro_rules! word_impl {
    ($t:ty) => {
        #[inline]
        fn topic_preimage_length<B: Borrow<Self::RustType>>(_: B) -> usize {
            32
        }

        #[inline]
        fn encode_topic_preimage<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
            out.extend(<$t as SolType>::tokenize(rust).0 .0);
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

impl<const N: usize> EventTopic for FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    word_impl!(FixedBytes<N>);
}

// Bytes-like types - preimage encoding: bytes padded to 32; hash: the bytes
macro_rules! bytes_impl {
    ($t:ty) => {
        #[inline]
        fn topic_preimage_length<B: Borrow<Self::RustType>>(rust: B) -> usize {
            crate::util::next_multiple_of_32(rust.borrow().len())
        }

        #[inline]
        fn encode_topic_preimage<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
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

// Complex types - preimage encoding and hash: iter each element
macro_rules! array_impl {
    ($T:ident) => {
        #[inline]
        fn topic_preimage_length<B: Borrow<Self::RustType>>(rust: B) -> usize {
            rust.borrow().iter().map($T::topic_preimage_length).sum()
        }

        #[inline]
        fn encode_topic_preimage<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
            let b = rust.borrow();
            out.reserve(Self::topic_preimage_length(b));
            for t in b {
                $T::encode_topic_preimage(t, out);
            }
        }

        #[inline]
        fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
            let mut out = Vec::new();
            Self::encode_topic_preimage(rust.borrow(), &mut out);
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
    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: EventTopic,)+> EventTopic for ($($t,)+) {
            #[inline]
            fn topic_preimage_length<B: Borrow<Self::RustType>>(rust: B) -> usize {
                let ($($t,)+) = rust.borrow();
                0usize $( + <$t>::topic_preimage_length($t) )+
            }

            #[inline]
            fn encode_topic_preimage<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
                let b @ ($($t,)+) = rust.borrow();
                out.reserve(Self::topic_preimage_length(b));
                $(
                    <$t>::encode_topic_preimage($t, out);
                )+
            }

            #[inline]
            fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
                let mut out = Vec::new();
                Self::encode_topic_preimage(rust.borrow(), &mut out);
                WordToken(keccak256(out))
            }
        }
    };
}

impl EventTopic for () {
    #[inline]
    fn topic_preimage_length<B: Borrow<Self::RustType>>(_: B) -> usize {
        0
    }

    #[inline]
    fn encode_topic_preimage<B: Borrow<Self::RustType>>(_: B, _: &mut Vec<u8>) {}

    #[inline]
    fn encode_topic<B: Borrow<Self::RustType>>(_: B) -> WordToken {
        WordToken::default()
    }
}

all_the_tuples!(tuple_impls);

fn encode_topic_bytes(sl: &[u8], out: &mut Vec<u8>) {
    let padding = 32 - sl.len() % 32;
    out.reserve(sl.len() + padding);

    static PAD: [u8; 32] = [0; 32];
    out.extend_from_slice(sl);
    out.extend_from_slice(&PAD[..padding]);
}
