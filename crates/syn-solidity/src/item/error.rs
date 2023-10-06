use crate::{kw, ParameterList, SolIdent, Spanned, Type};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Attribute, Result, Token,
};

/// An error definition: `error Foo(uint256 a, uint256 b);`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.errorDefinition>
#[derive(Clone)]
pub struct ItemError {
    pub attrs: Vec<Attribute>,
    pub error_token: kw::error,
    pub name: SolIdent,
    pub paren_token: Paren,
    pub parameters: ParameterList,
    pub semi_token: Token![;],
}

impl fmt::Display for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error {}({});", self.name, self.parameters)
    }
}

impl fmt::Debug for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemError")
            .field("attrs", &self.attrs)
            .field("name", &self.name)
            .field("fields", &self.parameters)
            .finish()
    }
}

impl Parse for ItemError {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            error_token: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(content in input),
            parameters: content.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for ItemError {
    fn span(&self) -> Span {
        self.name.span()
    }

    fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
    }
}

impl ItemError {
    pub fn as_type(&self) -> Type {
        let mut ty = Type::Tuple(self.parameters.types().cloned().collect());
        ty.set_span(self.span());
        ty
    }
}
