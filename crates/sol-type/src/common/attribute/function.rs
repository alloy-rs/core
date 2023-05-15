use crate::common::{kw, Modifier, Mutability, Override, Visibility};
use proc_macro2::Span;
use std::{
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    mem,
};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::{Brace, Bracket},
    Error, Ident, Result, Token,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionAttributes(pub HashSet<FunctionAttribute>);

impl Parse for FunctionAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attributes = HashSet::<FunctionAttribute>::new();
        while !(input.is_empty()
            || input.peek(kw::returns)
            || input.peek(Token![;])
            || input.peek(Bracket))
        {
            let attr = input.parse()?;
            if let Some(prev) = attributes.get(&attr) {
                let mut e = Error::new(attr.span(), "duplicate attribute");
                e.combine(Error::new(prev.span(), "previous declaration is here"));
                return Err(e)
            }
            attributes.insert(attr);
        }
        Ok(Self(attributes))
    }
}

#[derive(Clone)]
pub enum FunctionAttribute {
    Visibility(Visibility),
    Mutability(Mutability),
    Virtual(kw::Virtual),
    Immutable(kw::immutable),
    Override(Override),
    Modifier(Modifier),
}

impl fmt::Debug for FunctionAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility(visibility) => f.debug_tuple("Visibility").field(visibility).finish(),
            Self::Mutability(mutability) => f.debug_tuple("Mutability").field(mutability).finish(),
            Self::Virtual(_) => f.write_str("Virtual"),
            Self::Immutable(_) => f.write_str("immutable"),
            Self::Override(o) => o.fmt(f),
            Self::Modifier(modifier) => modifier.fmt(f),
        }
    }
}

impl PartialEq for FunctionAttribute {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
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
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::external)
            || lookahead.peek(kw::public)
            || lookahead.peek(kw::internal)
            || lookahead.peek(kw::private)
        {
            Ok(Self::Visibility(input.parse()?))
        } else if lookahead.peek(kw::pure)
            || lookahead.peek(kw::view)
            || lookahead.peek(kw::constant)
            || lookahead.peek(kw::payable)
        {
            Ok(Self::Mutability(input.parse()?))
        } else if lookahead.peek(kw::Virtual) {
            Ok(Self::Virtual(input.parse()?))
        } else if lookahead.peek(kw::Override) {
            Ok(Self::Override(input.parse()?))
        } else if lookahead.peek(kw::immutable) {
            Ok(Self::Immutable(input.parse()?))
        } else if !input.peek(kw::returns) && lookahead.peek(Ident::peek_any) {
            Ok(Self::Modifier(input.parse()?))
        } else if input.peek(Brace) {
            // special case for function with implementation
            Err(input.error("functions cannot have an implementation"))
        } else {
            Err(lookahead.error())
        }
    }
}

impl FunctionAttribute {
    pub fn span(&self) -> Span {
        match self {
            Self::Visibility(v) => v.span(),
            Self::Mutability(m) => m.span(),
            Self::Virtual(v) => v.span(),
            Self::Override(o) => o.span(),
            Self::Immutable(i) => i.span(),
            Self::Modifier(m) => m.span(),
        }
    }
}
