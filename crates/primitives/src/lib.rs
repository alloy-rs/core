#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg"
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
extern crate alloc;

// Used in Serde tests.
#[cfg(test)]
use serde_json as _;

pub mod aliases;
#[doc(no_inline)]
pub use aliases::{
    BlockHash, BlockNumber, ChainId, Selector, StorageKey, StorageValue, TxHash, TxIndex, TxNumber,
    I128, I16, I256, I32, I64, I8, U128, U16, U256, U32, U512, U64, U8,
};

mod bits;
pub use bits::{
    Address, AddressError, Bloom, BloomInput, FixedBytes, B128, B256, B512, B64,
    BLOOM_BITS_PER_ITEM, BLOOM_SIZE_BITS, BLOOM_SIZE_BYTES,
};

mod bytes;
pub use self::bytes::Bytes;

mod signed;
pub use signed::{BigIntConversionError, ParseSignedError, Sign, Signed};

mod utils;
pub use utils::keccak256;

#[doc(no_inline)]
pub use ruint::{self, uint, Uint};

#[doc(no_inline)]
pub use tiny_keccak::{self, Hasher, Keccak};

// Not public API.
#[doc(hidden)]
pub mod private {
    pub use derive_more;

    #[cfg(feature = "getrandom")]
    pub use getrandom;

    #[cfg(feature = "rlp")]
    pub use alloy_rlp;

    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "arbitrary")]
    pub use arbitrary;
    #[cfg(feature = "arbitrary")]
    pub use derive_arbitrary;
    #[cfg(feature = "arbitrary")]
    pub use proptest;
    #[cfg(feature = "arbitrary")]
    pub use proptest_derive;
}
