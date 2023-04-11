use proc_macro2::TokenStream;
use syn::{braced, parse::Parse, punctuated::Punctuated, token::Struct, Ident, Index, Token};

use quote::{quote, ToTokens};

use crate::r#type::SolType;

#[derive(Debug, Clone)]
pub struct SolStructField {
    ty: SolType,
    name: Ident,
}

impl Parse for SolStructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(SolStructField {
            ty: input.parse()?,
            name: input.parse()?,
        })
    }
}

impl ToTokens for SolStructField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let ty = &self.ty;
        tokens.extend(quote! {
            #name: <#ty as ::ethers_abi_enc::SolType>::RustType
        });
    }
}

pub struct SolStructDef {
    _struct: Struct,
    name: Ident,
    _brace: syn::token::Brace,
    fields: Punctuated<SolStructField, Token![;]>,
}

impl Parse for SolStructDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        Ok(SolStructDef {
            _struct: input.parse()?,
            name: input.parse()?,
            _brace: braced!(content in input),
            fields: content.parse_terminated(SolStructField::parse, Token![;])?,
        })
    }
}

impl SolStructDef {
    fn name(&self) -> Ident {
        self.name.clone()
    }

    fn expand_from(&self) -> TokenStream {
        let name = self.name();
        let field_names = self.fields.iter().map(|f| &f.name);

        let field_ty = self.fields.iter().map(|f| &f.ty);
        let field_ty2 = field_ty.clone();

        let (f_no, f_name): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| (Index::from(i), &f.name))
            .unzip();

        quote! {
            type UnderlyingSolTuple = (#(#field_ty),*);
            type UnderlyingRustTuple = (#(<#field_ty2 as ::ethers_abi_enc::SolType>::RustType),*);

            impl From<#name> for UnderlyingRustTuple {
                fn from(value: #name) -> UnderlyingRustTuple {
                    (#(value.#field_names),*)
                }
            }

            impl From<UnderlyingRustTuple> for #name {
                fn from(tuple: UnderlyingRustTuple) -> Self {
                    #name {
                        #(#f_name: tuple.#f_no),*
                    }
                }
            }
        }
    }

    fn expand_impl(&self) -> TokenStream {
        let name = self.name();

        let doc = format!(
            "A Rust type containg info for the Solidity type {}",
            &self.name
        );

        let fields = self.fields.iter();

        let (f_ty, f_name): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .map(|f| (f.ty.to_string(), f.name.to_string()))
            .unzip();

        let props_tys: Vec<_> = self.fields.iter().map(|f| f.ty.clone()).collect();

        let convert = self.expand_from();
        let mod_name = Ident::new(&format!("__{}", name), name.span());

        quote! {
            pub use #mod_name::#name;
            #[allow(non_snake_case)]
            mod #mod_name {
                extern crate alloc;
                use super::*;

                use ::ethers_abi_enc::{SolType, no_std_prelude::*};

                #[doc = #doc]
                #[derive(Debug, Clone, PartialEq)]
                pub struct #name {
                    #(pub #fields),*
                }

                #convert

                impl ::ethers_abi_enc::SolStruct for #name {
                    type Tuple = UnderlyingSolTuple;

                    const NAME: &'static str = stringify!(#name);

                    const FIELDS: &'static [(&'static str, &'static str)] = &[
                        #((#f_ty, #f_name)),*
                    ];

                    fn to_tuple(&self) -> UnderlyingRustTuple {
                        self.clone().into()
                    }

                    fn from_tuple(tuple: UnderlyingRustTuple) -> Self {
                        tuple.into()
                    }

                    fn encode_type() -> String {
                        let mut types: Vec<String> = Default::default();
                        let mut tails: Vec<String> = Default::default();
                        #(
                            {
                                type Prop = #props_tys;
                                types.push(Prop::sol_type_name());
                                if let Some(tail) = Prop::struct_type() {
                                    tails.push(tail);
                                }
                            }
                        )*
                        format!(
                            "{}({}){}", Self::NAME, types.join(","),
                            tails.join("")
                        )
                    }

                    fn encode_data(&self) -> Vec<u8> {
                        todo!()
                    }
            }}
        }
    }
}

impl ToTokens for SolStructDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.expand_impl());
    }
}
