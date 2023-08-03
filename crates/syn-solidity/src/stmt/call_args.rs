use syn::{
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Paren},
    Error, Ident, Token,
};

use crate::expr::Stmt;

#[derive(Debug, Clone)]
pub enum CallArgs {
    Map(MapArgs),
    List(ListArgs),
}

#[derive(Debug, Clone)]
pub struct ListArgs(pub Punctuated<Stmt, Token![,]>);

#[derive(Debug, Clone)]
pub struct MapArgs(pub Punctuated<Map, Token![,]>);

#[derive(Debug, Clone)]
pub struct Map {
    pub key: Ident,
    pub semi: Token![:],
    pub value: Box<Stmt>,
}

impl Parse for CallArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek2(Brace) {
            Ok(Self::Map(input.parse()?))
        } else if input.peek(Paren) {
            Ok(Self::List(input.parse()?))
        } else {
            Err(Error::new(input.span(), "invalid call args"))
        }
    }
}
impl Parse for MapArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut map = Punctuated::new();
        while input.peek2(Token![:]) {
            let entry = input.parse()?;
            map.push(entry);
        }

        Ok(Self(map))
    }
}

impl Parse for Map {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            semi: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for ListArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut list = Punctuated::new();
        while input.peek(Token![,]) {
            list.push(input.parse()?);
        }
        list.push(input.parse()?);
        Ok(Self(list))
    }
}
