use crate::{kw, utils::DebugPunctuated, Spanned, YulBlock, YulIdent};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Result, Token,
};

/// Yul function definition: `function f() -> a, b { ... }`.
///
/// Solitify Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulFunctionDefinition>
#[derive(Clone)]
pub struct YulFunctionDef {
    pub function_token: kw::function,
    pub ident: YulIdent,
    pub paren_token: Paren,
    pub arguments: Punctuated<YulIdent, Token![,]>,
    pub returns: Option<YulReturns>,
    pub body: YulBlock,
}

impl Parse for YulFunctionDef {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            function_token: input.parse()?,
            ident: input.parse()?,
            paren_token: parenthesized!(content in input),
            arguments: Punctuated::parse_terminated(&content)?,
            returns: input.call(YulReturns::parse_opt)?,
            body: input.parse()?,
        })
    }
}

impl Spanned for YulFunctionDef {
    fn span(&self) -> Span {
        let span = self.function_token.span();
        span.join(self.body.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.function_token.set_span(span);
        self.ident.set_span(span);
        self.paren_token = Paren(span);
        self.arguments.set_span(span);
        self.returns.set_span(span);
        self.body.set_span(span);
    }
}

impl fmt::Debug for YulFunctionDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulFunctionDef")
            .field("ident", &self.ident)
            .field("arguments", DebugPunctuated::new(&self.arguments))
            .field("returns", &self.returns)
            .field("body", &self.body)
            .finish()
    }
}

/// The return attribute of a Yul function definition.
#[derive(Clone)]
pub struct YulReturns {
    pub arrow_token: Token![->],
    pub returns: Punctuated<YulIdent, Token![,]>,
}

impl YulReturns {
    pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
        if input.peek(Token![->]) {
            Ok(Some(Self {
                arrow_token: input.parse()?,
                returns: Punctuated::parse_separated_nonempty(input)?,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Parse for YulReturns {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            arrow_token: input.parse()?,
            returns: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

impl Spanned for YulReturns {
    fn span(&self) -> Span {
        let span = self.arrow_token.span();
        span.join(self.returns.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.arrow_token.set_span(span);
        self.returns.set_span(span);
    }
}

impl fmt::Debug for YulReturns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulReturns")
            .field("returns", DebugPunctuated::new(&self.returns))
            .finish()
    }
}
