use crate::{kw, Expr, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// A revert statement: `revert("error");`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.revertStatement>
#[derive(Clone)]
pub struct StmtRevert {
    pub revert_token: kw::revert,
    pub expr: Expr,
    // pub list: ArgList, // TODO
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtRevert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtRevert")
            .field("expr", &self.expr)
            // .field("list", &self.list)
            .finish()
    }
}

impl Parse for StmtRevert {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            revert_token: input.parse()?,
            expr: input.parse()?,
            // list: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for StmtRevert {
    fn span(&self) -> Span {
        let span = self.revert_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.revert_token.span = span;
        self.semi_token.span = span;
    }
}
