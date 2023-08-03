use crate::expr::Expr;
use syn::{parse::Parse, Token};

#[derive(Debug, Clone)]
pub struct Ternary {
    pub var1: Box<Expr>,
    pub q: Token![?],
    pub res_0: Box<Expr>,
    pub semi: Token![;],
    pub res_1: Box<Expr>,
}

impl Parse for Ternary {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let var1 = Box::new(input.parse()?);
        let q = input.parse()?;
        let res_0 = Box::new(input.parse()?);
        let semi = input.parse()?;
        let res_1 = Box::new(input.parse()?);

        Ok(Self {
            res_1,
            semi,
            res_0,
            var1,
            q,
        })
    }
}
