//! [`ItemEvent`] expansion.

use super::{ExpCtxt, anon_name};
use crate::codegen::{EventCodegen, EventFieldInfo};
use alloy_sol_macro_input::{ContainsSolAttrs, mk_doc};
use ast::{EventParameter, ItemEvent, SolIdent, Spanned};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

/// Expands an [`ItemEvent`]:
///
/// ```ignore (pseudo-code)
/// pub struct #name {
///     #(pub #parameter_name: #parameter_type,)*
/// }
///
/// impl SolEvent for #name {
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, event: &ItemEvent) -> Result<TokenStream> {
    let params = event.params();

    let (sol_attrs, mut attrs) = event.split_attrs()?;
    cx.derives(&mut attrs, &params, true);
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);
    let abi = sol_attrs.abi.or(cx.attrs.abi).unwrap_or(false);

    cx.assert_resolved(&params)?;
    event.assert_valid()?;

    let name = cx.overloaded_name(event.into());
    let signature = cx.event_signature(event);
    let selector = crate::utils::event_selector(&signature);
    let anonymous = event.is_anonymous();

    let (fields, fields_info): (Vec<_>, Vec<_>) = event
        .parameters
        .iter()
        .enumerate()
        .map(|(i, p)| {
            (
                expand_event_topic_field(i, p, p.name.as_ref(), cx),
                EventFieldInfo {
                    name: anon_name((i, p.name.as_ref())),
                    sol_type: cx.expand_type(&p.ty),
                    is_indexed: p.is_indexed(),
                    indexed_as_hash: cx.indexed_as_hash(p),
                    span: p.ty.span(),
                },
            )
        })
        .unzip();

    let doc = docs.then(|| {
        let selector = hex::encode_prefixed(selector.array.as_slice());
        mk_doc(format!(
            "Event with signature `{signature}` and selector `{selector}`.\n\
            ```solidity\n{event}\n```"
        ))
    });

    let abi: Option<TokenStream> = abi.then(|| {
        if_json! {
            let event = super::to_abi::generate(event, cx);
            quote! {
                #[automatically_derived]
                impl alloy_sol_types::JsonAbiExt for #name {
                    type Abi = alloy_sol_types::private::alloy_json_abi::Event;

                    fn abi() -> Self::Abi {
                        #event
                    }
                }
            }
        }
    });

    let alloy_sol_types = &cx.crates.sol_types;

    let event_struct = if event.parameters.is_empty() {
        quote! {
            pub struct #name;
        }
    } else {
        quote! {
            pub struct #name {
                #(#fields,)*
            }
        }
    };

    let event_impl = EventCodegen::new(anonymous, fields_info).expand(
        &name.0,
        &signature,
        &quote!(#alloy_sol_types),
    );

    let tokens = quote! {
        #(#attrs)*
        #doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        #[derive(Clone)]
        #event_struct

        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        const _: () = {
            use #alloy_sol_types as alloy_sol_types;

            #event_impl

            #abi
        };
    };
    Ok(tokens)
}

fn expand_event_topic_field(
    i: usize,
    param: &EventParameter,
    name: Option<&SolIdent>,
    cx: &ExpCtxt<'_>,
) -> TokenStream {
    let name = anon_name((i, name));
    let ty = if cx.indexed_as_hash(param) {
        let bytes32 = ast::Type::FixedBytes(name.span(), core::num::NonZeroU16::new(32).unwrap());
        cx.expand_rust_type(&bytes32)
    } else {
        cx.expand_rust_type(&param.ty)
    };
    let attrs = &param.attrs;
    quote! {
        #(#attrs)*
        #[allow(missing_docs)]
        pub #name: #ty
    }
}
