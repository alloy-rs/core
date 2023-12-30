//! [`ItemUdt`] expansion.

use super::ExpCtxt;
use crate::expand::expand_type;
use ast::ItemUdt;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub(super) fn expand(cx: &ExpCtxt<'_>, udt: &ItemUdt) -> Result<TokenStream> {
    let ItemUdt { name, ty, attrs, .. } = udt;

    let (sol_attrs, mut attrs) = crate::attr::SolAttrs::parse(attrs)?;
    cx.type_derives(&mut attrs, Some(ty), true);

    let type_check = if let Some(lit_str) = sol_attrs.type_check {
        let func_path: syn::Path = syn::parse_str(&lit_str.value()).unwrap();
        quote! { #func_path }
    } else {
        quote! { ::alloy_sol_types::private::just_ok }
    };

    let underlying = expand_type(ty);
    let tokens = quote! {
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct #name (
            <#underlying as ::alloy_sol_types::SolType>::RustType,
        );

        #[automatically_derived]
        impl ::alloy_sol_types::private::SolTypeValue<#name> for <#underlying as ::alloy_sol_types::SolType>::RustType {
            #[inline]
            fn stv_to_tokens(&self) -> <#underlying as ::alloy_sol_types::SolType>::Token<'_> {
                ::alloy_sol_types::private::SolTypeValue::<#underlying>::stv_to_tokens(self)
            }

            #[inline]
            fn stv_eip712_data_word(&self) -> ::alloy_sol_types::Word {
                <#underlying as ::alloy_sol_types::SolType>::tokenize(self).0
            }

            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut ::alloy_sol_types::private::Vec<u8>) {
                <#underlying as ::alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
        }

        #[automatically_derived]
        impl #name {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@name);

            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: <#underlying as ::alloy_sol_types::SolType>::RustType) -> Self {
                Self(value)
            }

            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> <#underlying as ::alloy_sol_types::SolType>::RustType {
                self.0
            }

            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> ::alloy_sol_types::private::Vec<u8> {
                <Self as ::alloy_sol_types::SolType>::abi_encode(&self.0)
            }

            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> ::alloy_sol_types::private::Vec<u8> {
                <Self as ::alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }

        #[automatically_derived]
        impl ::alloy_sol_types::SolType for #name {
            type RustType = <#underlying as ::alloy_sol_types::SolType>::RustType;
            type Token<'a> = <#underlying as ::alloy_sol_types::SolType>::Token<'a>;

            const ENCODED_SIZE: Option<usize> = <#underlying as ::alloy_sol_types::SolType>::ENCODED_SIZE;

            #[inline]
            fn sol_type_name() -> ::alloy_sol_types::private::Cow<'static, str> {
                Self::NAME.into()
            }

            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }

            #[inline]
            fn type_check(token: &Self::Token<'_>) -> ::alloy_sol_types::Result<()> {
                <#underlying as ::alloy_sol_types::SolType>::type_check(token)?;
                #type_check(token)
            }

            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <#underlying as ::alloy_sol_types::SolType>::detokenize(token)
            }
        }

        #[automatically_derived]
        impl ::alloy_sol_types::EventTopic for #name {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <#underlying as ::alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }

            #[inline]
            fn encode_topic_preimage(rust: &Self::RustType, out: &mut ::alloy_sol_types::private::Vec<u8>) {
                <#underlying as ::alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }

            #[inline]
            fn encode_topic(rust: &Self::RustType) -> ::alloy_sol_types::abi::token::WordToken {
                <#underlying as ::alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    Ok(tokens)
}
