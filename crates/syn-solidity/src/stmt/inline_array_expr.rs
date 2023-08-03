use crate::expr::Stmt;
use syn::{bracketed, parse::Parse, punctuated::Punctuated, token::Bracket, Token};

#[derive(Debug, Clone)]
pub struct InlineArrayExpr {
    bracket: Bracket,
    exprs: Punctuated<Stmt, Token![,]>,
}

impl Parse for InlineArrayExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let bracket = bracketed!(content in input);

        let mut exprs = Punctuated::new();
        exprs.push(input.parse()?);
        while input.peek(Token![,]) {
            exprs.push(input.parse()?);
        }
        exprs.push(input.parse()?);

        Ok(Self { exprs, bracket })
    }
}
