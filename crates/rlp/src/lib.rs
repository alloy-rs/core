#![doc = include_str!("../README.md")]
// This doctest uses derive and alloc, so it cannot be in the README :(
#![cfg_attr(
    all(feature = "derive", feature = "std"),
    doc = r##"

## Usage Example

```rust
use alloy_rlp::{RlpEncodable, RlpDecodable, Decodable, Encodable};

#[derive(Debug, RlpEncodable, RlpDecodable, PartialEq)]
pub struct MyStruct {
    pub a: u64,
    pub b: Vec<u8>,
}

fn main() {
    let my_struct = MyStruct {
        a: 42,
        b: vec![1, 2, 3],
    };

    let mut buffer = Vec::<u8>::new();
    let encoded = my_struct.encode(&mut buffer);
    let decoded = MyStruct::decode(&mut buffer.as_slice()).unwrap();
    assert_eq!(my_struct, decoded);
}
```
"##
)]
#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    clippy::missing_const_for_fn
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
#[allow(unused_imports)]
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
pub use types::{Header, EMPTY_LIST_CODE, EMPTY_STRING_CODE};

#[cfg(feature = "derive")]
pub use alloy_rlp_derive::{
    RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper, RlpMaxEncodedLen,
};
