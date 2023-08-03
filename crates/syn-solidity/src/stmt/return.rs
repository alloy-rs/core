use crate::expr::Stmt;
use syn::{parse::Parse, Token};

#[derive(Debug, Clone)]
pub struct Return {
    token: Token![return],
    expr: Box<Stmt>,
    semi: Token![;],
}

impl Parse for Return {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            token: input.parse()?,
            expr: Box::new(input.parse()?),
            semi: input.parse()?,
        })
    }
}
