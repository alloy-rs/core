use crate::{Spanned, YulBlock, YulExpr};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// A Yul if statement: `if lt(a, b) { sstore(0, 1) }`.
///
/// Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulIfStatement>
#[derive(Clone)]
pub struct YulIf {
    pub if_token: Token![if],
    pub cond: YulExpr,
    pub then_branch: Box<YulBlock>,
}

impl Parse for YulIf {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            if_token: input.parse()?,
            cond: input.parse()?,
            then_branch: input.parse()?,
        })
    }
}

impl Spanned for YulIf {
    fn span(&self) -> Span {
        let span = self.if_token.span;
        span.join(self.then_branch.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.if_token.set_span(span);
        self.cond.set_span(span);
        self.then_branch.set_span(span);
    }
}

impl fmt::Debug for YulIf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulIf")
            .field("cond", &self.cond)
            .field("then_branch", &self.then_branch)
            .finish()
    }
}
