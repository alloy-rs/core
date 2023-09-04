use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

use crate::{
    yul::{expr::YulExpr, fn_call::YulFnCall, ident::YulIdent},
    Spanned,
};

// Declaration of one or more Yul variables with optional initial value.
// If multiple variables are declared, only a function call is a valid initial
// value.
//
// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulVariableDeclaration>
#[derive(Clone, Debug)]
pub enum YulVarDecl {
    // Declare a single variable.
    Single(SingleVarDecl),
    // Declare many variables, initialized only via function call.
    Many(ManyVarDecl),
}

impl Parse for YulVarDecl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // TODO: Figure if we can do this without forking
        let lookahead = input.fork();
        // both declarations share same first two tokens
        lookahead.parse::<Token![let]>()?;
        lookahead.parse::<YulIdent>()?;

        // declaration type is then judged based on next token
        if lookahead.peek(Token![,]) {
            Ok(YulVarDecl::Many(input.parse()?))
        } else {
            Ok(YulVarDecl::Single(input.parse()?))
        }
    }
}

impl Spanned for YulVarDecl {
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
pub struct SingleVarDecl {
    pub let_token: Token![let],
    pub name: YulIdent,
    pub assignment: Option<(WalrusOperator, YulExpr)>,
}

impl Parse for SingleVarDecl {
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

impl Spanned for SingleVarDecl {
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

impl fmt::Debug for SingleVarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SingleVarDecl")
            .field("let_token", &self.let_token)
            .field("name", &self.name)
            .field("assignment", &self.assignment)
            .finish()
    }
}

#[derive(Clone)]
pub struct ManyVarDecl {
    pub let_token: Token![let],
    pub vars: Punctuated<YulIdent, Token![,]>,
    pub assignment: Option<(WalrusOperator, YulFnCall)>,
}

impl Parse for ManyVarDecl {
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

impl Spanned for ManyVarDecl {
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

impl fmt::Debug for ManyVarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManyVarDecl")
            .field("let_token", &self.let_token)
            .field("vars", &self.vars)
            .field("assignment", &self.assignment)
            .finish()
    }
}

// Represents the walrus operator `:=`
#[derive(Clone, Debug)]
pub struct WalrusOperator {
    pub colon: Token![:],
    pub equals: Token![=],
}

impl Parse for WalrusOperator {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let colon = input.parse()?;
        let equals = input.parse()?;

        Ok(Self { colon, equals })
    }
}

impl Spanned for WalrusOperator {
    fn span(&self) -> Span {
        self.colon
            .span()
            .join(self.equals.span())
            .unwrap_or(self.colon.span())
    }

    fn set_span(&mut self, span: Span) {
        self.colon.set_span(span);
        self.equals.set_span(span);
    }
}
