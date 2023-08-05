use crate::Spanned;
use proc_macro2::{Span, TokenStream};
use std::fmt;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Result,
};

#[derive(Clone)]
pub struct YulBlock {
    pub brace_token: Brace,
    // TODO
    pub stmts: TokenStream,
}

impl fmt::Debug for YulBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulBlock")
            // .field("stmts", &self.stmts)
            .field("stmts", &self.stmts.to_string())
            .finish()
    }
}

impl Parse for YulBlock {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            stmts: content.parse()?,
        })
    }
}

impl Spanned for YulBlock {
    fn span(&self) -> Span {
        self.brace_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.brace_token = Brace(span);
    }
}
