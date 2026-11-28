use crate::{Override, SolPath, Spanned, Visibility, kw};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
    mem,
};
use syn::{
    Error, Result, Token,
    parse::{Parse, ParseStream},
};

/// A list of unique variable attributes.
#[derive(Clone, Debug)]
pub struct VariableAttributes(pub Vec<VariableAttribute>);

impl fmt::Display for VariableAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, attr) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            write!(f, "{attr}")?;
        }
        Ok(())
    }
}

impl Parse for VariableAttributes {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut attributes = Vec::new();
        let mut exclusive_attribute_index = None;
        while let Ok(attribute) = input.parse::<VariableAttribute>() {
            let error = |prev: &VariableAttribute, msg: &str| {
                let mut e = Error::new(attribute.span(), msg);
                e.combine(Error::new(prev.span(), "previous declaration is here"));
                e
            };

            if let Some(prev) = attributes.iter().find(|a| **a == attribute) {
                return Err(error(prev, "duplicate attribute"));
            }

            // Only one of: `constant`, `immutable`, `transient`.
            if matches!(
                &attribute,
                VariableAttribute::Constant(_)
                    | VariableAttribute::Immutable(_)
                    | VariableAttribute::Transient(_)
            ) {
                if let Some(i) = exclusive_attribute_index {
                    let conflicting = &attributes[i];
                    return Err(error(conflicting, "conflicting attribute"));
                }

                exclusive_attribute_index = Some(attributes.len());
            }

            attributes.push(attribute);
        }
        Ok(Self(attributes))
    }
}

impl Spanned for VariableAttributes {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
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

    pub fn has_transient(&self) -> bool {
        self.0.iter().any(VariableAttribute::is_transient)
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
    /// `transient`.
    Transient(kw::transient),
    /// An [Override] attribute.
    Override(Override),
}

impl fmt::Display for VariableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility(v) => v.fmt(f),
            Self::Constant(_) => f.write_str("constant"),
            Self::Immutable(_) => f.write_str("immutable"),
            Self::Transient(_) => f.write_str("transient"),
            Self::Override(o) => o.fmt(f),
        }
    }
}

impl fmt::Debug for VariableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility(v) => v.fmt(f),
            Self::Constant(_) => f.write_str("Constant"),
            Self::Immutable(_) => f.write_str("Immutable"),
            Self::Transient(_) => f.write_str("Transient"),
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
        // Like solc: `transient` before `=` or `;` is the variable name, e.g. `uint transient;`.
        } else if lookahead.peek(kw::transient)
            && !input.peek2(Token![=])
            && !input.peek2(Token![;])
        {
            input.parse().map(Self::Transient)
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
            Self::Immutable(i) => i.span,
            Self::Transient(t) => t.span,
            Self::Override(o) => o.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Visibility(v) => v.set_span(span),
            Self::Constant(c) => c.span = span,
            Self::Immutable(i) => i.span = span,
            Self::Transient(t) => t.span = span,
            Self::Override(o) => o.set_span(span),
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
    pub const fn is_transient(&self) -> bool {
        matches!(self, Self::Transient(_))
    }

    #[inline]
    pub fn is_override(&self, path: Option<&SolPath>) -> bool {
        self.r#override().is_some_and(|o| match path {
            Some(path) => o.paths.iter().any(|p| p == path),
            None => true,
        })
    }
}
