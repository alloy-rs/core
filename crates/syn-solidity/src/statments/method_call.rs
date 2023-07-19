use proc_macro2::Ident;

use crate::{Parameters, VariableDeclaration};
use syn::{parse::Parse, token::Paren, Token};

pub struct MethodCall {
    pub fn_name: Ident,
    pub paren_token: Paren,
    pub arguments: Parameters<Token![,]>,
}

impl Parse for MethodCall {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let fn_name = input.parse()?;
        let paren = input.parse()?;
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
