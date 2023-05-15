use crate::common::{kw, Override, Visibility};
use proc_macro2::Span;
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    mem,
};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Result,
};

pub struct VariableAttributes(pub HashSet<VariableAttribute>);

impl Parse for VariableAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
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

#[derive(Clone)]
pub enum VariableAttribute {
    Visibility(Visibility),
    Constant(kw::constant),
    Immutable(kw::immutable),
    Override(Override),
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
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::external)
            || lookahead.peek(kw::public)
            || lookahead.peek(kw::internal)
            || lookahead.peek(kw::private)
        {
            Ok(Self::Visibility(input.parse()?))
        } else if lookahead.peek(kw::constant) {
            Ok(Self::Constant(input.parse()?))
        } else if lookahead.peek(kw::Override) {
            Ok(Self::Override(input.parse()?))
        } else if lookahead.peek(kw::immutable) {
            Ok(Self::Immutable(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl VariableAttribute {
    pub fn span(&self) -> Span {
        match self {
            Self::Visibility(v) => v.span(),
            Self::Constant(c) => c.span(),
            Self::Override(o) => o.span(),
            Self::Immutable(i) => i.span(),
        }
    }
}
