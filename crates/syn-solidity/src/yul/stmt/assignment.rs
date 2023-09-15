use crate::{Spanned, WalrusToken, YulExpr, YulFnCall, YulPath};

use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

/// Assignaration of one or more Yul variables with optional initial value.
/// If multiple variables are declared, only a function call is a valid initial
/// value.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulAssignment>
#[derive(Clone)]
pub enum YulVarAssign {
    /// Assign a single variable.
    Single(YulSingleAssign),

    /// Assign many variables, only assignable via function call.
    Multi(YulMultiAssign),
}

impl Parse for YulVarAssign {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek2(Token![,]) {
            // if an identifier is followed by a comma, parse as Multi
            Ok(YulVarAssign::Multi(input.parse()?))
        } else {
            // otherwise, parse as Single
            Ok(YulVarAssign::Single(input.parse()?))
        }
    }
}

impl Spanned for YulVarAssign {
    fn span(&self) -> Span {
        match self {
            Self::Single(asgn) => asgn.span(),
            Self::Multi(asgn) => asgn.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Single(asgn) => asgn.set_span(span),
            Self::Multi(asgn) => asgn.set_span(span),
        }
    }
}

impl fmt::Debug for YulVarAssign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YulVarAssign::")?;
        match self {
            Self::Single(asgn) => asgn.fmt(f),
            Self::Multi(asgn) => asgn.fmt(f),
        }
    }
}

/// Assign a value to a single Yul variable: `x := 0`.
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

/// Assign values to multiple Yul variables via funciton call: `x, y := foo()`.
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
