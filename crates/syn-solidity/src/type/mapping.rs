use crate::{kw, SolIdent, Type};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Result, Token,
};

/// A mapping type: `mapping(uint key => string value)`
#[derive(Clone)]
pub struct TypeMapping {
    pub mapping_token: kw::mapping,
    pub paren_token: Paren,
    pub key: Box<Type>,
    pub key_name: Option<SolIdent>,
    pub fat_arrow_token: Token![=>],
    pub value: Box<Type>,
    pub value_name: Option<SolIdent>,
}

impl PartialEq for TypeMapping {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl Eq for TypeMapping {}

impl Hash for TypeMapping {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.value.hash(state);
    }
}

impl fmt::Debug for TypeMapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypeMapping")
            .field("key", &self.key)
            .field("key_name", &self.key_name)
            .field("value", &self.value)
            .field("value_name", &self.value_name)
            .finish()
    }
}

impl fmt::Display for TypeMapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mapping({} ", self.key)?;
        if let Some(key_name) = &self.key_name {
            write!(f, "{key_name} ")?;
        }
        write!(f, "=> {} ", self.value)?;
        if let Some(value_name) = &self.value_name {
            write!(f, "{value_name}")?;
        }
        f.write_str(")")
    }
}

impl Parse for TypeMapping {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            mapping_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            key: content.parse()?,
            key_name: content.call(SolIdent::parse_opt)?,
            fat_arrow_token: content.parse()?,
            value: content.parse()?,
            value_name: content.call(SolIdent::parse_opt)?,
        })
    }
}

impl TypeMapping {
    pub fn span(&self) -> Span {
        let span = self.mapping_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.mapping_token.span = span;
        self.paren_token = Paren(span);
        self.key.set_span(span);
        if let Some(key_name) = &mut self.key_name {
            key_name.set_span(span);
        }
        self.value.set_span(span);
        if let Some(value_name) = &mut self.value_name {
            value_name.set_span(span);
        }
    }
}
