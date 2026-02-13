//! Context-free code generation utilities.
//!
//! # ⚠️ STABILITY
//!
//! This module is entirely unstable and subject to breaking changes without notice.
//! It is exposed for use by other alloy crates and proc-macro authors, but no semver
//! guarantees are made for any items within this module.

mod call;
mod contract;
mod r#enum;
mod error;
mod event;
mod interface;
mod r#struct;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

// Re-export all public types
pub use call::{CallCodegen, ReturnInfo};
pub use contract::{
    CallLayout, ConstructorInfo, ContractCodegen, ContractEventInfo, ContractFunctionInfo,
    is_reserved_method_name,
};
pub use r#enum::EnumCodegen;
pub use error::ErrorCodegen;
pub use event::{EventCodegen, EventFieldInfo};
pub use interface::{InterfaceCodegen, SolInterfaceKind};
pub use r#struct::{Eip712Options, StructCodegen};

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
///
/// NOTE: The generated code assumes `alloy_sol_types` is in scope (typically via
/// `use ::alloy_sol_types as alloy_sol_types;` in the outer const block).
pub fn gen_from_into_tuple(
    struct_name: &Ident,
    field_names: &[Ident],
    sol_types: &[TokenStream],
    rust_types: &[TokenStream],
    layout: StructLayout,
) -> TokenStream {
    debug_assert_eq!(sol_types.len(), rust_types.len());
    debug_assert_eq!(sol_types.len(), field_names.len());

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
                    fn from(_: UnderlyingRustTuple<'_>) -> Self { Self }
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

/// Generates the tokenize implementation body.
///
/// Returns a TokenStream that produces a tuple of tokenized fields.
///
/// NOTE: The generated code assumes `alloy_sol_types` is in scope.
pub fn gen_tokenize(
    field_names: &[Ident],
    sol_types: &[TokenStream],
    is_tuple_struct: bool,
) -> TokenStream {
    debug_assert_eq!(field_names.len(), sol_types.len());
    debug_assert!(!is_tuple_struct || field_names.len() == 1,);

    if field_names.is_empty() {
        quote! { () }
    } else if is_tuple_struct {
        // Tuple struct: access via self.0
        let ty = &sol_types[0];
        quote! { (<#ty as alloy_sol_types::SolType>::tokenize(&self.0),) }
    } else {
        // Named struct: access via self.field_name
        let tokenize_fields = field_names.iter().zip(sol_types.iter()).map(|(name, sol_ty)| {
            quote! { <#sol_ty as alloy_sol_types::SolType>::tokenize(&self.#name) }
        });
        quote! { (#(#tokenize_fields,)*) }
    }
}
