use syn::{parenthesized, parse::Parse, punctuated::Punctuated, token::Paren};

use crate::{expr::Expr, Parameters};

#[derive(Debug, Clone)]
pub struct TupleExpr {
    paran: Paren,
    exprs: Punctuated<Expr, Token![,]>,
}

impl Parse for TupleExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let paren = parenthesized!(content in input);

        let exprs = Punctuated::new();
        exprs.push(input.parse()?);
        while input.peek(Token![,]) {
            exprs.push(input.parse()?);
        }
        exprs.push(input.parse()?);

        Ok(Self { exprs, paran })
    }
}
