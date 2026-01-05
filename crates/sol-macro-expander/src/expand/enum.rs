//! [`ItemEnum`] expansion.

use super::ExpCtxt;
use crate::codegen::EnumCodegen;
use alloy_sol_macro_input::{ContainsSolAttrs, derives_mapped, mk_doc};
use ast::{ItemEnum, Spanned};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Result;

/// Expands an [`ItemEnum`]:
///
/// ```ignore (pseudo-code)
/// #[repr(u8)]
/// pub enum #name {
///     #(#variant,)*
/// }
///
/// impl SolEnum for #name {
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, enumm: &ItemEnum) -> Result<TokenStream> {
    let ItemEnum { name, variants, .. } = enumm;

    let (sol_attrs, mut attrs) = enumm.split_attrs()?;
    cx.derives(&mut attrs, [], false);
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);

    let count = variants.len();
    if count == 0 {
        return Err(syn::Error::new(enumm.span(), "enum has no variants"));
    }
    if count > 256 {
        return Err(syn::Error::new(enumm.span(), "enum has too many variants"));
    }
    let max = (count - 1) as u8;

    let has_invalid_variant = max != u8::MAX;
    let invalid_variant = has_invalid_variant.then(|| {
        let comma = (!variants.trailing_punct()).then(syn::token::Comma::default);

        let has_serde = derives_mapped(&attrs).any(|path| {
            let Some(last) = path.segments.last() else {
                return false;
            };
            last.ident == "Serialize" || last.ident == "Deserialize"
        });
        let serde_other = has_serde.then(|| quote!(#[serde(other)]));

        quote! {
            #comma
            /// Invalid variant.
            ///
            /// This is only used when decoding an out-of-range `u8` value.
            #[doc(hidden)]
            #serde_other
            __Invalid = u8::MAX,
        }
    });

    let alloy_sol_types = &cx.crates.sol_types;

    // Collect variant idents for codegen
    let variant_idents: Vec<Ident> =
        variants.iter().map(|v| Ident::new(&v.ident.to_string(), v.ident.span())).collect();

    let enum_impl = EnumCodegen::new(variant_idents, has_invalid_variant).expand(&name.0);

    let doc = docs.then(|| mk_doc(format!("```solidity\n{enumm}\n```")));
    let tokens = quote! {
        #(#attrs)*
        #doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        #[derive(Clone, Copy)]
        #[repr(u8)]
        pub enum #name {
            #variants
            #invalid_variant
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        const _: () = {
            use #alloy_sol_types as alloy_sol_types;

            #enum_impl
        };
    };
    Ok(tokens)
}
