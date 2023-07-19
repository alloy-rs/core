use proc_macro2::Ident;
use syn::{parse::Parse, token::Bracket};

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub struct Index {
    pub name: Ident,
    pub bracket: Bracket,
    pub index_by: Box<Expr>,
}

impl Parse for Index {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            bracket: input.parse()?,
            index_by: input.parse()?,
        })
    }
}
