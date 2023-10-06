use crate::{kw, utils::DebugPunctuated, Spanned, Type};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Error, Result, Token,
};

/// A tuple type.
#[derive(Clone)]
pub struct TypeTuple {
    pub tuple_token: Option<kw::tuple>,
    pub paren_token: Paren,
    pub types: Punctuated<Type, Token![,]>,
}

impl PartialEq for TypeTuple {
    fn eq(&self, other: &Self) -> bool {
        self.types == other.types
    }
}

impl Eq for TypeTuple {}

impl Hash for TypeTuple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.types.hash(state);
    }
}

impl fmt::Display for TypeTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        for (i, ty) in self.types.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            ty.fmt(f)?;
        }
        f.write_str(")")
    }
}

impl fmt::Debug for TypeTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypeTuple")
            .field(DebugPunctuated::new(&self.types))
            .finish()
    }
}

impl Parse for TypeTuple {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let this = Self {
            tuple_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            types: content.parse_terminated(Type::parse, Token![,])?,
        };
        match this.types.len() {
            0 => Err(Error::new(
                this.paren_token.span.join(),
                "empty tuples are not allowed",
            )),
            1 if !this.types.trailing_punct() => Err(Error::new(
                this.paren_token.span.close(),
                "single element tuples must have a trailing comma",
            )),
            _ => Ok(this),
        }
    }
}

impl FromIterator<Type> for TypeTuple {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        Self {
            tuple_token: None,
            paren_token: Paren::default(),
            types: {
                let mut types = iter.into_iter().collect::<Punctuated<_, _>>();
                // ensure trailing comma for single item tuple
                if !types.trailing_punct() && types.len() == 1 {
                    types.push_punct(Default::default());
                }
                types
            },
        }
    }
}

impl Spanned for TypeTuple {
    fn span(&self) -> Span {
        let span = self.paren_token.span.join();
        self.tuple_token
            .and_then(|tuple_token| tuple_token.span.join(span))
            .unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        if let Some(tuple_token) = &mut self.tuple_token {
            tuple_token.span = span;
        }
        self.paren_token = Paren(span);
    }
}

impl TypeTuple {
    /// See [`Type::is_abi_dynamic`].
    pub fn is_abi_dynamic(&self) -> bool {
        self.types.iter().any(Type::is_abi_dynamic)
    }
}
