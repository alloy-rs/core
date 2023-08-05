use crate::{kw, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Lookahead1, Parse, ParseStream},
    LitBool, Result,
};

mod number;
pub use number::{LitDenominated, LitNumber, SubDenomination};

mod str;
pub use self::str::{HexStr, LitHexStr, LitStr, LitUnicodeStr, UnicodeStr};

/// A Solidity literal such as a string or integer or boolean.
#[derive(Clone)]
pub enum Lit {
    /// A boolean literal: `true` or `false`.
    Bool(LitBool),

    /// A hex string literal: `hex"1234"`.
    Hex(LitHexStr),

    /// An integer or fixed-point number literal: `1` or `1.0`.
    Number(LitNumber),

    /// A string literal.
    Str(LitStr),

    /// A unicode string literal.
    Unicode(LitUnicodeStr),
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Lit::")?;
        match self {
            Self::Bool(lit) => lit.fmt(f),
            Self::Hex(lit) => lit.fmt(f),
            Self::Number(lit) => lit.fmt(f),
            Self::Str(lit) => lit.fmt(f),
            Self::Unicode(lit) => lit.fmt(f),
        }
    }
}

impl Parse for Lit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            input.parse().map(Self::Str)
        } else if LitNumber::peek(&lookahead) {
            input.parse().map(Self::Number)
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

impl Spanned for Lit {
    fn span(&self) -> Span {
        match self {
            Self::Bool(lit) => lit.span(),
            Self::Hex(lit) => lit.span(),
            Self::Number(lit) => lit.span(),
            Self::Str(lit) => lit.span(),
            Self::Unicode(lit) => lit.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Bool(lit) => lit.set_span(span),
            Self::Hex(lit) => lit.set_span(span),
            Self::Number(lit) => lit.set_span(span),
            Self::Str(lit) => lit.set_span(span),
            Self::Unicode(lit) => lit.set_span(span),
        }
    }
}

impl Lit {
    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        lookahead.peek(syn::Lit) || lookahead.peek(kw::unicode) || lookahead.peek(kw::hex)
    }
}
