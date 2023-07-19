use crate::expr::Expr;
use proc_macro2::Ident;
use syn::{parse::Parse, Token};

#[derive(Debug, Clone)]
pub struct Field {
    pub base: Box<Expr>,
    pub dot: Token![.],
    pub name: Ident,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            base: input.parse()?,
            dot: input.parse()?,
            name: input.parse()?,
        })
    }
}
