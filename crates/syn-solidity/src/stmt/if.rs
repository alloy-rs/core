use crate::{Expr, Spanned, Stmt};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    Result, Token,
};

/// An `if` statement with an optional `else` block: `if (expr) { ... } else {
/// ... }`.
#[derive(Clone)]
pub struct StmtIf {
    pub if_token: Token![if],
    pub paren_token: Paren,
    pub cond: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<(Token![else], Box<Stmt>)>,
}

impl fmt::Debug for StmtIf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtIf")
            .field("cond", &self.cond)
            .field("then_branch", &self.then_branch)
            .field("else_branch", &self.else_branch)
            .finish()
    }
}

impl Parse for StmtIf {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            if_token: input.parse()?,
            paren_token: syn::parenthesized!(content in input),
            cond: content.parse()?,
            then_branch: input.parse()?,
            else_branch: if input.peek(Token![else]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
        })
    }
}

impl Spanned for StmtIf {
    fn span(&self) -> Span {
        let span = self.if_token.span;
        self.else_branch
            .as_ref()
            .and_then(|(_, stmt)| stmt.span().join(span))
            .or_else(|| span.join(self.then_branch.span()))
            .unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.if_token.span = span;
        self.then_branch.set_span(span);
        if let Some((_, stmt)) = &mut self.else_branch {
            stmt.set_span(span);
        }
    }
}
