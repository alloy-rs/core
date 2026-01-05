//! [`ItemStruct`] expansion.

use super::{ExpCtxt, expand_fields};
use crate::codegen::{Eip712Options, StructCodegen};
use alloy_sol_macro_input::{ContainsSolAttrs, mk_doc};
use ast::{Item, ItemStruct, Spanned, Type};
use proc_macro2::TokenStream;
use quote::quote;
use std::num::NonZeroU16;
use syn::Result;

/// Expands an [`ItemStruct`]:
///
/// ```ignore (pseudo-code)
/// pub struct #name {
///     #(pub #field_name: #field_type,)*
/// }
///
/// impl SolStruct for #name {
///     ...
/// }
///
/// // Needed to use in event parameters
/// impl EventTopic for #name {
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, s: &ItemStruct) -> Result<TokenStream> {
    let ItemStruct { name, fields, .. } = s;

    let (sol_attrs, mut attrs) = s.split_attrs()?;

    cx.derives(&mut attrs, fields, true);
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);

    let (field_names, (sol_types, rust_types)): (Vec<_>, (Vec<_>, Vec<_>)) = fields
        .iter()
        .map(|f| {
            (
                f.name.as_ref().unwrap().0.clone(),
                (cx.expand_type(&f.ty), cx.expand_rust_type(&f.ty)),
            )
        })
        .unzip();

    let alloy_sol_types = &cx.crates.sol_types;

    let struct_impl = StructCodegen::new(
        field_names,
        rust_types,
        sol_types,
        gen_eip712_options(cx, fields, name),
    )
    .expand(&name.0, &quote!(#alloy_sol_types));

    let attrs = attrs.iter();
    let fields = expand_fields(fields, cx);

    let doc = docs.then(|| mk_doc(format!("```solidity\n{s}\n```")));
    let tokens = quote! {
        #(#attrs)*
        #doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        #[derive(Clone)]
        pub struct #name {
            #(#fields),*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        const _: () = {
            use #alloy_sol_types as alloy_sol_types;
            #struct_impl
        };
    };
    Ok(tokens)
}

fn gen_eip712_options(
    cx: &ExpCtxt<'_>,
    fields: &ast::Parameters<syn::token::Semi>,
    name: &ast::SolIdent,
) -> Eip712Options {
    // account for UDVTs and enums which do not implement SolStruct
    let mut fields = fields.clone();
    fields.visit_types_mut(|ty| {
        let Type::Custom(name) = ty else { return };
        match cx.try_item(name) {
            // keep as custom
            Some(Item::Struct(_)) | None => {}
            // convert to underlying
            Some(Item::Contract(_)) => *ty = Type::Address(ty.span(), None),
            Some(Item::Enum(_)) => *ty = Type::Uint(ty.span(), NonZeroU16::new(8)),
            Some(Item::Udt(udt)) => *ty = udt.ty.clone(),
            Some(item) => {
                proc_macro_error2::abort!(item.span(), "Invalid type in struct field: {:?}", item)
            }
        }
    });

    let root = fields.eip712_signature(name.as_string());

    let custom = fields.iter().filter(|f| f.ty.has_custom());
    let n_custom = custom.clone().count();

    let components_impl = if n_custom > 0 {
        let bits = custom.map(|field| {
            // need to recurse to find the inner custom type
            let mut ty = None;
            field.ty.visit(|field_ty| {
                if ty.is_none() && field_ty.is_custom() {
                    ty = Some(field_ty.clone());
                }
            });
            // cannot panic as this field is guaranteed to contain a custom type
            let ty = cx.expand_type(&ty.unwrap());

            quote! {
                components.push(<#ty as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<#ty as alloy_sol_types::SolStruct>::eip712_components());
            }
        });
        let capacity = proc_macro2::Literal::usize_unsuffixed(n_custom);
        Some(quote! {
            let mut components = alloy_sol_types::private::Vec::with_capacity(#capacity);
            #(#bits)*
            components
        })
    } else {
        None
    };

    Eip712Options { root, components_impl, encode_type_impl: None }
}
