//! SolCall trait generation.

use super::quote_byte_array;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Describes the return value structure for computing derived TokenStreams.
#[derive(Debug, Clone)]
pub enum ReturnInfo {
    /// No return value - empty struct, but still uses the return struct name for API compat
    Empty {
        /// The return struct name (e.g., `fooReturn`)
        return_name: Ident,
    },
    /// Single return value
    Single {
        /// The sol_data type (e.g., `sol_data::Uint<256>`)
        sol_type: TokenStream,
        /// The Rust type for the return (e.g., `U256`)
        rust_type: TokenStream,
        /// Field name to extract (e.g., `_0` or a named field)
        field_name: Ident,
        /// The return struct name for type annotation
        return_name: Ident,
    },
    /// Multiple return values - delegate to return struct
    Multiple {
        /// The return struct name (e.g., `fooReturn`)
        return_name: Ident,
    },
}

/// Data for generating SolCall trait implementation.
#[derive(Debug)]
pub struct CallCodegen {
    /// Parameter tuple type (e.g., `(alloy_sol_types::sol_data::Uint<256>,)`)
    call_tuple: TokenStream,
    /// Return tuple type
    return_tuple: TokenStream,
    /// Tokenize implementation body
    tokenize_impl: TokenStream,
    /// Return value structure info
    return_info: ReturnInfo,
}

impl CallCodegen {
    /// Creates a new call codegen.
    pub fn new(
        call_tuple: TokenStream,
        return_tuple: TokenStream,
        tokenize_impl: TokenStream,
        return_info: ReturnInfo,
    ) -> Self {
        Self { call_tuple, return_tuple, tokenize_impl, return_info }
    }

    /// Returns the return type for `SolCall::Return`.
    pub fn return_type(&self) -> TokenStream {
        match &self.return_info {
            ReturnInfo::Empty { return_name } => quote! { #return_name },
            ReturnInfo::Single { rust_type, .. } => rust_type.clone(),
            ReturnInfo::Multiple { return_name } => quote! { #return_name },
        }
    }

    /// Returns the tokenize_returns implementation.
    pub fn tokenize_returns(&self, crate_path: &TokenStream) -> TokenStream {
        match &self.return_info {
            ReturnInfo::Empty { return_name } | ReturnInfo::Multiple { return_name } => {
                quote! { #return_name::_tokenize(ret) }
            }
            ReturnInfo::Single { sol_type, .. } => {
                quote! { (<#sol_type as #crate_path::SolType>::tokenize(ret),) }
            }
        }
    }

    /// Returns the decode_returns implementation.
    pub fn decode_returns(&self, crate_path: &TokenStream) -> TokenStream {
        let decode_seq =
            quote!(<Self::ReturnTuple<'_> as #crate_path::SolType>::abi_decode_sequence(data));
        match &self.return_info {
            ReturnInfo::Empty { .. } => quote! { #decode_seq.map(Into::into) },
            ReturnInfo::Single { field_name, return_name, .. } => {
                quote! {
                    #decode_seq.map(|r| {
                        let r: #return_name = r.into();
                        r.#field_name
                    })
                }
            }
            ReturnInfo::Multiple { .. } => quote! { #decode_seq.map(Into::into) },
        }
    }

    /// Returns the decode_returns_validate implementation.
    pub fn decode_returns_validate(&self, crate_path: &TokenStream) -> TokenStream {
        let decode_seq = quote!(<Self::ReturnTuple<'_> as #crate_path::SolType>::abi_decode_sequence_validate(data));
        match &self.return_info {
            ReturnInfo::Empty { .. } => quote! { #decode_seq.map(Into::into) },
            ReturnInfo::Single { field_name, return_name, .. } => {
                quote! {
                    #decode_seq.map(|r| {
                        let r: #return_name = r.into();
                        r.#field_name
                    })
                }
            }
            ReturnInfo::Multiple { .. } => quote! { #decode_seq.map(Into::into) },
        }
    }
}

impl CallCodegen {
    /// Generates the `SolCall` trait implementation.
    ///
    /// NOTE: the `crate_path` should be a path to `alloy_sol_types`.
    pub fn expand(self, name: &Ident, signature: &str, crate_path: &TokenStream) -> TokenStream {
        let call_tuple = &self.call_tuple;
        let return_tuple = &self.return_tuple;
        let tokenize_impl = &self.tokenize_impl;

        // Computed via methods
        let return_type = self.return_type();
        let tokenize_returns = self.tokenize_returns(crate_path);
        let decode_returns = self.decode_returns(crate_path);
        let decode_returns_validate = self.decode_returns_validate(crate_path);

        let selector = crate::utils::calc_selector(signature);
        let selector_tokens = quote_byte_array(&selector);

        quote! {
            #[automatically_derived]
            impl #crate_path::SolCall for #name {
                type Parameters<'a> = #call_tuple;
                type Token<'a> = <Self::Parameters<'a> as #crate_path::SolType>::Token<'a>;

                type Return = #return_type;

                type ReturnTuple<'a> = #return_tuple;
                type ReturnToken<'a> = <Self::ReturnTuple<'a> as #crate_path::SolType>::Token<'a>;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector_tokens;

                #[inline]
                fn new<'a>(tuple: <Self::Parameters<'a> as #crate_path::SolType>::RustType) -> Self {
                    tuple.into()
                }

                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    #tokenize_impl
                }

                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    #tokenize_returns
                }

                #[inline]
                fn abi_decode_returns(data: &[u8]) -> #crate_path::Result<Self::Return> {
                    #decode_returns
                }

                #[inline]
                fn abi_decode_returns_validate(data: &[u8]) -> #crate_path::Result<Self::Return> {
                    #decode_returns_validate
                }
            }
        }
    }
}
