use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Result,
};

mod attribute;
pub use attribute::{
    FunctionAttribute, FunctionAttributes, Modifier, Mutability, Override, VariableAttribute,
    VariableAttributes, Visibility,
};

mod ident;
pub use ident::{is_id_continue, is_id_start, is_ident, SolIdent, SolPath};

mod variable;
pub use variable::{Parameters, VariableDeclaration};

mod utils;
pub use utils::*;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Storage {
    Memory(kw::memory),
    Storage(kw::storage),
    Calldata(kw::calldata),
}

impl fmt::Debug for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_debug_str())
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Parse for Storage {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::memory) {
            Ok(Self::Memory(input.parse()?))
        } else if lookahead.peek(kw::storage) {
            Ok(Self::Storage(input.parse()?))
        } else if lookahead.peek(kw::calldata) {
            Ok(Self::Calldata(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Storage {
    pub fn span(&self) -> Span {
        match self {
            Self::Memory(kw) => kw.span(),
            Self::Storage(kw) => kw.span(),
            Self::Calldata(kw) => kw.span(),
        }
    }

    pub const fn as_debug_str(&self) -> &'static str {
        match self {
            Self::Memory(_) => "Memory",
            Self::Storage(_) => "Storage",
            Self::Calldata(_) => "Calldata",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Memory(_) => "memory",
            Self::Storage(_) => "storage",
            Self::Calldata(_) => "calldata",
        }
    }
}
