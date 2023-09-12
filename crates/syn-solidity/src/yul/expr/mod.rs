use std::fmt;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Paren,
};

mod fn_call;
pub use fn_call::YulFnCall;

mod path;
pub use path::YulPath;

use crate::Spanned;

use super::lit::YulLit;

// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulExpression>
#[derive(Clone)]
pub enum YulExpr {
    Path(YulPath),
    Call(YulFnCall),
    Literal(YulLit),
}

impl Parse for YulExpr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek2(Paren) {
            return input.parse().map(Self::Call)
        }

        // fork to find next type
        let fork = input.fork();
        if fork.parse::<YulLit>().is_ok() {
            return input.parse().map(Self::Literal)
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
