use crate::{kw, LitHexStr, LitStr, Spanned};

use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream, Result},
    LitBool, LitInt,
};

mod hex_num;
pub use hex_num::YulHexNum;

/// Yul literals e.g. 0x123, 42 or "abc".
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulLiteral>
#[derive(Clone)]
pub enum YulLit {
    /// A decimal literal
    Decimal(LitInt),

    /// A hex string literal: `hex"1234"`.
    HexStr(LitHexStr),

    /// A boolean literal: `true` or `false`.
    Boolean(LitBool),

    /// A string literal.
    Str(LitStr),

    // A hexnumber begining with 0x prefix: 0xbadf00d
    HexNum(YulHexNum),
}

impl Parse for YulLit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            input.parse().map(Self::Decimal)
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Boolean)
        } else if lookahead.peek(syn::LitStr) {
            input.parse().map(Self::Str)
        } else if lookahead.peek(kw::hex) {
            input.parse().map(Self::HexStr)
        } else {
            input.parse().map(Self::HexNum)
        }
    }
}

impl Spanned for YulLit {
    fn span(&self) -> Span {
        match self {
            Self::Decimal(decimal) => decimal.span(),
            Self::HexStr(hex_str) => hex_str.span(),
            Self::Boolean(boolean) => boolean.span(),
            Self::Str(str) => str.span(),
            Self::HexNum(hex_num) => hex_num.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Decimal(decimal) => decimal.set_span(span),
            Self::HexStr(hex_str) => hex_str.set_span(span),
            Self::Boolean(boolean) => boolean.set_span(span),
            Self::Str(str) => str.set_span(span),
            Self::HexNum(hex_num) => hex_num.set_span(span),
        }
    }
}

impl fmt::Debug for YulLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YulLit::")?;
        match self {
            Self::Decimal(decimal) => decimal.fmt(f),
            Self::HexStr(hex_str) => hex_str.fmt(f),
            Self::Boolean(boolean) => boolean.fmt(f),
            Self::Str(str) => str.fmt(f),
            Self::HexNum(hex_num) => hex_num.fmt(f),
        }
    }
}
