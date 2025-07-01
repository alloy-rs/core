//! [`ItemError`] expansion.

use super::{ExpCtxt, expand_fields, expand_from_into_tuples, expand_tokenize};
use alloy_sol_macro_input::{ContainsSolAttrs, mk_doc};
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
    let ItemError { parameters: params, .. } = error;
    cx.assert_resolved(params)?;

    let (sol_attrs, mut attrs) = error.split_attrs()?;
    cx.derives(&mut attrs, params, true);
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);
    let abi = sol_attrs.abi.or(cx.attrs.abi).unwrap_or(false);

    let tokenize_impl = expand_tokenize(params, cx, super::FieldKind::Deconstruct);

    let name = cx.overloaded_name(error.into());
    let signature = cx.error_signature(error);
    let selector = crate::utils::selector(&signature);

    let alloy_sol_types = &cx.crates.sol_types;

    let converts = expand_from_into_tuples(&name.0, params, cx, super::FieldKind::Deconstruct);

    let doc = docs.then(|| {
        let selector = hex::encode_prefixed(selector.array.as_slice());
        mk_doc(format!(
            "Custom error with signature `{signature}` and selector `{selector}`.\n\
             ```solidity\n{error}\n```"
        ))
    });
    let abi: Option<TokenStream> = abi.then(|| {
        if_json! {
            let error = super::to_abi::generate(error, cx);
            quote! {
                #[automatically_derived]
                impl alloy_sol_types::JsonAbiExt for #name {
                    type Abi = alloy_sol_types::private::alloy_json_abi::Error;

                    #[inline]
                    fn abi() -> Self::Abi {
                        #error
                    }
                }
            }
        }
    });

    let err_struct = match params.len() {
        0 => {
            // Expanded as a unit struct.
            quote! {
                pub struct #name;
            }
        }
        1 if params[0].name.is_none() => {
            let ty = cx.expand_rust_type(&params[0].ty);
            // Expanded as tuple struct if only one _unnamed_ parameter.
            quote! {
                pub struct #name(pub #ty);
            }
        }
        _ => {
            let fields = expand_fields(params, cx);
            quote! {
                pub struct #name {
                    #(#fields),*
                }
            }
        }
    };

    let tokens = quote! {
        #(#attrs)*
        #doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        #[derive(Clone)]
        #err_struct

        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        const _: () = {
            use #alloy_sol_types as alloy_sol_types;

            #converts

            #[automatically_derived]
            impl alloy_sol_types::SolError for #name {
                type Parameters<'a> = UnderlyingSolTuple<'a>;
                type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector;

                #[inline]
                fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    #tokenize_impl
                }

                #[inline]
                fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                    <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
                }
            }

            #abi
        };
    };
    Ok(tokens)
}
