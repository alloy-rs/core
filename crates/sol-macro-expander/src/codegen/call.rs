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
    fn tokenize_returns(&self) -> TokenStream {
        match &self.return_info {
            ReturnInfo::Empty { return_name } | ReturnInfo::Multiple { return_name } => {
                quote! { #return_name::_tokenize(ret) }
            }
            ReturnInfo::Single { sol_type, .. } => {
                quote! { (<#sol_type as alloy_sol_types::SolType>::tokenize(ret),) }
            }
        }
    }

    /// Returns the decode_returns implementation.
    fn decode_returns(&self) -> TokenStream {
        let decode_seq =
            quote!(<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data));
        self.decode_returns_with(decode_seq)
    }

    /// Returns the decode_returns_validate implementation.
    fn decode_returns_validate(&self) -> TokenStream {
        let decode_seq = quote!(
            <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
        );
        self.decode_returns_with(decode_seq)
    }

    /// Shared decode logic for both `decode_returns` and `decode_returns_validate`.
    fn decode_returns_with(&self, decode_seq: TokenStream) -> TokenStream {
        match &self.return_info {
            ReturnInfo::Empty { .. } | ReturnInfo::Multiple { .. } => {
                quote! { #decode_seq.map(Into::into) }
            }
            ReturnInfo::Single { field_name, return_name, .. } => {
                quote! {
                    #decode_seq.map(|r| {
                        let r: #return_name = r.into();
                        r.#field_name
                    })
                }
            }
        }
    }

    /// Generates the `SolCall` trait implementation.
    ///
    /// NOTE: The generated code assumes `alloy_sol_types` is in scope.
    pub fn expand(self, name: &Ident, signature: &str) -> TokenStream {
        self.expand_with_selector(name, signature, crate::utils::calc_selector(signature))
    }

    /// Generates the `SolCall` trait implementation with a precomputed selector.
    ///
    /// NOTE: The generated code assumes `alloy_sol_types` is in scope.
    pub fn expand_with_selector(
        self,
        name: &Ident,
        signature: &str,
        selector: [u8; 4],
    ) -> TokenStream {
        let call_tuple = &self.call_tuple;
        let return_tuple = &self.return_tuple;
        let tokenize_impl = &self.tokenize_impl;

        // Computed via methods
        let return_type = self.return_type();
        let tokenize_returns = self.tokenize_returns();
        let decode_returns = self.decode_returns();
        let decode_returns_validate = self.decode_returns_validate();

        let selector_tokens = quote_byte_array(&selector);

        quote! {
            #[automatically_derived]
            impl alloy_sol_types::SolCall for #name {
                type Parameters<'a> = #call_tuple;
                type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;

                type Return = #return_type;

                type ReturnTuple<'a> = #return_tuple;
                type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector_tokens;

                #[inline]
                fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
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
                fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                    #decode_returns
                }

                #[inline]
                fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                    #decode_returns_validate
                }
            }
        }
    }
}
