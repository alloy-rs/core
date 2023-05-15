//! # ethers-sol-type
//!
//! This crate provides the [`sol`][sol!] procedural macro, which parses
//! Solidity syntax to generate types that implement [`ethers-abi-enc`] traits.
//!
//! Refer to the [macro's documentation][sol!] for more information.

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod common;
mod error;
mod function;
mod input;
mod r#struct;
mod r#type;
mod udt;

/// Parses Solidity syntax to generate types that implement [`ethers-abi-enc`]
/// traits.
///
/// These types may then be used for safe [ABI] and [EIP-712] encoding and
/// decoding.
///
/// [ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712
/// [`ethers-abi-enc`]: https://docs.rs/ethers-abi-enc
///
/// # Examples
///
/// Note: the following examples cannot tested here because the generated code
/// references `ethers-abi-enc`, so they are [tested in that crate][tests] and
/// included with `include_str!` in this doc instead.
///
/// [tests]: https://github.com/ethers-rs/core/tree/main/crates/abi/tests/
///
/// Structs and enums:
///
/// ```ignore.
#[doc = include_str!("../../abi/tests/doc_structs.rs")]
/// ```
/// 
/// UDVT and type aliases:
/// ```ignore.
#[doc = include_str!("../../abi/tests/doc_types.rs")]
/// ```
/// 
/// Functions, errors, and events:
/// ```ignore.
#[doc = include_str!("../../abi/tests/doc_function_like.rs")]
/// ```
/// 
/// Contracts/interfaces:
/// ```ignore.
#[doc = include_str!("../../abi/tests/doc_contracts.rs")]
/// ```
#[proc_macro]
pub fn sol(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as input::Input);
    s.to_token_stream().into()
}
