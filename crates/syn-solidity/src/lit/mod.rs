use crate::kw;
use proc_macro2::Span;
use syn::{
    parse::{Lookahead1, Parse, ParseStream},
    LitBool, LitFloat, LitInt, Result,
};

mod str;
pub use str::{HexStr, LitHex, LitStr, LitUnicode, UnicodeStr};

/// A literal.
#[derive(Clone, Debug)]
pub enum Lit {
    Str(LitStr),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Hex(LitHex),
    Unicode(LitUnicode),
}

impl Parse for Lit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            input.parse().map(Self::Str)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Int)
        } else if lookahead.peek(LitFloat) {
            input.parse().map(Self::Float)
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Bool)
        } else if lookahead.peek(kw::unicode) {
            input.parse().map(Self::Unicode)
        } else if lookahead.peek(kw::hex) {
            input.parse().map(Self::Hex)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Lit {
    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        lookahead.peek(syn::Lit) || lookahead.peek(kw::unicode) || lookahead.peek(kw::hex)
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Str(lit) => lit.span(),
            Self::Int(lit) => lit.span(),
            Self::Float(lit) => lit.span(),
            Self::Bool(lit) => lit.span(),
            Self::Hex(lit) => lit.span(),
            Self::Unicode(lit) => lit.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Str(lit) => lit.set_span(span),
            Self::Int(lit) => lit.set_span(span),
            Self::Float(lit) => lit.set_span(span),
            Self::Bool(lit) => lit.set_span(span),
            Self::Hex(lit) => lit.set_span(span),
            Self::Unicode(lit) => lit.set_span(span),
        }
    }
}
