use crate::{Item, Spanned};
use proc_macro2::Span;
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
