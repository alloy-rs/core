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

//! Solidity type modeling & ABI Coding implementation
//!
//! This library provides tools for expressing Solidity types in Rust, and for
//! encoding these representations into ABI blobs suitable for smart contract
//! processing. In other words, you can represent your smart contract args in
//! native Rust, easily encode them to pass to a smart contract, and easily
//! decode the smart contract return values.
//!
//! We do this by representing solidity types in rust via the [`SolType`] trait.
//! This trait maps Solidity types to Rust types via the associated
//! [`SolType::RustType`].
//!
//! Each [`SolType`] also has an associated [`SolType::TokenType`]. This is the
//! intermediate representation of the data suitable for ABI encoding. The ABI
//! `encode` and `decode` methods operate on objects implementing
//! [`TokenType`].
//!
//! ```
//! use ethers_abi_enc::sol_type::*;
//! # pub fn main() -> ethers_abi_enc::AbiResult<()> {
//! // Represent a solidity type in rust
//! type MySolType = FixedArray<Bool, 2>;
//!
//! let data = [true, false];
//! let validate = true;
//!
//! // SolTypes expose their solidity name :)
//! assert_eq!(&MySolType::sol_type_name(), "bool[2]");
//!
//! // SolTypes are used to transform Rust into ABI blobs, and back.
//! let encoded: Vec<u8> = MySolType::encode(data);
//! let decoded: [bool; 2] = MySolType::decode(&encoded, validate)?;
//! assert_eq!(data, decoded);
//! # Ok(())
//! # }
//! ```
//!
//! See the [`SolType`] docs for an implementer's guide.
//!
//! ## `sol!` type parsing
//!
//! The `sol!` proc macro parses complex soltypes from valid solidity. Right now
//! it's limited to the solidity types defines in this library. It's useful for
//! defining complex structures using familiar syntax.
//!
//! In the future, `sol!` will support macro definitions, functions, and more!
//!
//! ```
//! # use ethers_abi_enc::{sol, sol_type, SolType};
//! # pub fn main() {
//! // outputs a type built that implements `SolType`
//! type B32 = sol! {bytes32};
//! assert_eq!(B32::sol_type_name(), "bytes32");
//! assert_eq!
//!     (B32::hex_encode_single([0; 32]),
//!     "0x0000000000000000000000000000000000000000000000000000000000000000"
//! ); // Wow!
//!
//! type Complex = sol! {((address, address)[],address)};
//! assert_eq!(
//!     Complex::sol_type_name(),
//!     "tuple(tuple(address,address)[],address)"
//! ); // Cool!
//!
//! type Gamut = sol! {
//!     (
//!         address, bool[], bytes15[12], uint256, uint24, int8, int56,
//!         (bytes17, string, bytes,)
//!     )
//! };
//!
//! assert_eq!(
//!     Gamut::sol_type_name(),
//!     "tuple(address,bool[],bytes15[12],uint256,uint24,int8,int56,tuple(bytes17,string,bytes))"
//! ); // Amazing!
//!
//! // `sol!` supports late binding of types, and your own custom types!
//! type Abstract<A> = sol! { A[] };
//!
//! // Incredible!
//! assert_eq!(Abstract::<sol_type::Address>::sol_type_name(), "address[]");
//! # }
//! ```
//!
//! ## [`sol!`] structs, the [`SolStruct`] trait, and the [`domain!`] macro
//!
//! The [`sol!`] macro also supports parsing structs. This is useful for
//! defining complex types that you want to encode and decode. It also provides
//! a convenient way to define EIP-712 structs at compile-time. It accepts a
//! standard solidity `struct` definition, and generating a Rust struct that
//! implements [`SolStruct`] and [`SolType`].
//!
//! The [`SolStruct`] trait primarily provides EIP-712 signing support.
//!
//! ### Gotchas:
//!
//! Right now, sol! structs works only **outside** of function scope, due to
//! rust import rules, and unfortunately, doesn't yet support late binding.
//!
//! ```
//! # use ethers_abi_enc::{sol, SolStruct};
//! # use ethers_primitives::U256;
//! // `sol!` allows you to define struct types!
//! // Usually you can just paste solidity into the macro :)
//! sol! {
//!     struct MyStruct {
//!         uint256 a;
//!         bytes32 b;
//!         address[] c;
//!     }
//! }
//!
//! sol! {
//!     struct MyStruct2 {
//!         MyStruct a;
//!         bytes32 b;
//!         address[] c;
//!     }
//! }
//!
//! # pub fn main() {
//! // All structs generated with `sol!` implement `crate::SolType` &
//! // `crate::SolStruct`. This means you get eip-712 signing for freeeeee
//! let my_struct = MyStruct {
//!     a: U256::from(1),
//!     b: [0; 32],
//!     c: vec![Default::default()],
//! };
//!
//! // The domain macro lets you easily define an EIP-712 domain object :)
//! let my_domain = ethers_abi_enc::domain!(
//!    name: "MyDomain",
//!    version: "1",
//! );
//!
//! // Because all the hard work is done by the `sol!` macro, EIP-712 is as easy
//! // as calling `eip712_signing_hash` with your domain
//! let signing_hash = my_struct.eip712_signing_hash(&my_domain);
//! # }
//! ```
//!
//! ### [`sol!`] User-defined Value Types
//!
//! Support for user-defined value types is new! These are currently
//! implemented as wrapper types. Watch this space for more
//! features!
//!
//! ```
//! # use ethers_abi_enc::{sol, sol_type, SolType};
//! # use ethers_primitives::U256;
//! // We also also support solidity value types
//! sol! {
//!     type MyValueType is uint256;
//! }
//!
//! # pub fn main() {
//! // UDTs are encoded as their underlying type
//! let mvt = MyValueType::from(U256::from(1));
//! assert_eq!(
//!     mvt.encode_single(),
//!     sol_type::Uint::<256>::encode_single(U256::from(1))
//! );
//! # }
//! ```
//!
//! ## Tokenization/Detokenization
//!
//! The process of converting from a Rust type to a to an abi token is called
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
//! interact with Tokens, except when implementing their own [`SolType`].

#[cfg_attr(not(feature = "std"), macro_use)]
extern crate alloc;

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub mod no_std_prelude {
    pub use alloc::{
        borrow::{Borrow, Cow, ToOwned},
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
    pub use core::marker::PhantomData;
}

#[cfg(feature = "std")]
#[doc(hidden)]
pub mod no_std_prelude {
    pub use std::{
        borrow::{Borrow, Cow, ToOwned},
        format,
        marker::PhantomData,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
}

/// The `sol!` proc macro parses Solidity types and structdefs, and outputs
/// Rust types that implement [`SolType`].
///
/// See the root crate docs for more information.
pub use sol_type_parser::sol;

/// The Word type for ABI Encoding
pub type Word = ethers_primitives::B256;

mod coder;
pub use coder::{
    decode, decode_params, decode_single, encode, encode_params, encode_single,
    token::{self, TokenType},
};
#[doc(hidden)]
pub use coder::{Decoder, Encoder};

mod errors;
pub use errors::{AbiResult, Error};

mod sol_types;
pub use sol_types::{sol_type, SolStruct, SolType};

mod util;
#[doc(hidden)]
pub use util::just_ok;
pub use util::keccak256;

mod eip712;
pub use eip712::Eip712Domain;
