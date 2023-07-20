use syn::{parenthesized, parse::Parse, punctuated::Punctuated, token::Paren, Token};

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub struct TupleExpr {
    paren: Paren,
    exprs: Punctuated<Expr, Token![,]>,
}

impl Parse for TupleExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let paren = parenthesized!(content in input);

        let mut exprs = Punctuated::new();
        exprs.push(input.parse()?);
        while input.peek(Token![,]) {
            exprs.push(input.parse()?);
        }
        exprs.push(input.parse()?);

        Ok(Self { exprs, paren })
    }
}
