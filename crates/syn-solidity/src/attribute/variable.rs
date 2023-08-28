use crate::{kw, Override, SolPath, Spanned, Visibility};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
    mem,
};
use syn::{
    parse::{Parse, ParseStream},
    Error, Result, Token,
};

/// A list of unique variable attributes.
#[derive(Clone, Debug)]
pub struct VariableAttributes(pub Vec<VariableAttribute>);

impl Parse for VariableAttributes {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut attributes = Vec::new();
        while let Ok(attribute) = input.parse::<VariableAttribute>() {
            let error = |prev: &VariableAttribute| {
                let mut e = Error::new(attribute.span(), "duplicate attribute");
                e.combine(Error::new(prev.span(), "previous declaration is here"));
                e
            };

            // Only one of: `constant`, `immutable`
            match attribute {
                VariableAttribute::Constant(_) => {
                    if let Some(prev) = attributes
                        .iter()
                        .find(|a| matches!(a, VariableAttribute::Immutable(_)))
                    {
                        return Err(error(prev))
                    }
                }
                VariableAttribute::Immutable(_) => {
                    if let Some(prev) = attributes
                        .iter()
                        .find(|a| matches!(a, VariableAttribute::Constant(_)))
                    {
                        return Err(error(prev))
                    }
                }
                _ => {}
            }

            if let Some(prev) = attributes.iter().find(|a| **a == attribute) {
                return Err(error(prev))
            }
            attributes.push(attribute);
        }
        Ok(Self(attributes))
    }
}

impl Spanned for VariableAttributes {
    fn span(&self) -> Span {
        crate::utils::join_spans(&self.0)
    }

    fn set_span(&mut self, span: Span) {
        crate::utils::set_spans_clone(&mut self.0, span);
    }
}

impl VariableAttributes {
    pub fn visibility(&self) -> Option<Visibility> {
        self.0.iter().find_map(VariableAttribute::visibility)
    }

    pub fn has_external(&self) -> bool {
        self.0.iter().any(VariableAttribute::is_external)
    }

    pub fn has_internal(&self) -> bool {
        self.0.iter().any(VariableAttribute::is_internal)
    }

    pub fn has_private(&self) -> bool {
        self.0.iter().any(VariableAttribute::is_private)
    }

    pub fn has_public(&self) -> bool {
        self.0.iter().any(VariableAttribute::is_public)
    }

    pub fn has_constant(&self) -> bool {
        self.0.iter().any(VariableAttribute::is_constant)
    }

    pub fn has_immutable(&self) -> bool {
        self.0.iter().any(VariableAttribute::is_immutable)
    }

    pub fn has_override(&self, path: Option<&SolPath>) -> bool {
        self.0.iter().any(|attr| attr.is_override(path))
    }
}

/// A variable attribute.
#[derive(Clone)]
pub enum VariableAttribute {
    /// A [Visibility] attribute.
    Visibility(Visibility),
    /// `constant`.
    Constant(kw::constant),
    /// `immutable`.
    Immutable(kw::immutable),
    /// An [Override] attribute.
    Override(Override),
}

impl fmt::Debug for VariableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility(v) => v.fmt(f),
            Self::Constant(_) => f.write_str("Constant"),
            Self::Immutable(_) => f.write_str("Immutable"),
            Self::Override(o) => o.fmt(f),
        }
    }
}

impl PartialEq for VariableAttribute {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Eq for VariableAttribute {}

impl Hash for VariableAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
    }
}

impl Parse for VariableAttribute {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if Visibility::peek(&lookahead) {
            input.parse().map(Self::Visibility)
        } else if lookahead.peek(kw::constant) {
            input.parse().map(Self::Constant)
        } else if lookahead.peek(Token![override]) {
            input.parse().map(Self::Override)
        } else if lookahead.peek(kw::immutable) {
            input.parse().map(Self::Immutable)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Spanned for VariableAttribute {
    fn span(&self) -> Span {
        match self {
            Self::Visibility(v) => v.span(),
            Self::Constant(c) => c.span,
            Self::Override(o) => o.span(),
            Self::Immutable(i) => i.span,
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Visibility(v) => v.set_span(span),
            Self::Constant(c) => c.span = span,
            Self::Override(o) => o.set_span(span),
            Self::Immutable(i) => i.span = span,
        }
    }
}

impl VariableAttribute {
    #[inline]
    pub const fn visibility(&self) -> Option<Visibility> {
        match self {
            Self::Visibility(v) => Some(*v),
            _ => None,
        }
    }

    #[inline]
    pub const fn r#override(&self) -> Option<&Override> {
        match self {
            Self::Override(o) => Some(o),
            _ => None,
        }
    }

    #[inline]
    pub const fn is_external(&self) -> bool {
        matches!(self, Self::Visibility(Visibility::External(_)))
    }

    #[inline]
    pub const fn is_public(&self) -> bool {
        matches!(self, Self::Visibility(Visibility::Public(_)))
    }

    #[inline]
    pub const fn is_internal(&self) -> bool {
        matches!(self, Self::Visibility(Visibility::Internal(_)))
    }

    #[inline]
    pub const fn is_private(&self) -> bool {
        matches!(self, Self::Visibility(Visibility::Private(_)))
    }

    #[inline]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    #[inline]
    pub const fn is_immutable(&self) -> bool {
        matches!(self, Self::Immutable(_))
    }

    #[inline]
    pub fn is_override(&self, path: Option<&SolPath>) -> bool {
        self.r#override().map_or(false, |o| match path {
            Some(path) => o.paths.iter().any(|p| p == path),
            None => true,
        })
    }
}
