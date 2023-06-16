//! [`ItemError`] expansion.

use super::{expand_fields, expand_from_into_tuples, ExpCtxt};
use ast::ItemError;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

/// Expands an [`ItemError`]:
///
/// ```ignore,pseudo-code
/// pub struct #name {
///     #(pub #parameter_name: #parameter_type,)*
/// }
///
/// impl SolError for #name {
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, error: &ItemError) -> Result<TokenStream> {
    let ItemError {
        parameters,
        name,
        attrs,
        ..
    } = error;
    cx.assert_resolved(parameters)?;

    let signature = cx.signature(name.as_string(), parameters);
    let selector = crate::utils::selector(&signature);

    let converts = expand_from_into_tuples(&name.0, parameters);
    let fields = expand_fields(parameters);
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
            impl ::alloy_sol_types::SolError for #name {
                type Tuple = UnderlyingSolTuple;
                type Token = <Self::Tuple as ::alloy_sol_types::SolType>::TokenType;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector;

                fn to_rust(&self) -> <Self::Tuple as ::alloy_sol_types::SolType>::RustType {
                    self.clone().into()
                }

                fn from_rust(tuple: <Self::Tuple as ::alloy_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }
            }
        };
    };
    Ok(tokens)
}
