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
//! This library provides tools for expressing Solidity types in Rust, and for
//! encoding these representations into ABI blobs suitable for smart contract
//! processing. In other words, you can represent your smart contract args in
//! native Rust, easily encode them to pass to a smart contract, and easily
//! decode the smart contract return values .
//!
//! We do this by representing solidity types as via the [`SolType`] trait.
//! This trait maps Solidity types to Rust types via the associated
//! [`SolType::RustType`].
//!
//! Each [`SolType`] also has a [`SolType::TokenType`]. This is the
//! intermediate representation of the data suitable for ABI encoding. The ABI
//! `encode` and `decode` methods operate on objects implementing
//! [`TokenType`].
//!
//! ```
//! use ethers_abi_enc::sol_type::*;
//! # pub fn main() -> ethers_abi_enc::AbiResult<()> {
//!     // Represent a solidity type in rust
//!     type MySolType = FixedArray<Bool, 2>;
//!
//!     let data = [true, false];
//!     let validate = true;
//!
//!     // SolTypes expose their solidity name :)
//!     assert_eq!(&MySolType::sol_type_name(), "bool[2]");
//!
//!     // SolTypes are used to transform Rust into ABI blobs, and back.
//!     let encoded: Vec<u8> = MySolType::encode(data);
//!     let decoded: [bool; 2] = MySolType::decode(&encoded, validate)?;
//!     assert_eq!(data, decoded);
//!     # Ok(())
//! # }
//! ```
//!
//! See the [`SolType`] docs for an implementer's guide.
//!
//! ## Tokenization/Detokenization
//!
//! The process of converting from a Rust type to a to a token is called
//! "Tokenization". Typical users will not access tokenizaiton directly.
//! Power users should use the [`SolType::tokenize()`] and
//! [`SolType::detokenize()`] methods.
//!
//! When implementing your own [`SolType`], a variety of `From` impls have been
//! provided on the token structs to aid in converting from Rust data to tokens.
//!
//! ## Encoding/Decoding
//!
//! The process of converting from a [`TokenType`] to a serialized ABI blob is
//! called "Encoding". It is the reciprocal of decoding.
//!
//! ABI encoding and decoding operates on sequences of tokens.
//!
//! The [`SolType`] encoding and decoding methods operate on Rust types. We
//! recommend users use them wherever possible. We do not recommend that users
//! interact with Tokens except when implementing their own [`SolType`];
//!
//! ### `encode/decode_single`
//!
//! [`SolType::encode_single()`] and [`crate::encode_single()`] operate on a
//! single token. They wrap this token in a tuple, and pass it to the encoder.
//! Use this interface when abi-encoding a single token. This is suitable for
//! encoding a type in isolation, or for encoding parameters for single-param
//! functions.
//!
//! ### `encode/decode_params`
//!
//! [`SolType::encode_params()`] and [`crate::encode_params()`] operate on a
//! sequence. If the sequence is a tuple, the tuple is inferred to be a set of
//! Solidity function parameters,
//!
//! The corresponding [`SolType::decode_params()`] and
//! [`crate::decode_params()`] reverse this operation, decoding a tuple from a
//! blob.
//!
//! [`SolType::encode_function`] adds a selector to front of the encoded params
//!
//! This is used to encode the parameters for a solidity function
//!
//! ### `encode/decode`
//!
//! [`SolType::encode()`] and [`crate::encode()`] operate on a sequence of
//! tokens. This sequence is inferred not to be function parameters.
//!
//! This is the least useful one. Most users will not need it.

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

use ethers_primitives::B256;
#[cfg(not(feature = "std"))]
use no_std_prelude::*;

mod decoder;
pub use decoder::{decode, decode_params, decode_single};

mod encoder;
pub use encoder::{encode, encode_params, encode_single};

mod token;
pub use token::{DynSeqToken, FixedSeqToken, PackedSeqToken, TokenType, WordToken};

mod errors;
pub use errors::{AbiResult, Error};

/// Solidity Types
pub mod sol_type;
pub use sol_type::SolType;

mod util;

/// The Word type for ABI Encoding
pub type Word = B256;
