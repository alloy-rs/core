//! [`Type`] expansion.

use super::{ExpCtxt, ExternCrates};
use ast::{Item, Parameters, Spanned, Type, TypeArray};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote_spanned, ToTokens};
use std::{fmt, num::NonZeroU16};

/// Expands a single [`Type`] recursively to its `alloy_sol_types::sol_data`
/// equivalent.
pub(crate) fn expand_type(ty: &Type, crates: &ExternCrates) -> TokenStream {
    let mut tokens = TokenStream::new();
    rec_expand_type(ty, crates, &mut tokens);
    tokens
}

/// Expands a single [`Type`] recursively to its Rust type equivalent.
///
/// This is the same as `<#expand_type(ty) as SolType>::RustType`, but generates
/// nicer code for documentation and IDE/LSP support when the type is not
/// ambiguous.
pub(crate) fn expand_rust_type(ty: &Type, crates: &ExternCrates) -> TokenStream {
    let mut tokens = TokenStream::new();
    rec_expand_rust_type(ty, crates, &mut tokens);
    tokens
}

/// The [`expand_type`] recursive implementation.
pub(crate) fn rec_expand_type(ty: &Type, crates: &ExternCrates, tokens: &mut TokenStream) {
    let alloy_sol_types = &crates.sol_types;
    let tts = match *ty {
        Type::Address(span, _) => quote_spanned! {span=> #alloy_sol_types::sol_data::Address },
        Type::Bool(span) => quote_spanned! {span=> #alloy_sol_types::sol_data::Bool },
        Type::String(span) => quote_spanned! {span=> #alloy_sol_types::sol_data::String },
        Type::Bytes(span) => quote_spanned! {span=> #alloy_sol_types::sol_data::Bytes },

        Type::FixedBytes(span, size) => {
            assert!(size.get() <= 32);
            let size = Literal::u16_unsuffixed(size.get());
            quote_spanned! {span=> #alloy_sol_types::sol_data::FixedBytes<#size> }
        }
        Type::Int(span, size) | Type::Uint(span, size) => {
            let name = match ty {
                Type::Int(..) => "Int",
                Type::Uint(..) => "Uint",
                _ => unreachable!(),
            };
            let name = Ident::new(name, span);

            let size = size.map_or(256, NonZeroU16::get);
            assert!(size <= 256 && size % 8 == 0);
            let size = Literal::u16_unsuffixed(size);

            quote_spanned! {span=> #alloy_sol_types::sol_data::#name<#size> }
        }

        Type::Tuple(ref tuple) => {
            return tuple.paren_token.surround(tokens, |tokens| {
                for pair in tuple.types.pairs() {
                    let (ty, comma) = pair.into_tuple();
                    rec_expand_type(ty, crates, tokens);
                    comma.to_tokens(tokens);
                }
            })
        }
        Type::Array(ref array) => {
            let ty = expand_type(&array.ty, crates);
            let span = array.span();
            if let Some(size) = array.size() {
                quote_spanned! {span=> #alloy_sol_types::sol_data::FixedArray<#ty, #size> }
            } else {
                quote_spanned! {span=> #alloy_sol_types::sol_data::Array<#ty> }
            }
        }
        Type::Function(ref function) => quote_spanned! {function.span()=>
            #alloy_sol_types::sol_data::Function
        },
        Type::Mapping(ref mapping) => quote_spanned! {mapping.span()=>
            ::core::compile_error!("Mapping types are not supported here")
        },

        Type::Custom(ref custom) => return custom.to_tokens(tokens),
    };
    tokens.extend(tts);
}

// IMPORTANT: Keep in sync with `sol-types/src/types/data_type.rs`
/// The [`expand_rust_type`] recursive implementation.
pub(crate) fn rec_expand_rust_type(ty: &Type, crates: &ExternCrates, tokens: &mut TokenStream) {
    // Display sizes that match with the Rust type, otherwise we lose information
    // (e.g. `uint24` displays the same as `uint32` because both use `u32`)
    fn allowed_int_size(size: Option<NonZeroU16>) -> bool {
        matches!(size.map_or(256, NonZeroU16::get), 8 | 16 | 32 | 64 | 128 | 256)
    }

    let alloy_sol_types = &crates.sol_types;
    let tts = match *ty {
        Type::Address(span, _) => quote_spanned! {span=> #alloy_sol_types::private::Address },
        Type::Bool(span) => return Ident::new("bool", span).to_tokens(tokens),
        Type::String(span) => quote_spanned! {span=> #alloy_sol_types::private::String },
        Type::Bytes(span) => quote_spanned! {span=> #alloy_sol_types::private::Vec<u8> },

        Type::FixedBytes(span, size) => {
            assert!(size.get() <= 32);
            let size = Literal::u16_unsuffixed(size.get());
            quote_spanned! {span=> #alloy_sol_types::private::FixedBytes<#size> }
        }
        Type::Int(span, size) | Type::Uint(span, size) if allowed_int_size(size) => {
            let size = size.map_or(256, NonZeroU16::get);
            if size <= 128 {
                let name = match ty {
                    Type::Int(..) => "i",
                    Type::Uint(..) => "u",
                    _ => unreachable!(),
                };
                return Ident::new(&format!("{name}{size}"), span).to_tokens(tokens);
            }
            assert_eq!(size, 256);
            match ty {
                Type::Int(..) => quote_spanned! {span=> #alloy_sol_types::private::I256 },
                Type::Uint(..) => quote_spanned! {span=> #alloy_sol_types::private::U256 },
                _ => unreachable!(),
            }
        }

        Type::Tuple(ref tuple) => {
            return tuple.paren_token.surround(tokens, |tokens| {
                for pair in tuple.types.pairs() {
                    let (ty, comma) = pair.into_tuple();
                    rec_expand_rust_type(ty, crates, tokens);
                    comma.to_tokens(tokens);
                }
            })
        }
        Type::Array(ref array) => {
            let ty = expand_rust_type(&array.ty, crates);
            let span = array.span();
            if let Some(size) = array.size() {
                quote_spanned! {span=> [#ty; #size] }
            } else {
                quote_spanned! {span=> #alloy_sol_types::private::Vec<#ty> }
            }
        }
        Type::Function(ref function) => quote_spanned! {function.span()=>
            #alloy_sol_types::private::Function
        },
        Type::Mapping(ref mapping) => quote_spanned! {mapping.span()=>
            ::core::compile_error!("Mapping types are not supported here")
        },

        // Exhaustive fallback to `SolType::RustType`
        ref ty @ (Type::Int(..) | Type::Uint(..) | Type::Custom(_)) => {
            let span = ty.span();
            let ty = expand_type(ty, crates);
            quote_spanned! {span=> <#ty as #alloy_sol_types::SolType>::RustType }
        }
    };
    tokens.extend(tts);
}

/// Calculates the base ABI-encoded size of the given parameters in bytes.
///
/// See [`type_base_data_size`] for more information.
pub(super) fn params_base_data_size<P>(cx: &ExpCtxt<'_>, params: &Parameters<P>) -> usize {
    params.iter().map(|param| type_base_data_size(cx, &param.ty)).sum()
}

/// Recursively calculates the base ABI-encoded size of the given parameter
/// in bytes.
///
/// That is, the minimum number of bytes required to encode `self` without
/// any dynamic data.
pub(super) fn type_base_data_size(cx: &ExpCtxt<'_>, ty: &Type) -> usize {
    match ty {
        // static types: 1 word
        Type::Address(..)
        | Type::Bool(_)
        | Type::Int(..)
        | Type::Uint(..)
        | Type::FixedBytes(..)
        | Type::Function(_) => 32,

        // dynamic types: 1 offset word, 1 length word
        Type::String(_) | Type::Bytes(_) | Type::Array(TypeArray { size: None, .. }) => 64,

        // fixed array: size * encoded size
        Type::Array(a @ TypeArray { ty: inner, size: Some(_), .. }) => {
            type_base_data_size(cx, inner) * a.size().unwrap()
        }

        // tuple: sum of encoded sizes
        Type::Tuple(tuple) => tuple.types.iter().map(|ty| type_base_data_size(cx, ty)).sum(),

        Type::Custom(name) => match cx.try_item(name) {
            Some(Item::Contract(_)) | Some(Item::Enum(_)) => 32,
            Some(Item::Error(error)) => {
                error.parameters.types().map(|ty| type_base_data_size(cx, ty)).sum()
            }
            Some(Item::Event(event)) => {
                event.parameters.iter().map(|p| type_base_data_size(cx, &p.ty)).sum()
            }
            Some(Item::Struct(strukt)) => {
                strukt.fields.types().map(|ty| type_base_data_size(cx, ty)).sum()
            }
            Some(Item::Udt(udt)) => type_base_data_size(cx, &udt.ty),
            Some(item) => abort!(item.span(), "Invalid type in struct field: {:?}", item),
            None => 0,
        },

        // not applicable
        Type::Mapping(_) => 0,
    }
}

const MAX_SUPPORTED_ARRAY_LEN: usize = 32;
const MAX_SUPPORTED_TUPLE_LEN: usize = 12;

/// Returns whether the given type can derive the [`Default`] trait.
pub(super) fn can_derive_default(cx: &ExpCtxt<'_>, ty: &Type) -> bool {
    match ty {
        Type::Array(a) => {
            a.size().map_or(true, |sz| sz <= MAX_SUPPORTED_ARRAY_LEN)
                && can_derive_default(cx, &a.ty)
        }
        Type::Tuple(tuple) => {
            if tuple.types.len() > MAX_SUPPORTED_TUPLE_LEN {
                false
            } else {
                tuple.types.iter().all(|ty| can_derive_default(cx, ty))
            }
        }

        Type::Custom(name) => match cx.try_item(name) {
            Some(Item::Contract(_)) | Some(Item::Enum(_)) => false,
            Some(Item::Error(error)) => {
                error.parameters.types().all(|ty| can_derive_default(cx, ty))
            }
            Some(Item::Event(event)) => {
                event.parameters.iter().all(|p| can_derive_default(cx, &p.ty))
            }
            Some(Item::Struct(strukt)) => {
                strukt.fields.types().all(|ty| can_derive_default(cx, ty))
            }
            Some(Item::Udt(udt)) => can_derive_default(cx, &udt.ty),
            Some(item) => abort!(item.span(), "Invalid type in struct field: {:?}", item),
            _ => false,
        },

        _ => true,
    }
}

/// Returns whether the given type can derive the builtin traits listed in
/// `ExprCtxt::derives`, minus `Default`.
pub(super) fn can_derive_builtin_traits(cx: &ExpCtxt<'_>, ty: &Type) -> bool {
    match ty {
        Type::Array(a) => can_derive_builtin_traits(cx, &a.ty),
        Type::Tuple(tuple) => {
            if tuple.types.len() > MAX_SUPPORTED_TUPLE_LEN {
                false
            } else {
                tuple.types.iter().all(|ty| can_derive_builtin_traits(cx, ty))
            }
        }

        Type::Custom(name) => match cx.try_item(name) {
            Some(Item::Contract(_)) | Some(Item::Enum(_)) => true,
            Some(Item::Error(error)) => {
                error.parameters.types().all(|ty| can_derive_builtin_traits(cx, ty))
            }
            Some(Item::Event(event)) => {
                event.parameters.iter().all(|p| can_derive_builtin_traits(cx, &p.ty))
            }
            Some(Item::Struct(strukt)) => {
                strukt.fields.types().all(|ty| can_derive_builtin_traits(cx, ty))
            }
            Some(Item::Udt(udt)) => can_derive_builtin_traits(cx, &udt.ty),
            Some(item) => abort!(item.span(), "Invalid type in struct field: {:?}", item),
            _ => false,
        },

        _ => true,
    }
}

/// Implements [`fmt::Display`] which formats a [`Type`] to its canonical
/// representation. This is then used in function, error, and event selector
/// generation.
pub(super) struct TypePrinter<'ast> {
    cx: &'ast ExpCtxt<'ast>,
    ty: &'ast Type,
}

impl<'ast> TypePrinter<'ast> {
    pub(super) fn new(cx: &'ast ExpCtxt<'ast>, ty: &'ast Type) -> Self {
        Self { cx, ty }
    }
}

impl fmt::Display for TypePrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            Type::Int(_, None) => f.write_str("int256"),
            Type::Uint(_, None) => f.write_str("uint256"),

            Type::Array(array) => {
                Self::new(self.cx, &array.ty).fmt(f)?;
                f.write_str("[")?;
                if let Some(size) = array.size() {
                    size.fmt(f)?;
                }
                f.write_str("]")
            }
            Type::Tuple(tuple) => {
                f.write_str("(")?;
                for (i, ty) in tuple.types.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    Self::new(self.cx, ty).fmt(f)?;
                }
                f.write_str(")")
            }

            Type::Custom(name) => Self::new(self.cx, self.cx.custom_type(name)).fmt(f),

            ty => ty.fmt(f),
        }
    }
}
