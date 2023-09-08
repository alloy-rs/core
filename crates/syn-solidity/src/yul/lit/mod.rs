use syn::{
    parse::{Parse, ParseStream, Result},
    LitBool, LitInt,
};

use crate::Spanned;

/// Yul literals e.g. 0x123, 42 or "abc"
///
/// Reference:
/// https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulIfStatement
#[derive(Clone, Debug)]
pub enum YulLit {
    Decimal(LitInt),

    String(),

    Hex(),

    Boolean(LitBool),

    HexString(),
}

impl Parse for YulLit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        todo!()
    }
}

impl Spanned for YulLit {
    fn span(&self) -> proc_macro2::Span {
        todo!()
    }

    fn set_span(&mut self, span: proc_macro2::Span) {
        todo!()
    }
}
