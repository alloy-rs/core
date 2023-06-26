//! [`ItemEnum`] expansion.

use super::ExpCtxt;
use ast::ItemEnum;
use proc_macro2::TokenStream;
use quote::quote;
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
pub(super) fn expand(_cx: &ExpCtxt<'_>, enumm: &ItemEnum) -> Result<TokenStream> {
    let ItemEnum {
        name,
        variants,
        attrs,
        ..
    } = enumm;

    let name_s = name.to_string();

    let count = variants.len();
    if count == 0 {
        return Err(syn::Error::new(enumm.span(), "enum has no variants"))
    }

    let max = count - 1;
    if max > u8::MAX as usize {
        return Err(syn::Error::new(enumm.span(), "enum has too many variants"))
    }
    let max = max as u8;

    let tokens = quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        #[derive(Clone, Copy)]
        #[repr(u8)]
        pub enum #name {
            #variants
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        const _: () = {
            #[automatically_derived]
            impl ::core::convert::From<#name> for u8 {
                #[inline]
                fn from(v: #name) -> Self {
                    v as u8
                }
            }

            #[automatically_derived]
            impl ::core::convert::TryFrom<u8> for #name {
                type Error = ::alloy_sol_types::Error;

                #[allow(unsafe_code)]
                #[inline]
                fn try_from(v: u8) -> ::alloy_sol_types::Result<Self> {
                    if v <= #max {
                        Ok(unsafe { ::core::mem::transmute(v) })
                    } else {
                        Err(::alloy_sol_types::Error::InvalidEnumValue {
                            name: #name_s,
                            value: v,
                            max: #max,
                        })
                    }
                }
            }

            #[automatically_derived]
            impl ::alloy_sol_types::SolEnum for #name {}
        };
    };
    Ok(tokens)
}
