use crate::expr::Expr;
use syn::{parse::Parse, token::Eq};

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub left: Box<Expr>,
    pub eq: Eq,
    pub assign: Box<Expr>,
}

impl Parse for AssignmentExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let left = input.parse()?;
        let eq: Eq = input.parse()?;
        let assign: Box<Expr> = Box::new(input.parse()?);

        Ok(Self { left, assign, eq })
    }
}
