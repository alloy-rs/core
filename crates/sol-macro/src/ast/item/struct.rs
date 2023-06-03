use crate::ast::{Parameters, SolIdent, Type};
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

/// A struct definition.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.structDefinition>
#[derive(Clone)]
pub struct Struct {
    pub attrs: Vec<Attribute>,
    pub struct_token: Token![struct],
    pub name: SolIdent,
    pub brace_token: Brace,
    pub fields: Parameters<Token![;]>,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields
    }
}

impl Eq for Struct {}

impl Hash for Struct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.fields.hash(state);
    }
}

impl fmt::Debug for Struct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Struct")
            .field("name", &self.name)
            .field("fields", &self.fields)
            .finish()
    }
}

impl Parse for Struct {
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

impl Struct {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.struct_token = Token![struct](span);
        self.name.set_span(span);
        self.brace_token = Brace(span);
    }

    pub fn ty(&self) -> Type {
        Type::Tuple(self.fields.types().cloned().collect())
    }
}
