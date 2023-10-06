use crate::{FieldList, SolIdent, Spanned, Type};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Attribute, Result, Token,
};

/// A struct definition: `struct Foo { uint256 bar; }`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.structDefinition>
#[derive(Clone)]
pub struct ItemStruct {
    pub attrs: Vec<Attribute>,
    pub struct_token: Token![struct],
    pub name: SolIdent,
    pub brace_token: Brace,
    pub fields: FieldList,
}

impl PartialEq for ItemStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields
    }
}

impl Eq for ItemStruct {}

impl Hash for ItemStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.fields.hash(state);
    }
}

impl fmt::Display for ItemStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct {} {{ {} }}", self.name, self.fields)
    }
}

impl fmt::Debug for ItemStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemStruct")
            .field("attrs", &self.attrs)
            .field("name", &self.name)
            .field("fields", &self.fields)
            .finish()
    }
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            struct_token: input.parse()?,
            name: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse()?,
        })
    }
}

impl Spanned for ItemStruct {
    fn span(&self) -> Span {
        self.name.span()
    }

    fn set_span(&mut self, span: Span) {
        self.struct_token = Token![struct](span);
        self.name.set_span(span);
        self.brace_token = Brace(span);
    }
}

impl ItemStruct {
    pub fn as_type(&self) -> Type {
        let mut ty = Type::Tuple(self.fields.types().cloned().collect());
        ty.set_span(self.span());
        ty
    }
}
