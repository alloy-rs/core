#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    clippy::missing_const_for_fn
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

// Used in Serde tests.
#[cfg(test)]
use serde_json as _;

pub mod aliases;
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
pub use signed::{const_eq, BigIntConversionError, ParseSignedError, Sign, Signed};

mod utils;
pub use utils::*;

pub use ruint::{self, uint, Uint};

// Not public API.
#[doc(hidden)]
pub mod private {
    pub use derive_more;
    pub use getrandom;

    #[cfg(feature = "rlp")]
    pub use ethers_rlp;

    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "arbitrary")]
    pub use arbitrary;
    #[cfg(feature = "arbitrary")]
    pub use proptest;
    #[cfg(feature = "arbitrary")]
    pub use proptest_derive;
}
