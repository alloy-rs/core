//! This crate contains the expansion logic for a Solidity proc_macro2::TokenStream.
//! Its used to expand and generate Rust bindings from Solidity.
//!
//! Note: This is not the procedural macro crate, it is intended to be used as library crate.
//!
//! This crate is used by [`sol!`] macro in `alloy-sol-macro` crate.

pub mod expand;
mod utils;
#[cfg(feature = "json")]
mod verbatim;

extern crate syn_solidity as ast;
