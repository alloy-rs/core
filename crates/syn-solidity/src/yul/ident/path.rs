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
pub enum YulPath {
    /// Dotless path, references a declared varaible inside the assembly block.
    SimplePath(YulIdent),

    /// Dotted path, references a declared variable outside the assembly block.
    DottedPath(Punctuated<YulIdent, Token![.]>),
}

impl Parse for YulPath {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek2(Token![.]) {
            Ok(Self::DottedPath(Punctuated::parse_separated_nonempty(
                input,
            )?))
        } else {
            input.parse().map(Self::SimplePath)
        }
    }
}

impl Spanned for YulPath {
    fn span(&self) -> Span {
        match self {
            YulPath::SimplePath(path) => path.span(),
            YulPath::DottedPath(path) => path.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            YulPath::SimplePath(path) => path.set_span(span),
            YulPath::DottedPath(path) => path.set_span(span),
        }
    }
}

impl fmt::Debug for YulPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YulPath::")?;
        match self {
            Self::SimplePath(path) => path.fmt(f),
            Self::DottedPath(path) => path.fmt(f),
        }
    }
}
