use syn::{parse::Parse, Token};

/// def not right way todo this but we ballin
#[derive(Debug, Clone)]
pub struct PowOps {
    pub star0: Token!(*),
    pub star1: Token!(*),
}

impl Parse for PowOps {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            star0: input.parse()?,
            star1: input.parse()?,
        })
    }
}
