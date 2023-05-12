#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! RLP Encoding and Decoding

#[cfg(feature = "alloc")]
extern crate alloc;

// Used in alloc tests.
#[cfg(test)]
#[allow(unused_extern_crates)]
extern crate hex_literal;

mod decode;
mod encode;
mod types;

pub use bytes::{Buf, BufMut};

pub use decode::{Decodable, DecodeError, Rlp};
pub use encode::{
    const_add, encode_fixed_size, encode_iter, encode_list, length_of_length, list_length,
    Encodable, MaxEncodedLen, MaxEncodedLenAssoc,
};
pub use types::*;

#[cfg(feature = "derive")]
pub use ethers_rlp_derive::{
    RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper, RlpMaxEncodedLen,
};
