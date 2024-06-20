//! This crate contains inputs to the `sol!` macro. It sits in-between
//! the `sol-macro` and `syn-solidity` crates, and contains an intermediate
//! representation of Solidity items. These items are then expanded into
//! Rust code by the `alloy-sol-macro` crate.
//!
//! This crate is not meant to be used directly, but rather is a tool for
//! writing macros that generate Rust code from Solidity code.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

extern crate syn_solidity as ast;

/// Tools for working with `#[...]` attributes.
mod attr;
pub use attr::{
    derives_mapped, docs_str, mk_doc, parse_derives, CasingStyle, ContainsSolAttrs, SolAttrs,
};

mod input;
pub use input::{SolInput, SolInputKind};

mod expander;
pub use expander::SolInputExpander;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::tokens_for_sol;
