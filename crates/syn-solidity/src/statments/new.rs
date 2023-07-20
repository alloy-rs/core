use crate::{kw, r#type::Type};
use syn::parse::Parse;

#[derive(Debug, Clone)]
pub struct New {
    new_kw: kw::new,
    type_name: Type,
}

impl Parse for New {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            new_kw: input.parse()?,
            type_name: input.parse()?,
        })
    }
}
