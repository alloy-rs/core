#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::module_inception)]
#![warn(missing_docs)]

//! ABI implementation

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
pub use decoder::{decode, decode_params, decode_params_validate, decode_validate};

mod encoder;
pub use encoder::{encode, encode_raw};

mod token;
pub use token::Token;

mod errors;
pub use errors::{Error, Result};

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
