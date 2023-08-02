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
    let ItemStruct {
        name,
        fields,
        attrs,
        ..
    } = s;

    let (_sol_attrs, mut attrs) = crate::attr::SolAttrs::parse(attrs)?;
    cx.derives(&mut attrs, fields, true);

    let (field_types, field_names): (Vec<_>, Vec<_>) = fields
        .iter()
        .map(|f| (expand_type(&f.ty), f.name.as_ref().unwrap()))
        .unzip();

    let eip712_encode_type_fns = expand_encode_type_fns(fields, name);

    let tokenize_impl = expand_tokenize_func(fields.iter());

    let encode_data_impl = match fields.len() {
        0 => unreachable!("struct with zero fields"),
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
            #convert

            #[automatically_derived]
            impl ::alloy_sol_types::SolStruct for #name {
                type Tuple<'a> = UnderlyingSolTuple<'a>;
                type Token<'a> = <Self::Tuple<'a> as ::alloy_sol_types::SolType>::TokenType<'a>;

                const NAME: &'static str = #name_s;

                fn to_rust<'a>(&self) -> UnderlyingRustTuple<'a> {
                    self.clone().into()
                }

                fn new<'a>(tuple: UnderlyingRustTuple<'a>) -> Self {
                    tuple.into()
                }

                fn tokenize<'a>(&'a self) -> Self::Token<'a> {
                    #tokenize_impl
                }

                #eip712_encode_type_fns

                fn eip712_encode_data(&self) -> Vec<u8> {
                    #encode_data_impl
                }
            }

            #[automatically_derived]
            impl ::alloy_sol_types::EventTopic for #name {
                #[inline]
                fn topic_preimage_length(rust: &Self::RustType) -> usize {
                    0usize
                    #(
                        + <#field_types as ::alloy_sol_types::EventTopic>::topic_preimage_length(&rust.#field_names)
                    )*
                }

                #[inline]
                fn encode_topic_preimage(rust: &Self::RustType, out: &mut Vec<u8>) {
                    out.reserve(<Self as ::alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                    #(
                        <#field_types as ::alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.#field_names, out);
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

fn expand_encode_type_fns(fields: &ast::FieldList, name: &ast::SolIdent) -> TokenStream {
    let components_impl = expand_eip712_components(fields);
    let root_type_impl = fields.eip712_signature(name.as_string());

    let encode_type_impl_opt = if fields.iter().any(|f| f.ty.is_custom()) {
        None
    } else {
        Some(quote! {
            fn eip712_encode_type() -> ::alloy_sol_types::private::Cow<'static, str> {
                Self::eip712_root_type()
            }
        })
    };

    quote! {
        fn eip712_components() -> ::alloy_sol_types::private::Vec<::alloy_sol_types::private::Cow<'static, str>> {
            #components_impl
        }

        fn eip712_root_type() -> ::alloy_sol_types::private::Cow<'static, str> {
            #root_type_impl.into()
        }

        #encode_type_impl_opt
    }
}

fn expand_eip712_components(fields: &ast::FieldList) -> TokenStream {
    let bits: Vec<TokenStream> = fields
        .iter()
        .filter(|f| f.ty.is_custom())
        .map(|field| {
            let ty = expand_type(&field.ty);
            quote! {
                components.push(<#ty as ::alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<#ty as ::alloy_sol_types::SolStruct>::eip712_components());
            }
        })
        .collect();

    if bits.is_empty() {
        quote! { ::alloy_sol_types::private::Vec::new() }
    } else {
        quote! {
            let mut components = ::alloy_sol_types::private::Vec::new();
            #(#bits)*
            components
        }
    }
}
