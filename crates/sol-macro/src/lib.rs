//! # ethers-sol-macro
//!
//! This crate provides the [`sol`][sol!] procedural macro, which parses
//! Solidity syntax to generate types that implement [`ethers-sol-types`]
//! traits.
//!
//! Refer to the [macro's documentation][sol!] for more information.

// #![warn(missing_docs)] // TODO: Enable for AST crate.
#![deny(unused_must_use, rust_2018_idioms)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod ast;
mod expand;
mod utils;

/// Parses Solidity syntax to generate types that implement [`ethers-sol-types`]
/// traits.
///
/// These types may then be used for safe [ABI] and [EIP-712] encoding and
/// decoding.
///
/// [ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712
/// [`ethers-sol-types`]: https://docs.rs/ethers-sol-types
///
/// # Examples
///
/// Note: the following examples cannot tested here because the generated code
/// references `ethers-sol-types`, so they are [tested in that crate][tests] and
/// included with `include_str!` in this doc instead.
///
/// [tests]: https://github.com/ethers-rs/core/tree/main/crates/sol-types/tests/
///
/// ## Structs and enums
/// ```ignore
#[doc = include_str!("../../sol-types/tests/doc_structs.rs")]
/// ```
/// 
/// ## UDVT and type aliases
/// ```ignore
#[doc = include_str!("../../sol-types/tests/doc_types.rs")]
/// ```
/// 
/// ## Functions, errors, and events
/// ```ignore
#[doc = include_str!("../../sol-types/tests/doc_function_like.rs")]
/// ```
/// 
/// ## Contracts/interfaces
/// ```ignore
#[doc = include_str!("../../sol-types/tests/doc_contracts.rs")]
/// ```
#[proc_macro]
pub fn sol(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ast::File);
    expand::expand(ast).into()
}
