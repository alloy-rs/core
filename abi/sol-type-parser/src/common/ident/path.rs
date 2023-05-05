use super::SolIdent;
use proc_macro2::Span;
use std::fmt;
use syn::{
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
        let path = input.parse_terminated(SolIdent::parse, Token![.])?;
        if path.empty_or_trailing() {
            Err(input.error("expected identifier path"))
        } else {
            Ok(Self(path))
        }
    }
}

impl SolPath {
    pub fn span(&self) -> Span {
        let mut path = self.0.iter();
        let first = path.next().unwrap().span();
        let span = path.fold(first, |span, ident| span.join(ident.span()).unwrap_or(span));
        span
    }
}
