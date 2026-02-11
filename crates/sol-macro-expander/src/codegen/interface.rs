//! SolInterface enum generation.

use super::quote_byte_array;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Specifies whether the interface is for function calls or errors.
#[derive(Debug, Clone, Copy, Default)]
pub enum SolInterfaceKind {
    /// Function calls (uses `SolCall` trait)
    #[default]
    Call,
    /// Custom errors (uses `SolError` trait)
    Error,
}

impl SolInterfaceKind {
    fn trait_path(&self) -> TokenStream {
        match self {
            Self::Call => quote! { alloy_sol_types::SolCall },
            Self::Error => quote! { alloy_sol_types::SolError },
        }
    }
}

/// Code generator for SolInterface enums with pre-computed selectors.
///
/// Generates a complete enum implementing `SolInterface` with binary-search dispatch.
/// Selectors are sorted at codegen time for O(log n) lookup performance.
#[derive(Debug)]
pub struct InterfaceCodegen {
    name: Ident,
    variants: Vec<Ident>,
    types: Vec<Ident>,
    selectors: Vec<[u8; 4]>,
    signatures: Vec<String>,
    min_data_len: usize,
    kind: SolInterfaceKind,
}

impl InterfaceCodegen {
    /// Creates a new interface codegen with pre-computed selectors and signatures.
    ///
    /// Selectors are sorted at codegen time for O(log n) binary-search dispatch.
    ///
    /// # Panics
    ///
    /// Panics if `variants`, `types`, `selectors`, or `signatures` have different lengths.
    pub fn precomputed(
        name: Ident,
        variants: Vec<Ident>,
        types: Vec<Ident>,
        selectors: Vec<[u8; 4]>,
        signatures: Vec<String>,
        min_data_len: usize,
        kind: SolInterfaceKind,
    ) -> Self {
        assert_eq!(variants.len(), types.len(), "variants and types must have the same length");
        assert_eq!(
            variants.len(),
            selectors.len(),
            "variants and selectors must have the same length"
        );
        assert_eq!(
            variants.len(),
            signatures.len(),
            "variants and signatures must have the same length"
        );
        Self { name, variants, types, selectors, signatures, min_data_len, kind }
    }

    /// Generates a complete SolInterface enum with all implementations.
    ///
    /// NOTE: The generated code assumes `alloy_sol_types` is in scope.
    pub fn expand(self) -> TokenStream {
        let Self { name, variants, types, selectors, signatures, min_data_len, kind } = self;

        let trait_path = kind.trait_path();
        let name_str = name.to_string();
        let count = variants.len();

        // Sort by selector for binary search
        let mut indexed: Vec<_> = selectors
            .iter()
            .zip(variants.iter())
            .zip(types.iter())
            .zip(signatures.iter())
            .map(|(((sel, var), ty), sig)| (*sel, var, ty, sig))
            .collect();
        indexed.sort_by_key(|(sel, _, _, _)| *sel);

        let sorted_selectors: Vec<_> =
            indexed.iter().map(|(sel, _, _, _)| quote_byte_array(sel)).collect();
        let sorted_variants: Vec<_> = indexed.iter().map(|(_, v, _, _)| *v).collect();
        let sorted_types: Vec<_> = indexed.iter().map(|(_, _, t, _)| *t).collect();
        let sorted_signatures: Vec<_> = indexed.iter().map(|(_, _, _, s)| *s).collect();

        let enum_def = quote! {
            /// Container for all variants.
            #[derive(Clone, Debug, PartialEq, Eq)]
            pub enum #name {
                #(
                    #[allow(missing_docs)]
                    #variants(#types),
                )*
            }
        };

        let from_impls = quote! {
            #(
                #[automatically_derived]
                impl ::core::convert::From<#types> for #name {
                    #[inline]
                    fn from(value: #types) -> Self {
                        Self::#variants(value)
                    }
                }

                #[automatically_derived]
                impl ::core::convert::TryFrom<#name> for #types {
                    type Error = #name;

                    #[inline]
                    fn try_from(value: #name) -> ::core::result::Result<Self, #name> {
                        match value {
                            #name::#variants(inner) => ::core::result::Result::Ok(inner),
                            _ => ::core::result::Result::Err(value),
                        }
                    }
                }
            )*
        };

        let inherent_impl = quote! {
            impl #name {
                /// All the selectors of this enum, sorted for binary search.
                pub const SELECTORS: &'static [[u8; 4]] = &[#(#sorted_selectors),*];

                /// The signatures in the same order as SELECTORS.
                pub const SIGNATURES: &'static [&'static str] = &[#(#sorted_signatures),*];

                /// Returns the signature for the given selector, if known.
                #[inline]
                pub fn signature_by_selector(selector: [u8; 4]) -> ::core::option::Option<&'static str> {
                    match Self::SELECTORS.binary_search(&selector) {
                        ::core::result::Result::Ok(idx) => ::core::option::Option::Some(Self::SIGNATURES[idx]),
                        ::core::result::Result::Err(_) => ::core::option::Option::None,
                    }
                }

                /// Returns the Solidity name for the given selector, if known.
                #[inline]
                pub fn name_by_selector(selector: [u8; 4]) -> ::core::option::Option<&'static str> {
                    let sig = Self::signature_by_selector(selector)?;
                    sig.split_once('(').map(|(name, _)| name)
                }
            }
        };

        let interface_impl = quote! {
            #[automatically_derived]
            impl alloy_sol_types::SolInterface for #name {
                const NAME: &'static str = #name_str;
                const MIN_DATA_LENGTH: usize = #min_data_len;
                const COUNT: usize = #count;

                #[inline]
                fn selector(&self) -> [u8; 4] {
                    match self {
                        #(Self::#variants(_) => <#types as #trait_path>::SELECTOR,)*
                    }
                }

                #[inline]
                fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
                    Self::SELECTORS.get(i).copied()
                }

                #[inline]
                fn valid_selector(selector: [u8; 4]) -> bool {
                    Self::SELECTORS.binary_search(&selector).is_ok()
                }

                #[inline]
                #[allow(non_snake_case)]
                fn abi_decode_raw(
                    selector: [u8; 4],
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self> {
                    static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<#name>] = &[
                        #({
                            fn #sorted_variants(data: &[u8]) -> alloy_sol_types::Result<#name> {
                                <#sorted_types as #trait_path>::abi_decode_raw(data)
                                    .map(#name::#sorted_variants)
                            }
                            #sorted_variants
                        }),*
                    ];

                    let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                        return Err(alloy_sol_types::Error::unknown_selector(
                            <Self as alloy_sol_types::SolInterface>::NAME,
                            selector,
                        ));
                    };
                    DECODE_SHIMS[idx](data)
                }

                #[inline]
                #[allow(non_snake_case)]
                fn abi_decode_raw_validate(
                    selector: [u8; 4],
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self> {
                    static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<#name>] = &[
                        #({
                            fn #sorted_variants(data: &[u8]) -> alloy_sol_types::Result<#name> {
                                <#sorted_types as #trait_path>::abi_decode_raw_validate(data)
                                    .map(#name::#sorted_variants)
                            }
                            #sorted_variants
                        }),*
                    ];

                    let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                        return Err(alloy_sol_types::Error::unknown_selector(
                            <Self as alloy_sol_types::SolInterface>::NAME,
                            selector,
                        ));
                    };
                    DECODE_VALIDATE_SHIMS[idx](data)
                }

                #[inline]
                fn abi_encoded_size(&self) -> usize {
                    match self {
                        #(Self::#variants(inner) => <#types as #trait_path>::abi_encoded_size(inner),)*
                    }
                }

                #[inline]
                fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                    match self {
                        #(Self::#variants(inner) => <#types as #trait_path>::abi_encode_raw(inner, out),)*
                    }
                }
            }
        };

        quote! {
            #enum_def
            #inherent_impl
            #interface_impl
            #from_impls
        }
    }
}
