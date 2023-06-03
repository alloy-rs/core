use crate::{
    common::{from_into_tuples, Parameters, SolIdent, VariableDeclaration},
    r#type::Type,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Attribute, Result, Token,
};

#[derive(Clone)]
pub struct Struct {
    struct_token: Token![struct],
    pub name: SolIdent,
    brace_token: Brace,
    pub fields: Parameters<Token![;]>,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields
    }
}

impl Eq for Struct {}

impl Hash for Struct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.fields.hash(state);
    }
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
            struct_token: input.parse()?,
            name: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse()?,
        })
    }
}

impl Struct {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.struct_token = Token![struct](span);
        self.name.set_span(span);
        self.brace_token = Brace(span);
    }

    fn expand_impl(&self, attrs: &[Attribute]) -> TokenStream {
        let name = &self.name;

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
                        if let Some(s) = <#props_tys as ::ethers_sol_types::SolType>::eip712_encode_type() {
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
            0 => unreachable!(),
            1 => {
                let VariableDeclaration { ty, name, .. } = self.fields.first().unwrap();
                quote!(<#ty as ::ethers_sol_types::SolType>::eip712_data_word(&self.#name).0.to_vec())
            }
            _ => quote! {
                [#(
                    <#props_tys as ::ethers_sol_types::SolType>::eip712_data_word(&self.#props).0,
                )*].concat()
            },
        };

        let attrs = attrs.iter();
        let convert = from_into_tuples(&self.name.0, &self.fields);
        let name_s = name.to_string();
        quote! {
            #(#attrs)*
            #[allow(non_camel_case_types, non_snake_case)]
            #[derive(Clone)]
            pub struct #name {
                #(pub #fields),*
            }

            #[allow(non_camel_case_types, non_snake_case, clippy::style)]
            const _: () = {
                use ::ethers_sol_types::no_std_prelude::*;

                #convert

                #[automatically_derived]
                impl ::ethers_sol_types::SolStruct for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

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

                    fn eip712_encode_type() -> Cow<'static, str> {
                        #encode_type_impl.into()
                    }

                    fn eip712_encode_data(&self) -> Vec<u8> {
                        #encode_data_impl
                    }
                }
            };
        }
    }

    pub fn to_tokens(&self, tokens: &mut TokenStream, attrs: &[Attribute]) {
        tokens.extend(self.expand_impl(attrs))
    }

    pub fn ty(&self) -> Type {
        Type::Tuple(self.fields.types().cloned().collect())
    }
}
