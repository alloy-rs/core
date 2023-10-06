use crate::Spanned;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Result,
};

mod path;
pub use path::YulPath;

/// A Yul identifier.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.YulIdentifier>
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct YulIdent(pub Ident);

impl quote::IdentFragment for YulIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }

    fn span(&self) -> Option<Span> {
        Some(self.0.span())
    }
}

impl fmt::Display for YulIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for YulIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("YulIdent").field(&self.0).finish()
    }
}

impl<T: ?Sized + AsRef<str>> PartialEq<T> for YulIdent {
    fn eq(&self, other: &T) -> bool {
        self.0 == other
    }
}

impl From<Ident> for YulIdent {
    fn from(value: Ident) -> Self {
        Self(value)
    }
}

impl From<YulIdent> for Ident {
    fn from(value: YulIdent) -> Self {
        value.0
    }
}

impl From<&str> for YulIdent {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Parse for YulIdent {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Self::parse_any(input)
    }
}

impl ToTokens for YulIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl Spanned for YulIdent {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
    }
}

impl YulIdent {
    pub fn new(s: &str) -> Self {
        Self(Ident::new(s, Span::call_site()))
    }

    pub fn new_spanned(s: &str, span: Span) -> Self {
        Self(Ident::new(s, span))
    }

    /// Returns the identifier as a string, without the `r#` prefix if present.
    pub fn as_string(&self) -> String {
        let mut s = self.0.to_string();
        if s.starts_with("r#") {
            s = s[2..].to_string();
        }
        s
    }

    /// Parses any identifier including keywords.
    pub fn parse_any(input: ParseStream<'_>) -> Result<Self> {
        input.call(Ident::parse_any).map(Self)
    }

    /// Peeks any identifier including keywords.
    pub fn peek_any(input: ParseStream<'_>) -> bool {
        input.peek(Ident::peek_any)
    }

    pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
        if Self::peek_any(input) {
            input.parse().map(Some)
        } else {
            Ok(None)
        }
    }
}
