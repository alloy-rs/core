//! [`ItemEnum`] expansion.

use super::ExpCtxt;
use ast::ItemEnum;
use proc_macro2::TokenStream;
use syn::Result;

/// Expands an [`ItemEnum`]:
///
/// ```ignore,pseudo-code
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
    let _ = cx;
    let ItemEnum {
        name: _,
        variants: _,
        attrs: _,
        ..
    } = enumm;

    todo!()
}
