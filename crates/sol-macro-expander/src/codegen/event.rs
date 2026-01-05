//! SolEvent trait generation.

use super::quote_byte_array;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};

/// Info about a single event field for computing derived TokenStreams.
#[derive(Debug, Clone)]
pub struct EventFieldInfo {
    /// Field name (e.g., `_0` or `from`)
    pub name: Ident,
    /// Sol data type (e.g., `sol_data::Address`)
    pub sol_type: TokenStream,
    /// Whether this field is indexed
    pub is_indexed: bool,
    /// Whether indexed type is hashed (true for string, bytes, arrays, structs)
    pub indexed_as_hash: bool,
    /// Source span for error diagnostics
    pub span: Span,
}

/// Data for generating SolEvent trait implementation.
#[derive(Debug)]
pub struct EventCodegen {
    /// Whether this is an anonymous event
    anonymous: bool,
    /// All event fields in declaration order
    fields: Vec<EventFieldInfo>,
}

impl EventCodegen {
    /// Creates a new event codegen.
    pub fn new(anonymous: bool, fields: Vec<EventFieldInfo>) -> Self {
        Self { anonymous, fields }
    }

    /// Tuple type for non-indexed params: `(Type1, Type2, ...)`
    pub fn data_tuple(&self) -> TokenStream {
        let types: Vec<_> =
            self.fields.iter().filter(|f| !f.is_indexed).map(|f| &f.sol_type).collect();
        if types.is_empty() { quote!(()) } else { quote!((#(#types,)*)) }
    }

    /// Topic list type: `(FixedBytes<32>, Type1, ...)` or just `(Type1, ...)` for anonymous
    pub fn topic_list(&self, crate_path: &TokenStream) -> TokenStream {
        let first = (!self.anonymous).then(|| quote!(#crate_path::sol_data::FixedBytes<32>));
        let indexed = self.fields.iter().filter(|f| f.is_indexed).map(|f| {
            if f.indexed_as_hash {
                let span = f.span;
                quote_spanned!(span=> #crate_path::sol_data::FixedBytes<32>)
            } else {
                f.sol_type.clone()
            }
        });
        let all: Vec<_> = first.into_iter().chain(indexed).collect();
        quote!((#(#all,)*))
    }

    /// `tokenize_body()` impl: tokenize non-indexed fields
    pub fn tokenize_body(&self, crate_path: &TokenStream) -> TokenStream {
        let tokenize_stmts: Vec<_> = self
            .fields
            .iter()
            .filter(|f| !f.is_indexed)
            .map(|f| {
                let name = &f.name;
                let sol_type = &f.sol_type;
                quote!(<#sol_type as #crate_path::SolType>::tokenize(&self.#name))
            })
            .collect();
        if tokenize_stmts.is_empty() { quote!(()) } else { quote!((#(#tokenize_stmts,)*)) }
    }

    /// `topics()` impl: collect indexed field values
    pub fn topics_impl(&self) -> TokenStream {
        let indexed_names: Vec<_> =
            self.fields.iter().filter(|f| f.is_indexed).map(|f| &f.name).collect();
        if self.anonymous {
            quote!((#(self.#indexed_names.clone(),)*))
        } else {
            quote!((Self::SIGNATURE_HASH.into(), #(self.#indexed_names.clone(),)*))
        }
    }

    /// `encode_topics_raw()` impl: encode each topic
    pub fn encode_topics_impl(&self, crate_path: &TokenStream) -> TokenStream {
        let first_topic = (!self.anonymous)
            .then(|| quote!(#crate_path::abi::token::WordToken(Self::SIGNATURE_HASH)));
        let indexed_encodes = self.fields.iter().filter(|f| f.is_indexed).map(|f| {
            let name = &f.name;
            let sol_type = &f.sol_type;
            if f.indexed_as_hash {
                quote!(<#crate_path::sol_data::FixedBytes<32> as #crate_path::EventTopic>::encode_topic(&self.#name))
            } else {
                quote!(<#sol_type as #crate_path::EventTopic>::encode_topic(&self.#name))
            }
        });
        let assignments = first_topic
            .into_iter()
            .chain(indexed_encodes)
            .enumerate()
            .map(|(i, assign)| quote!(out[#i] = #assign;));
        quote!(#(#assignments)*)
    }

    /// `new()` constructor impl: interleave topics/data
    pub fn new_impl(&self) -> TokenStream {
        let mut topic_i = if self.anonymous { 0usize } else { 1usize };
        let mut data_i = 0usize;
        let field_inits: Vec<_> = self
            .fields
            .iter()
            .map(|f| {
                let name = &f.name;
                let (source, idx) = if f.is_indexed {
                    let i = topic_i;
                    topic_i += 1;
                    (quote!(topics), syn::Index::from(i))
                } else {
                    let i = data_i;
                    data_i += 1;
                    (quote!(data), syn::Index::from(i))
                };
                quote!(#name: #source.#idx)
            })
            .collect();
        quote!(Self { #(#field_inits,)* })
    }
}

impl EventCodegen {
    /// Generates the `SolEvent` trait implementation.
    ///
    /// NOTE: the `crate_path` should be a path to `alloy_sol_types`.
    pub fn expand(self, name: &Ident, signature: &str, crate_path: &TokenStream) -> TokenStream {
        let data_tuple = self.data_tuple();
        let topic_list = self.topic_list(crate_path);
        let tokenize_body = self.tokenize_body(crate_path);
        let topics_impl = self.topics_impl();
        let encode_topics_impl = self.encode_topics_impl(crate_path);
        let new_impl = self.new_impl();
        let anonymous = self.anonymous;

        let signature_hash = crate::utils::keccak256(signature);
        let hash_tokens = quote_byte_array(&signature_hash);

        let check_signature = (!anonymous).then(|| {
            quote! {
                #[inline]
                fn check_signature(topics: &<Self::TopicList as #crate_path::SolType>::RustType) -> #crate_path::Result<()> {
                    if topics.0 != Self::SIGNATURE_HASH {
                        return Err(#crate_path::Error::invalid_event_signature_hash(Self::SIGNATURE, topics.0, Self::SIGNATURE_HASH));
                    }
                    Ok(())
                }
            }
        });

        quote! {
            #[automatically_derived]
            impl #crate_path::SolEvent for #name {
                type DataTuple<'a> = #data_tuple;
                type DataToken<'a> = <Self::DataTuple<'a> as #crate_path::SolType>::Token<'a>;

                type TopicList = #topic_list;

                const SIGNATURE: &'static str = #signature;
                const SIGNATURE_HASH: #crate_path::private::B256 =
                    #crate_path::private::B256::new(#hash_tokens);

                const ANONYMOUS: bool = #anonymous;

                #[allow(unused_variables)]
                #[inline]
                fn new(
                    topics: <Self::TopicList as #crate_path::SolType>::RustType,
                    data: <Self::DataTuple<'_> as #crate_path::SolType>::RustType,
                ) -> Self {
                    #new_impl
                }

                #check_signature

                #[inline]
                fn tokenize_body(&self) -> Self::DataToken<'_> {
                    #tokenize_body
                }

                #[inline]
                fn topics(&self) -> <Self::TopicList as #crate_path::SolType>::RustType {
                    #topics_impl
                }

                #[inline]
                fn encode_topics_raw(
                    &self,
                    out: &mut [#crate_path::abi::token::WordToken],
                ) -> #crate_path::Result<()> {
                    if out.len() < <Self::TopicList as #crate_path::TopicList>::COUNT {
                        return Err(#crate_path::Error::Overrun);
                    }
                    #encode_topics_impl
                    Ok(())
                }
            }

            #[automatically_derived]
            impl #crate_path::private::IntoLogData for #name {
                fn to_log_data(&self) -> #crate_path::private::LogData {
                    From::from(self)
                }

                fn into_log_data(self) -> #crate_path::private::LogData {
                    From::from(&self)
                }
            }

            #[automatically_derived]
            impl From<&#name> for #crate_path::private::LogData {
                #[inline]
                fn from(this: &#name) -> #crate_path::private::LogData {
                    #crate_path::SolEvent::encode_log_data(this)
                }
            }
        }
    }
}
