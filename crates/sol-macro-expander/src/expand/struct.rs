//! [`ItemStruct`] expansion.

use super::{ExpCtxt, expand_fields, expand_from_into_tuples, expand_tokenize};
use alloy_sol_macro_input::{ContainsSolAttrs, mk_doc};
use ast::{Item, ItemStruct, SolIdent, SolPath, Spanned, Type};
use proc_macro2::TokenStream;
use quote::quote;
use std::{collections::BTreeSet, fmt::Write, num::NonZeroU16};
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
    let ItemStruct { name, fields, .. } = s;

    let (sol_attrs, mut attrs) = s.split_attrs()?;
    let sol_name = cx.sol_name(name, &s.attrs);

    cx.derives(&mut attrs, fields, true);
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);

    let (field_types, field_names): (Vec<_>, Vec<_>) =
        fields.iter().map(|f| (cx.expand_type(&f.ty), f.name.as_ref().unwrap())).unzip();

    let eip712_encode_type_fns = expand_encode_type_fns(cx, fields, name, &s.attrs);

    let tokenize_impl = expand_tokenize(fields, cx, super::FieldKind::Original);

    let encode_data_impl = match fields.len() {
        0 => unreachable!("struct with zero fields"),
        1 => {
            let name = *field_names.first().unwrap();
            let ty = field_types.first().unwrap();
            quote!(<#ty as alloy_sol_types::SolType>::eip712_data_word(&self.#name).0.to_vec())
        }
        _ => quote! {
            [#(
                <#field_types as alloy_sol_types::SolType>::eip712_data_word(&self.#field_names).0,
            )*].concat()
        },
    };

    let alloy_sol_types = &cx.crates.sol_types;

    let attrs = attrs.iter();
    let convert = expand_from_into_tuples(&name.0, fields, cx, super::FieldKind::Original);
    let name_s = sol_name;
    let fields = expand_fields(fields, cx);

    let doc = docs.then(|| mk_doc(format!("```solidity\n{s}\n```")));
    let tokens = quote! {
        #(#attrs)*
        #doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        #[derive(Clone)]
        pub struct #name {
            #(#fields),*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        const _: () = {
            use #alloy_sol_types as alloy_sol_types;

            #convert

            #[automatically_derived]
            impl alloy_sol_types::SolValue for #name {
                type SolType = Self;
            }

            #[automatically_derived]
            impl alloy_sol_types::private::SolTypeValue<Self> for #name {
                #[inline]
                fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                    #tokenize_impl
                }

                #[inline]
                fn stv_abi_encoded_size(&self) -> usize {
                    if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                        return size;
                    }

                    let sum = 0usize #(
                        + <#field_types as alloy_sol_types::SolType>::abi_encoded_size(&self.#field_names)
                    )*;
                    if <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::DYNAMIC {
                        32 + sum
                    } else {
                        sum
                    }
                }

                #[inline]
                fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                    <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
                }

                #[inline]
                fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                    #(
                        <#field_types as alloy_sol_types::SolType>::abi_encode_packed_to(&self.#field_names, out);
                    )*
                }

                #[inline]
                fn stv_abi_packed_encoded_size(&self) -> usize {
                    if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                        return size;
                    }

                    0usize #(
                        + <#field_types as alloy_sol_types::SolType>::abi_packed_encoded_size(&self.#field_names)
                    )*
                }
            }

            #[automatically_derived]
            impl alloy_sol_types::SolType for #name {
                type RustType = Self;
                type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;

                const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
                const ENCODED_SIZE: Option<usize> =
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
                const PACKED_ENCODED_SIZE: Option<usize> =
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;

                #[inline]
                fn valid_token(token: &Self::Token<'_>) -> bool {
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
                }

                #[inline]
                fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                    let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                    <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
                }
            }

            #[automatically_derived]
            impl alloy_sol_types::SolStruct for #name {
                const NAME: &'static str = #name_s;

                #eip712_encode_type_fns

                #[inline]
                fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                    #encode_data_impl
                }
            }

            #[automatically_derived]
            impl alloy_sol_types::EventTopic for #name {
                #[inline]
                fn topic_preimage_length(rust: &Self::RustType) -> usize {
                    0usize
                    #(
                        + <#field_types as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.#field_names)
                    )*
                }

                #[inline]
                fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
                    out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                    #(
                        <#field_types as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.#field_names, out);
                    )*
                }

                #[inline]
                fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                    let mut out = alloy_sol_types::private::Vec::new();
                    <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                    alloy_sol_types::abi::token::WordToken(
                        alloy_sol_types::private::keccak256(out)
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
    name: &SolIdent,
    attrs: &[syn::Attribute],
) -> TokenStream {
    let fields = substitute_non_structs(cx, fields, &cx.current_namespace);

    let root = eip712_signature(cx, &fields, &cx.sol_name(name, attrs), attrs);

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
            let ty = cx.expand_type(&ty.unwrap());

            quote! {
                components.push(<#ty as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<#ty as alloy_sol_types::SolStruct>::eip712_components());
            }
        });
        let capacity = proc_macro2::Literal::usize_unsuffixed(n_custom);
        quote! {
            let mut components = alloy_sol_types::private::Vec::with_capacity(#capacity);
            #(#bits)*
            components
        }
    } else {
        quote! { alloy_sol_types::private::Vec::new() }
    };

    let encode_type_impl_opt = encode_type_const(cx, name, &root, &fields).map(|encode_type| {
        let type_hash = crate::utils::keccak256(&encode_type);
        quote! {
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(#encode_type)
            }

            #[inline]
            fn eip712_type_hash(&self) -> alloy_sol_types::private::B256 {
                alloy_sol_types::private::B256::new([#(#type_hash),*])
            }
        }
    });

    quote! {
        #[inline]
        fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
            alloy_sol_types::private::Cow::Borrowed(#root)
        }

        #[inline]
        fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
            #components_impl
        }

        #encode_type_impl_opt
    }
}

fn eip712_signature(
    cx: &ExpCtxt<'_>,
    fields: &ast::Parameters<syn::token::Semi>,
    name: &str,
    attrs: &[syn::Attribute],
) -> String {
    let mut out = String::with_capacity(name.len() + 2 + fields.len() * 32);
    out.push_str(name);
    out.push('(');
    for (i, field) in fields.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        fmt_eip712_type(cx, &field.ty, &mut out);
        if let Some(field_name) = &field.name {
            out.push(' ');
            out.push_str(&cx.child_sol_name(field_name, &field.attrs, attrs));
        }
    }
    out.push(')');
    out
}

fn fmt_eip712_type(cx: &ExpCtxt<'_>, ty: &Type, out: &mut String) {
    match ty {
        Type::Custom(name) => {
            if let Some(name) = cx.custom_sol_name(name) {
                out.push_str(&name);
            } else {
                write!(out, "{}", name.last()).unwrap();
            }
        }
        Type::Array(array) => {
            fmt_eip712_type(cx, &array.ty, out);
            out.push('[');
            if let Some(size) = array.size_lit() {
                out.push_str(size.base10_digits());
            }
            out.push(']');
        }
        Type::Tuple(tuple) => {
            out.push('(');
            for (i, ty) in tuple.types.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                fmt_eip712_type(cx, ty, out);
            }
            out.push(')');
        }
        ty => write!(out, "{ty}").unwrap(),
    }
}

/// Replaces UDVTs, enums and contracts, which do not implement `SolStruct`, with their underlying
/// types, resolving names in `namespace`.
fn substitute_non_structs(
    cx: &ExpCtxt<'_>,
    fields: &ast::Parameters<syn::token::Semi>,
    namespace: &Option<SolIdent>,
) -> ast::Parameters<syn::token::Semi> {
    let mut fields = fields.clone();
    fields.visit_types_mut(|ty| {
        let Type::Custom(name) = ty else { return };
        match cx.try_item_in_namespace(name, namespace).map(|(_, item)| item) {
            // keep as custom
            Some(Item::Struct(_)) | None => {}
            // convert to underlying
            Some(Item::Contract(_)) => *ty = Type::Address(ty.span(), None),
            Some(Item::Enum(_)) => *ty = Type::Uint(ty.span(), NonZeroU16::new(8)),
            Some(Item::Udt(udt)) => *ty = udt.ty.clone(),
            Some(item) => {
                proc_macro_error3::abort!(item.span(), "Invalid type in struct field: {:?}", item)
            }
        }
    });
    fields
}

/// Computes the full EIP-712 `encodeType` string of `name` at expansion time: the root type
/// followed by the sorted, deduplicated root types of all transitively referenced structs.
///
/// Returns `None` if the type graph is cyclic or a custom type does not resolve to a struct.
fn encode_type_const(
    cx: &ExpCtxt<'_>,
    name: &SolIdent,
    root: &str,
    fields: &ast::Parameters<syn::token::Semi>,
) -> Option<String> {
    let mut components = BTreeSet::new();
    let mut stack = vec![(cx.current_namespace.clone(), name.clone())];
    let mut done = BTreeSet::new();
    collect_components(cx, fields, &cx.current_namespace, &mut stack, &mut done, &mut components)?;
    Some(std::iter::once(root).chain(components.iter().map(String::as_str)).collect())
}

/// Adds the root types of all structs referenced by the substituted `fields`, which are resolved
/// in `namespace`, to `components`.
fn collect_components(
    cx: &ExpCtxt<'_>,
    fields: &ast::Parameters<syn::token::Semi>,
    namespace: &Option<SolIdent>,
    stack: &mut Vec<(Option<SolIdent>, SolIdent)>,
    done: &mut BTreeSet<(Option<SolIdent>, SolIdent)>,
    components: &mut BTreeSet<String>,
) -> Option<()> {
    let mut paths = Vec::<SolPath>::new();
    for field in fields {
        field.ty.visit(|ty| {
            if let Type::Custom(path) = ty {
                paths.push(path.clone());
            }
        });
    }

    for path in paths {
        let (struct_namespace, Item::Struct(s)) = cx.try_item_in_namespace(&path, namespace)?
        else {
            return None;
        };
        // The generated Rust code resolves a bare name to the local item, which this lookup
        // does not when a global item has the same name.
        if path.len() == 1
            && struct_namespace.is_none()
            && namespace.is_some()
            && cx.all_items.get_entry_by_name_and_namespace(namespace, path.last()).is_some()
        {
            return None;
        }
        let key = (struct_namespace.clone(), s.name.clone());
        if stack.contains(&key) {
            return None;
        }
        if !done.insert(key.clone()) {
            continue;
        }

        let s_fields = substitute_non_structs(cx, &s.fields, struct_namespace);
        components.insert(eip712_signature(
            cx,
            &s_fields,
            &cx.sol_name(&s.name, &s.attrs),
            &s.attrs,
        ));
        stack.push(key);
        collect_components(cx, &s_fields, struct_namespace, stack, done, components)?;
        stack.pop();
    }
    Some(())
}
