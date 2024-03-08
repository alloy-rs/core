// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
#![cfg_attr(feature = "std", allow(unused_imports))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;

pub extern crate alloy_sol_type_parser as parser;

use serde::{Deserialize, Serialize};

mod abi;
pub use abi::{ContractObject, IntoItems, Items, JsonAbi};

mod item;
pub use item::{AbiItem, Constructor, Error, Event, Fallback, Function, Receive};

mod param;
pub use param::{EventParam, Param};

mod internal_type;
pub use internal_type::InternalType;

mod to_sol;
pub use to_sol::ToSolConfig;

pub(crate) mod utils;

/// A JSON ABI function's state mutability.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    /// Pure functions promise not to read from or modify the state.
    Pure,
    /// View functions promise not to modify the state.
    View,
    /// Nonpayable functions promise not to receive Ether.
    ///
    /// This is the solidity default: <https://docs.soliditylang.org/en/latest/abi-spec.html#json>
    ///
    /// The state mutability nonpayable is reflected in Solidity by not specifying a state
    /// mutability modifier at all.
    #[default]
    NonPayable,
    /// Payable functions make no promises.
    Payable,
}

impl StateMutability {
    /// Returns the string representation of the state mutability.
    #[inline]
    pub const fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Pure => Some("pure"),
            Self::View => Some("view"),
            Self::Payable => Some("payable"),
            Self::NonPayable => None,
        }
    }
}
