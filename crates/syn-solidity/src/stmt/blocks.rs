use crate::{kw, Spanned, Stmt};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    token::Brace,
    Result,
};

/// A curly-braced block of statements: `{ ... }`.
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
                let mut vec = Vec::new();
                debug!("> Block: {:?}", content.to_string());
                while !content.is_empty() {
                    vec.push(content.parse()?);
                }
                debug!("< Block: {} stmts\n", vec.len());
                vec
            },
        })
    }
}

impl Spanned for Block {
    fn span(&self) -> Span {
        self.brace_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.brace_token = Brace(span);
    }
}

/// An unchecked block: `unchecked { ... }`.
#[derive(Clone)]
pub struct UncheckedBlock {
    pub unchecked_token: kw::unchecked,
    pub block: Block,
}

impl fmt::Debug for UncheckedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UncheckedBlock").field("stmts", &self.block.stmts).finish()
    }
}

impl Parse for UncheckedBlock {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { unchecked_token: input.parse()?, block: input.parse()? })
    }
}

impl Spanned for UncheckedBlock {
    fn span(&self) -> Span {
        let span = self.unchecked_token.span;
        span.join(self.block.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.unchecked_token.span = span;
        self.block.set_span(span);
    }
}
