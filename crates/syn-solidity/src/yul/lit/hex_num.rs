use std::fmt;

use proc_macro2::{Ident, Punct, Span};
use syn::{
    parse::{Parse, ParseStream},
    LitInt, Result,
};

use crate::Spanned;

#[derive(Clone)]
pub struct YulHexNum {
    pub prefix_token: ZeroExPrefix,
    pub value: Ident,
}

impl Parse for YulHexNum {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            prefix_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Spanned for YulHexNum {
    fn span(&self) -> Span {
        let span = self.prefix_token.span();
        span.join(self.value.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.prefix_token.set_span(span);
        self.value.set_span(span);
    }
}

impl fmt::Debug for YulHexNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulHexNum")
            .field("prefix_token", &self.prefix_token)
            .field("value", &self.value)
            .finish()
    }
}

// Represents the `0x` prefix
#[derive(Clone)]
pub struct ZeroExPrefix {
    zero_token: LitInt,
    x_token: Punct,
}

impl Parse for ZeroExPrefix {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        // capture the 0 prefix (and sanitize)
        let zero_token: LitInt = input.parse()?;
        if zero_token.base10_parse::<u8>()? != 0 {
            return Err(input.error("expected `0x` prefix"))
        }

        let x_token: Punct = input.parse()?;
        if x_token.as_char() != 'x' {
            return Err(input.error("expected `0x` prefix"))
        }

        Ok(Self {
            zero_token,
            x_token,
        })
    }
}

impl Spanned for ZeroExPrefix {
    fn span(&self) -> Span {
        let span = self.zero_token.span();
        span.join(self.x_token.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.zero_token.set_span(span);
        self.x_token.set_span(span);
    }
}

impl fmt::Debug for ZeroExPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZeroExPrefix")
            .field("zero_token", &self.zero_token)
            .field("x_token", &self.x_token)
            .finish()
    }
}
