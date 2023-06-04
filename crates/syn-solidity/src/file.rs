use super::Item;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

/// A Solidity file. The root of the AST.
#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

impl Parse for File {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        if items.is_empty() {
            let message = "\
                expected at least one of: \
                `type`, `struct`, `function`, `error`, Solidity type";
            Err(input.error(message))
        } else {
            Ok(Self { items })
        }
    }
}
