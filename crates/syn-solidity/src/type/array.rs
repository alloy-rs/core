use crate::{Expr, Lit, LitNumber, Spanned, Type};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    token::Bracket,
    Result,
};

/// An array type.
#[derive(Clone)]
pub struct TypeArray {
    pub ty: Box<Type>,
    pub bracket_token: Bracket,
    pub size: Option<Box<Expr>>,
}

impl PartialEq for TypeArray {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.size() == other.size()
    }
}

impl Eq for TypeArray {}

impl Hash for TypeArray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.size().hash(state);
    }
}

impl fmt::Display for TypeArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ty.fmt(f)?;
        f.write_str("[")?;
        if let Some(s) = self.size_lit() {
            f.write_str(s.base10_digits())?;
        }
        f.write_str("]")
    }
}

impl fmt::Debug for TypeArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypeArray").field(&self.ty).field(&self.size()).finish()
    }
}

impl Parse for TypeArray {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ty = input.parse()?;
        Self::parse_nested(Box::new(ty), input)
    }
}

impl Spanned for TypeArray {
    fn span(&self) -> Span {
        let span = self.ty.span();
        span.join(self.bracket_token.span.join()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.ty.set_span(span);
        self.bracket_token = Bracket(span);
        if let Some(size) = &mut self.size {
            size.set_span(span);
        }
    }
}

impl TypeArray {
    /// Returns the size of the array, or None if dynamic.
    pub fn size(&self) -> Option<usize> {
        self.size_lit().and_then(|s| s.base10_parse().ok())
    }

    /// Returns the size of the array, or None if dynamic.
    pub fn size_lit(&self) -> Option<&LitNumber> {
        self.size.as_ref().and_then(|s| match &**s {
            Expr::Lit(Lit::Number(n)) => Some(n),
            _ => None,
        })
    }

    /// See [`Type::is_abi_dynamic`].
    pub fn is_abi_dynamic(&self) -> bool {
        match self.size {
            Some(_) => self.ty.is_abi_dynamic(),
            None => true,
        }
    }

    /// Parses an array type from the given input stream, wrapping `ty` with it.
    pub fn parse_nested(ty: Box<Type>, input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            ty,
            bracket_token: bracketed!(content in input),
            size: {
                if content.is_empty() {
                    None
                } else {
                    Some(content.parse()?)
                }
            },
        })
    }
}
