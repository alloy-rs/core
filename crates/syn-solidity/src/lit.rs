use std::fmt;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

use crate::kw;

/// A string literal.
#[derive(Clone)]
pub struct LitStr {
    pub unicode_token: Option<kw::unicode>,
    pub values: Vec<syn::LitStr>,
}

impl fmt::Debug for LitStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LitStr")
            .field("unicode", &self.unicode_token.is_some())
            .field("values", &self.values)
            .finish()
    }
}

impl fmt::Display for LitStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for value in &self.values {
            f.write_str(&value.value())?;
        }
        Ok(())
    }
}

impl Parse for LitStr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            unicode_token: input.parse()?,
            values: {
                let mut values = Vec::new();
                while !input.peek(syn::LitStr) {
                    values.push(input.parse()?);
                }
                if values.is_empty() {
                    return Err(input.parse::<syn::LitStr>().unwrap_err())
                }
                values
            },
        })
    }
}

impl LitStr {
    pub fn span(&self) -> Span {
        let mut span = if let Some(kw) = &self.unicode_token {
            kw.span
        } else {
            self.values.first().unwrap().span()
        };
        for value in &self.values {
            span = span.join(value.span()).unwrap_or(span);
        }
        span
    }

    pub fn set_span(&mut self, span: Span) {
        if let Some(kw) = &mut self.unicode_token {
            kw.span = span;
        }
        for value in &mut self.values {
            value.set_span(span);
        }
    }
}
