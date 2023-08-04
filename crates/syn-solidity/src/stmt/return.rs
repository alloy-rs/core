use crate::{Expr, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// A return statement: `return 42;`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.returnStatement>
#[derive(Clone)]
pub struct StmtReturn {
    pub return_token: Token![return],
    pub expr: Option<Expr>,
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtReturn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtReturn")
            .field("expr", &self.expr)
            .finish()
    }
}

impl Parse for StmtReturn {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            return_token: input.parse()?,
            expr: if input.peek(Token![;]) {
                None
            } else {
                Some(input.parse()?)
            },
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for StmtReturn {
    fn span(&self) -> Span {
        let span = self.return_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.return_token.span = span;
        self.semi_token.span = span;
    }
}
