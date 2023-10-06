use crate::{Spanned, YulIdent};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

/// In inline assembly, only dot-less identifiers can be declared, but dotted
/// paths can reference declarations made outside the assembly block.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulPath>
#[derive(Clone)]
pub struct YulPath(Punctuated<YulIdent, Token![.]>);

impl Parse for YulPath {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self(Punctuated::parse_separated_nonempty(input)?))
    }
}

impl Spanned for YulPath {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
    }
}

impl fmt::Debug for YulPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("YulPath").field(&self.0).finish()
    }
}
