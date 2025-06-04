use crate::{Expr, Spanned, utils::DebugPunctuated};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
};

/// A tuple expression: `(a, b, c, d)`.
#[derive(Clone)]
pub struct ExprTuple {
    pub paren_token: Paren,
    pub elems: Punctuated<Expr, Token![,]>,
}

impl fmt::Debug for ExprTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprTuple").field("elems", DebugPunctuated::new(&self.elems)).finish()
    }
}

impl Parse for ExprTuple {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            elems: content.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

impl Spanned for ExprTuple {
    fn span(&self) -> Span {
        self.paren_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.paren_token = Paren(span);
    }
}
