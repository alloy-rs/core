use proc_macro2::TokenStream;
use syn::{braced, parse::Parse, token::Brace};

use crate::kw;

#[derive(Debug, Clone)]
pub struct Assembly {
    kw: kw::assembly,
    // stuff here
    brace: Brace,
    input: TokenStream,
}

impl Parse for Assembly {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let kw = input.parse()?;
        let brace = braced!(content in input);
        let input = input.parse()?;

        Ok(Self { input, kw, brace })
    }
}
