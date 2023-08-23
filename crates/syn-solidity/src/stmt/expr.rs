use crate::{Expr, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// An expression with a trailing semicolon.
#[derive(Clone)]
pub struct StmtExpr {
    pub expr: Expr,
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtExpr")
            .field("expr", &self.expr)
            .finish()
    }
}

impl Parse for StmtExpr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for StmtExpr {
    fn span(&self) -> Span {
        let span = self.expr.span();
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.expr.set_span(span);
        self.semi_token.span = span;
    }
}
