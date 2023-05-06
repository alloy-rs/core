use crate::r#type::SolDataType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

mod kw {
    syn::custom_keyword!(is);
}

pub struct Udt {
    _type_token: Token![type],
    name: syn::Ident,
    _is: kw::is,
    ty: SolDataType,
    _semi_token: Token![;],
}

impl Parse for Udt {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Udt {
            _type_token: input.parse()?,
            name: input.parse()?,
            _is: input.parse()?,
            ty: input.parse()?,
            _semi_token: input.parse()?,
        })
    }
}

impl Udt {
    pub fn to_tokens(&self, tokens: &mut TokenStream, attrs: &[Attribute]) {
        let Self { name, ty, .. } = self;
        tokens.extend(quote! {
            ::ethers_abi_enc::define_udt! {
                #(#attrs)*
                #name,
                underlying: #ty,
            }
        });
    }
}
