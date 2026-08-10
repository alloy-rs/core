use super::ExpCtxt;
use crate::verbatim::Verbatim;
use alloy_json_abi::{
    Constructor, Error, Event, EventParam, Fallback, Function, InternalType, Param, Receive,
    StateMutability,
};
use ast::{ItemError, ItemEvent, ItemFunction};
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::Write;

pub(crate) fn generate<T>(t: &T, cx: &ExpCtxt<'_>) -> TokenStream
where
    T: ToAbi,
    T::DynAbi: Verbatim,
{
    crate::verbatim::verbatim(&t.to_dyn_abi(cx), &cx.crates)
}

pub(crate) trait ToAbi {
    type DynAbi;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi;
}

impl ToAbi for ast::ItemFunction {
    type DynAbi = Function;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        Function {
            name: self.name.as_ref().map(|i| i.as_string()).unwrap_or_default(),
            inputs: self.parameters.to_dyn_abi(cx),
            outputs: self.returns.as_ref().map(|r| r.returns.to_dyn_abi(cx)).unwrap_or_default(),
            state_mutability: self.attributes.to_dyn_abi(cx),
        }
    }
}

impl ToAbi for ast::ItemError {
    type DynAbi = Error;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        Error { name: self.name.as_string(), inputs: self.parameters.to_dyn_abi(cx) }
    }
}

impl ToAbi for ast::ItemEvent {
    type DynAbi = Event;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        Event {
            name: self.name.as_string(),
            inputs: self.parameters.iter().map(|e| e.to_dyn_abi(cx)).collect(),
            anonymous: self.is_anonymous(),
        }
    }
}

impl<P> ToAbi for ast::Parameters<P> {
    type DynAbi = Vec<Param>;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        self.iter().map(|p| p.to_dyn_abi(cx)).collect()
    }
}

impl ToAbi for ast::VariableDeclaration {
    type DynAbi = Param;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        ty_to_param(self.name.as_ref().map(ast::SolIdent::as_string), &self.ty, cx)
    }
}

impl ToAbi for ast::EventParameter {
    type DynAbi = EventParam;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        let name = self.name.as_ref().map(ast::SolIdent::as_string);
        let Param { ty, name, components, internal_type } = ty_to_param(name, &self.ty, cx);
        EventParam { ty, name, indexed: self.is_indexed(), internal_type, components }
    }
}

impl ToAbi for ast::FunctionAttributes {
    type DynAbi = StateMutability;

    fn to_dyn_abi(&self, _cx: &ExpCtxt<'_>) -> Self::DynAbi {
        match self.mutability() {
            Some(ast::Mutability::Pure(_) | ast::Mutability::Constant(_)) => StateMutability::Pure,
            Some(ast::Mutability::View(_)) => StateMutability::View,
            Some(ast::Mutability::Payable(_)) => StateMutability::Payable,
            None => StateMutability::NonPayable,
        }
    }
}

fn ty_to_param(name: Option<String>, ty: &ast::Type, cx: &ExpCtxt<'_>) -> Param {
    let mut ty_name = ty_abi_string(ty, cx);

    // HACK: `cx.custom_type` resolves the custom type recursively, so in recursive structs the
    // peeled `ty` will be `Tuple` rather than `Custom`.
    if ty_name.starts_with('(') {
        let paren_i = ty_name.rfind(')').expect("malformed tuple type");
        let suffix = &ty_name[paren_i + 1..];
        ty_name = format!("tuple{suffix}");
    }

    // For struct types, get the original fields to preserve type information (like UDVTs)
    let original_fields = match ty.peel_arrays() {
        ast::Type::Custom(name) => {
            if let ast::Item::Struct(s) = cx.item(name) {
                Some(s.fields.clone())
            } else {
                None
            }
        }
        _ => None,
    };

    let resolved = match ty.peel_arrays() {
        ast::Type::Custom(name) => cx.custom_type(name),
        ty => ty,
    };

    let components = if let Some(fields) = original_fields {
        // Use original struct fields to preserve UDVT and other custom type names
        fields
            .iter()
            .map(|field| ty_to_param(field.name.as_ref().map(|n| n.as_string()), &field.ty, cx))
            .collect()
    } else if let ast::Type::Tuple(tuple) = resolved {
        // For non-struct tuples, use the resolved types
        tuple.types.iter().map(|ty| ty_to_param(None, ty, cx)).collect()
    } else {
        vec![]
    };

    let internal_type = ty_to_internal_type(ty, cx);

    Param { ty: ty_name, name: name.unwrap_or_default(), internal_type, components }
}

/// Generates the internal type for a given Solidity type.
/// This represents the source-level type as it appears in the Solidity code.
fn ty_to_internal_type(ty: &ast::Type, cx: &ExpCtxt<'_>) -> Option<InternalType> {
    // Collect array suffixes
    let mut array_suffix = String::new();
    rec_ty_abi_string_suffix(cx, ty, &mut array_suffix);

    // Peel arrays to get the base type
    let base_ty = ty.peel_arrays();

    match base_ty {
        ast::Type::Address(_, Some(_)) => {
            // Address payable
            Some(InternalType::AddressPayable(format!("address payable{array_suffix}")))
        }
        ast::Type::Custom(path) => {
            // Determine the contract qualifier.
            let contract = if path.len() == 2 {
                // Explicit namespace: MyContract.MyStruct
                Some(path.first().as_string())
            } else if path.len() == 1 {
                // Single component: check if we're in a contract namespace
                // and if the item is defined in that namespace
                cx.current_namespace.as_ref().map(|ns| ns.as_string())
            } else {
                None
            };

            // Get the type name (last component of the path)
            let type_name = path.last().as_string();

            // Look up what kind of item this is
            match cx.try_item(path) {
                Some(ast::Item::Struct(_)) => Some(InternalType::Struct {
                    contract,
                    ty: format!("{type_name}{array_suffix}"),
                }),
                Some(ast::Item::Enum(_)) => {
                    Some(InternalType::Enum { contract, ty: format!("{type_name}{array_suffix}") })
                }
                Some(ast::Item::Contract(_)) => {
                    Some(InternalType::Contract(format!("{type_name}{array_suffix}")))
                }
                Some(ast::Item::Udt(_)) => {
                    Some(InternalType::Other { contract, ty: format!("{type_name}{array_suffix}") })
                }
                _ => {
                    // Fallback for unresolved custom types
                    Some(InternalType::Other { contract, ty: format!("{type_name}{array_suffix}") })
                }
            }
        }
        _ => {
            // For built-in types, generate the internal type string
            let ty_str = format!("{}{array_suffix}", super::ty::TypePrinter::new(cx, base_ty));
            Some(InternalType::Other { contract: None, ty: ty_str })
        }
    }
}

fn ty_abi_string(ty: &ast::Type, cx: &ExpCtxt<'_>) -> String {
    let mut suffix = String::new();
    rec_ty_abi_string_suffix(cx, ty, &mut suffix);

    let mut ty = ty.peel_arrays();
    if let ast::Type::Custom(name) = ty {
        match cx.try_custom_type(name) {
            Some(ast::Type::Tuple(_)) => return format!("tuple{suffix}"),
            Some(custom) => ty = custom,
            None => {}
        }
    }
    format!("{}{suffix}", super::ty::TypePrinter::new(cx, ty))
}

fn rec_ty_abi_string_suffix(cx: &ExpCtxt<'_>, ty: &ast::Type, s: &mut String) {
    if let ast::Type::Array(array) = ty {
        rec_ty_abi_string_suffix(cx, &array.ty, s);
        if let Some(size) = cx.eval_array_size(array) {
            write!(s, "[{size}]").unwrap();
        } else {
            s.push_str("[]");
        }
    }
}

pub(super) fn constructor(function: &ItemFunction, cx: &ExpCtxt<'_>) -> Constructor {
    assert!(function.kind.is_constructor());
    Constructor {
        inputs: function.parameters.to_dyn_abi(cx),
        state_mutability: function.attributes.to_dyn_abi(cx),
    }
}

pub(super) fn fallback(function: &ItemFunction, cx: &ExpCtxt<'_>) -> Fallback {
    assert!(function.kind.is_fallback());
    Fallback { state_mutability: function.attributes.to_dyn_abi(cx) }
}

pub(super) fn receive(function: &ItemFunction, _cx: &ExpCtxt<'_>) -> Receive {
    assert!(function.kind.is_receive());
    Receive { state_mutability: StateMutability::Payable }
}

macro_rules! make_map {
    ($items:expr, $cx:expr, $get_ident:ident, $ty:ident) => {{
        let mut items_map = std::collections::BTreeMap::<String, Vec<_>>::new();
        let alloy_sol_types = &$cx.crates.sol_types;
        for item in $items {
            let name = item.to_dyn_abi($cx).name;
            let ident = $cx.$get_ident(item.into());
            let item = quote::quote!(<#ident as #alloy_sol_types::JsonAbiExt>::abi);
            items_map.entry(name).or_default().push(item);
        }
        let items = items_map.into_iter().map(|(name, items)| {
            quote!((#name, &[#(#items),*]))
        });
        quote! {
            static LAZY_ITEMS: &[(&str, &[fn() -> #alloy_sol_types::private::alloy_json_abi::$ty])] = &[#(#items,)*];
            let items = LAZY_ITEMS
                .iter()
                .map(|(name, item_fns)| (
                    alloy_sol_types::private::str_to_owned(name),
                    item_fns.iter().map(|item_fn| item_fn()).collect()
                ))
                .collect();
            alloy_sol_types::private::make_btree_map(items)
        }
    }};
}

pub(super) fn functions_map(functions: &[ItemFunction], cx: &ExpCtxt<'_>) -> TokenStream {
    make_map!(functions, cx, call_name, Function)
}

pub(super) fn events_map(events: &[&ItemEvent], cx: &ExpCtxt<'_>) -> TokenStream {
    make_map!(events.iter().copied(), cx, overloaded_name, Event)
}

pub(super) fn errors_map(errors: &[&ItemError], cx: &ExpCtxt<'_>) -> TokenStream {
    make_map!(errors.iter().copied(), cx, overloaded_name, Error)
}
