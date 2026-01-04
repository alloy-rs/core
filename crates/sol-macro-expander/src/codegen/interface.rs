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

/// Pre-computed selector and signature data for optimized binary-search lookup.
#[derive(Debug)]
struct PrecomputedData {
    /// 4-byte selectors in same order as variants.
    selectors: Vec<[u8; 4]>,
    /// Function/error signatures in same order as variants.
    signatures: Vec<String>,
    /// Minimum ABI-encoded data length across all variants.
    min_data_len: usize,
}

/// Code generator for SolInterface enums.
///
/// Supports two modes:
/// - **Precomputed**: Selectors/signatures are known at codegen time, enabling sorted binary-search
///   lookups.
/// - **Deferred**: Selectors/signatures are derived from variant types at compile time, using
///   linear if-chain lookups. Required when struct parameters are involved.
#[derive(Debug)]
pub struct InterfaceCodegen {
    name: Ident,
    variants: Vec<Ident>,
    types: Vec<Ident>,
    kind: SolInterfaceKind,
    precomputed: Option<PrecomputedData>,
}

impl InterfaceCodegen {
    /// Creates a codegen with pre-computed selectors and signatures.
    ///
    /// This mode sorts selectors for binary-search lookup, providing O(log n) performance.
    /// Use when selectors can be computed at macro expansion time.
    pub fn precomputed(
        name: Ident,
        variants: Vec<Ident>,
        types: Vec<Ident>,
        selectors: Vec<[u8; 4]>,
        signatures: Vec<String>,
        min_data_len: usize,
        kind: SolInterfaceKind,
    ) -> Self {
        Self {
            name,
            variants,
            types,
            kind,
            precomputed: Some(PrecomputedData { selectors, signatures, min_data_len }),
        }
    }

    /// Creates a codegen with deferred selector/signature resolution.
    ///
    /// Selectors and signatures are derived from variant types' `SELECTOR` and `SIGNATURE`
    /// constants at compile time. Uses O(n) if-chain lookup.
    ///
    /// Required when variant types have struct parameters, since their ABI signatures
    /// (which replace structs with tuple representations) are only computable after
    /// the struct types have been fully generated.
    pub fn deferred(
        name: Ident,
        variants: Vec<Ident>,
        types: Vec<Ident>,
        kind: SolInterfaceKind,
    ) -> Self {
        Self { name, variants, types, kind, precomputed: None }
    }

    /// Generates a complete SolInterface enum with all implementations.
    pub fn expand(self) -> TokenStream {
        let Self { name, variants, types, kind, precomputed } = self;

        let trait_path = kind.trait_path();
        let name_str = name.to_string();
        let count = variants.len();

        // Common: enum definition and From/TryFrom impls
        let enum_def = Self::expand_enum(&name, &variants, &types);
        let from_impls = Self::expand_from_impls(&name, &variants, &types);

        // Mode-specific: inherent methods and SolInterface impl
        let (inherent_impl, interface_impl) = match precomputed {
            Some(data) => Self::expand_precomputed(
                &name,
                &name_str,
                &variants,
                &types,
                &trait_path,
                count,
                data,
            ),
            None => Self::expand_deferred(&name, &name_str, &variants, &types, &trait_path, count),
        };

        quote! {
            #enum_def
            #inherent_impl
            #interface_impl
            #from_impls
        }
    }

    fn expand_enum(name: &Ident, variants: &[Ident], types: &[Ident]) -> TokenStream {
        quote! {
            /// Container for all variants.
            #[derive(Clone, Debug, PartialEq, Eq)]
            pub enum #name {
                #(
                    #[allow(missing_docs)]
                    #variants(#types),
                )*
            }
        }
    }

    fn expand_from_impls(name: &Ident, variants: &[Ident], types: &[Ident]) -> TokenStream {
        quote! {
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
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn expand_precomputed(
        name: &Ident,
        name_str: &str,
        variants: &[Ident],
        types: &[Ident],
        trait_path: &TokenStream,
        count: usize,
        data: PrecomputedData,
    ) -> (TokenStream, TokenStream) {
        let PrecomputedData { selectors, signatures, min_data_len } = data;

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

        let inherent_impl = quote! {
            impl #name {
                /// All the selectors of this enum, sorted for binary search.
                pub const SELECTORS: &'static [[u8; 4]] = &[#(#sorted_selectors),*];

                /// The variant names in the same order as SELECTORS.
                pub const VARIANT_NAMES: &'static [&'static str] = &[#(::core::stringify!(#sorted_variants)),*];

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

                /// Returns the enum variant name for the given selector, if known.
                #[inline]
                pub fn name_by_selector(selector: [u8; 4]) -> ::core::option::Option<&'static str> {
                    match Self::SELECTORS.binary_search(&selector) {
                        ::core::result::Result::Ok(idx) => ::core::option::Option::Some(Self::VARIANT_NAMES[idx]),
                        ::core::result::Result::Err(_) => ::core::option::Option::None,
                    }
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

        (inherent_impl, interface_impl)
    }

    fn expand_deferred(
        name: &Ident,
        name_str: &str,
        variants: &[Ident],
        types: &[Ident],
        trait_path: &TokenStream,
        count: usize,
    ) -> (TokenStream, TokenStream) {
        let indices: Vec<_> = (0..variants.len()).collect();

        let inherent_impl = quote! {
            impl #name {
                /// All the selectors of this enum.
                /// Note: Order matches variant declaration order, not sorted.
                pub const SELECTORS: &'static [[u8; 4]] = &[
                    #(<#types as #trait_path>::SELECTOR),*
                ];

                /// The variant names in the same order as SELECTORS.
                pub const VARIANT_NAMES: &'static [&'static str] = &[
                    #(::core::stringify!(#variants)),*
                ];

                /// The signatures in the same order as SELECTORS.
                pub const SIGNATURES: &'static [&'static str] = &[
                    #(<#types as #trait_path>::SIGNATURE),*
                ];

                /// Returns the signature for the given selector, if known.
                #[inline]
                pub fn signature_by_selector(selector: [u8; 4]) -> ::core::option::Option<&'static str> {
                    #(
                        if selector == <#types as #trait_path>::SELECTOR {
                            return ::core::option::Option::Some(<#types as #trait_path>::SIGNATURE);
                        }
                    )*
                    ::core::option::Option::None
                }

                /// Returns the enum variant name for the given selector, if known.
                #[inline]
                pub fn name_by_selector(selector: [u8; 4]) -> ::core::option::Option<&'static str> {
                    #(
                        if selector == <#types as #trait_path>::SELECTOR {
                            return ::core::option::Option::Some(::core::stringify!(#variants));
                        }
                    )*
                    ::core::option::Option::None
                }
            }
        };

        let interface_impl = quote! {
            #[automatically_derived]
            impl alloy_sol_types::SolInterface for #name {
                const NAME: &'static str = #name_str;
                const MIN_DATA_LENGTH: usize = 0; // Conservative; could compute from types
                const COUNT: usize = #count;

                #[inline]
                fn selector(&self) -> [u8; 4] {
                    match self {
                        #(Self::#variants(_) => <#types as #trait_path>::SELECTOR,)*
                    }
                }

                #[inline]
                fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
                    match i {
                        #(#indices => ::core::option::Option::Some(<#types as #trait_path>::SELECTOR),)*
                        _ => ::core::option::Option::None,
                    }
                }

                #[inline]
                fn valid_selector(selector: [u8; 4]) -> bool {
                    #(
                        if selector == <#types as #trait_path>::SELECTOR {
                            return true;
                        }
                    )*
                    false
                }

                #[inline]
                #[allow(non_snake_case)]
                fn abi_decode_raw(
                    selector: [u8; 4],
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self> {
                    #(
                        if selector == <#types as #trait_path>::SELECTOR {
                            return <#types as #trait_path>::abi_decode_raw(data)
                                .map(#name::#variants);
                        }
                    )*
                    Err(alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ))
                }

                #[inline]
                #[allow(non_snake_case)]
                fn abi_decode_raw_validate(
                    selector: [u8; 4],
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self> {
                    #(
                        if selector == <#types as #trait_path>::SELECTOR {
                            return <#types as #trait_path>::abi_decode_raw_validate(data)
                                .map(#name::#variants);
                        }
                    )*
                    Err(alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ))
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

        (inherent_impl, interface_impl)
    }
}
