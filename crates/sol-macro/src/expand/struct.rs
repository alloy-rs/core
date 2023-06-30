//! [`ItemStruct`] expansion.

use super::{
    expand_fields, expand_from_into_tuples, expand_type, ty::expand_tokenize_func, ExpCtxt,
};
use ast::{ItemStruct, VariableDeclaration};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

/// Expands an [`ItemStruct`]:
///
/// ```ignore,pseudo-code
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
pub(super) fn expand(_cx: &ExpCtxt<'_>, s: &ItemStruct) -> Result<TokenStream> {
    let ItemStruct {
        name,
        fields,
        attrs,
        ..
    } = s;

    let field_types_s = fields.iter().map(|f| f.ty.to_string());
    let field_names_s = fields.iter().map(|f| f.name.as_ref().unwrap().to_string());

    let (field_types, field_names): (Vec<_>, Vec<_>) = fields
        .iter()
        .map(|f| (expand_type(&f.ty), f.name.as_ref().unwrap()))
        .unzip();

    let encoded_type = fields.eip712_signature(name.as_string());
    let encode_type_impl = if fields.iter().any(|f| f.ty.is_custom()) {
        quote! {
            {
                let mut encoded = String::from(#encoded_type);
                #(
                    if let Some(s) = <#field_types as ::alloy_sol_types::SolType>::eip712_encode_type() {
                        encoded.push_str(&s);
                    }
                )*
                encoded
            }
        }
    } else {
        quote!(#encoded_type)
    };

    let tokenize_impl = expand_tokenize_func(fields.iter());

    let encode_data_impl = match fields.len() {
        0 => unreachable!(),
        1 => {
            let VariableDeclaration { ty, name, .. } = fields.first().unwrap();
            let ty = expand_type(ty);
            quote!(<#ty as ::alloy_sol_types::SolType>::eip712_data_word(&self.#name).0.to_vec())
        }
        _ => quote! {
            [#(
                <#field_types as ::alloy_sol_types::SolType>::eip712_data_word(&self.#field_names).0,
            )*].concat()
        },
    };

    let attrs = attrs.iter();
    let convert = expand_from_into_tuples(&name.0, fields);
    let name_s = name.to_string();
    let fields = expand_fields(fields);

    let tokens = quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case)]
        #[derive(Clone)]
        pub struct #name {
            #(pub #fields),*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        const _: () = {
            use ::alloy_sol_types::private::*;

            #convert

            #[automatically_derived]
            impl ::alloy_sol_types::SolStruct for #name {
                type Tuple<'a> = UnderlyingSolTuple<'a>;
                type Token<'a> = <Self::Tuple<'a> as ::alloy_sol_types::SolType>::TokenType<'a>;

                const NAME: &'static str = #name_s;

                const FIELDS: &'static [(&'static str, &'static str)] = &[
                    #((#field_types_s, #field_names_s)),*
                ];

                fn to_rust<'a>(&self) -> UnderlyingRustTuple<'a> {
                    self.clone().into()
                }

                fn new<'a>(tuple: UnderlyingRustTuple<'a>) -> Self {
                    tuple.into()
                }

                fn tokenize<'a>(&'a self) -> Self::Token<'a> {
                    #tokenize_impl
                }

                fn eip712_encode_type() -> Cow<'static, str> {
                    #encode_type_impl.into()
                }

                fn eip712_encode_data(&self) -> Vec<u8> {
                    #encode_data_impl
                }
            }

            #[automatically_derived]
            impl ::alloy_sol_types::EventTopic for #name {
                #[inline]
                fn topic_preimage_length(rust: &Self::RustType) -> usize {
                    let b = rust.borrow();
                    0usize
                    #(
                        + <#field_types as ::alloy_sol_types::EventTopic>::topic_preimage_length(&b.#field_names)
                    )*
                }

                #[inline]
                fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>) {
                    let b = rust.borrow();
                    out.reserve(<Self as ::alloy_sol_types::EventTopic>::topic_preimage_length(b));
                    #(
                        <#field_types as ::alloy_sol_types::EventTopic>::encode_topic_preimage(&b.#field_names, out);
                    )*
                }

                #[inline]
                fn encode_topic(
                    rust: &Self::RustType
                ) -> ::alloy_sol_types::token::WordToken {
                    let mut out = Vec::new();
                    <Self as ::alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                    ::alloy_sol_types::token::WordToken(
                        ::alloy_sol_types::private::keccak256(out)
                    )
                }
            }
        };
    };
    Ok(tokens)
}
