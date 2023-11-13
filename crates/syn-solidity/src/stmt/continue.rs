use crate::Spanned;
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// A continue statement: `continue;`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.continueStatement>
#[derive(Clone)]
pub struct StmtContinue {
    pub continue_token: Token![continue],
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtContinue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtContinue").finish()
    }
}

impl Parse for StmtContinue {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { continue_token: input.parse()?, semi_token: input.parse()? })
    }
}

impl Spanned for StmtContinue {
    fn span(&self) -> Span {
        let span = self.continue_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.continue_token.span = span;
        self.semi_token.span = span;
    }
}
