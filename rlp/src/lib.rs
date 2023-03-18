#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod decode;
mod encode;
mod types;

pub use bytes::BufMut;

pub use decode::{Decodable, DecodeError, Rlp};
pub use encode::{
    const_add, encode_fixed_size, encode_iter, encode_list, length_of_length, list_length,
    Encodable, MaxEncodedLen, MaxEncodedLenAssoc,
};
pub use types::*;

// #[cfg(feature = "derive")]
// pub use reth_rlp_derive::{
//     RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper, RlpMaxEncodedLen,
// };
