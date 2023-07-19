use crate::expr::Expr;
use syn::{parse::Parse, token::Eq, Ident};

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub ident: Ident,
    pub eq: Eq,
    pub assign: Box<Expr>,
}

impl Parse for AssignmentExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let eq: Eq = input.parse()?;
        let assign: Box<Expr> = Box::new(input.parse()?);

        Ok(Self { assign, ident, eq })
    }
}
