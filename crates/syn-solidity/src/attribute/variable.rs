use super::{kw, Override, Visibility};
use proc_macro2::Span;
use std::{
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    mem,
};
use syn::{
    parse::{Parse, ParseStream},
    Error, Result,
};

/// A list of unique variable attributes.
#[derive(Clone, Debug)]
pub struct VariableAttributes(pub HashSet<VariableAttribute>);

impl Parse for VariableAttributes {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut attributes = HashSet::new();
        while let Ok(attribute) = input.parse::<VariableAttribute>() {
            let error = |prev: &VariableAttribute| {
                let mut e = Error::new(attribute.span(), "duplicate attribute");
                e.combine(Error::new(prev.span(), "previous declaration is here"));
                e
            };

            // Only one of: `constant`, `immutable`
            match attribute {
                VariableAttribute::Constant(_) => {
                    if let Some(prev) =
                        attributes.get(&VariableAttribute::Immutable(Default::default()))
                    {
                        return Err(error(prev))
                    }
                }
                VariableAttribute::Immutable(_) => {
                    if let Some(prev) =
                        attributes.get(&VariableAttribute::Constant(Default::default()))
                    {
                        return Err(error(prev))
                    }
                }
                _ => {}
            }

            if let Some(prev) = attributes.get(&attribute) {
                return Err(error(prev))
            }
            attributes.insert(attribute);
        }
        Ok(Self(attributes))
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
        mem::discriminant(self).hash(state)
    }
}

impl Parse for VariableAttribute {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if Visibility::peek(&lookahead) {
            input.parse().map(Self::Visibility)
        } else if lookahead.peek(kw::constant) {
            input.parse().map(Self::Constant)
        } else if lookahead.peek(kw::Override) {
            input.parse().map(Self::Override)
        } else if lookahead.peek(kw::immutable) {
            input.parse().map(Self::Immutable)
        } else {
            Err(lookahead.error())
        }
    }
}

impl VariableAttribute {
    pub fn span(&self) -> Span {
        match self {
            Self::Visibility(v) => v.span(),
            Self::Constant(c) => c.span,
            Self::Override(o) => o.span(),
            Self::Immutable(i) => i.span,
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Visibility(v) => v.set_span(span),
            Self::Constant(c) => c.span = span,
            Self::Override(o) => o.set_span(span),
            Self::Immutable(i) => i.span = span,
        }
    }
}
