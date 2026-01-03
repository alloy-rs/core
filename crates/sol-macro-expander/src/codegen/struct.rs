//! SolStruct trait generation.

use super::{StructLayout, expand_tokenize_simple, gen_from_into_tuple};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Options for EIP-712 encoding in struct implementations.
#[derive(Debug, Default)]
pub struct Eip712Options {
    /// The EIP-712 root type signature (e.g., "MyStruct(uint256 a,address b)")
    pub root: String,
    /// Custom implementation for `eip712_components()`. If None, returns empty Vec.
    ///
    /// When this is `Some(...)` and `encode_type_impl` is `None`, the `eip712_encode_type()`
    /// override is omitted, allowing the trait's default implementation to be used (which
    /// correctly concatenates root type + sorted components).
    pub components_impl: Option<TokenStream>,
    /// Custom implementation for `eip712_encode_type()`.
    ///
    /// - `None` → infer from `components_impl`: if components exist, use trait default; otherwise
    ///   emit override returning `eip712_root_type()`
    /// - `Some(tokens)` → emit custom implementation
    pub encode_type_impl: Option<TokenStream>,
}

/// Data for generating SolStruct trait implementation.
#[derive(Debug)]
pub struct StructCodegen {
    /// Field names
    field_names: Vec<Ident>,
    /// Rust types for fields
    rust_types: Vec<TokenStream>,
    /// Solidity types for fields
    sol_types: Vec<TokenStream>,
    /// EIP-712 options
    eip712: Eip712Options,
}

impl StructCodegen {
    /// Creates a new struct codegen.
    pub fn new(
        field_names: Vec<Ident>,
        rust_types: Vec<TokenStream>,
        sol_types: Vec<TokenStream>,
        eip712: Eip712Options,
    ) -> Self {
        Self { field_names, rust_types, sol_types, eip712 }
    }

    /// Generates the `SolStruct` trait implementation.
    pub fn expand(self, name: &Ident) -> TokenStream {
        let Self { field_names, rust_types, sol_types, eip712 } = self;

        let name_s = name.to_string();
        // Strip raw identifier prefix for SOL_NAME
        let name_s = name_s.strip_prefix("r#").unwrap_or(&name_s);
        let root = &eip712.root;

        let layout = if field_names.is_empty() { StructLayout::Unit } else { StructLayout::Named };
        let tupl_impl = gen_from_into_tuple(name, &field_names, &sol_types, &rust_types, layout);

        // Build EIP-712 functions
        let has_components = eip712.components_impl.is_some();
        let components_impl = eip712.components_impl.unwrap_or_else(|| {
            quote! { alloy_sol_types::private::Vec::new() }
        });

        // Infer encode_type behavior:
        // - If custom implementation provided, use it
        // - Else if components exist, rely on trait default (don't emit override)
        // - Else emit override returning root_type (optimization for no-deps case)
        let encode_type_fn = match eip712.encode_type_impl {
            Some(tokens) => Some(quote! {
                #[inline]
                fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                    #tokens
                }
            }),
            None if has_components => None, // rely on trait default
            None => Some(quote! {
                #[inline]
                fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                    <Self as alloy_sol_types::SolStruct>::eip712_root_type()
                }
            }),
        };

        let eip712_fns = quote! {
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(#root)
            }

            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
                #components_impl
            }

            #encode_type_fn
        };

        let traits = gen_sol_struct_traits(name, name_s, &field_names, &sol_types, eip712_fns);

        quote! {
            #tupl_impl

            #traits
        }
    }

    /// Generates the full const block including the wrapper.
    ///
    /// Use this for external crates that want to generate Alloy-compatible types.
    /// For use with the `sol!` macro (which has configurable crate paths), use
    /// [`expand_inner`](Self::expand_inner) instead and wrap with your own const block.
    pub fn expand_with_const(self, name: &Ident) -> TokenStream {
        let inner = self.expand(name);
        quote! {
            #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
            const _: () = {
                use alloy_sol_types as alloy_sol_types;
                #inner
            };
        }
    }
}

/// Generates just the SolStruct trait implementations (SolValue, SolTypeValue, SolType, SolStruct,
/// EventTopic).
///
/// - Does NOT generate the `const _: () = { ... }` wrapper
/// - Does NOT generate type aliases or From/Into conversions
/// - Assumes `UnderlyingSolTuple<'a>` and `UnderlyingRustTuple<'a>` type aliases already exist
///
/// `name_str` should be the struct name without any raw identifier prefix (e.g., "const" not
/// "r#const").
fn gen_sol_struct_traits(
    struct_name: &Ident,
    name_s: &str,
    field_names: &[Ident],
    sol_types: &[TokenStream],
    eip712_fns: TokenStream,
) -> TokenStream {
    let tokenize_impl = expand_tokenize_simple(field_names, sol_types);

    let encode_data_impl = if field_names.is_empty() {
        quote! { alloy_sol_types::private::Vec::new() }
    } else if field_names.len() == 1 {
        let name = &field_names[0];
        let ty = &sol_types[0];
        quote! { <#ty as alloy_sol_types::SolType>::eip712_data_word(&self.#name).0.to_vec() }
    } else {
        quote! {
            [#(
                <#sol_types as alloy_sol_types::SolType>::eip712_data_word(&self.#field_names).0,
            )*].concat()
        }
    };

    quote! {
        #[automatically_derived]
        impl alloy_sol_types::SolValue for #struct_name {
            type SolType = Self;
        }

        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for #struct_name {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                #tokenize_impl
            }

            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }

            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }

            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }

            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }

        #[automatically_derived]
        impl alloy_sol_types::SolType for #struct_name {
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
        impl alloy_sol_types::SolStruct for #struct_name {
            const NAME: &'static str = #name_s;

            #eip712_fns

            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                #encode_data_impl
            }
        }

        #[automatically_derived]
        impl alloy_sol_types::EventTopic for #struct_name {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                #(+ <#sol_types as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.#field_names))*
            }

            #[inline]
            fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                #(<#sol_types as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.#field_names, out);)*
            }

            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    }
}
