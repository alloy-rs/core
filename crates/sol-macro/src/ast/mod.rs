//! A Solidity [`syn`] AST.
//!
//! Note that this is not intended to be a complete representation of the
//! Solidity AST, but rather a subset of the AST that is useful for generating
//! [`ethers-sol-types`]

#![allow(dead_code)] // TODO: Remove once this is a separate crate.

mod attribute;
pub use attribute::{
    FunctionAttribute, FunctionAttributes, Modifier, Mutability, Override, VariableAttribute,
    VariableAttributes, Visibility,
};

mod ident;
pub use ident::{SolIdent, SolPath};

mod file;
pub use file::File;

pub mod item;
pub use item::Item;

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
