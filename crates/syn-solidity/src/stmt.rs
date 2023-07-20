use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::Brace,
    Result,
};

use crate::expr::Expr;

/// A curly-braced block of statements.
#[derive(Clone)]
pub struct Block {
    pub brace_token: Brace,
    pub stmts: Vec<Expr>,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Block").field(&self.stmts).finish()
    }
}

impl Parse for Block {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let mut exprs = Vec::new();

        while !input.peek(Brace) {
            exprs.push(input.parse()?);
        }

        Ok(Self {
            brace_token: syn::braced!(content in input),
            stmts: exprs,
        })
    }
}
