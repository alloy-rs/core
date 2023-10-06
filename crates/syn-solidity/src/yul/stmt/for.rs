use crate::{Spanned, YulBlock, YulExpr};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// Yul for loop e.g `for {let i := 0} lt(i,10) {i := add(i,1)} {mstore(i,7)}`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulForStatement>
/// breakdown of parts: <https://docs.soliditylang.org/en/latest/yul.html#loops>
#[derive(Clone)]
pub struct YulFor {
    for_token: Token![for],
    initialization: YulBlock,
    condition: YulExpr,
    post_iteration: YulBlock,
    body: YulBlock,
}

impl Parse for YulFor {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            initialization: input.parse()?,
            condition: input.parse()?,
            post_iteration: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl Spanned for YulFor {
    fn span(&self) -> Span {
        let span = self.for_token.span();
        span.join(self.body.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.for_token.set_span(span);
        self.initialization.set_span(span);
        self.condition.set_span(span);
        self.post_iteration.set_span(span);
        self.body.set_span(span);
    }
}

impl fmt::Debug for YulFor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulFor")
            .field("initialization", &self.initialization)
            .field("condition", &self.condition)
            .field("post_iteration", &self.post_iteration)
            .field("body", &self.body)
            .finish()
    }
}
