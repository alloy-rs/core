use crate::{Expr, Spanned, Stmt, StmtExpr, StmtVarDecl};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    Result, Token,
};

/// A for statement: `for (uint256 i; i < 42; ++i) { ... }`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.forStatement>
#[derive(Clone)]
pub struct StmtFor {
    pub for_token: Token![for],
    pub paren_token: Paren,
    pub init: Option<ForInitStmt>,
    pub semi_token1: Token![;],
    pub cond: Option<Expr>,
    pub semi_token2: Token![;],
    pub post: Option<Expr>,
    pub body: Box<Stmt>,
}

impl fmt::Debug for StmtFor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtFor")
            .field("init", &self.init)
            .field("cond", &self.cond)
            .field("post", &self.post)
            .field("body", &self.body)
            .finish()
    }
}

impl Parse for StmtFor {
    fn parse(_input: ParseStream<'_>) -> Result<Self> {
        todo!()
    }
}

impl Spanned for StmtFor {
    fn span(&self) -> Span {
        let span = self.for_token.span;
        span.join(self.body.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.for_token.span = span;
        self.body.set_span(span);
    }
}

#[derive(Clone, Debug)]
pub enum ForInitStmt {
    VarDecl(StmtVarDecl),
    Expression(StmtExpr),
}

impl Parse for ForInitStmt {
    fn parse(_input: ParseStream<'_>) -> Result<Self> {
        todo!()
    }
}
