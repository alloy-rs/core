use crate::{Lit, Spanned, YulPath};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    token::Paren,
};

mod fn_call;
pub use fn_call::YulFnCall;

/// A Yul expression.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulExpression>
#[derive(Clone)]
pub enum YulExpr {
    Path(YulPath),
    Call(YulFnCall),
    Literal(Lit),
}

impl Parse for YulExpr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek2(Paren) {
            return input.parse().map(Self::Call);
        }

        let speculative_parse = input.fork();

        if let Ok(lit) = speculative_parse.parse::<Lit>() {
            input.advance_to(&speculative_parse);
            return Ok(Self::Literal(lit));
        }

        input.parse().map(Self::Path)
    }
}

impl Spanned for YulExpr {
    fn span(&self) -> Span {
        match self {
            Self::Path(path) => path.span(),
            Self::Call(call) => call.span(),
            Self::Literal(lit) => lit.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Path(path) => path.set_span(span),
            Self::Call(call) => call.set_span(span),
            Self::Literal(lit) => lit.set_span(span),
        }
    }
}

impl fmt::Debug for YulExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YulExpr::")?;
        match self {
            Self::Path(path) => path.fmt(f),
            Self::Call(call) => call.fmt(f),
            Self::Literal(lit) => lit.fmt(f),
        }
    }
}
