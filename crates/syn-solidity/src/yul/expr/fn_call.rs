use std::fmt;

use proc_macro2::Span;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Paren,
    Token,
};

use crate::{
    yul::{ident::YulIdent, r#type::evm_builtin::YulEVMBuiltIn},
    Spanned,
};

use super::YulExpr;

// Yul function call.
//
// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulFunctionCall>
#[derive(Clone)]
pub struct YulFnCall {
    pub function_type: FunctionType,
    pub paren_token: Paren,
    pub arguments: Punctuated<YulExpr, Token![,]>,
}

impl Parse for YulFnCall {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            function_type: input.parse()?,
            paren_token: parenthesized!(content in input),
            arguments: content.parse_terminated(YulExpr::parse, Token![,])?,
        })
    }
}

impl Spanned for YulFnCall {
    fn span(&self) -> Span {
        self.arguments.span()
    }

    fn set_span(&mut self, span: Span) {
        self.paren_token = Paren(span);
    }
}

impl fmt::Debug for YulFnCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulFnCall")
            .field("function_type", &self.function_type)
            .field("paren_token", &self.paren_token)
            .field("arguments", &self.arguments)
            .finish()
    }
}

// what type of function is called
#[derive(Clone, Debug)]
pub enum FunctionType {
    // when executing a self defined function
    YulFunctionCall(YulIdent),
    // when executing a built in evm opcode
    EVMOpcodeCall(YulEVMBuiltIn),
}

impl Parse for FunctionType {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let speculative_parse = input.fork();

        if speculative_parse.parse::<YulEVMBuiltIn>().is_ok() {
            Ok(Self::EVMOpcodeCall(input.parse::<YulEVMBuiltIn>()?))
        } else {
            Ok(Self::YulFunctionCall(input.parse::<YulIdent>()?))
        }
    }
}
