use crate::{utils::DebugPunctuated, Spanned, YulEVMBuiltIn, YulExpr, YulIdent};

use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Paren,
    Token,
};

/// Yul function call.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulFunctionCall>
#[derive(Clone)]
pub struct YulFnCall {
    pub function_type: FnType,
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
        let span = self.function_type.span();
        span.join(self.arguments.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.function_type.set_span(span);
        self.paren_token = Paren(span);
        self.arguments.set_span(span);
    }
}

impl fmt::Debug for YulFnCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulFnCall")
            .field("function_type", &self.function_type)
            .field("arguments", DebugPunctuated::new(&self.arguments))
            .finish()
    }
}

/// What type of function is called.
#[derive(Clone)]
pub enum FnType {
    /// When calling a self defined function
    Custom(YulIdent),

    /// When calling a built in evm opcode
    EVMOpcode(YulEVMBuiltIn),
}

impl Parse for FnType {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let speculative_parse = input.fork();

        if let Ok(evm_builtin) = speculative_parse.parse::<YulEVMBuiltIn>() {
            input.advance_to(&speculative_parse);
            Ok(Self::EVMOpcode(evm_builtin))
        } else {
            Ok(Self::Custom(input.parse::<YulIdent>()?))
        }
    }
}

impl Spanned for FnType {
    fn span(&self) -> Span {
        match self {
            Self::Custom(custom) => custom.span(),
            Self::EVMOpcode(opcode) => opcode.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Custom(custom) => custom.set_span(span),
            Self::EVMOpcode(opcode) => opcode.set_span(span),
        }
    }
}

impl fmt::Debug for FnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FnType::")?;
        match self {
            Self::Custom(custom) => custom.fmt(f),
            Self::EVMOpcode(opcode) => opcode.fmt(f),
        }
    }
}
