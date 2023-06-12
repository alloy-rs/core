use super::ExpCtxt;
use ast::{Item, Parameters, SolArray, SolPath, Type};
use proc_macro2::{Literal, TokenStream};
use quote::{quote_spanned, ToTokens};
use std::{fmt, num::NonZeroU16};

/// Expands a single [`Type`] recursively.
pub fn expand_type(ty: &Type) -> TokenStream {
    let mut tokens = TokenStream::new();
    rec_expand_type(ty, &mut tokens);
    tokens
}

fn rec_expand_type(ty: &Type, tokens: &mut TokenStream) {
    let tts = match *ty {
        Type::Address(span, _) => quote_spanned! {span=> ::alloy_sol_types::sol_data::Address },
        Type::Bool(span) => quote_spanned! {span=> ::alloy_sol_types::sol_data::Bool },
        Type::String(span) => quote_spanned! {span=> ::alloy_sol_types::sol_data::String },
        Type::Bytes(span) => quote_spanned! {span=> ::alloy_sol_types::sol_data::Bytes },

        Type::FixedBytes(span, size) => {
            let size = Literal::u16_unsuffixed(size.get());
            quote_spanned! {span=>
                ::alloy_sol_types::sol_data::FixedBytes<#size>
            }
        }
        Type::Int(span, size) => {
            let size = Literal::u16_unsuffixed(size.map_or(256, NonZeroU16::get));
            quote_spanned! {span=>
                ::alloy_sol_types::sol_data::Int<#size>
            }
        }
        Type::Uint(span, size) => {
            let size = Literal::u16_unsuffixed(size.map_or(256, NonZeroU16::get));
            quote_spanned! {span=>
                ::alloy_sol_types::sol_data::Uint<#size>
            }
        }

        Type::Tuple(ref tuple) => {
            tuple.paren_token.surround(tokens, |tokens| {
                for pair in tuple.types.pairs() {
                    let (ty, comma) = pair.into_tuple();
                    rec_expand_type(ty, tokens);
                    comma.to_tokens(tokens);
                }
            });
            return
        }
        Type::Array(ref array) => {
            let ty = expand_type(&array.ty);
            let span = array.span();
            if let Some(size) = &array.size {
                quote_spanned! {span=>
                    ::alloy_sol_types::sol_data::FixedArray<#ty, #size>
                }
            } else {
                quote_spanned! {span=>
                    ::alloy_sol_types::sol_data::Array<#ty>
                }
            }
        }
        Type::Custom(ref custom) => return custom.to_tokens(tokens),
    };
    tokens.extend(tts);
}

impl ExpCtxt<'_> {
    fn get_item(&self, name: &SolPath) -> &Item {
        let name = name.last_tmp();
        match self.all_items.iter().find(|item| item.name() == name) {
            Some(item) => item,
            None => panic!("unresolved item: {name}"),
        }
    }

    pub(super) fn custom_type(&self, name: &SolPath) -> &Type {
        match self.custom_types.get(name.last_tmp()) {
            Some(item) => item,
            None => panic!("unresolved item: {name}"),
        }
    }

    pub(super) fn min_data_size<P>(&self, params: &Parameters<P>) -> usize {
        params
            .iter()
            .map(|param| self.type_base_data_size(&param.ty))
            .max()
            .unwrap_or(0)
    }

    /// Recursively calculates the base ABI-encoded size of `self` in bytes.
    ///
    /// That is, the minimum number of bytes required to encode `self` without
    /// any dynamic data.
    pub(super) fn type_base_data_size(&self, ty: &Type) -> usize {
        match ty {
            // static types: 1 word
            Type::Address(..)
            | Type::Bool(_)
            | Type::Int(..)
            | Type::Uint(..)
            | Type::FixedBytes(..) => 32,

            // dynamic types: 1 offset word, 1 length word
            Type::String(_) | Type::Bytes(_) | Type::Array(SolArray { size: None, .. }) => 64,

            // fixed array: size * encoded size
            Type::Array(SolArray {
                ty: inner,
                size: Some(size),
                ..
            }) => self.type_base_data_size(inner) * size.base10_parse::<usize>().unwrap(),

            // tuple: sum of encoded sizes
            Type::Tuple(tuple) => tuple
                .types
                .iter()
                .map(|ty| self.type_base_data_size(ty))
                .sum(),

            Type::Custom(name) => match self.get_item(name) {
                Item::Struct(strukt) => strukt
                    .fields
                    .types()
                    .map(|ty| self.type_base_data_size(ty))
                    .sum(),
                Item::Udt(udt) => self.type_base_data_size(&udt.ty),
                Item::Contract(_) | Item::Error(_) | Item::Event(_) | Item::Function(_) => {
                    unreachable!()
                }
            },
        }
    }
}

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
            Type::Custom(name) => self.cx.custom_type(name).fmt(f),
            Type::Array(array) => {
                Self::new(self.cx, &array.ty).fmt(f)?;
                f.write_str("[")?;
                if let Some(size) = &array.size {
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
            ty => ty.fmt(f),
        }
    }
}
