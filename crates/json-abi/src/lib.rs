// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ABI JSON file format for Solidity contracts.
//!
//! Please consult the [specification] for full details.
//!
//! This crate is a reimplementation of [ethabi]. There's only one right way to
//! implement a JSON serialization scheme in rust. So while the internals are
//! nearly-identical, the API is our own.
//!
//! [specification]: https://docs.soliditylang.org/en/latest/abi-spec.html#json
//! [ethabi]: https://github.com/rust-ethereum/ethabi

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_docs,
    unreachable_pub,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_const_for_fn
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;

use serde::{Deserialize, Serialize};

mod abi;
pub use abi::AbiJson;

mod item;
pub use item::{AbiItem, Constructor, Error, Event, Fallback, Function, Receive};

mod param;
pub use param::{EventParam, Param};

pub(crate) mod utils;

/// A JSON ABI function's state mutability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    /// Pure functions promise not to read from or modify the state.
    Pure,
    /// View functions promise not to modify the state.
    View,
    /// Nonpayable functions promise not to receive Ether.
    NonPayable,
    /// Payable functions make no promises
    Payable,
}
