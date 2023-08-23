use crate::{kw, Spanned, Type};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Result, Token,
};

/// A `type()` expression: `type(uint256)`
#[derive(Clone)]
pub struct ExprTypeCall {
    pub type_token: Token![type],
    pub paren_token: Paren,
    pub ty: Type,
}

impl fmt::Debug for ExprTypeCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprTypeCall")
            .field("ty", &self.ty)
            .finish()
    }
}

impl Parse for ExprTypeCall {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            type_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            ty: content.parse()?,
        })
    }
}

impl Spanned for ExprTypeCall {
    fn span(&self) -> Span {
        let span = self.type_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.type_token.span = span;
        self.paren_token = Paren(span);
    }
}

/// A `new` expression: `new Contract`.
///
/// i.e. a contract creation or the allocation of a dynamic memory array.
#[derive(Clone)]
pub struct ExprNew {
    pub new_token: kw::new,
    pub ty: Type,
}

impl fmt::Debug for ExprNew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprNew").field("ty", &self.ty).finish()
    }
}

impl Parse for ExprNew {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            new_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Spanned for ExprNew {
    fn span(&self) -> Span {
        let span = self.new_token.span;
        span.join(self.ty.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.new_token.span = span;
        self.ty.set_span(span);
    }
}
