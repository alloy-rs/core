use crate::Expr;
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// A ternary (AKA conditional) expression: `foo ? bar : baz`.
#[derive(Clone)]
pub struct ExprTernary {
    pub cond: Box<Expr>,
    pub question_token: Token![?],
    pub if_true: Box<Expr>,
    pub colon_token: Token![:],
    pub if_false: Box<Expr>,
}

impl fmt::Debug for ExprTernary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprTernary")
            .field("cond", &self.cond)
            .field("if_true", &self.if_true)
            .field("if_false", &self.if_false)
            .finish()
    }
}

impl Parse for ExprTernary {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            cond: input.parse()?,
            question_token: input.parse()?,
            if_true: input.parse()?,
            colon_token: input.parse()?,
            if_false: input.parse()?,
        })
    }
}

impl ExprTernary {
    pub fn span(&self) -> Span {
        let span = self.cond.span();
        span.join(self.if_false.span()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.cond.set_span(span);
        self.if_false.set_span(span);
    }
}
