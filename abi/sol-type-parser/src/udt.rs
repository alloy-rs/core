use crate::{
    common::{kw, SolIdent},
    r#type::Type,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

pub struct Udt {
    _type_token: Token![type],
    pub name: SolIdent,
    _is: kw::is,
    pub ty: Type,
    _semi_token: Token![;],
}

impl fmt::Debug for Udt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Udt")
            .field("name", &self.name)
            .field("ty", &self.ty)
            .finish()
    }
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
