use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

mod args;
pub use args::{CallArgumentList, CallArgumentListImpl, NamedArg};

/// An expression.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.expression>
#[derive(Clone, Debug)]
pub enum Expr {}

impl Parse for Expr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let _ = input;
        todo!()
    }
}

impl Expr {
    pub fn span(&self) -> Span {
        match *self {}
    }

    pub fn set_span(&mut self, span: Span) {
        let _ = span;
        match *self {}
    }
}
