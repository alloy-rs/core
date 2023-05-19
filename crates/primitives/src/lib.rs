#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    clippy::missing_const_for_fn
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate alloc;

pub mod aliases;
pub use aliases::{
    BlockHash, BlockNumber, ChainId, Selector, StorageKey, StorageValue, TxHash, TxIndex, TxNumber,
    I128, I16, I256, I32, I64, I8, U128, U16, U256, U32, U512, U64, U8,
};

mod bits;
pub use bits::{
    Address, AddressError, Bloom, BloomInput, BloomRef, FixedBytes, B128, B256, B512, B64,
    BLOOM_BITS, BLOOM_SIZE,
};

mod bytes;
pub use self::bytes::Bytes;

#[cfg(feature = "serde")]
pub mod serde;

mod signed;
pub use signed::{const_eq, BigIntConversionError, ParseSignedError, Sign, Signed};

mod utils;
pub use utils::*;

pub use ruint::{self, uint, Uint};

// Not public API.
#[doc(hidden)]
pub use derive_more;
