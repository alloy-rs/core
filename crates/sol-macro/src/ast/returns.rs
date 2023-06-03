use crate::ast::{kw, Parameters};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Error, Result, Token,
};

/// The `returns` attribute of a function.
#[derive(Clone, PartialEq, Eq)]
pub struct Returns {
    pub returns_token: kw::returns,
    pub paren_token: Paren,
    pub returns: Parameters<Token![,]>,
}

impl fmt::Debug for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Returns").field(&self.returns).finish()
    }
}

impl fmt::Display for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("returns (")?;
        for (i, r) in self.returns.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{r}")?;
        }
        f.write_str(")")
    }
}

impl Parse for Returns {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let this = Self {
            returns_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            returns: content.parse()?,
        };
        if this.returns.is_empty() {
            Err(Error::new(
                this.paren_token.span.join(),
                "expected at least one return type",
            ))
        } else {
            Ok(this)
        }
    }
}

impl Returns {
    pub fn span(&self) -> Span {
        let span = self.returns_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.returns_token.span = span;
        self.paren_token = Paren(span);
    }
}
