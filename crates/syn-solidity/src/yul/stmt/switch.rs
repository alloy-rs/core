use proc_macro2::Span;
use std::fmt;
use syn::parse::{Parse, ParseStream, Result};

use crate::{
    kw,
    yul::{expr::YulExpr, lit::YulLit},
    Spanned, YulBlock,
};

// A Yul switch statement can consist of only a default-case or one
// or more non-default cases optionally followed by a default-case.
//
// Example switch statement in Yul:
//
// switch exponent
// case 0 { result := 1 }
// case 1 { result := base }
// default { revert(0, 0) }
//
// Solidity Reference:
// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulSwitchStatement>
#[derive(Clone)]
struct YulSwitch {
    switch_token: kw::switch,
    selector: YulExpr,
    branches: Vec<SwitchBranch>,
    default_case: Option<SwitchDefault>,
}

impl Parse for YulSwitch {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            switch_token: input.parse()?,
            selector: input.parse()?,
            branches: {
                let mut branches = vec![];
                while input.peek(kw::case) {
                    branches.push(input.parse()?);
                }
                branches
            },
            default_case: {
                if input.peek(kw::default) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
        })
    }
}

impl Spanned for YulSwitch {
    fn span(&self) -> Span {
        let span = self.switch_token.span();
        if let Some(default_case) = &self.default_case {
            return span.join(default_case.span()).unwrap_or(span)
        }
        span.join(self.branches.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.switch_token.set_span(span);
        self.selector.set_span(span);
        self.branches.set_span(span);
        self.default_case.set_span(span);
    }
}

// represents a non-default case of a Yul switch stmt.
#[derive(Clone)]
struct SwitchBranch {
    case_token: kw::case,
    constant: YulLit,
    body: YulBlock,
}

impl Parse for SwitchBranch {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            case_token: input.parse()?,
            constant: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl fmt::Debug for SwitchBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SwitchBranch")
            .field("case_token", &self.case_token)
            .field("constant", &self.constant)
            .field("body", &self.body)
            .finish()
    }
}

impl Spanned for SwitchBranch {
    fn span(&self) -> Span {
        let span = self.case_token.span();
        span.join(self.body.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.case_token.set_span(span);
        self.constant.set_span(span);
        self.body.set_span(span);
    }
}

// represents the default case of a Yul switch stmt.
#[derive(Clone)]
struct SwitchDefault {
    default_token: kw::default,
    body: YulBlock,
}

impl Parse for SwitchDefault {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            default_token: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl fmt::Debug for SwitchDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SwitchDefault")
            .field("default_token", &self.default_token)
            .field("body", &self.body)
            .finish()
    }
}

impl Spanned for SwitchDefault {
    fn span(&self) -> Span {
        let span = self.default_token.span();
        span.join(self.body.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.default_token.set_span(span);
        self.body.set_span(span);
    }
}
