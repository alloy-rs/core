//! # alloy-sol-macro
//!
//! This crate provides the [`sol`][sol!] procedural macro, which parses
//! Solidity syntax to generate types that implement [`alloy-sol-types`]
//! traits.
//!
//! Refer to the [macro's documentation][sol!] for more information.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg"
)]
#![warn(missing_docs)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

extern crate syn_solidity as ast;

use proc_macro::TokenStream;

mod expand;
mod utils;

/// Parses Solidity syntax to generate types that implement [`alloy-sol-types`]
/// traits.
///
/// These types may then be used for safe [ABI] and [EIP-712] encoding and
/// decoding.
///
/// [ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712
/// [`alloy-sol-types`]: https://docs.rs/alloy-sol-types
///
/// # Examples
///
/// Note: the following examples cannot be tested here because the generated
/// code references `alloy-sol-types`, so they are [tested in that crate]
/// [tests] and included with `include_str!` in this doc instead.
///
/// [tests]: https://github.com/alloy-rs/core/tree/main/crates/sol-types/tests/
///
/// ## Structs and enums
/// ```ignore
#[doc = include_str!("../doctests/doc_structs.rs")]
/// ```
/// 
/// ## UDVT and type aliases
/// ```ignore
#[doc = include_str!("../doctests/doc_types.rs")]
/// ```
/// 
/// ## Functions and errors
/// ```ignore
#[doc = include_str!("../doctests/doc_function_like.rs")]
/// ```
/// 
/// ## Events
/// ```ignore
#[doc = include_str!("../doctests/doc_events.rs")]
/// ```
/// 
/// ## Contracts/interfaces
/// ```ignore
#[doc = include_str!("../doctests/doc_contracts.rs")]
/// ```
#[proc_macro]
pub fn sol(input: TokenStream) -> TokenStream {
    let result = match ast::parse(input.clone()) {
        Ok(ast) => expand::expand(ast),
        // TODO: Should we still support this?
        Err(e) => match syn::parse::<ast::Type>(input) {
            Ok(ast::Type::Custom(_)) | Err(_) => Err(e),
            Ok(ty) => Ok(expand::expand_type(&ty)),
        },
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}
