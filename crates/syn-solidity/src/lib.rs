#![doc = include_str!("../README.md")]
#![warn(unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

extern crate proc_macro;

use syn::Result;

mod attribute;
pub use attribute::{
    FunctionAttribute, FunctionAttributes, Modifier, Mutability, Override, VariableAttribute,
    VariableAttributes, Visibility,
};

mod ident;
pub use ident::{SolIdent, SolPath};

mod file;
pub use file::File;

mod item;
pub use item::{Item, ItemError, ItemFunction, ItemStruct, ItemUdt};

mod storage;
pub use storage::Storage;

mod r#type;
pub use r#type::{CustomType, SolArray, SolTuple, Type};

mod returns;
pub use returns::Returns;

mod variable;
pub use variable::{Parameters, VariableDeclaration};

/// Solidity keywords.
pub mod kw {
    use syn::custom_keyword;
    pub use syn::token::{Override, Virtual};

    custom_keyword!(is);

    // Types
    custom_keyword!(error);
    custom_keyword!(function);
    custom_keyword!(tuple);

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

    // Function
    custom_keyword!(immutable);
    custom_keyword!(returns);

    // Storage
    custom_keyword!(memory);
    custom_keyword!(storage);
    custom_keyword!(calldata);
}

/// Parse a Solidity [`proc_macro::TokenStream`] into a [`File`].
pub fn parse(input: proc_macro::TokenStream) -> Result<File> {
    syn::parse(input)
}

/// Parse a Solidity [`proc_macro2::TokenStream`] into a [`File`].
pub fn parse2(input: proc_macro2::TokenStream) -> Result<File> {
    syn::parse2(input)
}
