use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use syn::{parse::Parse, parse_macro_input, token::Struct};

use quote::{quote, ToTokens};

mod r#type;
use r#type::*;

mod r#struct;
use r#struct::*;

enum SolInput {
    SolType(SolType),
    SolStructDef(SolStructDef),
}

impl Parse for SolInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Struct) {
            Ok(SolInput::SolStructDef(input.parse()?))
        } else {
            Ok(SolInput::SolType(input.parse()?))
        }
    }
}

impl ToTokens for SolInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // todo!()
        match self {
            SolInput::SolType(ty) => ty.to_tokens(tokens),
            SolInput::SolStructDef(def) => def.to_tokens(tokens),
        }
    }
}

#[proc_macro]
pub fn sol(input: TS) -> TS {
    let s: SolInput = parse_macro_input!(input);
    quote! {
        #s
    }
    .into()
}
