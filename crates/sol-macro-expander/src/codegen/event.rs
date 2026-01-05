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
    fn data_tuple(&self) -> TokenStream {
        let types: Vec<_> =
            self.fields.iter().filter(|f| !f.is_indexed).map(|f| &f.sol_type).collect();
        if types.is_empty() { quote!(()) } else { quote!((#(#types,)*)) }
    }

    /// Topic list type: `(FixedBytes<32>, Type1, ...)` or just `(Type1, ...)` for anonymous
    fn topic_list(&self) -> TokenStream {
        let first = (!self.anonymous).then(|| quote!(alloy_sol_types::sol_data::FixedBytes<32>));
        let indexed = self.fields.iter().filter(|f| f.is_indexed).map(|f| {
            if f.indexed_as_hash {
                let span = f.span;
                quote_spanned!(span=> alloy_sol_types::sol_data::FixedBytes<32>)
            } else {
                f.sol_type.clone()
            }
        });
        let all: Vec<_> = first.into_iter().chain(indexed).collect();
        quote!((#(#all,)*))
    }

    /// `tokenize_body()` impl: tokenize non-indexed fields
    fn tokenize_body(&self) -> TokenStream {
        let tokenize_stmts: Vec<_> = self
            .fields
            .iter()
            .filter(|f| !f.is_indexed)
            .map(|f| {
                let name = &f.name;
                let sol_type = &f.sol_type;
                quote!(<#sol_type as alloy_sol_types::SolType>::tokenize(&self.#name))
            })
            .collect();
        if tokenize_stmts.is_empty() { quote!(()) } else { quote!((#(#tokenize_stmts,)*)) }
    }

    /// `topics()` impl: collect indexed field values
    fn topics_impl(&self) -> TokenStream {
        let indexed_names: Vec<_> =
            self.fields.iter().filter(|f| f.is_indexed).map(|f| &f.name).collect();
        if self.anonymous {
            quote!((#(self.#indexed_names.clone(),)*))
        } else {
            quote!((Self::SIGNATURE_HASH.into(), #(self.#indexed_names.clone(),)*))
        }
    }

    /// `encode_topics_raw()` impl: encode each topic
    fn encode_topics_impl(&self) -> TokenStream {
        let first_topic = (!self.anonymous)
            .then(|| quote!(alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH)));
        let indexed_encodes = self.fields.iter().filter(|f| f.is_indexed).map(|f| {
            let name = &f.name;
            let sol_type = &f.sol_type;
            if f.indexed_as_hash {
                quote!(<alloy_sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(&self.#name))
            } else {
                quote!(<#sol_type as alloy_sol_types::EventTopic>::encode_topic(&self.#name))
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
    fn new_impl(&self) -> TokenStream {
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

    /// Generates the `SolEvent` trait implementation.
    ///
    /// NOTE: The generated code assumes `alloy_sol_types` is in scope.
    pub fn expand(self, name: &Ident, signature: &str) -> TokenStream {
        let data_tuple = self.data_tuple();
        let topic_list = self.topic_list();
        let tokenize_body = self.tokenize_body();
        let topics_impl = self.topics_impl();
        let encode_topics_impl = self.encode_topics_impl();
        let new_impl = self.new_impl();
        let anonymous = self.anonymous;

        let signature_hash = crate::utils::keccak256(signature);
        let hash_tokens = quote_byte_array(&signature_hash);

        let check_signature = (!anonymous).then(|| {
            quote! {
                #[inline]
                fn check_signature(topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType) -> alloy_sol_types::Result<()> {
                    if topics.0 != Self::SIGNATURE_HASH {
                        return Err(alloy_sol_types::Error::invalid_event_signature_hash(Self::SIGNATURE, topics.0, Self::SIGNATURE_HASH));
                    }
                    Ok(())
                }
            }
        });

        quote! {
            #[automatically_derived]
            impl alloy_sol_types::SolEvent for #name {
                type DataTuple<'a> = #data_tuple;
                type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;

                type TopicList = #topic_list;

                const SIGNATURE: &'static str = #signature;
                const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                    alloy_sol_types::private::B256::new(#hash_tokens);

                const ANONYMOUS: bool = #anonymous;

                #[allow(unused_variables)]
                #[inline]
                fn new(
                    topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                    data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    #new_impl
                }

                #check_signature

                #[inline]
                fn tokenize_body(&self) -> Self::DataToken<'_> {
                    #tokenize_body
                }

                #[inline]
                fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                    #topics_impl
                }

                #[inline]
                fn encode_topics_raw(
                    &self,
                    out: &mut [alloy_sol_types::abi::token::WordToken],
                ) -> alloy_sol_types::Result<()> {
                    if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                        return Err(alloy_sol_types::Error::Overrun);
                    }
                    #encode_topics_impl
                    Ok(())
                }
            }

            #[automatically_derived]
            impl alloy_sol_types::private::IntoLogData for #name {
                fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                    From::from(self)
                }

                fn into_log_data(self) -> alloy_sol_types::private::LogData {
                    From::from(&self)
                }
            }

            #[automatically_derived]
            impl From<&#name> for alloy_sol_types::private::LogData {
                #[inline]
                fn from(this: &#name) -> alloy_sol_types::private::LogData {
                    alloy_sol_types::SolEvent::encode_log_data(this)
                }
            }
        }
    }
}
