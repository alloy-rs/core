use crate::Stmt;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::Brace,
    Result,
};

/// A curly-braced block of statements.
#[derive(Clone)]
pub struct Block {
    pub brace_token: Brace,
    pub stmts: Vec<Stmt>,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Block").field(&self.stmts).finish()
    }
}

impl Parse for Block {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: syn::braced!(content in input),
            stmts: {
                let mut stmts = Vec::new();
                while !content.is_empty() {
                    stmts.push(content.parse()?);
                }
                stmts
            },
        })
    }
}
