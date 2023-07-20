use syn::{
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Paren},
    Error, Ident, Token,
};

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum CallArgs {
    Map(MapArgs),
    List(ListArgs),
}

#[derive(Debug, Clone)]
pub struct MapArgs(Punctuated<Map, Token![,]>);

#[derive(Debug, Clone)]
pub struct Map {
    pub key: Ident,
    pub semi: Token![:],
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ListArgs(Punctuated<Expr, Token![,]>);

impl Parse for CallArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        // map
        if input.peek2(Brace) {
            Ok(Self::Map(input.parse()?))
        }
        // list
        else if input.peek(Paren) {
            Ok(Self::List(input.parse()?))
        } else {
            Err(Error::new(input.span(), "invalid call args"))
        }
    }
}
impl Parse for MapArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        while input.peek(token)

    }
}
impl Parse for Map {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {}
}
impl Parse for ListArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {}
}
