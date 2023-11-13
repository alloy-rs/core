use crate::{kw, utils::ParseNested, Expr, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

/// A unary operation: `!x`, `-x`.
#[derive(Clone, Debug)]
pub struct ExprUnary {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl Parse for ExprUnary {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { op: input.parse()?, expr: input.parse()? })
    }
}

impl Spanned for ExprUnary {
    fn span(&self) -> Span {
        let span = self.op.span();
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.op.set_span(span);
        self.expr.set_span(span);
    }
}

/// A unary `delete` expression: `delete vector`.
#[derive(Clone)]
pub struct ExprDelete {
    pub delete_token: kw::delete,
    pub expr: Box<Expr>,
}

impl fmt::Debug for ExprDelete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprDelete").field("expr", &self.expr).finish()
    }
}

impl Parse for ExprDelete {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { delete_token: input.parse()?, expr: input.parse()? })
    }
}

impl Spanned for ExprDelete {
    fn span(&self) -> Span {
        let span = self.delete_token.span;
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.delete_token.span = span;
        self.expr.set_span(span);
    }
}

/// A postfix unary expression: `foo++`.
#[derive(Clone, Debug)]
pub struct ExprPostfix {
    pub expr: Box<Expr>,
    pub op: PostUnOp,
}

impl ParseNested for ExprPostfix {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { expr, op: input.parse()? })
    }
}

derive_parse!(ExprPostfix);

impl Spanned for ExprPostfix {
    fn span(&self) -> Span {
        let span = self.op.span();
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.op.set_span(span);
        self.expr.set_span(span);
    }
}

op_enum! {
    /// Unary operators.
    pub enum UnOp {
        Increment(++) peek2,
        Decrement(--) peek2,
        Not(!),
        BitNot(~),
        Neg(-),
    }
}

op_enum! {
    /// Postfix unary operators.
    pub enum PostUnOp {
        Increment(++) peek2,
        Decrement(--) peek2,
    }
}
