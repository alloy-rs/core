use syn::{parse::Parse, punctuated::Punctuated, Ident, Token};

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum CallArgs {
    Map(MapArgs),
    List(ListArgs),
}

#[derive(Debug, Clone)]
pub struct MapArgs(Punctuated<Map, Token!(,)>);

#[derive(Debug, Clone)]
pub struct Map {
    pub key: Ident,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ListArgs(Punctuated<Expr, Token![,]>);

impl Parse for CallArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {}
}
impl Parse for MapArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {}
}
impl Parse for Map {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {}
}
impl Parse for ListArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {}
}
