//! [`ItemFunction`] expansion.

use super::{expand_fields, expand_from_into_tuples, r#type::expand_tokenize_func, ExpCtxt};
use ast::{ItemFunction, Parameters};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Result, Token};

/// Expands an [`ItemFunction`]:
///
/// ```ignore,pseudo-code
/// pub struct #{name}Call {
///     #(pub #argument_name: #argument_type,)*
/// }
///
/// impl SolCall for #{name}Call {
///     ...
/// }
///
/// pub struct #{name}Return {
///     #(pub #return_name: #return_type,)*
/// }
///
/// impl SolCall for #{name}Return {
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, function: &ItemFunction) -> Result<TokenStream> {
    let function_name = cx.function_name(function);
    let call_name = cx.call_name(function_name.clone());
    let mut tokens = expand_call(cx, function, &call_name, &function.arguments)?;

    if let Some(ret) = &function.returns {
        assert!(!ret.returns.is_empty());
        let return_name = cx.return_name(function_name);
        let ret = expand_call(cx, function, &return_name, &ret.returns)?;
        tokens.extend(ret);
    }

    Ok(tokens)
}

fn expand_call(
    cx: &ExpCtxt<'_>,
    function: &ItemFunction,
    call_name: &Ident,
    params: &Parameters<Token![,]>,
) -> Result<TokenStream> {
    cx.assert_resolved(params)?;

    let fields = expand_fields(params);

    let signature = cx.signature(function.name().as_string(), params);
    let selector = crate::utils::selector(&signature);

    let converts = expand_from_into_tuples(call_name, params);

    let tokenize_impl = if params.is_empty() {
        quote! { ::core::convert::From::from([]) }
    } else {
        expand_tokenize_func(params.iter())
    };

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
            impl ::alloy_sol_types::SolCall for #call_name {
                type Tuple<'a> = UnderlyingSolTuple<'a>;
                type Token<'a> = <Self::Tuple<'a> as ::alloy_sol_types::SolType>::TokenType<'a>;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector;

                fn to_rust<'a>(&self) -> <Self::Tuple<'a> as ::alloy_sol_types::SolType>::RustType {
                    self.clone().into()
                }

                fn from_rust<'a>(tuple: <Self::Tuple<'a> as ::alloy_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                fn tokenize(&self) -> Self::Token<'_> {
                    #tokenize_impl
                }
            }
        };
    };
    Ok(tokens)
}
