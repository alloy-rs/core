//! [`ItemUdt`] expansion.

use super::{expand_type, ExpCtxt};
use ast::ItemUdt;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub(super) fn expand(cx: &ExpCtxt<'_>, udt: &ItemUdt) -> Result<TokenStream> {
    let ItemUdt {
        name, ty, attrs, ..
    } = udt;

    let (_sol_attrs, mut attrs) = crate::attr::SolAttrs::parse(attrs)?;
    cx.type_derives(&mut attrs, Some(ty), true);

    let ty = expand_type(ty);
    let tokens = quote! {
        ::alloy_sol_types::define_udt! {
            #(#attrs)*
            #name,
            underlying: #ty,
        }
    };
    Ok(tokens)
}
