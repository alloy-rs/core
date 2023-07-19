use crate::binops::Binop;
use syn::parse::Parse;

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub struct BinopExpr {
    pub left: Box<Expr>,
    pub op: Binop,
    pub right: Box<Expr>,
}

impl Parse for BinopExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            left: Box::new(input.parse()?),
            op: input.parse()?,
            right: Box::new(input.parse()?),
        })
    }
}
