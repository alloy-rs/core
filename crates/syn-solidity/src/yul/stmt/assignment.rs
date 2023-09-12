use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

use crate::{
    yul::{
        expr::{YulExpr, YulFnCall, YulPath},
        ident::YulIdent,
    },
    Spanned,
};

use super::walrus_token::WalrusToken;

// Assignaration of one or more Yul variables with optional initial value.
// If multiple variables are declared, only a function call is a valid initial
// value.
//
// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulVariableAssignaration>
#[derive(Clone, Debug)]
pub enum YulVarAssign {
    // Assignare a single variable.
    Single(YulSingleAssign),
    // Assignare many variables, initialized only via function call.
    Many(YulMultiAssign),
}

impl Parse for YulVarAssign {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // TODO: Figure if we can do this without forking
        let lookahead = input.fork();
        // both declarations share same first token
        lookahead.parse::<YulIdent>()?;

        // declaration type is then judged based on next token
        if lookahead.peek(Token![,]) {
            Ok(YulVarAssign::Many(input.parse()?))
        } else {
            Ok(YulVarAssign::Single(input.parse()?))
        }
    }
}

impl Spanned for YulVarAssign {
    fn span(&self) -> Span {
        match self {
            Self::Single(decl) => decl.span(),
            Self::Many(decl) => decl.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Single(decl) => decl.set_span(span),
            Self::Many(decl) => decl.set_span(span),
        }
    }
}

#[derive(Clone)]
pub struct YulSingleAssign {
    pub name: YulPath,
    pub walrus_token: WalrusToken,
    pub assigned_value: YulExpr,
}

impl Parse for YulSingleAssign {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            walrus_token: input.parse()?,
            assigned_value: input.parse()?,
        })
    }
}

impl Spanned for YulSingleAssign {
    fn span(&self) -> Span {
        let span = self.name.span();
        span.join(self.assigned_value.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
        self.walrus_token.set_span(span);
        self.assigned_value.set_span(span);
    }
}

impl fmt::Debug for YulSingleAssign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SingleVarAssign")
            .field("name", &self.name)
            .field("walrus_token", &self.walrus_token)
            .field("assigned_value", &self.assigned_value)
            .finish()
    }
}

#[derive(Clone)]
pub struct YulMultiAssign {
    pub variables: Punctuated<YulPath, Token![,]>,
    pub walrus_token: WalrusToken,
    pub assigned_value: YulFnCall,
}

impl Parse for YulMultiAssign {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            variables: Punctuated::parse_separated_nonempty(input)?,
            walrus_token: input.parse()?,
            assigned_value: input.parse()?,
        })
    }
}

impl Spanned for YulMultiAssign {
    fn span(&self) -> Span {
        let span = self.variables.span();
        span.join(self.assigned_value.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.variables.set_span(span);
        self.walrus_token.set_span(span);
        self.assigned_value.set_span(span);
    }
}

impl fmt::Debug for YulMultiAssign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManyVarAssign")
            .field("variables", &self.variables)
            .field("walrus_token", &self.walrus_token)
            .field("assigned_value", &self.assigned_value)
            .finish()
    }
}
