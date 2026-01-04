//! Context-free code generation utilities.
//!
//! These functions generate common Solidity-related Rust code without
//! requiring the full `ExpCtxt` context, making them reusable by external crates
//! that want to generate Alloy-compatible types from Rust syntax.
//!
//! The `sol!` macro uses the `expand/` modules which have access to the full
//! parsing context. These utilities are designed for the simpler case where
//! you already have the types resolved.

mod call;
mod r#enum;
mod error;
mod event;
mod interface;
mod r#struct;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

// Re-export all public types
pub use call::{CallCodegen, ReturnInfo};
pub use r#enum::EnumCodegen;
pub use error::ErrorCodegen;
pub use event::{EventCodegen, EventFieldInfo};
pub use interface::{InterfaceCodegen, SolInterfaceKind};
pub use r#struct::{Eip712Options, StructCodegen};

// ============================================================================
// Helper functions
// ============================================================================

/// Quotes a fixed-size byte array as a TokenStream.
fn quote_byte_array<const N: usize>(bytes: &[u8; N]) -> TokenStream {
    let elems = bytes.iter().map(|b| quote! { #b });
    quote! { [#(#elems),*] }
}

/// Quotes a tuple type from a slice of types, returning `()` for empty.
fn quote_tuple_type(types: &[TokenStream]) -> TokenStream {
    if types.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#types,)*) }
    }
}

// ============================================================================
// Struct layout and From/Into generation
// ============================================================================

/// Struct layout pattern for From/Into generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructLayout {
    /// Unit struct: `struct Foo;`
    Unit,
    /// Tuple struct: `struct Foo(T);`
    Tuple,
    /// Named struct: `struct Foo { field: T }`
    Named,
}

/// Generates type aliases, type assertion, and From/Into tuple conversions.
///
/// Generates:
/// - Type aliases: `UnderlyingSolTuple<'a>` and `UnderlyingRustTuple<'a>`
/// - Type assertion function (cfg(test) only)
/// - From/Into conversions between struct and underlying rust tuple
///
/// The `layout` parameter determines how fields are accessed in the generated code.
pub fn gen_from_into_tuple(
    struct_name: &Ident,
    field_names: &[Ident],
    sol_types: &[TokenStream],
    rust_types: &[TokenStream],
    layout: StructLayout,
) -> TokenStream {
    let sol_tuple = quote_tuple_type(sol_types);
    let rust_tuple = quote_tuple_type(rust_types);

    let from_into_impls = match layout {
        StructLayout::Unit => {
            quote! {
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<#struct_name> for UnderlyingRustTuple<'_> {
                    fn from(value: #struct_name) -> Self { () }
                }

                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for #struct_name {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self { Self }
                }
            }
        }
        StructLayout::Tuple => {
            quote! {
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<#struct_name> for UnderlyingRustTuple<'_> {
                    fn from(value: #struct_name) -> Self {
                        (value.0,)
                    }
                }

                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for #struct_name {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self(tuple.0)
                    }
                }
            }
        }
        StructLayout::Named => {
            let indices: Vec<_> = (0..field_names.len()).map(syn::Index::from).collect();
            quote! {
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<#struct_name> for UnderlyingRustTuple<'_> {
                    fn from(value: #struct_name) -> Self {
                        (#(value.#field_names,)*)
                    }
                }

                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for #struct_name {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { #(#field_names: tuple.#indices),* }
                    }
                }
            }
        }
    };

    quote! {
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = #sol_tuple;
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = #rust_tuple;

        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }

        #from_into_impls
    }
}

/// Generates the tokenize() implementation body.
///
/// Returns a TokenStream that produces a tuple of tokenized fields.
pub fn expand_tokenize_simple(field_names: &[Ident], sol_types: &[TokenStream]) -> TokenStream {
    if field_names.is_empty() {
        quote! { () }
    } else {
        let tokenize_fields = field_names.iter().zip(sol_types.iter()).map(|(name, sol_ty)| {
            quote! { <#sol_ty as alloy_sol_types::SolType>::tokenize(&self.#name) }
        });
        quote! { (#(#tokenize_fields,)*) }
    }
}
