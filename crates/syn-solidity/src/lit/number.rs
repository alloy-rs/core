use crate::{kw, Spanned};
use proc_macro2::{Literal, Span};
use std::{fmt, str::FromStr};
use syn::{
    parse::{Lookahead1, Parse, ParseStream},
    LitFloat, LitInt, Result,
};

// TODO: Fixed point numbers

/// An integer or fixed-point number literal: `1` or `1.0`.
#[derive(Clone, Debug)]
pub struct LitNumber {
    pub kind: LitNumberKind,
    pub denom: Option<SubDenomination>,
}

impl Parse for LitNumber {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            kind: input.parse()?,
            denom: input.call(SubDenomination::parse_opt)?,
        })
    }
}

impl Spanned for LitNumber {
    fn span(&self) -> Span {
        let span = self.kind.span();
        self.denom
            .as_ref()
            .and_then(|d| d.span().join(span))
            .unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.kind.set_span(span);
        if let Some(denom) = &mut self.denom {
            denom.set_span(span);
        }
    }
}

impl LitNumber {
    pub fn new(kind: LitNumberKind) -> Self {
        Self { kind, denom: None }
    }

    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        LitNumberKind::peek(lookahead)
    }
}

#[derive(Clone)]
pub enum LitNumberKind {
    Int(LitInt),
    Float(LitFloat),
}

impl fmt::Debug for LitNumberKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(lit) => lit.fmt(f),
            Self::Float(lit) => lit.fmt(f),
        }
    }
}

impl Parse for LitNumberKind {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            input.parse().map(Self::Int)
        } else if lookahead.peek(LitFloat) {
            input.parse().map(Self::Float)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Spanned for LitNumberKind {
    fn span(&self) -> Span {
        match self {
            Self::Int(lit) => lit.span(),
            Self::Float(lit) => lit.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Int(lit) => lit.set_span(span),
            Self::Float(lit) => lit.set_span(span),
        }
    }
}

impl LitNumberKind {
    pub fn new_int(repr: &str, span: Span) -> Self {
        Self::Int(LitInt::new(repr, span))
    }

    pub fn new_fixed(repr: &str, span: Span) -> Self {
        Self::Float(LitFloat::new(repr, span))
    }

    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        lookahead.peek(LitInt) || lookahead.peek(LitFloat)
    }

    /// Returns the base-10 digits of the literal.
    pub fn base10_digits(&self) -> &str {
        match self {
            Self::Int(lit) => lit.base10_digits(),
            Self::Float(lit) => lit.base10_digits(),
        }
    }

    /// Parses the literal into a selected number type.
    ///
    /// This is equivalent to `lit.base10_digits().parse()` except that the
    /// resulting errors will be correctly spanned to point to the literal token
    /// in the macro input.
    pub fn base10_parse<N>(&self) -> Result<N>
    where
        N: FromStr,
        N::Err: fmt::Display,
    {
        match self {
            Self::Int(lit) => lit.base10_parse(),
            Self::Float(lit) => lit.base10_parse(),
        }
    }

    pub fn suffix(&self) -> &str {
        match self {
            Self::Int(lit) => lit.suffix(),
            Self::Float(lit) => lit.suffix(),
        }
    }

    pub fn token(&self) -> Literal {
        match self {
            Self::Int(lit) => lit.token(),
            Self::Float(lit) => lit.token(),
        }
    }
}

kw_enum! {
    pub enum SubDenomination {
        Wei(kw::wei),
        Gwei(kw::gwei),
        Ether(kw::ether),

        Seconds(kw::seconds),
        Minutes(kw::minutes),
        Hours(kw::hours),
        Days(kw::days),
        Weeks(kw::weeks),
        Years(kw::years),
    }
}
