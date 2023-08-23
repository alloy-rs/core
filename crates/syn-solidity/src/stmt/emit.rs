use crate::{kw, Expr, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// An emit statement: `emit FooBar(42);`.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.emitStatement>
#[derive(Clone)]
pub struct StmtEmit {
    pub emit_token: kw::emit,
    pub expr: Expr,
    // pub list: ArgList, // TODO
    pub semi_token: Token![;],
}

impl fmt::Debug for StmtEmit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtEmit")
            .field("expr", &self.expr)
            // .field("list", &self.list)
            .finish()
    }
}

impl Parse for StmtEmit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            emit_token: input.parse()?,
            expr: input.parse()?,
            // list: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for StmtEmit {
    fn span(&self) -> Span {
        let span = self.emit_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.emit_token.span = span;
        self.semi_token.span = span;
    }
}
