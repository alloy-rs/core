use super::SolIdent;
use proc_macro2::{Ident, Span};
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SolPath(pub Punctuated<SolIdent, Token![.]>);

impl fmt::Debug for SolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl fmt::Display for SolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut path = self.0.iter();
        path.next().unwrap().fmt(f)?;
        for ident in path {
            write!(f, ".{ident}")?;
        }
        Ok(())
    }
}

impl Parse for SolPath {
    fn parse(input: ParseStream) -> Result<Self> {
        // Modified from: `syn::Path::parse_mod_style`
        let mut segments = Punctuated::new();
        loop {
            if !input.peek(Ident::peek_any) {
                break;
            }
            segments.push_value(input.parse()?);
            if !input.peek(Token![.]) {
                break;
            }
            segments.push_punct(input.parse()?);
        }

        if segments.is_empty() {
            Err(input.parse::<SolIdent>().unwrap_err())
        } else if segments.trailing_punct() {
            Err(input.error("expected path segment after `.`"))
        } else {
            Ok(Self(segments))
        }
    }
}

impl SolPath {
    pub fn span(&self) -> Span {
        let mut path = self.0.iter();
        let first = path.next().unwrap().span();

        path.fold(first, |span, ident| span.join(ident.span()).unwrap_or(span))
    }
}
