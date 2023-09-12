use std::fmt;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

use crate::{yul::ident::YulIdent, Spanned};

// While only identifiers without dots can be declared within inline assembly,
// paths containing dots can refer to declarations outside the inline assembly
// block.
//
// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulPath>
#[derive(Clone)]
pub enum YulPath {
    SimplePath(YulIdent),
    DottedPath(YulDottedPath),
}

impl Parse for YulPath {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek2(Token![.]) {
            input.parse().map(Self::DottedPath)
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

// Representation of a Yul path that includes dots
#[derive(Clone)]
pub struct YulDottedPath {
    path: Punctuated<YulIdent, Token![.]>,
}

impl Parse for YulDottedPath {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            path: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

impl fmt::Debug for YulDottedPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulDottedPath")
            .field("path", &self.path)
            .finish()
    }
}

impl Spanned for YulDottedPath {
    fn span(&self) -> Span {
        self.path.span()
    }

    fn set_span(&mut self, span: Span) {
        self.path.set_span(span)
    }
}
