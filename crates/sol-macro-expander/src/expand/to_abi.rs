use super::{ExpCtxt, ty::TypePrinter};
use alloy_json_abi::{Error, Event, EventParam, Function, InternalType, Param, StateMutability};
use ast::{ItemError, ItemEvent, ItemFunction, SolIdent, Spanned};
use proc_macro_error3::abort;
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
            let internal_type = internal_type_tokens(internal_type.as_ref());
            let components = params_tokens(components, alloy_sol_types);
            quote! {
                #alloy_sol_types::private::StaticEventParam {
                    ty: #ty,
                    name: #name,
                    indexed: #indexed,
                    internal_type: #internal_type,
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
        let internal_type = internal_type_tokens(internal_type.as_ref());
        let components = params_tokens(components, alloy_sol_types);
        quote! {
            #alloy_sol_types::private::StaticParam {
                ty: #ty,
                name: #name,
                internal_type: #internal_type,
                components: #components,
            }
        }
    });
    quote!(&[#(#params),*])
}

fn internal_type_tokens(internal_type: Option<&InternalType>) -> TokenStream {
    match internal_type {
        Some(internal_type) => {
            let internal_type = internal_type.to_string();
            quote!(::core::option::Option::Some(#internal_type))
        }
        None => quote!(::core::option::Option::None),
    }
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
            name: cx.sol_name(self.name(), &self.attrs),
            inputs: params_to_dyn_abi(&self.parameters, &cx.current_namespace, cx, &self.attrs),
            outputs: self
                .returns
                .as_ref()
                .map(|r| params_to_dyn_abi(&r.returns, &cx.current_namespace, cx, &self.attrs))
                .unwrap_or_default(),
            state_mutability: self.attributes.to_dyn_abi(cx),
        }
    }
}

impl ToAbi for ast::ItemError {
    type DynAbi = Error;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        Error {
            name: cx.sol_name(&self.name, &self.attrs),
            inputs: params_to_dyn_abi(&self.parameters, &cx.current_namespace, cx, &self.attrs),
        }
    }
}

impl ToAbi for ast::ItemEvent {
    type DynAbi = Event;

    fn to_dyn_abi(&self, cx: &ExpCtxt<'_>) -> Self::DynAbi {
        Event {
            name: cx.sol_name(&self.name, &self.attrs),
            inputs: self
                .parameters
                .iter()
                .map(|event| event_param_to_dyn_abi(event, cx, &self.attrs))
                .collect(),
            anonymous: self.is_anonymous(),
        }
    }
}

fn params_to_dyn_abi<P>(
    params: &ast::Parameters<P>,
    namespace: &Option<SolIdent>,
    cx: &ExpCtxt<'_>,
    parent_attrs: &[syn::Attribute],
) -> Vec<Param> {
    params
        .iter()
        .map(|param| {
            let name = param
                .name
                .as_ref()
                .map(|name| cx.child_sol_name(name, &param.attrs, parent_attrs, namespace));
            ty_to_param(name, &param.ty, namespace, cx)
        })
        .collect()
}

fn event_param_to_dyn_abi(
    param: &ast::EventParameter,
    cx: &ExpCtxt<'_>,
    parent_attrs: &[syn::Attribute],
) -> EventParam {
    let name = param
        .name
        .as_ref()
        .map(|name| cx.child_sol_name(name, &param.attrs, parent_attrs, &cx.current_namespace));
    let Param { ty, name, components, internal_type } =
        ty_to_param(name, &param.ty, &cx.current_namespace, cx);
    EventParam { ty, name, indexed: param.is_indexed(), internal_type, components }
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

/// Converts a type to a parameter, resolving custom types in `namespace`.
fn ty_to_param(
    name: Option<String>,
    ty: &ast::Type,
    namespace: &Option<SolIdent>,
    cx: &ExpCtxt<'_>,
) -> Param {
    let mut suffix = String::new();
    rec_ty_abi_string_suffix(cx, ty, &mut suffix);

    let mut components = vec![];
    let (ty, internal_type) = match ty.peel_arrays() {
        ast::Type::Custom(path) => {
            let Some((item_namespace, resolved)) = cx.custom_types.resolve_entry(path, namespace)
            else {
                abort!(path.span(), "unresolved custom type: {}", path);
            };
            let (_, item) =
                cx.all_items.get_entry_by_name_and_namespace(item_namespace, path.last()).unwrap();
            // Canonical names of types declared in a contract are qualified by the contract name.
            let mut canonical = match item {
                ast::Item::Contract(_) => path.last().as_string(),
                item => cx.sol_name_in_namespace(
                    path.last(),
                    item.attrs().map_or(&[], Vec::as_slice),
                    item_namespace,
                ),
            };
            if let Some(contract) = item_namespace {
                canonical = format!("{}.{canonical}", contract.as_string());
            }
            let (ty, internal_type) = match item {
                ast::Item::Struct(strukt) => {
                    components =
                        params_to_dyn_abi(&strukt.fields, item_namespace, cx, &strukt.attrs);
                    ("tuple".into(), format!("struct {canonical}"))
                }
                item => {
                    let kind = match item {
                        ast::Item::Contract(_) => "contract ",
                        ast::Item::Enum(_) => "enum ",
                        _ => "",
                    };
                    (TypePrinter::new(cx, resolved).to_string(), format!("{kind}{canonical}"))
                }
            };
            (ty, Some(internal_type))
        }
        ast::Type::Tuple(tuple) => {
            components =
                tuple.types.iter().map(|ty| ty_to_param(None, ty, namespace, cx)).collect();
            ("tuple".into(), None)
        }
        ty => {
            let abi_ty = TypePrinter::new(cx, ty).to_string();
            let internal_type = match ty {
                ast::Type::Address(_, Some(_)) => Some("address payable".into()),
                ast::Type::Function(_) => None,
                _ => Some(abi_ty.clone()),
            };
            (abi_ty, internal_type)
        }
    };

    Param {
        ty: format!("{ty}{suffix}"),
        name: name.unwrap_or_default(),
        internal_type: internal_type
            .filter(|_| !cx.from_json)
            .and_then(|ty| InternalType::parse(&format!("{ty}{suffix}"))),
        components,
    }
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
    let inputs = params_tokens(
        &params_to_dyn_abi(&function.parameters, &cx.current_namespace, cx, &function.attrs),
        alloy_sol_types,
    );
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
