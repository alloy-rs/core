use crate::{utils::DebugPunctuated, SolIdent};
use proc_macro2::Span;
use std::fmt;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Attribute, Result, Token,
};

/// An enum definition: `enum Foo { A, B, C }`
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.enumDefinition>
#[derive(Clone)]
pub struct ItemEnum {
    pub attrs: Vec<Attribute>,
    pub enum_token: Token![enum],
    pub name: SolIdent,
    pub brace_token: Brace,
    pub variants: Punctuated<SolIdent, Token![,]>,
}

impl fmt::Debug for ItemEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Enum")
            .field("attrs", &self.attrs)
            .field("name", &self.name)
            .field("variants", DebugPunctuated::new(&self.variants))
            .finish()
    }
}

impl Parse for ItemEnum {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            enum_token: input.parse()?,
            name: input.parse()?,
            brace_token: braced!(content in input),
            variants: content.parse_terminated(SolIdent::parse, Token![,])?,
        })
    }
}

impl ItemEnum {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
    }
}
