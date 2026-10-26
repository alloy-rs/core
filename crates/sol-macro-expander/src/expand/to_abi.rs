use super::ExpCtxt;
use alloy_json_abi::{Error, Event, EventParam, Function, Param, StateMutability};
use ast::{ItemError, ItemEvent, ItemFunction};
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::Write;

/// Expands to the owned [`alloy_json_abi`] item, built by a single call from a constant mirror.
pub(crate) fn generate<T>(t: &T, cx: &ExpCtxt<'_>) -> TokenStream
where
    T: ToAbi,
    T::DynAbi: StaticAbi,
{
    t.to_dyn_abi(cx).to_static_tokens(&cx.crates.sol_types)
}

/// Conversion to tokens that build the item from its `private::Static*` mirror.
pub(crate) trait StaticAbi {
    fn to_static_tokens(&self, alloy_sol_types: &syn::Path) -> TokenStream;
}

impl StaticAbi for Function {
    fn to_static_tokens(&self, alloy_sol_types: &syn::Path) -> TokenStream {
        let Self { name, inputs, outputs, state_mutability } = self;
        let inputs = params_tokens(inputs, alloy_sol_types);
        let outputs = params_tokens(outputs, alloy_sol_types);
        let state_mutability = mutability_tokens(*state_mutability, alloy_sol_types);
        quote! {
            #alloy_sol_types::private::StaticFunction {
                name: #name,
                inputs: #inputs,
                outputs: #outputs,
                state_mutability: #state_mutability,
            }.to_abi()
        }
    }
}

impl StaticAbi for Error {
    fn to_static_tokens(&self, alloy_sol_types: &syn::Path) -> TokenStream {
        let Self { name, inputs } = self;
        let inputs = params_tokens(inputs, alloy_sol_types);
        quote! {
            #alloy_sol_types::private::StaticError { name: #name, inputs: #inputs }.to_abi()
        }
    }
}

impl StaticAbi for Event {
    fn to_static_tokens(&self, alloy_sol_types: &syn::Path) -> TokenStream {
        let Self { name, inputs, anonymous } = self;
        let inputs = inputs.iter().map(|p| {
            let EventParam { ty, name, indexed, components, internal_type } = p;
            debug_assert!(internal_type.is_none());
            let components = params_tokens(components, alloy_sol_types);
            quote! {
                #alloy_sol_types::private::StaticEventParam {
                    ty: #ty,
                    name: #name,
                    indexed: #indexed,
                    components: #components,
                }
            }
        });
        quote! {
            #alloy_sol_types::private::StaticEvent {
                name: #name,
                inputs: &[#(#inputs),*],
                anonymous: #anonymous,
            }.to_abi()
        }
    }
}

fn params_tokens(params: &[Param], alloy_sol_types: &syn::Path) -> TokenStream {
    let params = params.iter().map(|p| {
        let Param { ty, name, components, internal_type } = p;
        debug_assert!(internal_type.is_none());
        let components = params_tokens(components, alloy_sol_types);
        quote! {
            #alloy_sol_types::private::StaticParam { ty: #ty, name: #name, components: #components }
        }
    });
    quote!(&[#(#params),*])
}

fn mutability_tokens(m: StateMutability, alloy_sol_types: &syn::Path) -> TokenStream {
    let variant = match m {
        StateMutability::Pure => quote!(Pure),
        StateMutability::View => quote!(View),
        StateMutability::NonPayable => quote!(NonPayable),
        StateMutability::Payable => quote!(Payable),
    };
    quote!(#alloy_sol_types::private::alloy_json_abi::StateMutability::#variant)
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

    let resolved = match ty.peel_arrays() {
        ast::Type::Custom(custom_name) => {
            if let ast::Item::Struct(s) = cx.item(custom_name) {
                return Param {
                    ty: ty_name,
                    name: name.unwrap_or_default(),
                    internal_type: None,
                    components: s.fields.to_dyn_abi(cx),
                };
            }
            cx.custom_type(custom_name)
        }
        ty => ty,
    };

    let components = if let ast::Type::Tuple(tuple) = resolved {
        tuple.types.iter().map(|ty| ty_to_param(None, ty, cx)).collect()
    } else {
        vec![]
    };

    // TODO: internal_type
    let internal_type = None;

    Param { ty: ty_name, name: name.unwrap_or_default(), internal_type, components }
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

pub(super) fn constructor(function: Option<&ItemFunction>, cx: &ExpCtxt<'_>) -> TokenStream {
    let alloy_sol_types = &cx.crates.sol_types;
    let Some(function) = function else { return quote!(::core::option::Option::None) };
    assert!(function.kind.is_constructor());
    let inputs = params_tokens(&function.parameters.to_dyn_abi(cx), alloy_sol_types);
    let state_mutability = mutability_tokens(function.attributes.to_dyn_abi(cx), alloy_sol_types);
    quote! {
        ::core::option::Option::Some(
            #alloy_sol_types::private::StaticConstructor {
                inputs: #inputs,
                state_mutability: #state_mutability,
            }.to_abi()
        )
    }
}

pub(super) fn fallback(function: Option<&ItemFunction>, cx: &ExpCtxt<'_>) -> TokenStream {
    let alloy_sol_types = &cx.crates.sol_types;
    let Some(function) = function else { return quote!(::core::option::Option::None) };
    assert!(function.kind.is_fallback());
    let state_mutability = mutability_tokens(function.attributes.to_dyn_abi(cx), alloy_sol_types);
    quote! {
        ::core::option::Option::Some(
            #alloy_sol_types::private::alloy_json_abi::Fallback { state_mutability: #state_mutability }
        )
    }
}

pub(super) fn receive(function: Option<&ItemFunction>, cx: &ExpCtxt<'_>) -> TokenStream {
    let alloy_sol_types = &cx.crates.sol_types;
    let Some(function) = function else { return quote!(::core::option::Option::None) };
    assert!(function.kind.is_receive());
    let state_mutability = mutability_tokens(StateMutability::Payable, alloy_sol_types);
    quote! {
        ::core::option::Option::Some(
            #alloy_sol_types::private::alloy_json_abi::Receive { state_mutability: #state_mutability }
        )
    }
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
