use crate::{Item, Spanned};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result,
};

/// A Solidity file. The root of the AST.
#[derive(Clone, Debug)]
pub struct File {
    /// The inner attributes of the file.
    pub attrs: Vec<Attribute>,
    /// The items in the file.
    pub items: Vec<Item>,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                f.write_str("\n\n")?;
            }
            item.fmt(f)?;
        }
        Ok(())
    }
}

impl Parse for File {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_inner)?;
        let mut items = Vec::new();
        let mut first = true;
        while first || !input.is_empty() {
            first = false;
            items.push(input.parse()?);
        }
        Ok(Self { attrs, items })
    }
}

impl Spanned for File {
    fn span(&self) -> Span {
        self.items.span()
    }

    fn set_span(&mut self, span: Span) {
        self.items.set_span(span);
    }
}
