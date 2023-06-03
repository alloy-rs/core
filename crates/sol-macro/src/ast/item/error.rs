use crate::ast::{kw, Parameters, SolIdent};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Attribute, Result, Token,
};

/// An error definition.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.errorDefinition>
pub struct Error {
    pub attrs: Vec<Attribute>,
    pub error_token: kw::error,
    pub name: SolIdent,
    pub paren_token: Paren,
    pub fields: Parameters<Token![,]>,
    pub semi_token: Token![;],
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("name", &self.name)
            .field("fields", &self.fields)
            .finish()
    }
}

impl Parse for Error {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            error_token: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(content in input),
            fields: content.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Error {
    pub fn span(&self) -> Span {
        self.name.span()
    }
}
