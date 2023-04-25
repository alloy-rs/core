use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use syn::{
    parse::{discouraged::Speculative, Parse},
    parse_macro_input,
};

use quote::{quote, ToTokens};

mod r#type;
use r#type::*;

mod r#struct;
use r#struct::*;

mod udt;
use udt::*;

enum SolInput {
    Type(SolType),
    StructDef(SolStructDef),
    ValueTypeDef(Udt),
}

impl Parse for SolInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(sol_ty) = fork.parse::<SolType>() {
            input.advance_to(&fork);
            return Ok(SolInput::Type(sol_ty));
        }

        let fork = input.fork();
        if let Ok(udt) = fork.parse::<Udt>() {
            input.advance_to(&fork);
            return Ok(SolInput::ValueTypeDef(udt));
        }

        Ok(SolInput::StructDef(input.parse()?))
    }
}

impl ToTokens for SolInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            SolInput::Type(ty) => ty.to_tokens(tokens),
            SolInput::StructDef(def) => def.to_tokens(tokens),
            SolInput::ValueTypeDef(def) => def.to_tokens(tokens),
        }
    }
}

#[proc_macro]
pub fn sol(input: TS) -> TS {
    let s: SolInput = parse_macro_input!(input);
    quote!(#s).into()
}
