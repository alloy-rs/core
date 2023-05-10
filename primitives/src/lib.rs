#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    clippy::missing_const_for_fn
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! Fundamental types shared by [reth](https://github.com/paradigmxyz/reth) [revm](https://github.com/bluealloy/revm) and [ethers](https://github.com/gakonst/ethers-rs).

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod bits;
pub use bits::{Address, AddressError, FixedBytes, B256};

mod signed;
pub use signed::{
    aliases::{self, I160, I256},
    const_eq, BigIntConversionError, ParseSignedError, Sign, Signed,
};

mod utils;
pub use utils::{keccak256, Hasher, Keccak};

// ruint reexports
pub use ruint::{
    aliases::{B128 as H128, B64 as H64, U128, U256, U64},
    uint,
};

#[doc(hidden)]
pub use derive_more;
