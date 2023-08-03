use crate::{binop::BinopExpr, expr::Stmt, Block};
use syn::{parenthesized, parse::Parse, token::Paren, Token};

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub for_token: Token![for],
    pub for_assign: ForAssignment,
}

#[derive(Debug, Clone)]
pub struct ForAssignment {
    pub brace: Paren,
    pub iter_asign: BinopExpr,
    pub semi: Token![;],
    pub cond: Box<Stmt>,
    pub semi2: Token![;],
    pub up_cond: Option<Box<Stmt>>,
    pub block: Block,
}

impl Parse for ForStmt {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            for_assign: input.parse()?,
        })
    }
}

impl Parse for ForAssignment {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        Ok(Self {
            brace: parenthesized!(content in input),
            iter_asign: input.parse()?,
            semi: input.parse()?,
            cond: Box::new(input.parse()?),
            semi2: input.parse()?,
            up_cond: input.parse().ok(),
            block: input.parse()?,
        })
    }
}
