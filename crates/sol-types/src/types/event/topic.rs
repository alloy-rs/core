use crate::{sol_data, token::WordToken, SolType};
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
    /// Encodes `self` as an element of an indexed event parameter.
    ///
    /// This is not the same as `encode_topic`.
    fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>);

    /// Encodes `self` as an indexed event parameter.
    fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken;
}

impl EventTopic for sol_data::String {
    fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        encode_topic_bytes(rust.borrow().as_bytes(), out);
    }

    fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
        WordToken(keccak256(rust.borrow()))
    }
}

impl EventTopic for sol_data::Bytes {
    fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        encode_topic_bytes(rust.borrow(), out);
    }

    fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
        WordToken(keccak256(rust.borrow()))
    }
}

// impl<const N: usize> EventTopic for sol_data::FixedBytes<N> {
//     fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut
// Vec<u8>) {         encode_topic_bytes(rust.borrow(), out);
//     }

//     fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
//         WordToken(keccak256(rust.borrow()))
//     }
// }

impl<T: EventTopic> EventTopic for sol_data::Array<T> {
    fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        for t in rust.borrow() {
            T::encode_topic_raw(t, out);
        }
    }

    fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
        let r = rust.borrow();
        let mut out = Vec::with_capacity(r.len() * 32);
        Self::encode_topic_raw(r, &mut out);
        WordToken(keccak256(out))
    }
}

impl<const N: usize, T: EventTopic> EventTopic for sol_data::FixedArray<T, N> {
    fn encode_topic_raw<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        for t in rust.borrow() {
            T::encode_topic_raw(t, out);
        }
    }

    fn encode_topic<B: Borrow<Self::RustType>>(rust: B) -> WordToken {
        let r = rust.borrow();
        let mut out = Vec::with_capacity(r.len() * 32);
        Self::encode_topic_raw(r, &mut out);
        WordToken(keccak256(out))
    }
}

fn encode_topic_bytes(sl: &[u8], out: &mut Vec<u8>) {
    let padding = 32 - sl.len() % 32;
    out.reserve(sl.len() + padding);

    static PAD: [u8; 32] = [0; 32];
    out.extend_from_slice(sl);
    out.extend_from_slice(&PAD[..padding]);
}

// impl<T: SolStruct> SolTopic for T {
//     fn encode_topic(&self) -> WordToken {}
// }
