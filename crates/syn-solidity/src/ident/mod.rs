use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

mod path;
pub use path::SolPath;

/// A Solidity identifier.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SolIdent(pub Ident);

impl quote::IdentFragment for SolIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }

    fn span(&self) -> Option<Span> {
        Some(self.span())
    }
}

impl fmt::Display for SolIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for SolIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SolIdent").field(&self.to_string()).finish()
    }
}

impl<T: ?Sized + AsRef<str>> PartialEq<T> for SolIdent {
    fn eq(&self, other: &T) -> bool {
        self.0 == other
    }
}

impl Parse for SolIdent {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for SolIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl From<Ident> for SolIdent {
    fn from(value: Ident) -> Self {
        Self(value)
    }
}

impl From<SolIdent> for Ident {
    fn from(value: SolIdent) -> Self {
        value.0
    }
}

impl SolIdent {
    pub fn new(s: &str) -> Self {
        Self(Ident::new(s, Span::call_site()))
    }

    pub fn new_spanned(s: &str, span: Span) -> Self {
        Self(Ident::new(s, span))
    }

    pub fn span(&self) -> Span {
        self.0.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
    }

    /// Returns the identifier as a string, without the `r#` prefix if present.
    pub fn as_string(&self) -> String {
        let mut s = self.0.to_string();
        if s.starts_with("r#") {
            s = s[2..].to_string();
        }
        s
    }
}
