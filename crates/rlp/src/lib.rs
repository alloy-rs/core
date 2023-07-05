#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;

mod decode;
mod encode;
mod header;

pub use bytes::{Buf, BufMut};

pub use decode::{Decodable, DecodeError, Rlp};
pub use encode::{
    const_add, encode_fixed_size, encode_iter, encode_list, length_of_length, list_length,
    Encodable, MaxEncodedLen, MaxEncodedLenAssoc,
};
pub use header::Header;

#[cfg(feature = "derive")]
pub use alloy_rlp_derive::{
    RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper, RlpMaxEncodedLen,
};

/// RLP prefix byte for 0-length string.
pub const EMPTY_STRING_CODE: u8 = 0x80;

/// RLP prefix byte for a 0-length array.
pub const EMPTY_LIST_CODE: u8 = 0xC0;
