//! [`ItemStruct`] expansion.

use super::{
    expand_fields, expand_from_into_tuples, expand_type, ty::expand_tokenize_func, ExpCtxt,
};
use ast::{Item, ItemStruct, Spanned, Type, VariableDeclaration};
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

    let eip712_encode_type_fns = expand_encode_type_fns(cx, fields, name);

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

fn expand_encode_type_fns(
    cx: &ExpCtxt<'_>,
    fields: &ast::Parameters<syn::token::Semi>,
    name: &ast::SolIdent,
) -> TokenStream {
    // account for UDVTs and enums which do not implement SolStruct
    let mut fields = fields.clone();
    fields.visit_types_mut(|ty| {
        let Type::Custom(name) = ty else { return };
        match cx.try_get_item(name) {
            // keep as custom
            Some(Item::Struct(_)) | None => {}
            // convert to underlying
            Some(Item::Enum(_)) => *ty = Type::Uint(ty.span(), NonZeroU16::new(8)),
            Some(Item::Udt(udt)) => *ty = udt.ty.clone(),
            Some(item) => panic!("Invalid type in struct field: {item:?}"),
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
            let ty = expand_type(&ty.unwrap());

            quote! {
                components.push(<#ty as ::alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<#ty as ::alloy_sol_types::SolStruct>::eip712_components());
            }
        });
        let capacity = proc_macro2::Literal::usize_unsuffixed(n_custom);
        quote! {
            let mut components = ::alloy_sol_types::private::Vec::with_capacity(#capacity);
            #(#bits)*
            components
        }
    } else {
        quote! { ::alloy_sol_types::private::Vec::new() }
    };

    let encode_type_impl_opt = (n_custom == 0).then(|| {
        quote! {
            #[inline]
            fn eip712_encode_type() -> ::alloy_sol_types::private::Cow<'static, str> {
                <Self as ::alloy_sol_types::SolStruct>::eip712_root_type()
            }
        }
    });

    quote! {
        #[inline]
        fn eip712_root_type() -> ::alloy_sol_types::private::Cow<'static, str> {
            ::alloy_sol_types::private::Cow::Borrowed(#root)
        }

        fn eip712_components() -> ::alloy_sol_types::private::Vec<::alloy_sol_types::private::Cow<'static, str>> {
            #components_impl
        }

        #encode_type_impl_opt
    }
}
