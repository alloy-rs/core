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
    if count > 256 {
        return Err(syn::Error::new(enumm.span(), "enum has too many variants"))
    }
    let max = (count - 1) as u8;

    let has_invalid_variant = max != u8::MAX;
    let invalid_variant = has_invalid_variant.then(|| {
        let comma = (!variants.trailing_punct()).then(syn::token::Comma::default);
        quote! {
            #comma
            /// Invalid variant.
            ///
            /// This is only used when decoding an out-of-range `u8` value.
            #[doc(hidden)]
            __Invalid = u8::MAX,
        }
    });
    let detokenize_unwrap = if has_invalid_variant {
        quote! { unwrap_or(Self::__Invalid) }
    } else {
        quote! { expect("unreachable") }
    };

    let uint8 = quote!(::alloy_sol_types::sol_data::Uint<8>);
    let uint8_st = quote!(<#uint8 as ::alloy_sol_types::SolType>);

    let tokens = quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        #[derive(Clone, Copy)]
        #[repr(u8)]
        pub enum #name {
            #variants
            #invalid_variant
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
                        ::core::result::Result::Ok(unsafe { ::core::mem::transmute(v) })
                    } else {
                        ::core::result::Result::Err(::alloy_sol_types::Error::InvalidEnumValue {
                            name: #name_s,
                            value: v,
                            max: #max,
                        })
                    }
                }
            }

            #[automatically_derived]
            impl ::alloy_sol_types::Encodable<#name> for #name {
                #[inline]
                fn to_tokens(&self) -> #uint8_st::TokenType<'_> {
                    ::alloy_sol_types::Word::with_last_byte(*self as u8).into()
                }
            }

            #[automatically_derived]
            impl ::alloy_sol_types::SolType for #name {
                type RustType = #name;
                type TokenType<'a> = #uint8_st::TokenType<'a>;

                const ENCODED_SIZE: ::core::option::Option<usize> = #uint8_st::ENCODED_SIZE;

                #[inline]
                fn sol_type_name() -> ::alloy_sol_types::private::Cow<'static, str> {
                    #uint8_st::sol_type_name()
                }

                #[inline]
                fn type_check(token: &Self::TokenType<'_>) -> ::alloy_sol_types::Result<()> {
                    #uint8_st::type_check(token)?;
                    <Self as ::core::convert::TryFrom<u8>>::try_from(
                        #uint8_st::detokenize(*token)
                    ).map(::core::mem::drop)
                }

                #[inline]
                fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
                    <Self as ::core::convert::TryFrom<u8>>::try_from(
                        #uint8_st::detokenize(token)
                    ).#detokenize_unwrap
                }

                #[inline]
                fn eip712_data_word(rust: &Self::RustType) -> ::alloy_sol_types::Word {
                    #uint8_st::eip712_data_word(rust.as_u8())
                }

                #[inline]
                fn encode_packed_to(rust: &Self::RustType, out: &mut ::alloy_sol_types::private::Vec<u8>) {
                    out.push(*rust as u8);
                }
            }

            #[automatically_derived]
            impl ::alloy_sol_types::SolEnum for #name {
                const COUNT: usize = #count;
            }

            #[automatically_derived]
            impl #name {
                #[allow(unsafe_code, clippy::inline_always)]
                #[inline(always)]
                fn as_u8(&self) -> &u8 {
                    unsafe { ::core::mem::transmute::<&Self, &u8>(self) }
                }
            }
        };
    };
    Ok(tokens)
}
