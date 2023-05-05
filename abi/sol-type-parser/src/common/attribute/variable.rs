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
    Result,
};

pub struct VariableAttributes(pub HashSet<VariableAttribute>);

impl Parse for VariableAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attributes = HashSet::new();
        while let Ok(attribute) = input.parse() {
            match attribute {
                VariableAttribute::Constant(_) => {
                    if attributes.contains(&VariableAttribute::Immutable(Default::default())) {
                        return Err(input.error("duplicate constant attribute"));
                    }
                }
                VariableAttribute::Immutable(_) => {
                    if attributes.contains(&VariableAttribute::Constant(Default::default())) {
                        return Err(input.error("duplicate constant attribute"));
                    }
                }
                _ => {}
            }
            let _ = attribute.span();
            if !attributes.insert(attribute) {
                return Err(input.error("duplicate attribute"));
            }
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
