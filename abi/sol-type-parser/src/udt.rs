use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::r#type::SolType;

mod kw {
    syn::custom_keyword!(is);
}

pub struct Udt {
    _type: syn::Token![type],
    name: syn::Ident,
    _is: kw::is,
    ty: SolType,
    _semi: syn::Token![;],
}

impl Parse for Udt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Udt {
            _type: input.parse()?,
            name: input.parse()?,
            _is: input.parse()?,
            ty: input.parse()?,
            _semi: input.parse()?,
        })
    }
}

impl ToTokens for Udt {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let mod_name = syn::Ident::new(&format!("__{}", name), name.span());
        let ty = &self.ty;
        tokens.extend(quote! {
            pub use #mod_name::#name;
            #[allow(non_snake_case)]
            mod #mod_name {
                use ::ethers_abi_enc::define_udt;
                define_udt! {
                    /// A solidity user-defined type
                    #name,
                    underlying: #ty,
                }
            }
        });
    }
}
