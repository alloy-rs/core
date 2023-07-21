#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    unreachable_pub,
    unused_crate_dependencies,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

extern crate proc_macro;

use syn::Result;

#[macro_use]
mod macros;

pub mod attribute;
pub mod literals;
pub use attribute::*;

pub mod statments;
pub use statments::*;

pub mod file;
pub use file::*;

pub mod ident;
pub use ident::*;

pub mod item;
pub use item::*;
pub mod lit;
pub use lit::*;

pub mod kw;

pub mod stmt;
pub use stmt::*;

pub mod r#type;
pub use r#type::Type;

pub(crate) mod utils;

pub mod variable;
pub use variable::*;

#[cfg(feature = "visit")]
pub mod visit;
#[cfg(feature = "visit")]
pub use visit::Visit;

#[cfg(feature = "visit-mut")]
pub mod visit_mut;
#[cfg(feature = "visit-mut")]
pub use visit_mut::VisitMut;

/// Parse a Solidity [`proc_macro::TokenStream`] into a [`File`].
pub fn parse(input: proc_macro::TokenStream) -> Result<File> {
    syn::parse(input)
}

/// Parse a Solidity [`proc_macro2::TokenStream`] into a [`File`].
pub fn parse2(input: proc_macro2::TokenStream) -> Result<File> {
    syn::parse2(input)
}
