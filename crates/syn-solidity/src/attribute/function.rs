use crate::{kw, Modifier, Mutability, Override, SolPath, Spanned, VariableAttribute, Visibility};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
    mem,
    ops::{Deref, DerefMut},
};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::Brace,
    Error, Ident, Result, Token,
};

/// A list of unique function attributes. Used in
/// [ItemFunction][crate::ItemFunction].
#[derive(Clone, Default, PartialEq, Eq)]
pub struct FunctionAttributes(pub Vec<FunctionAttribute>);

impl fmt::Debug for FunctionAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for FunctionAttributes {
    type Target = Vec<FunctionAttribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FunctionAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Parse for FunctionAttributes {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut attributes = Vec::<FunctionAttribute>::new();
        while !input.is_empty() && !input.peek(kw::returns) && input.peek(Ident::peek_any) {
            let attr: FunctionAttribute = input.parse()?;
            if let Some(prev) = attributes.iter().find(|a| **a == attr) {
                let mut e = Error::new(attr.span(), "duplicate attribute");
                e.combine(Error::new(prev.span(), "previous declaration is here"));
                return Err(e)
            }
            attributes.push(attr);
        }
        Ok(Self(attributes))
    }
}

impl Spanned for FunctionAttributes {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
    }
}

impl FunctionAttributes {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn visibility(&self) -> Option<Visibility> {
        self.0.iter().find_map(FunctionAttribute::visibility)
    }

    pub fn mutability(&self) -> Option<Mutability> {
        self.0.iter().find_map(FunctionAttribute::mutability)
    }

    pub fn r#override(&self) -> Option<&Override> {
        self.0.iter().find_map(FunctionAttribute::r#override)
    }

    pub fn modifier(&self) -> Option<&Modifier> {
        self.0.iter().find_map(FunctionAttribute::modifier)
    }

    pub fn has_external(&self) -> bool {
        self.0.iter().any(FunctionAttribute::is_external)
    }

    pub fn has_internal(&self) -> bool {
        self.0.iter().any(FunctionAttribute::is_internal)
    }

    pub fn has_private(&self) -> bool {
        self.0.iter().any(FunctionAttribute::is_private)
    }

    pub fn has_public(&self) -> bool {
        self.0.iter().any(FunctionAttribute::is_public)
    }

    pub fn has_virtual(&self) -> bool {
        self.0.iter().any(FunctionAttribute::is_virtual)
    }

    pub fn has_override(&self, path: Option<&SolPath>) -> bool {
        self.0.iter().any(|attr| attr.is_override(path))
    }

    pub fn has_modifier(&self, path: Option<&SolPath>) -> bool {
        self.0.iter().any(|attr| attr.is_modifier(path))
    }
}

/// A function attribute.
#[derive(Clone)]
pub enum FunctionAttribute {
    /// A [Visibility] attribute.
    Visibility(Visibility),
    /// A [Mutability] attribute.
    Mutability(Mutability),
    /// A [Modifier] attribute.
    Modifier(Modifier),
    /// `virtual`
    Virtual(Token![virtual]),
    /// An [Override] attribute.
    Override(Override),
}

impl fmt::Display for FunctionAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility(visibility) => visibility.fmt(f),
            Self::Mutability(mutability) => mutability.fmt(f),
            Self::Virtual(_) => f.write_str("virtual"),
            Self::Override(o) => o.fmt(f),
            Self::Modifier(modifier) => modifier.fmt(f),
        }
    }
}

impl fmt::Debug for FunctionAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility(visibility) => f.debug_tuple("Visibility").field(visibility).finish(),
            Self::Mutability(mutability) => f.debug_tuple("Mutability").field(mutability).finish(),
            Self::Virtual(_) => f.write_str("Virtual"),
            Self::Override(o) => o.fmt(f),
            Self::Modifier(modifier) => modifier.fmt(f),
        }
    }
}

impl PartialEq for FunctionAttribute {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Modifier(a), Self::Modifier(b)) => a == b,
            _ => mem::discriminant(self) == mem::discriminant(other),
        }
    }
}

impl Eq for FunctionAttribute {}

impl Hash for FunctionAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        if let Self::Modifier(m) = self {
            m.hash(state);
        }
    }
}

impl Parse for FunctionAttribute {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if Visibility::peek(&lookahead) {
            input.parse().map(Self::Visibility)
        } else if Mutability::peek(&lookahead) {
            input.parse().map(Self::Mutability)
        } else if lookahead.peek(Token![virtual]) {
            input.parse().map(Self::Virtual)
        } else if lookahead.peek(Token![override]) {
            input.parse().map(Self::Override)
        } else if !input.peek(kw::returns) && lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::Modifier)
        } else if input.peek(Brace) {
            // special case for function with implementation
            Err(input.error("functions cannot have an implementation"))
        } else {
            Err(lookahead.error())
        }
    }
}

impl From<VariableAttribute> for FunctionAttribute {
    /// Converts a variable attribute to its corresponding function attribute.
    ///
    /// - `constant` -> `pure`
    /// - `immutable` -> `view`
    fn from(value: VariableAttribute) -> Self {
        match value {
            VariableAttribute::Visibility(v) => Self::Visibility(v),
            VariableAttribute::Constant(c) => Self::Mutability(Mutability::new_pure(c.span)),
            VariableAttribute::Immutable(i) => Self::Mutability(Mutability::new_view(i.span)),
            VariableAttribute::Override(o) => Self::Override(o),
        }
    }
}

impl Spanned for FunctionAttribute {
    fn span(&self) -> Span {
        match self {
            Self::Visibility(v) => v.span(),
            Self::Mutability(m) => m.span(),
            Self::Virtual(v) => v.span,
            Self::Override(o) => o.span(),
            Self::Modifier(m) => m.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Visibility(v) => v.set_span(span),
            Self::Mutability(m) => m.set_span(span),
            Self::Virtual(v) => v.span = span,
            Self::Override(o) => o.set_span(span),
            Self::Modifier(m) => m.set_span(span),
        }
    }
}

impl FunctionAttribute {
    #[inline]
    pub const fn visibility(&self) -> Option<Visibility> {
        match self {
            Self::Visibility(v) => Some(*v),
            _ => None,
        }
    }

    #[inline]
    pub const fn mutability(&self) -> Option<Mutability> {
        match self {
            Self::Mutability(m) => Some(*m),
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
    pub const fn modifier(&self) -> Option<&Modifier> {
        match self {
            Self::Modifier(m) => Some(m),
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
    pub const fn is_virtual(&self) -> bool {
        matches!(self, Self::Virtual(_))
    }

    #[inline]
    pub fn is_override(&self, path: Option<&SolPath>) -> bool {
        self.r#override().map_or(false, |o| match path {
            Some(path) => o.paths.iter().any(|p| p == path),
            None => true,
        })
    }

    #[inline]
    pub fn is_modifier(&self, path: Option<&SolPath>) -> bool {
        self.modifier().map_or(false, |m| match path {
            Some(path) => m.name == *path,
            None => true,
        })
    }
}
