#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate alloc;

#[cfg(feature = "postgres")]
pub mod postgres;

pub mod aliases;
#[doc(no_inline)]
pub use aliases::{
    BlockHash, BlockNumber, ChainId, Selector, StorageKey, StorageValue, TxHash, TxIndex, TxNumber,
    B128, B256, B512, B64, I128, I16, I160, I256, I32, I64, I8, U128, U16, U160, U256, U32, U512,
    U64, U8,
};

#[macro_use]
mod bits;
pub use bits::{
    Address, AddressError, Bloom, BloomInput, FixedBytes, Function, BLOOM_BITS_PER_ITEM,
    BLOOM_SIZE_BITS, BLOOM_SIZE_BYTES,
};

#[path = "bytes/mod.rs"]
mod bytes_;
pub use self::bytes_::Bytes;

mod log;
pub use log::Log;

mod sealed;
pub use sealed::{Sealable, Sealed};

mod signed;
pub use signed::{BigIntConversionError, ParseSignedError, Sign, Signed};

pub mod utils;
pub use utils::{eip191_hash_message, keccak256};

#[doc(no_inline)]
pub use {
    ::bytes,
    ::hex,
    hex_literal::{self, hex},
    ruint::{self, Uint},
    tiny_keccak::{self, Hasher, Keccak},
};

#[cfg(feature = "asm-keccak")]
#[doc(no_inline)]
pub use keccak_asm::{self, digest, Keccak256};

/// Re-export of [`ruint::uint!`] for convenience. Note that users of this macro
/// must also add [`ruint`] to their `Cargo.toml` as a dependency.
#[doc(inline)]
pub use ruint::uint;

#[cfg(feature = "serde")]
#[doc(no_inline)]
pub use ::hex::serde as serde_hex;

/// 20-byte [fixed byte-array][FixedBytes] type.
///
/// You'll likely want to use [`Address`] instead, as it is a different type
/// from `FixedBytes<20>`, and implements methods useful for working with
/// Ethereum addresses.
///
/// If you are sure you want to use this type, and you don't want the
/// deprecation warning, you can use `aliases::B160`.
#[deprecated(
    since = "0.3.2",
    note = "you likely want to use `Address` instead. \
            `B160` and `Address` are different types, \
            see this type's documentation for more."
)]
pub type B160 = FixedBytes<20>;

// Not public API.
#[doc(hidden)]
pub mod private {
    pub use alloc::vec::Vec;
    pub use core::{
        self,
        borrow::{Borrow, BorrowMut},
        cmp::Ordering,
        prelude::rust_2021::*,
    };
    pub use derive_more;

    #[cfg(feature = "getrandom")]
    pub use getrandom;

    #[cfg(feature = "rand")]
    pub use rand;

    #[cfg(feature = "rlp")]
    pub use alloy_rlp;

    #[cfg(feature = "ssz")]
    pub use ssz;

    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "arbitrary")]
    pub use {arbitrary, derive_arbitrary, proptest, proptest_derive};
}
