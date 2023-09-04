use std::fmt;

use syn::parse::{Parse, ParseStream, Result};

use crate::Spanned;

// Yul function call.
//
// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulFunctionCall>
#[derive(Clone)]
pub struct YulFnCall {}

impl Parse for YulFnCall {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        todo!()
    }
}

impl Spanned for YulFnCall {
    fn span(&self) -> proc_macro2::Span {
        todo!()
    }

    fn set_span(&mut self, span: proc_macro2::Span) {
        todo!()
    }
}

impl fmt::Debug for YulFnCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulFnCall").finish()
    }
}
