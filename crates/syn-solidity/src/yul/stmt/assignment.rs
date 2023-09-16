use crate::{utils::DebugPunctuated, Spanned, WalrusToken, YulExpr, YulPath};

use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

/// Yul variable assignment. `x := 0` or `x, y := foo()`.
/// Assigning values to multiple variables requires a function call.
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulAssignment>
#[derive(Clone)]
pub struct YulVarAssign {
    pub vars: Punctuated<YulPath, Token![,]>,
    pub walrus_token: WalrusToken,
    pub assigned_value: YulExpr,
}

impl Parse for YulVarAssign {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let vars = Punctuated::parse_separated_nonempty(input)?;
        let walrus_token = input.parse()?;
        let assigned_value = input.parse()?;

        if vars.len() > 1 && !matches!(assigned_value, YulExpr::Call(_)) {
            return Err(input.error("Multiple variables require a function call for assignment"))
        }

        Ok(Self {
            vars,
            walrus_token,
            assigned_value,
        })
    }
}

impl Spanned for YulVarAssign {
    fn span(&self) -> Span {
        let span = crate::utils::join_spans(&self.vars);
        span.join(self.assigned_value.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        crate::utils::set_spans(&mut self.vars, span);
        self.walrus_token.set_span(span);
        self.assigned_value.set_span(span);
    }
}

impl fmt::Debug for YulVarAssign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulVarAssign")
            .field("vars", DebugPunctuated::new(&self.vars))
            .field("assigned_value", &self.assigned_value)
            .finish()
    }
}
