use crate::Spanned;
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// A break statement: `break;`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.breakStatement>
#[derive(Clone)]
pub struct StmtBreak {
    pub break_token: Token![break],
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtBreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtBreak").finish()
    }
}

impl Parse for StmtBreak {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            break_token: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for StmtBreak {
    fn span(&self) -> Span {
        let span = self.break_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.break_token.span = span;
        self.semi_token.span = span;
    }
}
