use crate::{utils::ParseNested, Expr, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// Access of a named member: `obj.k`.
#[derive(Clone)]
pub struct ExprMember {
    pub expr: Box<Expr>,
    pub dot_token: Token![.],
    pub member: Box<Expr>,
}

impl fmt::Debug for ExprMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprMember")
            .field("expr", &self.expr)
            .field("member", &self.member)
            .finish()
    }
}

impl ParseNested for ExprMember {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            expr,
            dot_token: input.parse()?,
            member: input.parse()?,
        })
    }
}

derive_parse!(ExprMember);

impl Spanned for ExprMember {
    fn span(&self) -> Span {
        self.expr
            .span()
            .join(self.member.span())
            .unwrap_or_else(|| {
                self.dot_token
                    .span
                    .join(self.member.span())
                    .unwrap_or_else(|| self.expr.span())
            })
    }

    fn set_span(&mut self, span: Span) {
        self.expr.set_span(span);
    }
}
