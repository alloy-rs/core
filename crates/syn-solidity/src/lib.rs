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
    EventParameter, ImportAlias, ImportAliases, ImportDirective, ImportGlob, ImportPath,
    ImportPlain, Item, ItemContract, ItemEnum, ItemError, ItemEvent, ItemFunction, ItemStruct,
    ItemUdt, PragmaDirective, PragmaTokens, Returns, UserDefinableOperator, UsingDirective,
    UsingList, UsingListItem, UsingType,
};

mod lit;
pub use lit::LitStr;

pub mod kw;

mod stmt;
pub use stmt::Block;

mod r#type;
pub use r#type::{Type, TypeArray, TypeFunction, TypeMapping, TypeTuple};

pub(crate) mod utils;

mod variable;
pub use variable::{FieldList, ParameterList, Parameters, VariableDeclaration, VariableDefinition};

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
