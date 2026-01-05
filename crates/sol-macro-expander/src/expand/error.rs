//! [`ItemError`] expansion.

use super::{ExpCtxt, anon_name, expand_fields};
use crate::codegen::ErrorCodegen;
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

    let name = cx.overloaded_name(error.into());
    let signature = cx.error_signature(error);
    let selector = crate::utils::calc_selector(&signature);

    let alloy_sol_types = &cx.crates.sol_types;

    // Collect field data for codegen
    let (param_names, (sol_types, rust_types)): (Vec<_>, (Vec<_>, Vec<_>)) = params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            (anon_name((i, p.name.as_ref())), (cx.expand_type(&p.ty), cx.expand_rust_type(&p.ty)))
        })
        .unzip();
    let is_tuple_struct = params.len() == 1 && params[0].name.is_none();

    let doc = docs.then(|| {
        let selector = hex::encode_prefixed(selector.as_slice());
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

    let error_impl = ErrorCodegen::new(param_names, sol_types, rust_types, is_tuple_struct).expand(
        &name.0,
        &signature,
        &quote!(#alloy_sol_types),
    );

    let tokens = quote! {
        #(#attrs)*
        #doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        #[derive(Clone)]
        #err_struct

        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        const _: () = {
            use #alloy_sol_types as alloy_sol_types;

            #error_impl

            #abi
        };
    };
    Ok(tokens)
}
