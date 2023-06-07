use super::SolIdent;
use proc_macro2::{Ident, Span};
use std::{
    fmt,
    ops::{Deref, DerefMut},
};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

/// A path of identifiers, separated by dots.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SolPath(pub Punctuated<SolIdent, Token![.]>);

impl Deref for SolPath {
    type Target = Punctuated<SolIdent, Token![.]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SolPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for SolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl fmt::Display for SolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, ident) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(".")?;
            }
            ident.fmt(f)?;
        }
        Ok(())
    }
}

impl FromIterator<SolIdent> for SolPath {
    fn from_iter<T: IntoIterator<Item = SolIdent>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Parse for SolPath {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // Modified from: `syn::Path::parse_mod_style`
        let mut segments = Punctuated::new();
        loop {
            if !input.peek(Ident::peek_any) {
                break
            }
            segments.push_value(input.parse()?);
            if !input.peek(Token![.]) {
                break
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
    pub const fn new() -> Self {
        Self(Punctuated::new())
    }

    pub fn span(&self) -> Span {
        let mut path = self.0.iter();
        let Some(first) = path.next() else { return Span::call_site() };
        path.fold(first.span(), |span, ident| {
            span.join(ident.span()).unwrap_or(span)
        })
    }

    pub fn set_span(&mut self, span: Span) {
        for ident in self.0.iter_mut() {
            ident.set_span(span);
        }
    }
}
