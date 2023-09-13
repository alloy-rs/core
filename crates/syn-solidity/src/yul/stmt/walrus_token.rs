use std::fmt;

use crate::Spanned;

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// Represents the walrus operator `:=`.
#[derive(Clone)]
pub struct WalrusToken {
    pub colon: Token![:],
    pub equals: Token![=],
}

impl Parse for WalrusToken {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let colon = input.parse()?;
        let equals = input.parse()?;

        Ok(Self { colon, equals })
    }
}

impl Spanned for WalrusToken {
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

impl fmt::Debug for WalrusToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalrusToken")
            .field("colon", &self.colon)
            .field("equals", &self.equals)
            .finish()
    }
}
