// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::module_inception)]
#![warn(missing_docs)]

//! ABI Encoding & Decoding implementation
//!
//! ### How this library works
//!
//! This library maps the solidity type scheme to Rust types via the
//! [`SolType`] trait.
//!
//! Every `SolType` corresponds to exactly 1 rust type, and
//! exactly 1 token type.
//!
//!
//!

#[cfg_attr(not(feature = "std"), macro_use)]
extern crate alloc;
#[cfg(not(feature = "std"))]
mod no_std_prelude {
    pub use alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        string::{self, String, ToString},
        vec::Vec,
    };
}

#[cfg(feature = "std")]
mod no_std_prelude {
    pub use std::borrow::Cow;
}

use ethers_primitives::{B160, B256};
#[cfg(not(feature = "std"))]
use no_std_prelude::*;

mod decoder;
pub use decoder::{decode, decode_params, decode_single};

mod encoder;
pub use encoder::{encode, encode_params, encode_single};

mod token;
pub use token::TokenType;

mod errors;
pub use errors::{AbiResult, Error};

/// Solidity Types
pub mod sol_type;
pub use sol_type::SolType;

pub mod util;

/// EVM Word
pub type Word = B256;
/// EVM Address
pub type Address = B160;
/// FixedBytes type
pub type FixedBytes = Vec<u8>;
/// Dynamic Byte array
pub type Bytes = Vec<u8>;
/// Signed int
pub type Int = B256;
/// Unsigned Int
pub type Uint = B256;
/// Hash
pub type Hash = B256;
