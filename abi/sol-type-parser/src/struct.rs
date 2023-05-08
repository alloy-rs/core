use crate::{
    common::{from_into_tuples, Parameters, SolIdent, VariableDeclaration},
    r#type::Type,
};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use std::fmt;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Attribute, Result, Token,
};

pub struct Struct {
    _struct_token: Token![struct],
    pub name: SolIdent,
    _brace_token: Brace,
    pub fields: Parameters<Token![;]>,
}

impl fmt::Debug for Struct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Struct")
            .field("name", &self.name)
            .field("fields", &self.fields)
            .finish()
    }
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _struct_token: input.parse()?,
            name: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse()?,
        })
    }
}

impl Struct {
    fn expand_impl(&self, attrs: &[Attribute]) -> TokenStream {
        let name = &self.name;

        let doc = format!("Represents the `{name}` Solidity struct type.");

        let fields = self.fields.iter();

        let (f_ty, f_name): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .map(|f| (f.ty.to_string(), f.name.as_ref().unwrap().to_string()))
            .unzip();

        let props_tys: Vec<_> = self.fields.iter().map(|f| f.ty.clone()).collect();
        let props = self.fields.iter().map(|f| &f.name);

        let encoded_type = self.fields.eip712_signature(self.name.to_string());
        let encode_type_impl = if self.fields.iter().any(|f| f.ty.is_struct()) {
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
                let VariableDeclaration { ty, name, .. } = self.fields.first().unwrap();
                quote!(<#ty as ::ethers_abi_enc::SolDataType>::eip712_data_word(&self.#name).0.to_vec())
            }
            _ => quote! {
                [#(
                    <#props_tys as ::ethers_abi_enc::SolDataType>::eip712_data_word(&self.#props).0,
                )*].concat()
            },
        };

        let attrs = attrs.iter();
        let convert = from_into_tuples(&self.name.0, &self.fields);
        let name_s = name.to_string();
        quote! {
            #[doc = #doc]
            #(#attrs)*
            #[allow(non_camel_case_types, non_snake_case)]
            #[derive(Debug, Clone, PartialEq)] // TODO: Derive traits dynamically
            pub struct #name {
                #(pub #fields),*
            }

            #[allow(non_camel_case_types, non_snake_case)]
            const _: () = {
                use ::ethers_abi_enc::no_std_prelude::*;

                #convert

                impl ::ethers_abi_enc::SolStruct for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_abi_enc::SolType>::TokenType;

                    const NAME: &'static str = #name_s;

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

    pub fn to_tokens(&self, tokens: &mut TokenStream, attrs: &[Attribute]) {
        if self.fields.is_empty() {
            tokens.extend(quote_spanned! {self.name.span()=>
                compile_error!("defining empty structs is disallowed.");
            });
        }
        tokens.extend(self.expand_impl(attrs))
    }

    pub fn ty(&self) -> Type {
        Type::Tuple(self.fields.types().cloned().collect())
    }
}
