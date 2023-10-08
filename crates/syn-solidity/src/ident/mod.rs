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
// couldn't find anything in syn
static RUST_KEYWORDS: [&'static str; 56] = [
    "as", "use", "break", "const", "continue", "crate", "else", "if", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "for", "let", "loop", "match", "mod", "move", "mut", "pub",
    "impl", "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "alignof", "become", "box", "do", "final",
    "macro", "offsetof", "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized",
    "virtual", "yield",
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
        if RUST_KEYWORDS.contains(&s) {
            Self(Ident::new(&format!("r#{}", s), Span::call_site()))
        } else {
            Self(Ident::new(s, Span::call_site()))
        }
    }

    pub fn new_spanned(s: &str, span: Span) -> Self {
        if RUST_KEYWORDS.contains(&s) {
            Self(Ident::new(&format!("r#{}", s), span))
        } else {
            Self(Ident::new(s, span))
        }
    }

    /// Strips the raw marker `r#`, if any, from the beginning of an ident.
    ///
    /// See [`IdentExt::unraw`].
    pub fn unwrawed(mut self) -> Self {
        self = self.unraw();
        self
    }

    /// Strips the raw marker `r#`, if any, from the beginning of an ident.
    ///
    /// See [`IdentExt::unraw`].
    pub fn unraw(&self) -> Self {
        Self(self.0.unraw())
    }

    /// Returns the identifier as a string, without the `r#` prefix if present.
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    /// Parses any identifier including keywords.
    pub fn parse_any(input: ParseStream<'_>) -> Result<Self> {
        check_dollar(input)?;
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

fn check_dollar(input: ParseStream<'_>) -> Result<()> {
    if input.peek(Token![$]) {
        Err(input.error("Solidity identifiers starting with `$` are unsupported. This is a known limitation of syn-solidity."))
    } else {
        Ok(())
    }
}
