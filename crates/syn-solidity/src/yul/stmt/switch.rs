use crate::{kw, Lit, Spanned, YulBlock, YulExpr};
use proc_macro2::Span;
use std::fmt;
use syn::parse::{Parse, ParseStream, Result};

/// A Yul switch statement can consist of only a default-case or one
/// or more non-default cases optionally followed by a default-case.
///
/// Example switch statement in Yul:
///
/// ```solidity
/// switch exponent
/// case 0 { result := 1 }
/// case 1 { result := base }
/// default { revert(0, 0) }
/// ```
///
/// Solidity Reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.yulSwitchStatement>
#[derive(Clone)]
pub struct YulSwitch {
    pub switch_token: kw::switch,
    pub selector: YulExpr,
    pub branches: Vec<YulCaseBranch>,
    pub default_case: Option<YulSwitchDefault>,
}

impl Parse for YulSwitch {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let switch_token = input.parse()?;
        let selector = input.parse()?;
        let branches = {
            let mut branches = vec![];
            while input.peek(kw::case) {
                branches.push(input.parse()?);
            }
            branches
        };
        let default_case = {
            if input.peek(kw::default) {
                Some(input.parse()?)
            } else {
                None
            }
        };

        if branches.is_empty() && default_case.is_none() {
            return Err(input.error("Must have at least one case or a default case."))
        }

        Ok(Self {
            switch_token,
            selector,
            branches,
            default_case,
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

impl fmt::Debug for YulSwitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulSwitch")
            .field("selector", &self.selector)
            .field("branches", &self.branches)
            .field("default_case", &self.default_case)
            .finish()
    }
}

/// Represents a non-default case of a Yul switch statement.
#[derive(Clone)]
pub struct YulCaseBranch {
    pub case_token: kw::case,
    pub constant: Lit,
    pub body: YulBlock,
}

impl Parse for YulCaseBranch {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            case_token: input.parse()?,
            constant: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl Spanned for YulCaseBranch {
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

impl fmt::Debug for YulCaseBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulCaseBranch")
            .field("constant", &self.constant)
            .field("body", &self.body)
            .finish()
    }
}

/// Represents the default case of a Yul switch statement.
#[derive(Clone)]
pub struct YulSwitchDefault {
    pub default_token: kw::default,
    pub body: YulBlock,
}

impl Parse for YulSwitchDefault {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            default_token: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl Spanned for YulSwitchDefault {
    fn span(&self) -> Span {
        let span = self.default_token.span();
        span.join(self.body.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.default_token.set_span(span);
        self.body.set_span(span);
    }
}

impl fmt::Debug for YulSwitchDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SwitchDefault")
            .field("body", &self.body)
            .finish()
    }
}
