use crate::{utils::DebugPunctuated, Spanned, WalrusToken, YulExpr, YulIdent};

use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

/// Declares Yul variables, which may or may not have initial values. E.x.
/// `let x := 0`
/// `let x`
/// `let x, y := foo()`
/// `let x, y, z`
///
/// Multiple variables can only be initialized via a function call.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulVariableDeclaration>
#[derive(Clone)]
pub struct YulVarDecl {
    pub let_token: Token![let],
    pub vars: Punctuated<YulIdent, Token![,]>,
    pub init_value: Option<(WalrusToken, YulExpr)>,
}

impl Parse for YulVarDecl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let let_token = input.parse()?;
        let vars = Punctuated::parse_separated_nonempty(input)?;
        let init_value = if input.peek(Token![:]) && input.peek2(Token![=]) {
            Some((input.parse()?, input.parse()?))
        } else {
            None
        };

        if vars.len() > 1
            && init_value
                .as_ref()
                .map_or(false, |(_, expr)| !matches!(expr, YulExpr::Call(_)))
        {
            return Err(input.error("Multiple variables can only be initialized by a function call"))
        }

        Ok(Self {
            let_token,
            vars,
            init_value,
        })
    }
}

impl Spanned for YulVarDecl {
    fn span(&self) -> Span {
        let span = self.let_token.span();
        if let Some((_, expr)) = &self.init_value {
            span.join(expr.span()).unwrap_or(span)
        } else {
            span.join(self.vars.span()).unwrap_or(span)
        }
    }

    fn set_span(&mut self, span: Span) {
        self.let_token.set_span(span);
        crate::utils::set_spans(&mut self.vars, span);
        if let Some((walrus_token, init_value)) = &mut self.init_value {
            walrus_token.set_span(span);
            init_value.set_span(span);
        }
    }
}

impl fmt::Debug for YulVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulVarDecl")
            .field("vars", DebugPunctuated::new(&self.vars))
            .field(
                "init_value",
                &self.init_value.as_ref().map(|(_, expr)| expr),
            )
            .finish()
    }
}
