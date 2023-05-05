use crate::common::{kw, VariableDeclaration};
use proc_macro2::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Attribute, Ident, Result, Token,
};

pub struct Error {
    _error_token: kw::error,
    name: Ident,
    _paren_token: Paren,
    fields: Punctuated<VariableDeclaration, Token![,]>,
    _semi_token: Token![;],
}

impl Parse for Error {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _error_token: input.parse()?,
            name: input.parse()?,
            _paren_token: parenthesized!(content in input),
            fields: content.parse_terminated(VariableDeclaration::parse, Token![,])?,
            _semi_token: input.parse()?,
        })
    }
}

impl Error {
    pub fn to_tokens(&self, _tokens: &mut TokenStream, _attrs: &[Attribute]) {
        let _ = &self.name;
        let _ = &self.fields;
        todo!()
    }
}
