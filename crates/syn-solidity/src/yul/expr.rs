use std::fmt;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Paren,
};

use crate::Spanned;

use super::{fn_call::YulFnCall, ident::YulIdent, lit::YulLit};

#[derive(Clone)]
pub enum YulExpr {
    Path(YulIdent),
    Call(YulFnCall),
    Literal(YulLit),
}

impl Parse for YulExpr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek2(Paren) {
            input.parse().map(Self::Call)
        }

        // fork to find next type
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
