#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(unused_imports))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate alloc;

/// Errors.
mod error;
pub use error::{Error, Result};

/// Solidity ident rules.
mod ident;
pub use ident::{is_id_continue, is_id_start, is_valid_identifier, IDENT_REGEX};

/// Root type specifier.
mod root;
pub use root::RootType;

/// Type stem.
mod stem;
pub use stem::TypeStem;

/// Tuple type specifier.
mod tuple;
pub use tuple::TupleSpecifier;

/// Type specifier.
mod type_spec;
pub use type_spec::TypeSpecifier;

/// Parameter specifier.
mod parameter;
pub use parameter::{ParameterSpecifier, Parameters, Storage};

/// Generic [`winnow`] parsing utilities.
pub mod utils;
