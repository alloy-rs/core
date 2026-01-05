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
    ///
    /// NOTE: the `crate_path` should be a path to `alloy_sol_types`.
    pub fn expand(self, name: &Ident, crate_path: &TokenStream) -> TokenStream {
        let Self { field_names, rust_types, sol_types, eip712 } = self;

        let name_s = name.to_string();
        // Strip raw identifier prefix for SOL_NAME
        let name_s = name_s.strip_prefix("r#").unwrap_or(&name_s);
        let root = &eip712.root;

        let layout = if field_names.is_empty() { StructLayout::Unit } else { StructLayout::Named };
        let tupl_impl =
            gen_from_into_tuple(name, &field_names, &sol_types, &rust_types, layout, crate_path);

        // Build EIP-712 functions
        let has_components = eip712.components_impl.is_some();
        let components_impl = eip712.components_impl.unwrap_or_else(|| {
            quote! { #crate_path::private::Vec::new() }
        });

        // Infer encode_type behavior:
        // - If custom implementation provided, use it
        // - Else if components exist, rely on trait default (don't emit override)
        // - Else emit override returning root_type (optimization for no-deps case)
        let encode_type_fn = match eip712.encode_type_impl {
            Some(tokens) => Some(quote! {
                #[inline]
                fn eip712_encode_type() -> #crate_path::private::Cow<'static, str> {
                    #tokens
                }
            }),
            None if has_components => None, // rely on trait default
            None => Some(quote! {
                #[inline]
                fn eip712_encode_type() -> #crate_path::private::Cow<'static, str> {
                    <Self as #crate_path::SolStruct>::eip712_root_type()
                }
            }),
        };

        let eip712_fns = quote! {
            #[inline]
            fn eip712_root_type() -> #crate_path::private::Cow<'static, str> {
                #crate_path::private::Cow::Borrowed(#root)
            }

            #[inline]
            fn eip712_components() -> #crate_path::private::Vec<#crate_path::private::Cow<'static, str>> {
                #components_impl
            }

            #encode_type_fn
        };

        let traits =
            gen_sol_struct_traits(name, name_s, &field_names, &sol_types, eip712_fns, crate_path);

        quote! {
            #tupl_impl

            #traits
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
    crate_path: &TokenStream,
) -> TokenStream {
    let tokenize_impl = expand_tokenize_simple(field_names, sol_types, crate_path);

    let encode_data_impl = if field_names.is_empty() {
        quote! { #crate_path::private::Vec::new() }
    } else if field_names.len() == 1 {
        let name = &field_names[0];
        let ty = &sol_types[0];
        quote! { <#ty as #crate_path::SolType>::eip712_data_word(&self.#name).0.to_vec() }
    } else {
        quote! {
            [#(
                <#sol_types as #crate_path::SolType>::eip712_data_word(&self.#field_names).0,
            )*].concat()
        }
    };

    quote! {
        #[automatically_derived]
        impl #crate_path::SolValue for #struct_name {
            type SolType = Self;
        }

        #[automatically_derived]
        impl #crate_path::private::SolTypeValue<Self> for #struct_name {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as #crate_path::SolType>::Token<'_> {
                #tokenize_impl
            }

            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as #crate_path::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as #crate_path::SolType>::abi_encoded_size(&tuple)
            }

            #[inline]
            fn stv_eip712_data_word(&self) -> #crate_path::Word {
                <Self as #crate_path::SolStruct>::eip712_hash_struct(self)
            }

            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut #crate_path::private::Vec<u8>) {
                let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as #crate_path::SolType>::abi_encode_packed_to(&tuple, out)
            }

            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as #crate_path::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as #crate_path::SolType>::abi_packed_encoded_size(&tuple)
            }
        }

        #[automatically_derived]
        impl #crate_path::SolType for #struct_name {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as #crate_path::SolType>::Token<'a>;

            const SOL_NAME: &'static str = <Self as #crate_path::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as #crate_path::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as #crate_path::SolType>::PACKED_ENCODED_SIZE;

            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as #crate_path::SolType>::valid_token(token)
            }

            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as #crate_path::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }

        #[automatically_derived]
        impl #crate_path::SolStruct for #struct_name {
            const NAME: &'static str = #name_s;

            #eip712_fns

            #[inline]
            fn eip712_encode_data(&self) -> #crate_path::private::Vec<u8> {
                #encode_data_impl
            }
        }

        #[automatically_derived]
        impl #crate_path::EventTopic for #struct_name {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                #(+ <#sol_types as #crate_path::EventTopic>::topic_preimage_length(&rust.#field_names))*
            }

            #[inline]
            fn encode_topic_preimage(rust: &Self::RustType, out: &mut #crate_path::private::Vec<u8>) {
                out.reserve(<Self as #crate_path::EventTopic>::topic_preimage_length(rust));
                #(<#sol_types as #crate_path::EventTopic>::encode_topic_preimage(&rust.#field_names, out);)*
            }

            #[inline]
            fn encode_topic(rust: &Self::RustType) -> #crate_path::abi::token::WordToken {
                let mut out = #crate_path::private::Vec::new();
                <Self as #crate_path::EventTopic>::encode_topic_preimage(rust, &mut out);
                #crate_path::abi::token::WordToken(#crate_path::private::keccak256(out))
            }
        }
    }
}
