use syn::parse::Parse;

use crate::{kw, Block};

#[derive(Debug, Clone)]
pub struct Unchecked {
    unchecked: kw::unchecked,
    block: Block,
}

impl Parse for Unchecked {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            unchecked: input.parse()?,
            block: input.parse()?,
        })
    }
}
