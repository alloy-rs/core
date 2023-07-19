use syn::{
    parse::Parse,
    token::{Brace, Paren},
    Token,
};

use crate::{expr::Expr, Block};

#[derive(Debug, Clone)]
pub struct DoWhile {
    pub do_token: Token![do],
    pub brace: Brace,
    pub block: Block,
    pub while_token: Token![while],
    pub paren: Paren,
    pub expr: Box<Expr>,
}

impl Parse for DoWhile {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let do_token = input.parse()?;
        let brace = input.parse()?;
        let block = input.parse()?;
        let while_token = input.parse()?;
        let paren = input.parse()?;
        let expr = Box::new(input.parse()?);

        Ok(Self {
            do_token,
            brace,
            block,
            while_token,
            paren,
            expr,
        })
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub while_token: Token![while],
    pub paren: Paren,
    pub expr: Box<Expr>,
    pub block: Block,
}

impl Parse for While {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let while_token = input.parse()?;
        let paren = input.parse()?;
        let expr = input.parse()?;
        let block = input.parse()?;

        Ok(Self {
            while_token,
            paren,
            expr,
            block,
        })
    }
}
