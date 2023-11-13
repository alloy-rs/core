//! [`ItemError`] expansion.

use super::{expand_fields, expand_from_into_tuples, expand_tokenize, ExpCtxt};
use crate::attr;
use ast::ItemError;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

/// Expands an [`ItemError`]:
///
/// ```ignore (pseudo-code)
/// pub struct #name {
///     #(pub #parameter_name: #parameter_type,)*
/// }
///
/// impl SolError for #name {
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, error: &ItemError) -> Result<TokenStream> {
    let ItemError { parameters: params, name, attrs, .. } = error;
    cx.assert_resolved(params)?;

    let (sol_attrs, mut attrs) = crate::attr::SolAttrs::parse(attrs)?;
    cx.derives(&mut attrs, params, true);
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);

    let tokenize_impl = expand_tokenize(params);

    let signature = cx.error_signature(error);
    let selector = crate::utils::selector(&signature);

    let converts = expand_from_into_tuples(&name.0, params);
    let fields = expand_fields(params);
    let doc = docs.then(|| {
        let selector = hex::encode_prefixed(selector.array);
        attr::mk_doc(format!(
            "Custom error with signature `{signature}` and selector `{selector}`.\n\
             ```solidity\n{error}\n```"
        ))
    });
    let tokens = quote! {
        #(#attrs)*
        #doc
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #name {
            #(#fields),*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        const _: () = {
            #converts

            #[automatically_derived]
            impl ::alloy_sol_types::SolError for #name {
                type Parameters<'a> = UnderlyingSolTuple<'a>;
                type Token<'a> = <Self::Parameters<'a> as ::alloy_sol_types::SolType>::TokenType<'a>;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector;

                #[inline]
                fn new<'a>(tuple: <Self::Parameters<'a> as ::alloy_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    #tokenize_impl
                }
            }
        };
    };
    Ok(tokens)
}
