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

/// Create a [`SolPath`] from a list of identifiers.
#[macro_export]
macro_rules! sol_path {
    () => { $crate::SolPath::new() };

    ($($e:expr),+) => {{
        let mut path = $crate::SolPath::new();
        $(path.push($crate::SolIdent::from($e));)+
        path
    }};

    ($($id:ident).+) => {{
        let mut path = $crate::SolPath::new();
        $(path.push($crate::SolIdent::new(stringify!($id))));+
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

    pub fn first(&self) -> &SolIdent {
        self.0.first().unwrap()
    }

    pub fn first_mut(&mut self) -> &mut SolIdent {
        self.0.first_mut().unwrap()
    }

    pub fn last(&self) -> &SolIdent {
        self.0.last().unwrap()
    }

    // TODO: paths resolution
    #[track_caller]
    pub fn last_tmp(&self) -> &SolIdent {
        if self.len() > 1 {
            todo!()
        }
        self.last()
    }

    pub fn last_mut(&mut self) -> &mut SolIdent {
        self.0.last_mut().unwrap()
    }

    pub fn span(&self) -> Span {
        let Some(first) = self.0.first() else { return Span::call_site() };
        let span = first.span();
        self.0
            .last()
            .and_then(|last| span.join(last.span()))
            .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        for ident in self.0.iter_mut() {
            ident.set_span(span);
        }
    }
}
