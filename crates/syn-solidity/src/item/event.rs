use crate::{kw, utils::DebugPunctuated, SolIdent, Type};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Attribute, Result, Token,
};

#[derive(Clone)]
pub struct ItemEvent {
    pub attrs: Vec<Attribute>,
    pub event_token: kw::event,
    pub name: SolIdent,
    pub paren_token: Paren,
    pub parameters: Punctuated<EventParameter, Token![,]>,
    pub anonymous: Option<kw::anonymous>,
    pub semi_token: Token![;],
}

impl fmt::Debug for ItemEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemEvent")
            .field("attrs", &self.attrs)
            .field("name", &self.name)
            .field("arguments", DebugPunctuated::new(&self.parameters))
            .field("anonymous", &self.anonymous.is_some())
            .finish()
    }
}

impl Parse for ItemEvent {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            event_token: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(content in input),
            parameters: content.parse_terminated(EventParameter::parse, Token![,])?,
            anonymous: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ItemEvent {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
    }

    /// Returns `true` if the event is anonymous.
    #[inline]
    pub const fn is_anonymous(&self) -> bool {
        self.anonymous.is_some()
    }

    pub fn partition_params(&self) -> (Vec<&EventParameter>, Vec<&EventParameter>) {
        self.parameters.iter().partition(|p| p.is_indexed())
    }

    pub fn non_indexed_params(&self) -> impl Iterator<Item = &EventParameter> {
        self.parameters.iter().filter(|p| !p.is_indexed())
    }

    pub fn indexed_params(&self) -> impl Iterator<Item = &EventParameter> {
        self.parameters.iter().filter(|p| p.is_indexed())
    }

    pub fn as_type(&self) -> Type {
        Type::Tuple(self.parameters.iter().map(|arg| arg.ty.clone()).collect())
    }
}

/// An event parameter.
///
/// `<ty> [indexed] [name]`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EventParameter {
    pub ty: Type,
    pub indexed: Option<kw::indexed>,
    pub name: Option<SolIdent>,
}

impl fmt::Debug for EventParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventParameter")
            .field("ty", &self.ty)
            .field("indexed", &self.indexed.is_some())
            .field("name", &self.name)
            .finish()
    }
}

impl Parse for EventParameter {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            indexed: input.parse()?,
            name: if SolIdent::peek_any(input) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl EventParameter {
    pub fn span(&self) -> Span {
        let span = self.ty.span();
        self.name
            .as_ref()
            .and_then(|name| span.join(name.span()))
            .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.ty.set_span(span);
        if let Some(kw) = &mut self.indexed {
            kw.span = span;
        }
        if let Some(name) = &mut self.name {
            name.set_span(span);
        }
    }

    /// Returns `true` if the parameter is indexed.
    #[inline]
    pub const fn is_indexed(&self) -> bool {
        self.indexed.is_some()
    }
}
