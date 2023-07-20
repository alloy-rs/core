use proc_macro2::Ident;

use crate::{Parameters, VariableDeclaration};
use syn::{parenthesized, parse::Parse, token::Paren, Token};

#[derive(Debug, Clone)]
pub struct MethodCall {
    pub fn_name: Ident,
    pub paren_token: Paren,
    pub arguments: Parameters<Token![,]>,
}

impl Parse for MethodCall {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let fn_name = input.parse()?;
        let paren = parenthesized!(content in input);
        let mut args = Parameters::new();
        while let Ok(arg) = input.parse::<VariableDeclaration>() {
            args.push(arg);
        }

        Ok(Self {
            fn_name,
            paren_token: paren,
            arguments: args,
        })
    }
}
