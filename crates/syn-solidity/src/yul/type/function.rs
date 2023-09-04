use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Result, Token,
};

use crate::{kw, yul::ident::YulIdent, YulBlock};

// Yul function definition.
//
// Solitify Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulFunctionDefinition>
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
            arguments: Punctuated::parse_separated_nonempty(&content)?,
            returns: input.call(YulReturns::parse_opt)?,
            body: input.parse()?,
        })
    }
}

// The return attribute of a Yul function defenition.
#[derive(Clone)]
pub struct YulReturns {
    pub arrow_token: Token![->],
    // cannot be parsed as empty
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
