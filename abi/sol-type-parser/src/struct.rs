use crate::r#type::SolDataType;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use std::fmt::{self, Write};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Attribute, Ident, Index, Result, Token,
};

#[derive(Debug, Clone)]
pub struct SolStructField {
    ty: SolDataType,
    name: Ident,
}

impl fmt::Display for SolStructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.ty, self.name)
    }
}

impl Parse for SolStructField {
    fn parse(input: ParseStream) -> Result<Self> {
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
    _struct_token: Token![struct],
    name: Ident,
    _brace_token: Brace,
    fields: Punctuated<SolStructField, Token![;]>,
}

impl Parse for SolStructDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _struct_token: input.parse()?,
            name: input.parse()?,
            _brace_token: braced!(content in input),
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

    fn expand_impl(&self, attrs: &[Attribute]) -> TokenStream {
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

        let encoded_type = self.signature();
        let encode_type_impl = if self.fields.iter().any(|f| f.ty.is_non_primitive()) {
            quote! {
                {
                    let mut encoded = String::from(#encoded_type);
                    #(
                        if let Some(s) = <#props_tys as ::ethers_abi_enc::SolDataType>::eip712_encode_type() {
                            encoded.push_str(&s);
                        }
                    )*
                    encoded
                }
            }
        } else {
            quote!(#encoded_type)
        };

        let encode_data_impl = match self.fields.len() {
            0 => quote! { vec![] },
            1 => {
                let SolStructField { ty, name } = self.fields.first().unwrap();
                quote!(<#ty as ::ethers_abi_enc::SolDataType>::eip712_data_word(&self.#name).0.to_vec())
            }
            _ => quote! {
                [#(
                    <#props_tys as ::ethers_abi_enc::SolDataType>::eip712_data_word(&self.#props).0,
                )*].concat()
            },
        };

        let attrs = attrs.iter();
        let convert = self.expand_from();
        quote! {
            #[doc = #doc]
            #(#attrs)*
            #[allow(non_snake_case)]
            #[derive(Debug, Clone, PartialEq)] // TODO: Derive traits dynamically
            pub struct #name {
                #(pub #fields),*
            }

            #[allow(non_snake_case)]
            const _: () = {
                use ::ethers_abi_enc::no_std_prelude::*;

                #convert

                impl ::ethers_abi_enc::SolStruct for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_abi_enc::SolType>::TokenType;

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

    fn signature(&self) -> String {
        let mut out = self.name.to_string();
        out.reserve(2 + self.fields.len() * 32);
        out.push('(');
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            write!(out, "{field}").unwrap();
        }
        out.push(')');
        out
    }

    pub fn to_tokens(&self, tokens: &mut TokenStream, attrs: &[Attribute]) {
        if self.fields.is_empty() {
            tokens.extend(quote_spanned! {self.name.span()=>
                compile_error!("Defining empty structs is disallowed.");
            });
        }
        tokens.extend(self.expand_impl(attrs))
    }
}
