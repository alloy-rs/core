use crate::Spanned;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Result, Token,
};

mod path;
pub use path::SolPath;

// taken from https://gist.github.com/ritz078/1be714dea593838587c8a5df463a583a
// this is the set difference of Rust - Solidity keywords
static RUST_KEYWORD_SET_DIFFERENCE: [&str; 28] = [
    "as", "use", "const", "extern", "false", "fn", "impl", "in", "move", "mut", "pub", "impl",
    "ref", "trait", "true", "type", "unsafe", "use", "where", "alignof", "become", "box",
    "offsetof", "priv", "proc", "unsized", "yield", "return",
];

/// A Solidity identifier.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SolIdent(pub Ident);

impl quote::IdentFragment for SolIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }

    fn span(&self) -> Option<Span> {
        Some(self.0.span())
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

impl From<Ident> for SolIdent {
    fn from(value: Ident) -> Self {
        Self::new_spanned(&value.to_string(), value.span())
    }
}

impl From<SolIdent> for Ident {
    fn from(value: SolIdent) -> Self {
        value.0
    }
}

impl From<&str> for SolIdent {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Parse for SolIdent {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // TODO: Deny Solidity keywords
        Self::parse_any(input)
    }
}

impl ToTokens for SolIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl Spanned for SolIdent {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
    }
}

impl SolIdent {
    pub fn new(s: &str) -> Self {
        Self::new_spanned(s, Span::call_site())
    }

    pub fn new_spanned(s: &str, span: Span) -> Self {
        if RUST_KEYWORD_SET_DIFFERENCE.contains(&s) {
            Self(Ident::new_raw(s, span))
        } else {
            Self(Ident::new(s, span))
        }
    }

    /// Strips the raw marker `r#`, if any, from the beginning of an ident.
    ///
    /// See [`IdentExt::unraw`].
    pub fn unwrawed(self) -> Self {
        self.clone()
    }

    /// Strips the raw marker `r#`, if any, from the beginning of an ident.
    ///
    /// See [`IdentExt::unraw`].
    pub fn unraw(&self) -> Self {
        self.clone()
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
        check_dollar(input)?;

        input.call(Ident::parse_any).map(Into::into)
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

fn check_dollar(input: ParseStream<'_>) -> Result<()> {
    if input.peek(Token![$]) {
        Err(input.error("Solidity identifiers starting with `$` are unsupported. This is a known limitation of syn-solidity."))
    } else {
        Ok(())
    }
}
