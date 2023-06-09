#![doc = include_str!("../README.md")]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    unreachable_pub,
    unused_crate_dependencies
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

extern crate proc_macro;

use syn::Result;

#[macro_use]
mod macros;

mod attribute;
pub use attribute::{
    FunctionAttribute, FunctionAttributes, Modifier, Mutability, Override, Storage,
    VariableAttribute, VariableAttributes, Visibility,
};

mod file;
pub use file::File;

mod ident;
pub use ident::{SolIdent, SolPath};

mod item;
pub use item::{
    EventParameter, Item, ItemContract, ItemError, ItemEvent, ItemFunction, ItemStruct, ItemUdt,
    Returns,
};

mod r#type;
pub use r#type::{SolArray, SolTuple, Type};

pub(crate) mod utils;

mod variable;
pub use variable::{FieldList, ParameterList, Parameters, VariableDeclaration};

#[cfg(feature = "visit")]
pub mod visit;
#[cfg(feature = "visit")]
pub use visit::Visit;

#[cfg(feature = "visit-mut")]
pub mod visit_mut;
#[cfg(feature = "visit-mut")]
pub use visit_mut::VisitMut;

/// Solidity keywords.
pub mod kw {
    use syn::custom_keyword;
    pub use syn::token::{Abstract, Override, Virtual};

    // Storage
    custom_keyword!(memory);
    custom_keyword!(storage);
    custom_keyword!(calldata);

    // Visibility
    custom_keyword!(external);
    custom_keyword!(public);
    custom_keyword!(internal);
    custom_keyword!(private);

    // Mutability
    custom_keyword!(pure);
    custom_keyword!(view);
    custom_keyword!(constant);
    custom_keyword!(payable);

    // Contract
    custom_keyword!(contract);
    custom_keyword!(interface);
    custom_keyword!(library);

    // Error
    custom_keyword!(error);

    // Event
    custom_keyword!(event);
    custom_keyword!(indexed);
    custom_keyword!(anonymous);

    // Function
    custom_keyword!(immutable);
    custom_keyword!(returns);
    custom_keyword!(function);

    // Types
    custom_keyword!(tuple);

    // Other
    custom_keyword!(is);
}

/// Parse a Solidity [`proc_macro::TokenStream`] into a [`File`].
pub fn parse(input: proc_macro::TokenStream) -> Result<File> {
    syn::parse(input)
}

/// Parse a Solidity [`proc_macro2::TokenStream`] into a [`File`].
pub fn parse2(input: proc_macro2::TokenStream) -> Result<File> {
    syn::parse2(input)
}
