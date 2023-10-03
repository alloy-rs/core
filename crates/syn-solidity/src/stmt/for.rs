use crate::{Expr, Spanned, Stmt, StmtExpr, StmtVarDecl};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
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
    pub init: ForInitStmt,
    pub cond: Option<Box<Expr>>,
    pub semi_token: Token![;],
    pub post: Option<Box<Expr>>,
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
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            for_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            init: content.parse()?,
            cond: if content.peek(Token![;]) {
                None
            } else {
                Some(content.parse()?)
            },
            semi_token: content.parse()?,
            post: if content.is_empty() {
                None
            } else {
                Some(content.parse()?)
            },
            body: input.parse()?,
        })
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

/// A for statement initializer.
///
/// This can either be empty, a variable declaration, or an expression.
#[derive(Clone, Debug)]
pub enum ForInitStmt {
    Empty(Token![;]),
    VarDecl(StmtVarDecl),
    Expr(StmtExpr),
}

impl Parse for ForInitStmt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![;]) {
            input.parse().map(Self::Empty)
        } else {
            match StmtVarDecl::parse_or_expr(input)? {
                Stmt::VarDecl(decl) => Ok(Self::VarDecl(decl)),
                Stmt::Expr(expr) => Ok(Self::Expr(expr)),
                s => unreachable!("StmtVarDecl::parse_or_expr: invalid output {s:?}"),
            }
        }
    }
}
