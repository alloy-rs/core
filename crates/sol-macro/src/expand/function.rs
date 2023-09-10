//! [`ItemFunction`] expansion.

use super::{
    expand_fields, expand_from_into_tuples, expand_tuple_types, ty::expand_tokenize_func, ExpCtxt,
};
use ast::ItemFunction;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

/// Expands an [`ItemFunction`]:
///
/// ```ignore (pseudo-code)
/// pub struct #{name}Call {
///     #(pub #argument_name: #argument_type,)*
/// }
///
/// pub struct #{name}Return {
///     #(pub #return_name: #return_type,)*
/// }
///
/// impl SolCall for #{name}Call {
///     type Return = #{name}Return;
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, function: &ItemFunction) -> Result<TokenStream> {
    let ItemFunction {
        attrs,
        arguments,
        returns,
        name: Some(_),
        ..
    } = function
    else {
        // ignore functions without names (constructors, modifiers...)
        return Ok(quote!())
    };
    let returns = returns.as_ref().map(|r| &r.returns).unwrap_or_default();

    cx.assert_resolved(arguments)?;
    if !returns.is_empty() {
        cx.assert_resolved(returns)?;
    }

    let (_sol_attrs, mut call_attrs) = crate::attr::SolAttrs::parse(attrs)?;
    let mut return_attrs = call_attrs.clone();
    cx.derives(&mut call_attrs, arguments, true);
    if !returns.is_empty() {
        cx.derives(&mut return_attrs, returns, true);
    }

    let call_name = cx.call_name(function);
    let return_name = cx.return_name(function);

    let call_fields = expand_fields(arguments);
    let return_fields = expand_fields(returns);

    let call_tuple = expand_tuple_types(arguments.types()).0;
    let return_tuple = expand_tuple_types(returns.types()).0;

    let converts = expand_from_into_tuples(&call_name, arguments);
    let return_converts = expand_from_into_tuples(&return_name, returns);

    let signature = cx.function_signature(function);
    let selector = crate::utils::selector(&signature);
    let tokenize_impl = expand_tokenize_func(arguments.iter());

    let tokens = quote! {
        #(#call_attrs)*
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #call_name {
            #(#call_fields),*
        }

        #(#return_attrs)*
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #return_name {
            #(#return_fields),*
        }

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
