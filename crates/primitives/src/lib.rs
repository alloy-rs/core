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
extern crate alloc;

// Used in Serde tests.
#[cfg(test)]
use serde as _;
#[cfg(test)]
use serde_json as _;

pub mod aliases;
#[doc(no_inline)]
pub use aliases::{
    BlockHash, BlockNumber, ChainId, Selector, StorageKey, StorageValue, TxHash, TxIndex, TxNumber,
    B128, B160, B256, B512, B64, I128, I16, I160, I256, I32, I64, I8, U128, U16, U160, U256, U32,
    U512, U64, U8,
};

mod bits;
pub use bits::{
    Address, AddressError, Bloom, BloomInput, FixedBytes, BLOOM_BITS_PER_ITEM, BLOOM_SIZE_BITS,
    BLOOM_SIZE_BYTES,
};

mod bytes;
pub use self::bytes::Bytes;

#[cfg(feature = "getrandom")]
mod impl_core;

mod signed;
pub use signed::{BigIntConversionError, ParseSignedError, Sign, Signed};

mod utils;
pub use utils::keccak256;

#[doc(no_inline)]
pub use ::hex;
#[doc(no_inline)]
pub use hex_literal::{self, hex};
#[doc(no_inline)]
pub use ruint::{self, uint, Uint};
#[doc(no_inline)]
pub use tiny_keccak::{self, Hasher, Keccak};

#[cfg(feature = "serde")]
#[doc(no_inline)]
pub use ::hex::serde as serde_hex;

// Not public API.
#[doc(hidden)]
pub mod private {
    pub use core::{
        self,
        borrow::{Borrow, BorrowMut},
        cmp::Ordering,
        prelude::rust_2021::*,
    };
    pub use derive_more;

    #[cfg(feature = "getrandom")]
    pub use getrandom;

    #[cfg(feature = "rlp")]
    pub use alloy_rlp;

    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "arbitrary")]
    pub use {arbitrary, derive_arbitrary, proptest, proptest_derive};
}
