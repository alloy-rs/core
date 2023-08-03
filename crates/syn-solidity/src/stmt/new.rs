use crate::{kw, Type};
use syn::parse::Parse;

#[derive(Debug, Clone)]
pub struct New {
    new_token: kw::new,
    ty: Type,
}

impl Parse for New {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            new_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}
