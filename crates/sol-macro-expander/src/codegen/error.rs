//! SolError trait generation.

use super::{StructLayout, gen_from_into_tuple, quote_byte_array};
use crate::utils::calc_selector;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Data for generating SolError trait implementation.
#[derive(Debug)]
pub struct ErrorCodegen {
    /// Parameter names
    param_names: Vec<Ident>,
    /// Solidity types (sol_data::*)
    sol_types: Vec<TokenStream>,
    /// Rust types
    rust_types: Vec<TokenStream>,
    /// Whether to use tuple struct pattern (single unnamed param)
    is_tuple_struct: bool,
}

impl ErrorCodegen {
    /// Creates a new error codegen.
    pub fn new(
        param_names: Vec<Ident>,
        sol_types: Vec<TokenStream>,
        rust_types: Vec<TokenStream>,
        is_tuple_struct: bool,
    ) -> Self {
        Self { param_names, sol_types, rust_types, is_tuple_struct }
    }

    /// Generates the `SolError` trait implementation.
    ///
    /// NOTE: the `crate_path` should be a path to `alloy_sol_types`.
    pub fn expand(self, name: &Ident, signature: &str, crate_path: &TokenStream) -> TokenStream {
        let Self { param_names, sol_types, rust_types, is_tuple_struct } = self;

        let layout = match (param_names.is_empty(), is_tuple_struct) {
            (true, _) => StructLayout::Unit,
            (_, true) => StructLayout::Tuple,
            _ => StructLayout::Named,
        };
        let tupl_impl =
            gen_from_into_tuple(name, &param_names, &sol_types, &rust_types, layout, crate_path);
        let tokenize_impl = gen_tokenize(&param_names, &sol_types, is_tuple_struct, crate_path);
        let sol_error_impl = gen_sol_error_trait(name, signature, &tokenize_impl, crate_path);

        quote! {
            #tupl_impl

            #sol_error_impl
        }
    }
}

/// Generates the tokenize implementation body.
fn gen_tokenize(
    param_names: &[Ident],
    sol_types: &[TokenStream],
    is_tuple_struct: bool,
    crate_path: &TokenStream,
) -> TokenStream {
    if param_names.is_empty() {
        quote! { () }
    } else if is_tuple_struct {
        // Tuple struct: access via self.0
        let ty = &sol_types[0];
        quote! { (<#ty as #crate_path::SolType>::tokenize(&self.0),) }
    } else {
        // Named struct: access via self.field_name
        let tokenize_fields = param_names.iter().zip(sol_types.iter()).map(|(name, sol_ty)| {
            quote! { <#sol_ty as #crate_path::SolType>::tokenize(&self.#name) }
        });
        quote! { (#(#tokenize_fields,)*) }
    }
}

/// Generates the SolError trait implementation.
fn gen_sol_error_trait(
    name: &Ident,
    signature: &str,
    tokenize_impl: &TokenStream,
    crate_path: &TokenStream,
) -> TokenStream {
    let selector = quote_byte_array(&calc_selector(signature));

    quote! {
        #[automatically_derived]
        impl #crate_path::SolError for #name {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as #crate_path::SolType>::Token<'a>;

            const SIGNATURE: &'static str = #signature;
            const SELECTOR: [u8; 4] = #selector;

            #[inline]
            fn new<'a>(tuple: <Self::Parameters<'a> as #crate_path::SolType>::RustType) -> Self {
                tuple.into()
            }

            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                #tokenize_impl
            }

            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> #crate_path::Result<Self> {
                <Self::Parameters<'_> as #crate_path::SolType>::abi_decode_sequence_validate(data).map(Self::new)
            }
        }
    }
}
