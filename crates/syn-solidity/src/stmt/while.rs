use crate::{Expr, Spanned, Stmt};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    Result, Token,
};

/// A while statement: `while (i < 42) { ... }`.
///
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.whileStatement>
#[derive(Clone)]
pub struct StmtWhile {
    pub while_token: Token![while],
    pub paren_token: Paren,
    pub cond: Expr,
    pub body: Box<Stmt>,
}

impl fmt::Debug for StmtWhile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtWhile").field("cond", &self.cond).field("body", &self.body).finish()
    }
}

impl Parse for StmtWhile {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            while_token: input.parse()?,
            paren_token: syn::parenthesized!(content in input),
            cond: content.parse()?,
            body: input.parse()?,
        })
    }
}

impl Spanned for StmtWhile {
    fn span(&self) -> Span {
        let span = self.while_token.span;
        span.join(self.body.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.while_token.span = span;
        self.body.set_span(span);
    }
}
