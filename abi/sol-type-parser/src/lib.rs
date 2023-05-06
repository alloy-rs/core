use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Result, Token,
};

use quote::{quote, quote_spanned, ToTokens};

mod r#type;
use r#type::SolDataType;

mod r#struct;
use r#struct::SolStructDef;

mod udt;
use udt::Udt;

struct SolInput {
    attrs: Vec<Attribute>,
    kind: SolInputKind,
}

impl Parse for SolInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            kind: input.parse()?,
        })
    }
}

impl ToTokens for SolInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self.kind {
            SolInputKind::Type(ty) => {
                if !self.attrs.is_empty() {
                    tokens.extend(quote_spanned! {ty.span()=>
                        compile_error!("attributes are not allowed on type aliases")
                    });
                }
                ty.to_tokens(tokens)
            }
            SolInputKind::StructDef(def) => def.to_tokens(tokens, &self.attrs),
            SolInputKind::ValueTypeDef(def) => def.to_tokens(tokens, &self.attrs),
        }
    }
}

enum SolInputKind {
    Type(SolDataType),
    StructDef(SolStructDef),
    ValueTypeDef(Udt),
}

impl Parse for SolInputKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let this = if input.peek(Token![type]) {
            Self::ValueTypeDef(input.parse()?)
        } else if input.peek(Token![struct]) {
            Self::StructDef(input.parse()?)
        } else {
            Self::Type(input.parse()?)
        };
        Ok(this)
    }
}

#[proc_macro]
pub fn sol(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as SolInput);
    quote!(#s).into()
}
