use crate::{Expr, Spanned, Stmt};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    Result, Token,
};

/// A do-while statement: `do { ... } while (condition);`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.doWhileStatement>
#[derive(Clone)]
pub struct StmtDoWhile {
    pub do_token: Token![do],
    pub body: Box<Stmt>,
    pub while_token: Token![while],
    pub paren_token: Paren,
    pub cond: Expr,
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtDoWhile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoWhile")
            .field("body", &self.body)
            .field("condition", &self.cond)
            .finish()
    }
}

impl Parse for StmtDoWhile {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            do_token: input.parse()?,
            body: input.parse()?,
            while_token: input.parse()?,
            paren_token: syn::parenthesized!(content in input),
            cond: content.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for StmtDoWhile {
    fn span(&self) -> Span {
        let span = self.do_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.do_token.span = span;
        self.semi_token.span = span;
    }
}
