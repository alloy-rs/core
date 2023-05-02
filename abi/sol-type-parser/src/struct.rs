use crate::r#type::SolType;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use std::fmt;
use syn::{
    braced,
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Struct},
    Attribute, Ident, Index, Token,
};

#[derive(Debug, Clone)]
pub struct SolStructField {
    ty: SolType,
    name: Ident,
}

impl fmt::Display for SolStructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.ty, self.name)
    }
}

impl Parse for SolStructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
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
    attrs: Vec<Attribute>,
    _struct: Struct,
    name: Ident,
    _brace: Brace,
    fields: Punctuated<SolStructField, Token![;]>,
}

impl Parse for SolStructDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            _struct: input.parse()?,
            name: input.parse()?,
            _brace: braced!(content in input),
            fields: content.parse_terminated(SolStructField::parse, Token![;])?,
        })
    }
}

impl SolStructDef {
    fn expand_from(&self) -> TokenStream {
        let name = &self.name;
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
            type UnderlyingSolTuple = (#(#field_ty,)*);
            type UnderlyingRustTuple = (#(<#field_ty2 as ::ethers_abi_enc::SolType>::RustType,)*);

            impl From<#name> for UnderlyingRustTuple {
                fn from(value: #name) -> UnderlyingRustTuple {
                    (#(value.#field_names,)*)
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
        debug_assert!(!self.fields.is_empty());
        let name = &self.name;

        let doc = format!("Represents the `{name}` Solidity struct type.");

        let fields = self.fields.iter();

        let (f_ty, f_name): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .map(|f| (f.ty.to_string(), f.name.to_string()))
            .unzip();

        let props_tys: Vec<_> = self.fields.iter().map(|f| f.ty.clone()).collect();
        let props = self.fields.iter().map(|f| &f.name);

        let convert = self.expand_from();

        let fields_string = self
            .fields
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let encoded_type = format!("{name}({fields_string})");
        let encode_type_impl = if self.fields.iter().any(|f| f.ty.is_non_primitive()) {
            quote! {
                {
                    let mut encoded = String::from(#encoded_type);
                    #(
                        if let Some(s) = <#props_tys as SolType>::eip712_encode_type() {
                            encoded.push_str(&s);
                        }
                    )*
                    encoded
                }
            }
        } else {
            quote!(#encoded_type)
        };

        let encode_data_impl = if self.fields.len() == 1 {
            let SolStructField { ty, name } = self.fields.first().unwrap();
            quote!(<#ty as SolType>::eip712_data_word(&self.#name).0.to_vec())
        } else {
            quote! {
                [#(
                    <#props_tys as SolType>::eip712_data_word(&self.#props).0,
                )*].concat()
            }
        };
        let attrs = self.attrs.iter();

        quote! {
            #[doc = #doc]
            #(#attrs)*
            #[allow(non_snake_case)]
            #[derive(Debug, Clone, PartialEq)]
            pub struct #name {
                #(pub #fields),*
            }

            #[allow(non_snake_case)]
            const _: () = {
                use ::ethers_abi_enc::{SolType, no_std_prelude::*};

                #convert

                impl ::ethers_abi_enc::SolStruct for #name {
                    type Tuple = UnderlyingSolTuple;

                    const NAME: &'static str = stringify!(#name);

                    const FIELDS: &'static [(&'static str, &'static str)] = &[
                        #((#f_ty, #f_name)),*
                    ];

                    fn to_rust(&self) -> UnderlyingRustTuple {
                        self.clone().into()
                    }

                    fn from_rust(tuple: UnderlyingRustTuple) -> Self {
                        tuple.into()
                    }

                    fn encode_type() -> Cow<'static, str> {
                        #encode_type_impl.into()
                    }

                    fn encode_data(&self) -> Vec<u8> {
                        #encode_data_impl
                    }
                }
            };
        }
    }
}

impl ToTokens for SolStructDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.fields.is_empty() {
            tokens.extend(quote_spanned! { self.name.span() =>
                compile_error!("Defining empty structs is disallowed.");
            });
        } else {
            tokens.extend(self.expand_impl());
        }
    }
}
