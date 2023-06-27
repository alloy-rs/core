//! [`ItemContract`] expansion.

use super::{attr, r#type, ExpCtxt};
use ast::{Item, ItemContract, ItemError, ItemFunction, SolIdent};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{ext::IdentExt, parse_quote, Attribute, Result};

/// Expands an [`ItemContract`]:
///
/// ```ignore,pseudo-code
/// pub mod #name {
///     pub enum #{name}Calls {
///         ...
///    }
///
///     pub enum #{name}Errors {
///         ...
///    }
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, contract: &ItemContract) -> Result<TokenStream> {
    let ItemContract {
        attrs, name, body, ..
    } = contract;

    let mut functions = Vec::with_capacity(contract.body.len());
    let mut errors = Vec::with_capacity(contract.body.len());
    let mut item_tokens = TokenStream::new();
    let d_attrs: Vec<Attribute> = attr::derives(attrs).cloned().collect();
    for item in body {
        match item {
            Item::Function(function) => functions.push(function),
            Item::Error(error) => errors.push(error),
            _ => {}
        }
        item_tokens.extend(quote!(#(#d_attrs)*));
        item_tokens.extend(cx.expand_item(item)?);
    }

    let functions_enum = if functions.len() > 1 {
        let mut attrs = d_attrs.clone();
        let doc_str = format!("Container for all the `{name}` function calls.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        Some(expand_functions_enum(cx, name, functions, &attrs))
    } else {
        None
    };

    let errors_enum = if errors.len() > 1 {
        let mut attrs = d_attrs;
        let doc_str = format!("Container for all the `{name}` custom errors.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        Some(expand_errors_enum(cx, name, errors, &attrs))
    } else {
        None
    };

    let mod_attrs = attr::docs(attrs);
    let tokens = quote! {
        #(#mod_attrs)*
        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        pub mod #name {
            #item_tokens
            #functions_enum
            #errors_enum
        }
    };
    Ok(tokens)
}

fn expand_functions_enum(
    cx: &ExpCtxt<'_>,
    contract_name: &SolIdent,
    functions: Vec<&ItemFunction>,
    attrs: &[Attribute],
) -> TokenStream {
    let name = format_ident!("{contract_name}Calls");
    let variants: Vec<_> = functions
        .iter()
        .map(|f| cx.function_name_ident(f).0)
        .collect();

    let types: Vec<_> = variants.iter().map(|name| cx.raw_call_name(name)).collect();

    let min_data_len = functions
        .iter()
        .map(|function| r#type::params_min_data_size(cx, &function.arguments))
        .min()
        .unwrap();
    let trt = Ident::new("SolCall", Span::call_site());
    expand_call_like_enum(name, &variants, &types, min_data_len, trt, attrs)
}

fn expand_errors_enum(
    cx: &ExpCtxt<'_>,
    contract_name: &SolIdent,
    errors: Vec<&ItemError>,
    attrs: &[Attribute],
) -> TokenStream {
    let name = format_ident!("{contract_name}Errors");
    let variants: Vec<_> = errors.iter().map(|error| error.name.0.clone()).collect();
    let min_data_len = errors
        .iter()
        .map(|error| r#type::params_min_data_size(cx, &error.parameters))
        .min()
        .unwrap();
    let trt = Ident::new("SolError", Span::call_site());
    expand_call_like_enum(name, &variants, &variants, min_data_len, trt, attrs)
}

fn expand_call_like_enum(
    name: Ident,
    variants: &[Ident],
    types: &[Ident],
    min_data_len: usize,
    trt: Ident,
    attrs: &[Attribute],
) -> TokenStream {
    assert_eq!(variants.len(), types.len());
    let name_s = name.to_string();
    let count = variants.len();
    let methods = variants.iter().zip(types).map(generate_variant_methods);
    quote! {
        #(#attrs)*
        pub enum #name {
            #(#variants(#types),)*
        }

        #[automatically_derived]
        impl ::alloy_sol_types::SolCalls for #name {
            const NAME: &'static str = #name_s;
            const MIN_DATA_LENGTH: usize = #min_data_len;
            const COUNT: usize = #count;

            #[inline]
            fn selector(&self) -> [u8; 4] {
                match self {#(
                    Self::#variants(_) => <#types as ::alloy_sol_types::#trt>::SELECTOR,
                )*}
            }

            #[inline]
            fn type_check(selector: [u8; 4]) -> ::alloy_sol_types::Result<()> {
                match selector {
                    #(<#types as ::alloy_sol_types::#trt>::SELECTOR)|* => Ok(()),
                    s => ::core::result::Result::Err(::alloy_sol_types::Error::unknown_selector(
                        Self::NAME,
                        s,
                    )),
                }
            }

            #[inline]
            fn decode_raw(
                selector: [u8; 4],
                data: &[u8],
                validate: bool
            )-> ::alloy_sol_types::Result<Self> {
                match selector {
                    #(<#types as ::alloy_sol_types::#trt>::SELECTOR => {
                        <#types as ::alloy_sol_types::#trt>::decode_raw(data, validate)
                            .map(Self::#variants)
                    })*
                    s => ::core::result::Result::Err(::alloy_sol_types::Error::unknown_selector(
                        Self::NAME,
                        s,
                    )),
                }
            }

            #[inline]
            fn encoded_size(&self) -> usize {
                match self {#(
                    Self::#variants(inner) =>
                        <#types as ::alloy_sol_types::#trt>::encoded_size(inner),
                )*}
            }

            #[inline]
            fn encode_raw(&self, out: &mut Vec<u8>) {
                match self {#(
                    Self::#variants(inner) =>
                        <#types as ::alloy_sol_types::#trt>::encode_raw(inner, out),
                )*}
            }
        }

        #[automatically_derived]
        impl #name {
            #(#methods)*
        }

        #(
            #[automatically_derived]
            impl ::core::convert::From<#types> for #name {
                #[inline]
                fn from(value: #types) -> Self {
                    Self::#variants(value)
                }
            }

            #[automatically_derived]
            impl ::core::convert::TryFrom<#name> for #types {
                type Error = #name;

                #[inline]
                fn try_from(value: #name) -> ::core::result::Result<Self, Self::Error> {
                    match value {
                        #name::#variants(value) => ::core::result::Result::Ok(value),
                        _ => ::core::result::Result::Err(value),
                    }
                }
            }
        )*
    }
}

fn generate_variant_methods((variant, ty): (&Ident, &Ident)) -> TokenStream {
    let name = variant.unraw().to_string().to_snake_case();

    let is_variant = format_ident!("is_{name}");
    let is_variant_doc = format!("Returns `true` if `self` matches [`{name}`](Self::{name}).");

    let as_variant = format_ident!("as_{name}");
    let as_variant_doc = format!(
        "Returns an immutable reference to the inner [`{ty}`] if `self` matches [`{name}`](Self::{name})."
    );

    let as_variant_mut = format_ident!("as_{name}_mut");
    let as_variant_mut_doc = format!(
        "Returns a mutable reference to the inner [`{ty}`] if `self` matches [`{name}`](Self::{name})."
    );

    quote! {
        #[doc = #is_variant_doc]
        #[inline]
        pub const fn #is_variant(&self) -> bool {
            ::core::matches!(self, Self::#variant(_))
        }

        #[doc = #as_variant_doc]
        #[inline]
        pub const fn #as_variant(&self) -> ::core::option::Option<&#ty> {
            match self {
                Self::#variant(inner) => ::core::option::Option::Some(inner),
                _ => ::core::option::Option::None,
            }
        }

        #[doc = #as_variant_mut_doc]
        #[inline]
        pub fn #as_variant_mut(&mut self) -> ::core::option::Option<&mut #ty> {
            match self {
                Self::#variant(inner) => ::core::option::Option::Some(inner),
                _ => ::core::option::Option::None,
            }
        }
    }
}
