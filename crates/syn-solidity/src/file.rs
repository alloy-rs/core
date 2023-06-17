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
            Err(input.parse::<Item>().unwrap_err())
        } else {
            Ok(Self { items })
        }
    }
}
