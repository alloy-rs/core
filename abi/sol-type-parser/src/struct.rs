use proc_macro2::TokenStream;
use syn::{braced, parse::Parse, punctuated::Punctuated, token::Struct, Ident, Index, Token};

use quote::{quote, ToTokens};

use crate::r#type::SolType;

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
    fn rust_name_alias(&self) -> Ident {
        Ident::new(&format!("{}Rust", self.name), self.name.span())
    }

    fn token_alias(&self) -> Ident {
        Ident::new(&format!("{}Token", self.name), self.name.span())
    }

    fn expand_aliases(&self) -> TokenStream {
        let rust_name = self.rust_name_alias();
        let sol_doc = format!("A type containg info for the Solidity type {}", &self.name);
        let token_alias = self.token_alias();
        let sol_name = &self.name;

        let field_ty = self.fields.iter().map(|f| &f.ty);
        let field_ty2 = field_ty.clone();
        let field_ty3 = field_ty.clone();

        quote! {
            #[doc = #sol_doc]
            #[derive(Debug, Clone, Copy, Default)]
            pub struct #sol_name(
                #(::core::marker::PhantomData<#field_ty>),*,
                ::core::marker::PhantomData<#rust_name>,
            );

            impl #sol_name {
                /// Instantiate at runtime for dynamic encoding
                pub fn new() -> Self {
                    Self::default()
                }
            }

            type UnderlyingTuple = (#(#field_ty2),*);

            pub type #token_alias = <UnderlyingTuple as ::ethers_abi_enc::SolType>::TokenType;

            type UnderlyingRustTuple = (#(<#field_ty3 as ::ethers_abi_enc::SolType>::RustType),*);
        }
    }

    fn expand_from(&self) -> TokenStream {
        let rust_name = self.rust_name_alias();
        let field_names = self.fields.iter().map(|f| &f.name);

        let (f_no, f_name): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| (Index::from(i), &f.name))
            .unzip();

        quote! {
            impl Into<UnderlyingRustTuple> for #rust_name {
                fn into(self) -> UnderlyingRustTuple {
                    (#(self.#field_names),*)
                }
            }

            impl From<UnderlyingRustTuple> for #rust_name {
                fn from(tuple: UnderlyingRustTuple) -> Self {
                    #rust_name {
                        #(#f_name: tuple.#f_no),*
                    }
                }
            }
        }
    }

    fn expand_detokenize(&self) -> TokenStream {
        let rust_name = self.rust_name_alias();

        let (field_nos, field_names): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| (Index::from(i), &f.name))
            .unzip();

        quote! {
            fn detokenize(token: Self::TokenType) -> ::ethers_abi_enc::AbiResult<#rust_name> {
                let tuple = <UnderlyingTuple as ::ethers_abi_enc::SolType>::detokenize(token)?;
                Ok(#rust_name {
                    #(#field_names: tuple.#field_nos),*
                })
            }
        }
    }

    fn expand_tokenize(&self) -> TokenStream {
        quote! {
            fn tokenize<Borrower>(rust: Borrower) -> Self::TokenType
            where
                Borrower: ::std::borrow::Borrow<Self::RustType>
            {
                let tuple: UnderlyingRustTuple = rust.borrow().clone().into();
                <UnderlyingTuple as ::ethers_abi_enc::SolType>::tokenize(tuple)
            }
        }
    }
}

impl ToTokens for SolStructDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let rust_name = self.rust_name_alias();
        let rust_name_str = rust_name.to_string();
        let sol_name = &self.name;

        let token_alias = Ident::new(&format!("{}Token", sol_name), sol_name.span());

        let mod_name = Ident::new(&format!("__{}", sol_name), sol_name.span());

        let doc = format!(
            "A rust struct that represents the Solidity type {}",
            sol_name
        );
        let fields = self.fields.iter();

        let aliases = self.expand_aliases();
        let from = self.expand_from();
        let detokenize = self.expand_detokenize();
        let tokenize = self.expand_tokenize();

        tokens.extend(quote! {
            pub use #mod_name::{#sol_name, #rust_name, #token_alias};
            #[allow(non_snake_case)]
            mod #mod_name {
                use super::*;

                #aliases

                #[doc = #doc]
                #[derive(Debug, Clone, PartialEq)]
                pub struct #rust_name {
                    #(pub #fields),*
                }

                #from

                impl ::ethers_abi_enc::SolType for #sol_name {
                    type RustType = #rust_name;
                    type TokenType = #token_alias;

                    fn sol_type_name() -> ::std::string::String {
                        #rust_name_str.to_owned()
                    }
                    fn is_dynamic() -> bool {
                        <UnderlyingTuple as ::ethers_abi_enc::SolType>::is_dynamic()
                    }
                    fn type_check(token: &Self::TokenType) -> ::ethers_abi_enc::AbiResult<()> {
                        <UnderlyingTuple as ::ethers_abi_enc::SolType>::type_check(token)
                    }

                    #detokenize
                    #tokenize

                    fn encode_packed_to<Borrower>(target: &mut Vec<u8>, rust: Borrower)
                    where
                        Borrower: ::std::borrow::Borrow<Self::RustType>
                    {
                        let tuple: UnderlyingRustTuple = rust.borrow().clone().into();
                        <UnderlyingTuple as ::ethers_abi_enc::SolType>::encode_packed_to(target, tuple)
                    }
                }
            }
        });
    }
}
