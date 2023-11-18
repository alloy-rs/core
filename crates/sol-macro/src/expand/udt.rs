//! [`ItemUdt`] expansion.

use super::{expand_type, ExpCtxt};
use ast::{ItemUdt, Spanned};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result};

pub(super) fn expand(cx: &ExpCtxt<'_>, udt: &ItemUdt) -> Result<TokenStream> {
    let ItemUdt { name, ty, attrs, .. } = udt;

    // TODO: Uncomment after migrating `define_udt!`
    let _ = cx;
    // let (_sol_attrs, mut attrs) = crate::attr::SolAttrs::parse(attrs)?;
    // cx.type_derives(&mut attrs, Some(ty), true);

    if !ty.is_value_type() {
        return Err(Error::new(
            ty.span(),
            "the underlying types of the user defined values must be elementary value types",
        ));
    }

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
