use crate::{SolIdent, Spanned};
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

/// Create a [`SolPath`] from a list of identifiers.
#[macro_export]
macro_rules! sol_path {
    () => { $crate::SolPath::new() };

    ($($e:expr),+) => {{
        let mut path = $crate::SolPath::new();
        $(path.push($crate::SolIdent::from($e));)+
        path
    }};
}

/// A list of identifiers, separated by dots.
///
/// This is never parsed as empty.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SolPath(Punctuated<SolIdent, Token![.]>);

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

impl fmt::Debug for SolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
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

impl Spanned for SolPath {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
    }
}

impl SolPath {
    pub const fn new() -> Self {
        Self(Punctuated::new())
    }

    pub fn first(&self) -> &SolIdent {
        self.0.first().unwrap()
    }

    pub fn first_mut(&mut self) -> &mut SolIdent {
        self.0.first_mut().unwrap()
    }

    pub fn last(&self) -> &SolIdent {
        self.0.last().unwrap()
    }

    pub fn last_mut(&mut self) -> &mut SolIdent {
        self.0.last_mut().unwrap()
    }
}
