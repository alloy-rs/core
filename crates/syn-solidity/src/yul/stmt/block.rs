use crate::{Spanned, YulStmt};
use proc_macro2::Span;
use std::fmt;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Result,
};

/// A Yul block contains `YulStmt` between curly braces.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulBlock>
#[derive(Clone)]
pub struct YulBlock {
    pub brace_token: Brace,
    pub stmts: Vec<YulStmt>,
}

impl Parse for YulBlock {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            stmts: {
                let mut stmts = Vec::new();
                while !content.is_empty() {
                    let stmt: YulStmt = content.parse()?;
                    stmts.push(stmt);
                }
                stmts
            },
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

impl fmt::Debug for YulBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulBlock")
            .field("stmt", &self.stmts)
            .finish()
    }
}
