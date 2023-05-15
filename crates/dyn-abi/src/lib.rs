// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_const_for_fn
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! Dynamic Solidity Type Encoder
//!
//! This library provides a runtime encoder/decoder for Solidity types. It is
//! intended to be used when the Solidity type is not known at compile time.
//! This is particularly useful for EIP-712 signing interfaces.
//!
//! We **strongly** recommend using the static encoder/decoder when possible.
//! The dyanmic encoder/decoder is significantly more expensive, especially for
//! complex types. It is also significantly more error prone, as the mapping
//! between Solidity types and rust types is not enforced by the compiler.
//!
//! ## Example
//!
//! ```
//! # use ethers_dyn_abi::{DynSolType, DynSolValue};
//! // parse a type from a string
//! // limitation: custom structs cannot currently be parsed this way.
//! let my_type: DynSolType = "uint8[2][]".parse().unwrap();
//!
//! // set values
//! let uints = DynSolValue::FixedArray(vec![0u8.into(), 1u8.into()]);
//! let my_values = DynSolValue::Array(vec![uints]);
//!
//! // encode
//! let encoded = my_type.encode_single(my_values.clone()).unwrap();
//!
//! // decode
//! let decoded = my_type.decode_single(&encoded).unwrap();
//!
//! assert_eq!(decoded, my_values);
//! ```
//!
//! ## How it works
//!
//! The dynamic encoder/decoder is implemented as a set of enums that represent
//! Solidity types, Solidity values (in rust representation form), and ABI
//! tokens. Unlike the static encoder, each of these must be instantiated at
//! runtime. The [`DynSolType`] enum represents a Solidity type, and is
//! equivalent to an enum over types implementing the [`crate::SolType`] trait.
//! The [`DynSolValue`] enum represents a Solidity value, and describes the
//! rust shapes of possible Solidity values. It is similar to, but not
//! equivalent to an enum over types used as [`crate::SolType::RustType`]. The
//! [`DynToken`] enum represents an ABI token, and is equivalent to an enum over
//! the types implementing the [`ethers_abi_enc::TokenType`] trait.
//!
//! Where the static encoding system encodes the expected type information into
//! the rust type system, the dynamic encoder/decoder encodes it as a concrete
//! instance of [`DynSolType`]. This type is used to tokenize and detokenize
//! [`DynSolValue`] instances. The [`std::str::FromStr`] impl is used to parse a
//! Solidity type string into a [`DynSolType`] object.
//!
//! Tokenizing - `DynSolType + `DynSolValue` = `DynToken`
//! Detokenizing - `DynSolType` + `DynToken` = `DynSolValue`
//!
//! Users must manually handle the conversions between [`DynSolValue`] and their
//! own rust types. We provide several `From` implementations, but they fall
//! short when dealing with arrays and tuples. We also provide fallible casts
//! into the contents of each variant.
//!
//! ## `DynToken::decode_populate`
//!
//! Because the shape of the data is known only at runtime, we cannot
//! compile-time allocate the memory needed to hold decoded data. Instead, we
//! pre-allocate a [`DynToken`] with the same shape as the expected type, and
//! empty values. We then populate the empty values with the decoded data.
//!
//! This is a significant behavior departure from the static decoder. We do not
//! recommend using the [`DynToken`] type directly. Instead, we recommend using
//! the encoding and decoding methods on [`DynSolType`].

#[macro_use]
extern crate alloc;

mod no_std_prelude {
    pub(crate) use alloc::{
        borrow::{Borrow, ToOwned},
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    };
}

mod error;
pub use error::DynAbiError;

pub use ethers_abi_enc::{Decoder, Eip712Domain, Encoder, Error, Result, SolType, Word};

mod r#type;
pub use r#type::DynSolType;

mod value;
pub use value::DynSolValue;

mod token;
pub use token::DynToken;

pub mod parser;

pub mod eip712;
pub use eip712::{parser as eip712_parser, Resolver, TypedData};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_e2e() {
        // parse a type from a string
        let my_type: DynSolType = "uint8[2][]".parse().unwrap();

        // set values
        let uints = DynSolValue::FixedArray(vec![64u8.into(), 128u8.into()]);
        let my_values = DynSolValue::Array(vec![uints]);

        // tokenize and detokenize
        let tokens = my_type.tokenize(my_values.clone()).unwrap();
        let detokenized = my_type.detokenize(tokens.clone()).unwrap();
        assert_eq!(detokenized, my_values);

        // encode
        let mut encoder = Encoder::default();
        tokens.encode_single(&mut encoder).unwrap();
        let encoded = encoder.into_bytes();

        // decode
        let mut decoder = Decoder::new(&encoded, true);
        let mut decoded = my_type.empty_dyn_token();
        decoded.decode_single_populate(&mut decoder).unwrap();

        assert_eq!(decoded, tokens);
    }
}
