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

// See `./kw.c`.

/// The set difference of the Rust and Solidity keyword sets. We need this so that we can emit raw
/// identifiers for Solidity keywords.
static KW_DIFFERENCE: &[&str] = &include!("./difference.expr");

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
        check_dollar(input)?;
        let id = Ident::parse_any(input)?;
        Ok(Self::from(id))
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

    pub fn new_spanned(mut s: &str, span: Span) -> Self {
        let mut new_raw = KW_DIFFERENCE.contains(&s);

        if s.starts_with("r#") {
            new_raw = true;
            s = &s[2..];
        }

        if matches!(s, "_" | "self" | "Self" | "super" | "crate") {
            new_raw = false;
        }

        if new_raw {
            Self(Ident::new_raw(s, span))
        } else {
            Self(Ident::new(s, span))
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sol_path;

    #[test]
    fn ident() {
        let id: SolIdent = syn::parse_str("a").unwrap();
        assert_eq!(id, SolIdent::new("a"));
    }

    #[test]
    fn keywords() {
        // keywords in Rust, but not Solidity; we try to make them "raw", although some, like
        // `crate` can never be made identifiers. See ./kw.c`.
        let difference: &[&str] = &include!("./difference.expr");
        for &s in difference {
            let id: SolIdent = syn::parse_str(s).unwrap();
            assert_eq!(id, SolIdent::new(s));
            assert_eq!(id.to_string(), format!("r#{s}"));
            assert_eq!(id.as_string(), s);
        }

        // keywords in both languages; we don't make them "raw" because they are always invalid.
        let intersection: &[&str] = &include!("./intersection.expr");
        for &s in intersection {
            let id: SolIdent = syn::parse_str(s).unwrap();
            assert_eq!(id, SolIdent::new(s));
            assert_eq!(id.to_string(), s);
            assert_eq!(id.as_string(), s);
        }
    }

    #[test]
    fn ident_path() {
        let path: SolPath = syn::parse_str("a.b.c").unwrap();
        assert_eq!(path, sol_path!["a", "b", "c"]);
    }

    #[test]
    fn ident_path_trailing() {
        let _e = syn::parse_str::<SolPath>("a.b.").unwrap_err();
    }

    #[test]
    fn ident_dollar() {
        assert!(syn::parse_str::<SolIdent>("$hello")
            .unwrap_err()
            .to_string()
            .contains("Solidity identifiers starting with `$` are unsupported."));
    }
}
