//! Functions which generate Rust code from the Solidity AST.

use crate::{
    ast::{
        item::{self, Item},
        File, Parameters, VariableDeclaration,
    },
    utils::from_into_tuples,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{ext::IdentExt, Result, Token};

/// The [`sol!`][crate::sol!] expansion implementation.
pub fn expand(ast: File) -> TokenStream {
    let mut tokens = TokenStream::new();
    for item in ast.items {
        let t = match expand_item(&item) {
            Ok(t) => t,
            Err(e) => {
                // TODO: Dummy items
                e.into_compile_error()
            }
        };
        tokens.extend(t);
    }
    tokens
}

fn expand_item(item: &Item) -> Result<TokenStream> {
    match item {
        Item::Error(error) => expand_error(error),
        Item::Function(function) => expand_function(function),
        Item::Struct(s) => expand_struct(s),
        Item::Type(ty) => Ok(ty.to_token_stream()),
        Item::Udt(udt) => expand_udt(udt),
    }
}

fn expand_error(error: &item::Error) -> Result<TokenStream> {
    let item::Error {
        fields,
        name,
        attrs,
        ..
    } = error;

    fields.assert_resolved();

    let (signature, selector) = fields.sig_and_sel(name.as_string());

    let size = fields.data_size(None);

    let converts = from_into_tuples(&name.0, fields);
    let fields = fields.iter();
    let tokens = quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #name {
            #(pub #fields,)*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        const _: () = {
            #converts

            #[automatically_derived]
            impl ::ethers_sol_types::SolError for #name {
                type Tuple = UnderlyingSolTuple;
                type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = [#(#selector),*];

                fn to_rust(&self) -> <Self::Tuple as ::ethers_sol_types::SolType>::RustType {
                    self.clone().into()
                }

                fn from_rust(tuple: <Self::Tuple as ::ethers_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                fn data_size(&self) -> usize {
                    #size
                }
            }
        };
    };
    Ok(tokens)
}

fn expand_function(function: &item::Function) -> Result<TokenStream> {
    let function_name = function.name.0.unraw();
    let call_name = format_ident!("{}Call", function_name);
    let mut tokens = expand_call(function, &call_name, &function.arguments)?;

    if let Some(ret) = &function.returns {
        assert!(!ret.returns.is_empty());
        let return_name = format_ident!("{}Return", function_name);
        let ret = expand_call(function, &return_name, &ret.returns)?;
        tokens.extend(ret);
    }

    Ok(tokens)
}

fn expand_call(
    function: &item::Function,
    call_name: &Ident,
    params: &Parameters<Token![,]>,
) -> Result<TokenStream> {
    params.assert_resolved();

    let fields = params.iter();

    let (signature, selector) = params.sig_and_sel(function.original_name.as_string());

    let size = params.data_size(None);

    let converts = from_into_tuples(call_name, params);

    let attrs = &function.attrs;
    let tokens = quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #call_name {
            #(pub #fields,)*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        const _: () = {
            #converts

            #[automatically_derived]
            impl ::ethers_sol_types::SolCall for #call_name {
                type Tuple = UnderlyingSolTuple;
                type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = [#(#selector),*];

                fn to_rust(&self) -> <Self::Tuple as ::ethers_sol_types::SolType>::RustType {
                    self.clone().into()
                }

                fn from_rust(tuple: <Self::Tuple as ::ethers_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                fn data_size(&self) -> usize {
                    #size
                }
            }
        };
    };
    Ok(tokens)
}

fn expand_struct(s: &item::Struct) -> Result<TokenStream> {
    let item::Struct {
        name,
        fields,
        attrs,
        ..
    } = s;

    let (f_ty, f_name): (Vec<_>, Vec<_>) = s
        .fields
        .iter()
        .map(|f| (f.ty.to_string(), f.name.as_ref().unwrap().to_string()))
        .unzip();

    let props_tys: Vec<_> = fields.iter().map(|f| f.ty.clone()).collect();
    let props = fields.iter().map(|f| &f.name);

    let encoded_type = fields.eip712_signature(name.to_string());
    let encode_type_impl = if fields.iter().any(|f| f.ty.is_struct()) {
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

    let encode_data_impl = match fields.len() {
        0 => unreachable!(),
        1 => {
            let VariableDeclaration { ty, name, .. } = fields.first().unwrap();
            quote!(<#ty as ::ethers_sol_types::SolType>::eip712_data_word(&self.#name).0.to_vec())
        }
        _ => quote! {
            [#(
                <#props_tys as ::ethers_sol_types::SolType>::eip712_data_word(&self.#props).0,
            )*].concat()
        },
    };

    let attrs = attrs.iter();
    let convert = from_into_tuples(&name.0, fields);
    let name_s = name.to_string();
    let fields = fields.iter();
    let tokens = quote! {
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
    };
    Ok(tokens)
}

fn expand_udt(udt: &item::Udt) -> Result<TokenStream> {
    let item::Udt {
        name, ty, attrs, ..
    } = udt;
    let tokens = quote! {
        ::ethers_sol_types::define_udt! {
            #(#attrs)*
            #name,
            underlying: #ty,
        }
    };
    Ok(tokens)
}
