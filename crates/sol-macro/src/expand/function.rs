//! [`ItemFunction`] expansion.

use super::{
    expand_fields, expand_from_into_tuples, expand_from_into_unit, expand_tuple_types,
    r#type::expand_tokenize_func, ExpCtxt,
};
use ast::ItemFunction;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

/// Expands an [`ItemFunction`]:
///
/// ```ignore,pseudo-code
/// pub struct #{name}Call {
///     #(pub #argument_name: #argument_type,)*
/// }
///
/// impl Sol for #{name}Call {
///     ...
/// }
///
/// pub struct #{name}Return {
///     #(pub #return_name: #return_type,)*
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, function: &ItemFunction) -> Result<TokenStream> {
    expand_call(cx, function)
}

/// Expands parameters into a struct def and tuple conversions
///
/// ```ignore,pseudo-code
/// 
/// pub struct #{name}Call {
///     #(pub #argument_name: #argument_type,)*
/// }
///
/// impl From<#tuple> for #{name}Call {
///     ...
/// }
///
/// impl From<#{name}Call> for #tuple {
///     ...
/// }
/// ```
fn expand_call_struct_def(cx: &ExpCtxt<'_>, function: &ItemFunction) -> TokenStream {
    let attrs = &function.attrs;
    let call_name = cx.call_name(function);
    let fields = expand_fields(&function.arguments);

    quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #call_name {
            #(pub #fields,)*
        }
    }
}

fn expand_return_struct_def(cx: &ExpCtxt<'_>, function: &ItemFunction) -> TokenStream {
    let attrs = &function.attrs;
    let return_name = cx.return_name(function);
    let fields = if let Some(ref returns) = function.returns {
        expand_fields(&returns.returns).collect::<Vec<_>>()
    } else {
        vec![]
    };

    quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #return_name {
            #(pub #fields,)*
        }
    }
}

fn expand_call(cx: &ExpCtxt<'_>, function: &ItemFunction) -> Result<TokenStream> {
    cx.assert_resolved(&function.arguments)?;
    if let Some(ref returns) = function.returns {
        cx.assert_resolved(&returns.returns)?;
    }

    let call_name = cx.call_name(function);
    let return_name = cx.return_name(function);

    let struct_def = expand_call_struct_def(cx, function);
    let return_def = expand_return_struct_def(cx, function);

    let call_tuple = expand_tuple_types(function.arguments.types()).0;
    let return_tuple = if let Some(returns) = &function.returns {
        expand_tuple_types(returns.returns.types()).0
    } else {
        quote! { () }
    };

    let converts = expand_from_into_tuples(&call_name, &function.arguments);

    let return_converts = function
        .returns
        .as_ref()
        .map(|returns| expand_from_into_tuples(&return_name, &returns.returns))
        .unwrap_or_else(|| expand_from_into_unit(&return_name));

    let signature = cx.signature(function.name().as_string(), &function.arguments);
    let selector = crate::utils::selector(&signature);

    let tokenize_impl = expand_tokenize_func(function.arguments.iter());

    let tokens = quote! {
        #struct_def
        #return_def

        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        const _: () = {
            { #converts }
            { #return_converts }

            #[automatically_derived]
            impl ::alloy_sol_types::SolCall for #call_name {
                type Arguments<'a> = #call_tuple;
                type Token<'a> = <Self::Arguments<'a> as ::alloy_sol_types::SolType>::TokenType<'a>;

                type Return = #return_name;

                type ReturnTuple<'a> = #return_tuple;
                type ReturnToken<'a> = <Self::ReturnTuple<'a> as ::alloy_sol_types::SolType>::TokenType<'a>;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector;

                fn new<'a>(tuple: <Self::Arguments<'a> as ::alloy_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                fn tokenize(&self) -> Self::Token<'_> {
                    #tokenize_impl
                }

                fn decode_returns(data: &[u8], validate: bool) -> ::alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<'_> as ::alloy_sol_types::SolType>::decode(data, validate).map(Into::into)
                }
            }
        };
    };
    Ok(tokens)
}
