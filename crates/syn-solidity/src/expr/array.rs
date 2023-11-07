use crate::{
    utils::{DebugPunctuated, ParseNested},
    Expr, Spanned,
};
use proc_macro2::Span;
use std::fmt;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Bracket,
    Result, Token,
};

/// An array literal expression: `[a, b, c, d]`.
#[derive(Clone)]
pub struct ExprArray {
    pub bracket_token: Bracket,
    pub elems: Punctuated<Expr, Token![,]>,
}

impl fmt::Debug for ExprArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprArray").field("elems", DebugPunctuated::new(&self.elems)).finish()
    }
}

impl Parse for ExprArray {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            elems: content.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

impl Spanned for ExprArray {
    fn span(&self) -> Span {
        self.bracket_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.bracket_token = Bracket(span);
    }
}

/// A square bracketed indexing expression: `vector[2]`.
#[derive(Clone)]
pub struct ExprIndex {
    pub expr: Box<Expr>,
    pub bracket_token: Bracket,
    pub start: Option<Box<Expr>>,
    pub colon_token: Option<Token![:]>,
    pub end: Option<Box<Expr>>,
}

impl fmt::Debug for ExprIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprIndex")
            .field("expr", &self.expr)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl ParseNested for ExprIndex {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);
        let start = if content.is_empty() || content.peek(Token![:]) {
            None
        } else {
            Some(content.parse()?)
        };
        let colon_token = if content.is_empty() { None } else { Some(content.parse()?) };
        let end =
            if content.is_empty() || colon_token.is_none() { None } else { Some(content.parse()?) };
        Ok(Self { expr, bracket_token, start, colon_token, end })
    }
}

derive_parse!(ExprIndex);

impl Spanned for ExprIndex {
    fn span(&self) -> Span {
        let span = self.expr.span();
        span.join(self.bracket_token.span.join()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.expr.set_span(span);
        self.bracket_token = Bracket(span);
    }
}

impl ExprIndex {
    pub fn is_range(&self) -> bool {
        self.colon_token.is_some()
    }
}
