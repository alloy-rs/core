//! [`ItemEvent`] expansion.

use super::{anon_name, expand_tuple_types, expand_type, ty, ExpCtxt};
use ast::{EventParameter, ItemEvent, SolIdent, Spanned};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Result;

/// Expands an [`ItemEvent`]:
///
/// ```ignore (pseudo-code)
/// pub struct #name {
///     #(pub #parameter_name: #parameter_type,)*
/// }
///
/// impl SolEvent for #name {
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, event: &ItemEvent) -> Result<TokenStream> {
    let ItemEvent { attrs, .. } = event;
    let params = event.params();

    let (_sol_attrs, mut attrs) = crate::attr::SolAttrs::parse(attrs)?;
    cx.derives(&mut attrs, &params, true);

    cx.assert_resolved(&params)?;
    event.assert_valid()?;

    let name = cx.overloaded_name(event.into());
    let signature = cx.signature(name.as_string(), &params);
    let selector = crate::utils::event_selector(&signature);
    let anonymous = event.is_anonymous();

    // prepend the first topic if not anonymous
    let first_topic = (!anonymous).then(|| quote!(::alloy_sol_types::sol_data::FixedBytes<32>));
    let topic_list = event.indexed_params().map(expand_event_topic_type);
    let topic_list = first_topic.into_iter().chain(topic_list);

    let (data_tuple, _) = expand_tuple_types(event.non_indexed_params().map(|p| &p.ty));

    // skip first topic if not anonymous, which is the hash of the signature
    let mut topic_i = !anonymous as usize;
    let mut data_i = 0usize;
    let new_impl = event.parameters.iter().enumerate().map(|(i, p)| {
        let name = anon_name((i, p.name.as_ref()));
        let param;
        if p.is_indexed() {
            let i = syn::Index::from(topic_i);
            param = quote!(topics.#i);
            topic_i += 1;
        } else {
            let i = syn::Index::from(data_i);
            param = quote!(data.#i);
            data_i += 1;
        }
        quote!(#name: #param)
    });

    let topic_tuple_names = event
        .indexed_params()
        .map(|p| p.name.as_ref())
        .enumerate()
        .map(anon_name);

    let topics_impl = if anonymous {
        quote! {(#(self.#topic_tuple_names.clone(),)*)}
    } else {
        quote! {(Self::SIGNATURE_HASH.into(), #(self.#topic_tuple_names.clone(),)*)}
    };

    let encode_first_topic = (!anonymous).then(|| {
        quote!(::alloy_sol_types::abi::token::WordToken(
            Self::SIGNATURE_HASH
        ))
    });

    let encode_topics_impl = event.indexed_params().enumerate().map(|(i, p)| {
        let name = anon_name((i, p.name.as_ref()));
        let ty = expand_type(&p.ty);

        if p.indexed_as_hash() {
            quote! {
                <::alloy_sol_types::sol_data::FixedBytes<32> as ::alloy_sol_types::EventTopic>::encode_topic(&self.#name)
            }
        } else {
            quote! {
                <#ty as ::alloy_sol_types::EventTopic>::encode_topic(&self.#name)
            }
        }
    });

    let fields = event
        .parameters
        .iter()
        .enumerate()
        .map(|(i, p)| expand_event_topic_field(i, p, p.name.as_ref()));

    let tokenize_body_impl = ty::expand_event_tokenize_func(event.parameters.iter());

    let encode_topics_impl = encode_first_topic
        .into_iter()
        .chain(encode_topics_impl)
        .enumerate()
        .map(|(i, assign)| quote!(out[#i] = #assign;));

    let tokens = quote! {
        #(#attrs)*
        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        pub struct #name {
            #(pub #fields,)*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        const _: () = {
            #[automatically_derived]
            impl ::alloy_sol_types::SolEvent for #name {
                type DataTuple<'a> = #data_tuple;
                type DataToken<'a> = <Self::DataTuple<'a> as ::alloy_sol_types::SolType>::TokenType<'a>;

                type TopicList = (#(#topic_list,)*);

                const SIGNATURE: &'static str = #signature;
                const SIGNATURE_HASH: ::alloy_sol_types::private::B256 =
                    ::alloy_sol_types::private::B256::new(#selector);

                const ANONYMOUS: bool = #anonymous;

                #[allow(unused_variables)]
                #[inline]
                fn new(
                    topics: <Self::TopicList as ::alloy_sol_types::SolType>::RustType,
                    data: <Self::DataTuple<'_> as ::alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    Self {
                        #(#new_impl,)*
                    }
                }

                #[inline]
                fn tokenize_body(&self) -> Self::DataToken<'_> {
                    #tokenize_body_impl
                }

                #[inline]
                fn topics(&self) -> <Self::TopicList as ::alloy_sol_types::SolType>::RustType {
                    #topics_impl
                }

                #[inline]
                fn encode_topics_raw(
                    &self,
                    out: &mut [::alloy_sol_types::abi::token::WordToken],
                ) -> ::alloy_sol_types::Result<()> {
                    if out.len() < <Self::TopicList as ::alloy_sol_types::TopicList>::COUNT {
                        return Err(::alloy_sol_types::Error::Overrun);
                    }
                    #(#encode_topics_impl)*
                    Ok(())
                }
            }
        };
    };
    Ok(tokens)
}

fn expand_event_topic_type(param: &EventParameter) -> TokenStream {
    assert!(param.is_indexed());
    if param.is_abi_dynamic() {
        quote_spanned! {param.ty.span()=> ::alloy_sol_types::sol_data::FixedBytes<32> }
    } else {
        expand_type(&param.ty)
    }
}

fn expand_event_topic_field(
    i: usize,
    param: &EventParameter,
    name: Option<&SolIdent>,
) -> TokenStream {
    let name = anon_name((i, name));
    let ty = if param.indexed_as_hash() {
        ty::expand_rust_type(&ast::Type::FixedBytes(
            name.span(),
            core::num::NonZeroU16::new(32).unwrap(),
        ))
    } else {
        ty::expand_rust_type(&param.ty)
    };
    quote!(#name: #ty)
}
