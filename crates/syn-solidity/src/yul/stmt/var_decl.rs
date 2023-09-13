use crate::{Spanned, WalrusToken, YulExpr, YulFnCall, YulIdent};

use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

/// Declaration of one or more Yul variables with optional initial value.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulVariableDeclaration>
#[derive(Clone)]
pub enum YulVarDecl {
    /// Declare a single yul variable.
    Single(YulSingleDecl),

    /// Declare many yul vars, values only set via function call.
    Multi(YulMultiDecl),
}

impl Parse for YulVarDecl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek3(Token![,]) {
            Ok(YulVarDecl::Multi(input.parse()?))
        } else {
            Ok(YulVarDecl::Single(input.parse()?))
        }
    }
}

impl Spanned for YulVarDecl {
    fn span(&self) -> Span {
        match self {
            Self::Single(decl) => decl.span(),
            Self::Multi(decl) => decl.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Single(decl) => decl.set_span(span),
            Self::Multi(decl) => decl.set_span(span),
        }
    }
}

impl fmt::Debug for YulVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YulVarDecl::")?;
        match self {
            Self::Single(decl) => decl.fmt(f),
            Self::Multi(decl) => decl.fmt(f),
        }
    }
}

/// Declare a single Yul variable: `let x := 0` or `let x`.
#[derive(Clone)]
pub struct YulSingleDecl {
    pub let_token: Token![let],
    pub name: YulIdent,
    pub assignment: Option<(WalrusToken, YulExpr)>,
}

impl Parse for YulSingleDecl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            let_token: input.parse()?,
            name: input.parse()?,
            assignment: if input.peek(Token![:]) && input.peek2(Token![=]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
        })
    }
}

impl Spanned for YulSingleDecl {
    fn span(&self) -> Span {
        let span = self.let_token.span();
        match &self.assignment {
            Some((_, expr)) => span.join(expr.span()),
            None => span.join(self.name.span()),
        }
        .unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.let_token.set_span(span);
        self.name.set_span(span);
        if let Some((walrus, expr)) = &mut self.assignment {
            walrus.set_span(span);
            expr.set_span(span);
        }
    }
}

impl fmt::Debug for YulSingleDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YulSingleDecl")
            .field("let_token", &self.let_token)
            .field("name", &self.name)
            .field("assignment", &self.assignment)
            .finish()
    }
}

/// Declare multiple Yul variables: `let x, y := foo()` or `let x, y, z`.
#[derive(Clone)]
pub struct YulMultiDecl {
    pub let_token: Token![let],
    pub vars: Punctuated<YulIdent, Token![,]>,
    pub assignment: Option<(WalrusToken, YulFnCall)>,
}

impl Parse for YulMultiDecl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            let_token: input.parse()?,
            vars: Punctuated::parse_separated_nonempty(input)?,
            assignment: if input.peek(Token![:]) && input.peek2(Token![=]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
        })
    }
}

impl Spanned for YulMultiDecl {
    fn span(&self) -> Span {
        let span = self.let_token.span();
        match &self.assignment {
            Some((_, expr)) => span.join(expr.span()),
            None => span.join(self.vars.span()),
        }
        .unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.let_token.set_span(span);
        self.vars.set_span(span);
        if let Some((walrus, expr)) = &mut self.assignment {
            walrus.set_span(span);
            expr.set_span(span);
        }
    }
}

impl fmt::Debug for YulMultiDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YulMultiDecl")
            .field("let_token", &self.let_token)
            .field("vars", &self.vars)
            .field("assignment", &self.assignment)
            .finish()
    }
}
