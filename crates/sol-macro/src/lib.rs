//! # alloy-sol-macro
//!
//! This crate provides the [`sol`][sol!] procedural macro, which parses
//! Solidity syntax to generate types that implement [`alloy-sol-types`]
//! traits.
//!
//! Refer to the [macro's documentation][sol!] for more information.
//!
//! [`alloy-sol-types`]: https://docs.rs/alloy-sol-types

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate proc_macro_error;
extern crate syn_solidity as ast;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod attr;
mod expand;
mod input;
#[cfg(feature = "json")]
mod json;
mod utils;

/// Generate types that implement [`alloy-sol-types`] traits, which can be used
/// for type-safe [ABI] and [EIP-712] serialization to interface with Ethereum
/// smart contracts.
///
/// [ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712
///
/// # Examples
///
/// > Note: the following example code blocks cannot be tested here because the
/// > generated code references [`alloy-sol-types`], so they are [tested in that
/// > crate][tests] and included with [`include_str!`] in this doc instead.
///
/// [tests]: https://github.com/alloy-rs/core/tree/main/crates/sol-types/tests/doctests
/// [`alloy-sol-types`]: https://docs.rs/alloy-sol-types
///
/// There are two main ways to use this macro:
/// - you can [write Solidity code](#solidity), or provide a path to a Solidity
///   file,
/// - if you enable the `json` feature, you can provide [an ABI, or a path to
///   one, in JSON format](#json-abi).
///
/// Note:
/// - relative file system paths are rooted at the `CARGO_MANIFEST_DIR`
///   environment variable
/// - no casing convention is enforced for any identifier,
/// - unnamed arguments will be given a name based on their index in the list,
///   e.g. `_0`, `_1`...
/// - a current limitation for certain items is that custom types, like structs,
///   must be defined in the same macro scope, otherwise a signature cannot be
///   generated at compile time. You can bring them in scope with a [Solidity
///   type alias](#udvt-and-type-aliases).
///
/// ## Solidity
///
/// This macro uses [`syn-solidity`][ast] to parse Solidity-like syntax. See
/// [its documentation][ast] for more.
///
/// Solidity input can be either one of the following:
/// - a Solidity item, which is a [Solidity source unit][sol-item] which
///   generates one or more Rust items,
/// - a [Solidity type name][sol-types], which simply expands to the
///   corresponding Rust type.
///
/// [sol-item]: https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.sourceUnit
/// [sol-types]: https://docs.soliditylang.org/en/latest/types.html
///
/// ### Attributes
///
/// Inner attributes (`#![...]`) are parsed at the top of the input, just like a
/// Rust module. These can only be `sol` attributes, and they will apply to the
/// entire input.
///
/// Outer attributes (`#[...]`) are parsed as part of each individual item, like
/// structs, enums, etc. These can be any Rust attribute, and they will be added
/// to every Rust item generated from the Solidity item.
///
/// This macro provides the `sol` attribute, which can be used to customize the
/// generated code. Note that unused attributes are currently silently ignored,
/// but this may change in the future.
///
/// List of all `#[sol(...)]` supported attributes:
/// - `all_derives [ = <bool = false>]`: adds all possible `#[derive(...)]`
///   attributes to all generated types. May significantly increase compile
///   times due to all the extra generated code. This is the default behaviour
///   of [`abigen`][abigen]
/// - `extra_methods [ = <bool = false>]`: adds extra implementations and
///   methods to all applicable generated types, such as `From` impls and
///   `as_<variant>` methods. May significantly increase compile times due to
///   all the extra generated code. This is the default behaviour of
///   [`abigen`][abigen]
/// - `docs [ = <bool = true>]`: adds doc comments to all generated types. This
///   is the default behaviour of [`abigen`][abigen]
/// - `bytecode = <hex string literal>`: specifies the creation/init bytecode of
///   a contract. This will emit a `static` item with the specified bytes.
/// - `deployed_bytecode = <hex string literal>`: specifies the deployed
///   bytecode of a contract. This will emit a `static` item with the specified
///   bytes.
///
/// ### Structs and enums
///
/// Structs and enums generate their corresponding Rust types. Enums are
/// additionally annotated with `#[repr(u8)]`, and as such can have a maximum of
/// 256 variants.
/// ```ignore
#[doc = include_str!("../doctests/structs.rs")]
/// ```
/// 
/// ### UDVT and type aliases
///
/// User defined value types (UDVT) generate a tuple struct with the type as
/// its only field, and type aliases simply expand to the corresponding Rust
/// type.
/// ```ignore
#[doc = include_str!("../doctests/types.rs")]
/// ```
/// 
/// ### Functions and errors
///
/// Functions generate two structs that implement `SolCall`: `<name>Call` for
/// the function arguments, and `<name>Return` for the return values.
///
/// In the case of overloaded functions, an underscore and the index of the
/// function will be appended to `<name>` (like `foo_0`, `foo_1`...) for
/// disambiguation, but the signature will remain the same.
///
/// E.g. if there are two functions named `foo`, the generated types will be
/// `foo_0Call` and `foo_1Call`, each of which will implement `SolCall`
/// with their respective signatures.
/// ```ignore
#[doc = include_str!("../doctests/function_like.rs")]
/// ```
/// 
/// ### Events
///
/// Events generate a struct that implements `SolEvent`.
///
/// Note that events have special encoding rules in Solidity. For example,
/// `string indexed` will be encoded in the topics as its `bytes32` Keccak-256
/// hash, and as such the generated field for this argument will be `bytes32`,
/// and not `string`.
/// ```ignore
#[doc = include_str!("../doctests/events.rs")]
/// ```
/// 
/// ### Contracts/interfaces
///
/// Contracts generate a module with the same name, which contains all the items.
/// This module will also contain 3 container enums which implement
/// `SolInterface`, one for each:
/// - functions: `<contract_name>Calls`
/// - errors: `<contract_name>Errors`
/// - events: `<contract_name>Events`
/// ```ignore
#[doc = include_str!("../doctests/contracts.rs")]
/// ```
/// 
/// ## JSON ABI
///
/// Contracts can also be generated from ABI JSON strings and files, similar to
/// the [ethers-rs `abigen!` macro][abigen].
///
/// JSON objects containing the `abi`, `evm`, `bytecode`, `deployedBytecode`,
/// and similar keys are also supported.
///
/// Note that only valid JSON is supported, and not the human-readable ABI
/// format, also used by [`abigen!`][abigen]. This should instead be easily converted to
/// [normal Solidity input](#solidity).
///
/// Prefer using [Solidity input](#solidity) when possible, as the JSON ABI
/// format omits some information which is useful to this macro, such as enum
/// variants and visibility modifiers on functions.
///
/// [abigen]: https://docs.rs/ethers/latest/ethers/contract/macro.abigen.html
/// ```ignore
#[doc = include_str!("../doctests/json.rs")]
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn sol(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as input::SolInput)
        .expand()
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
