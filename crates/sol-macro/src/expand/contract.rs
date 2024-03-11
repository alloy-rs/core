//! [`ItemContract`] expansion.

use super::{anon_name, ty, ExpCtxt};
use crate::{attr, utils::ExprArray};
use ast::{Item, ItemContract, ItemError, ItemEvent, ItemFunction, SolIdent, Spanned};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, Attribute, Result};

/// Expands an [`ItemContract`]:
///
/// ```ignore (pseudo-code)
/// pub mod #name {
///     #(#items)*
///
///     pub enum #{name}Calls {
///         ...
///    }
///
///     pub enum #{name}Errors {
///         ...
///    }
///
///     pub enum #{name}Events {
///         ...
///    }
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, contract: &ItemContract) -> Result<TokenStream> {
    let ItemContract { attrs, name, body, .. } = contract;

    let (sol_attrs, attrs) = attr::SolAttrs::parse(attrs)?;
    let extra_methods = sol_attrs.extra_methods.or(cx.attrs.extra_methods).unwrap_or(false);
    let rpc = sol_attrs.rpc.or(cx.attrs.rpc).unwrap_or(false);
    let abi = sol_attrs.abi.or(cx.attrs.abi).unwrap_or(false);
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);

    let bytecode = sol_attrs.bytecode.map(|lit| {
        let name = Ident::new("BYTECODE", lit.span());
        let hex = lit.value();
        let bytes = hex::decode(&hex).unwrap();
        let lit_bytes = proc_macro2::Literal::byte_string(&bytes).with_span(lit.span());
        quote! {
            /// The creation / init bytecode of the contract.
            ///
            /// ```text
            #[doc = #hex]
            /// ```
            #[rustfmt::skip]
            pub static #name: alloy_sol_types::private::Bytes =
                alloy_sol_types::private::Bytes::from_static(#lit_bytes);
        }
    });
    let deployed_bytecode = sol_attrs.deployed_bytecode.map(|lit| {
        let name = Ident::new("DEPLOYED_BYTECODE", lit.span());
        let hex = lit.value();
        let bytes = hex::decode(&hex).unwrap();
        let lit_bytes = proc_macro2::Literal::byte_string(&bytes).with_span(lit.span());
        quote! {
            /// The runtime bytecode of the contract, as deployed on the network.
            ///
            /// ```text
            #[doc = #hex]
            /// ```
            #[rustfmt::skip]
            pub static #name: alloy_sol_types::private::Bytes =
                alloy_sol_types::private::Bytes::from_static(#lit_bytes);
        }
    });

    let mut constructor = None;
    let mut fallback = None;
    let mut receive = None;
    let mut functions = Vec::with_capacity(contract.body.len());
    let mut errors = Vec::with_capacity(contract.body.len());
    let mut events = Vec::with_capacity(contract.body.len());

    let (mut mod_attrs, item_attrs) =
        attrs.into_iter().partition::<Vec<_>, _>(|a| a.path().is_ident("doc"));
    mod_attrs.extend(item_attrs.iter().filter(|a| !a.path().is_ident("derive")).cloned());

    let mut item_tokens = TokenStream::new();
    for item in body {
        match item {
            Item::Function(function) => match function.kind {
                ast::FunctionKind::Function(_) if function.name.is_some() => {
                    functions.push(function.clone());
                }
                ast::FunctionKind::Function(_) => {}
                ast::FunctionKind::Modifier(_) => {}
                ast::FunctionKind::Constructor(_) => {
                    if constructor.is_none() {
                        constructor = Some(function);
                    } else {
                        let msg = "duplicate constructor";
                        return Err(syn::Error::new(function.span(), msg));
                    }
                }
                ast::FunctionKind::Fallback(_) => {
                    if fallback.is_none() {
                        fallback = Some(function);
                    } else {
                        let msg = "duplicate fallback function";
                        return Err(syn::Error::new(function.span(), msg));
                    }
                }
                ast::FunctionKind::Receive(_) => {
                    if receive.is_none() {
                        receive = Some(function);
                    } else {
                        let msg = "duplicate receive function";
                        return Err(syn::Error::new(function.span(), msg));
                    }
                }
            },
            Item::Error(error) => errors.push(error),
            Item::Event(event) => events.push(event),
            Item::Variable(var_def) => {
                if let Some(function) = super::var_def::var_as_function(cx, var_def)? {
                    functions.push(function);
                }
            }
            _ => {}
        }

        if item.attrs().is_none() || item_attrs.is_empty() {
            // avoid cloning item if we don't have to
            item_tokens.extend(cx.expand_item(item)?);
        } else {
            // prepend `item_attrs` to `item.attrs`
            let mut item = item.clone();
            item.attrs_mut().expect("is_none checked above").splice(0..0, item_attrs.clone());
            item_tokens.extend(cx.expand_item(&item)?);
        }
    }

    let enum_expander = CallLikeExpander { cx, contract_name: name.clone(), extra_methods };

    let functions_enum = (!functions.is_empty()).then(|| {
        let mut attrs = item_attrs.clone();
        let doc_str = format!("Container for all the [`{name}`](self) function calls.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        enum_expander.expand(ToExpand::Functions(&functions), attrs)
    });

    let errors_enum = (!errors.is_empty()).then(|| {
        let mut attrs = item_attrs.clone();
        let doc_str = format!("Container for all the [`{name}`](self) custom errors.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        enum_expander.expand(ToExpand::Errors(&errors), attrs)
    });

    let events_enum = (!events.is_empty()).then(|| {
        let mut attrs = item_attrs;
        let doc_str = format!("Container for all the [`{name}`](self) events.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        enum_expander.expand(ToExpand::Events(&events), attrs)
    });

    let mod_descr_doc = (docs && attr::docs_str(&mod_attrs).trim().is_empty())
        .then(|| attr::mk_doc("Module containing a contract's types and functions."));
    let mod_iface_doc = (docs && !attr::docs_str(&mod_attrs).contains("```solidity\n"))
        .then(|| attr::mk_doc(format!("\n\n```solidity\n{contract}\n```")));

    let abi = abi.then(|| {
        if_json! {
            use crate::verbatim::verbatim;
            use super::to_abi;

            let crates = &cx.crates;
            let constructor = verbatim(&constructor.map(|x| to_abi::constructor(x, cx)), crates);
            let fallback = verbatim(&fallback.map(|x| to_abi::fallback(x, cx)), crates);
            let receive = verbatim(&receive.map(|x| to_abi::receive(x, cx)), crates);
            let functions_map = to_abi::functions_map(&functions, cx);
            let events_map = to_abi::events_map(&events, cx);
            let errors_map = to_abi::errors_map(&errors, cx);
            quote! {
                /// Contains [dynamic ABI definitions](alloy_sol_types::private::alloy_json_abi) for [this contract](self).
                pub mod abi {
                    use super::*;
                    use alloy_sol_types::private::{alloy_json_abi as json, BTreeMap, Vec};

                    /// Returns the ABI for [this contract](super).
                    pub fn contract() -> json::JsonAbi {
                        json::JsonAbi {
                            constructor: constructor(),
                            fallback: fallback(),
                            receive: receive(),
                            functions: functions(),
                            events: events(),
                            errors: errors(),
                        }
                    }

                    /// Returns the [`Constructor`](json::Constructor) of [this contract](super), if any.
                    pub fn constructor() -> Option<json::Constructor> {
                        #constructor
                    }

                    /// Returns the [`Fallback`](json::Fallback) function of [this contract](super), if any.
                    pub fn fallback() -> Option<json::Fallback> {
                        #fallback
                    }

                    /// Returns the [`Receive`](json::Receive) function of [this contract](super), if any.
                    pub fn receive() -> Option<json::Receive> {
                        #receive
                    }

                    /// Returns a map of all the [`Function`](json::Function)s of [this contract](super).
                    pub fn functions() -> BTreeMap<String, Vec<json::Function>> {
                        #functions_map
                    }

                    /// Returns a map of all the [`Event`](json::Event)s of [this contract](super).
                    pub fn events() -> BTreeMap<String, Vec<json::Event>> {
                        #events_map
                    }

                    /// Returns a map of all the [`Error`](json::Error)s of [this contract](super).
                    pub fn errors() -> BTreeMap<String, Vec<json::Error>> {
                        #errors_map
                    }
                }
            }
        }
    });

    let rpc = rpc.then(|| {
        let contract_name = name;
        let name = format_ident!("{contract_name}Instance");
        let name_s = name.to_string();
        let methods = functions.iter().map(|f| call_builder_method(f, cx));
        let new_fn_doc = format!(
            "Creates a new wrapper around an on-chain [`{contract_name}`](self) contract instance.\n\
             \n\
             See the [wrapper's documentation](`{name}`) for more details."
        );
        let struct_doc = format!(
            "A [`{contract_name}`](self) instance.\n\
             \n\
             Contains type-safe methods for interacting with an on-chain instance of the\n\
             [`{contract_name}`](self) contract located at a given `address`, using a given\n\
             provider `P`.\n\
             \n\
             If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)\n\
             documentation on how to provide it), the `deploy` and `deploy_builder` methods can\n\
             be used to deploy a new instance of the contract.\n\
             \n\
             See the [module-level documentation](self) for all the available methods."
        );
        let (deploy_fn, deploy_method) = option_unzip(bytecode.is_some().then(|| {
            let deploy_doc_str =
                "Deploys this contract using the given `provider` and constructor arguments, if any.\n\
                 \n\
                 Returns a new instance of the contract, if the deployment was successful.\n\
                 \n\
                 For more fine-grained control over the deployment process, use [`deploy_builder`] instead.";
            let deploy_doc = attr::mk_doc(deploy_doc_str);

            let deploy_builder_doc_str =
                "Creates a `RawCallBuilder` for deploying this contract using the given `provider`\n\
                 and constructor arguments, if any.\n\
                 \n\
                 This is a simple wrapper around creating a `RawCallBuilder` with the data set to\n\
                 the bytecode concatenated with the constructor's ABI-encoded arguments.";
            let deploy_builder_doc = attr::mk_doc(deploy_builder_doc_str);

            let (params, args) = option_unzip(constructor.and_then(|c| {
                if c.parameters.is_empty() {
                    return None;
                }

                let names1 = c.parameters.names().enumerate().map(anon_name);
                let names2 = names1.clone();
                let tys = c.parameters.types().map(|ty| {
                    super::ty::expand_rust_type(ty, &cx.crates)
                });
                Some((quote!(#(#names1: #tys),*), quote!(#(#names2,)*)))
            }));
            let deploy_builder_data = if matches!(constructor, Some(c) if !c.parameters.is_empty()) {
                quote! {
                    [
                        &BYTECODE[..],
                        &alloy_sol_types::SolConstructor::abi_encode(&constructorCall { #args })[..]
                    ].concat().into()
                }
            } else {
                quote! {
                    ::core::clone::Clone::clone(&BYTECODE)
                }
            };

            (
                quote! {
                    #deploy_doc
                    #[inline]
                    pub fn deploy<N: alloy_contract::private::Network, T: alloy_contract::private::Transport + ::core::clone::Clone, P: alloy_contract::private::Provider<N, T>>(provider: P, #params)
                        -> impl ::core::future::Future<Output = alloy_contract::Result<#name<N, T, P>>>
                    {
                        #name::<N, T, P>::deploy(provider, #args)
                    }

                    #deploy_builder_doc
                    #[inline]
                    pub fn deploy_builder<N: alloy_contract::private::Network, T: alloy_contract::private::Transport + ::core::clone::Clone, P: alloy_contract::private::Provider<N, T>>(provider: P, #params)
                        -> alloy_contract::RawCallBuilder<N, T, P>
                    {
                        #name::<N, T, P>::deploy_builder(provider, #args)
                    }
                },
                quote! {
                    #deploy_doc
                    #[inline]
                    pub async fn deploy(provider: P, #params)
                        -> alloy_contract::Result<#name<N, T, P>>
                    {
                        let call_builder = Self::deploy_builder(provider, #args);
                        let contract_address = call_builder.deploy().await?;
                        Ok(Self::new(contract_address, call_builder.provider))
                    }

                    #deploy_builder_doc
                    #[inline]
                    pub fn deploy_builder(provider: P, #params)
                        -> alloy_contract::RawCallBuilder<N, T, P>
                    {
                        alloy_contract::RawCallBuilder::new_raw(provider, #deploy_builder_data)
                    }
                },
            )
        }));

        let alloy_contract = &cx.crates.contract;

        quote! {
            use #alloy_contract as alloy_contract;

            #[doc = #new_fn_doc]
            #[inline]
            pub const fn new<N: alloy_contract::private::Network, T: alloy_contract::private::Transport + ::core::clone::Clone, P: alloy_contract::private::Provider<N, T>>(
                address: alloy_sol_types::private::Address,
                provider: P,
            ) -> #name<N, T, P> {
                #name::<N, T, P>::new(address, provider)
            }

            #deploy_fn

            #[doc = #struct_doc]
            #[derive(Clone)]
            pub struct #name<N, T, P> {
                address: alloy_sol_types::private::Address,
                provider: P,
                _network_transport: ::core::marker::PhantomData<(N, T)>,
            }

            #[automatically_derived]
            impl<N, T, P> ::core::fmt::Debug for #name<N, T, P> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_tuple(#name_s).field(&self.address).finish()
                }
            }

            /// Instantiation and getters/setters.
            #[automatically_derived]
            impl<N: alloy_contract::private::Network, T: alloy_contract::private::Transport + ::core::clone::Clone, P: alloy_contract::private::Provider<N, T>> #name<N, T, P> {
                #[doc = #new_fn_doc]
                #[inline]
                pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
                    Self { address, provider, _network_transport: ::core::marker::PhantomData }
                }

                #deploy_method

                /// Returns a reference to the address.
                #[inline]
                pub const fn address(&self) -> &alloy_sol_types::private::Address {
                    &self.address
                }

                /// Sets the address.
                #[inline]
                pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
                    self.address = address;
                }

                /// Sets the address and returns `self`.
                pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
                    self.set_address(address);
                    self
                }

                /// Returns a reference to the provider.
                #[inline]
                pub const fn provider(&self) -> &P {
                    &self.provider
                }
            }

            impl<N, T, P: ::core::clone::Clone> #name<N, T, &P> {
                /// Clones the provider and returns a new instance with the cloned provider.
                #[inline]
                pub fn with_cloned_provider(self) -> #name<N, T, P> {
                    #name { address: self.address, provider: ::core::clone::Clone::clone(&self.provider), _network_transport: ::core::marker::PhantomData }
                }
            }

            /// Function calls.
            #[automatically_derived]
            impl<N: alloy_contract::private::Network, T: alloy_contract::private::Transport + ::core::clone::Clone, P: alloy_contract::private::Provider<N, T>> #name<N, T, P> {
                /// Creates a new call builder using this contract instance's provider and address.
                ///
                /// Note that the call can be any function call, not just those defined in this
                /// contract. Prefer using the other methods for building type-safe contract calls.
                pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C)
                    -> alloy_contract::SolCallBuilder<N, T, &P, C>
                {
                    alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
                }

                #(#methods)*
            }
        }
    });

    let alloy_sol_types = &cx.crates.sol_types;

    let tokens = quote! {
        #mod_descr_doc
        #(#mod_attrs)*
        #mod_iface_doc
        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        pub mod #name {
            use super::*;
            use #alloy_sol_types as alloy_sol_types;

            #bytecode
            #deployed_bytecode

            #item_tokens

            #functions_enum
            #errors_enum
            #events_enum

            #abi

            #rpc
        }
    };
    Ok(tokens)
}

// note that item impls generated here do not need to be wrapped in an anonymous
// constant (`const _: () = { ... };`) because they are in one already

/// Expands a `SolInterface` enum:
///
/// ```ignore (pseudo-code)
/// #name = #{contract_name}Calls | #{contract_name}Errors | #{contract_name}Events;
///
/// pub enum #name {
///    #(#variants(#types),)*
/// }
///
/// impl SolInterface for #name {
///     ...
/// }
///
/// impl #name {
///     pub const SELECTORS: &'static [[u8; _]] = &[...];
/// }
///
/// #if extra_methods
/// #(
///     impl From<#types> for #name { ... }
///     impl TryFrom<#name> for #types { ... }
/// )*
///
/// impl #name {
///     #(
///         pub fn #is_variant,#as_variant,#as_variant_mut(...) -> ... { ... }
///     )*
/// }
/// #endif
/// ```
struct CallLikeExpander<'a> {
    cx: &'a ExpCtxt<'a>,
    contract_name: SolIdent,
    extra_methods: bool,
}

#[derive(Clone, Debug)]
struct ExpandData {
    name: Ident,
    variants: Vec<Ident>,
    types: Option<Vec<Ident>>,
    min_data_len: usize,
    trait_: Ident,
    selectors: Vec<ExprArray<u8>>,
}

impl ExpandData {
    fn types(&self) -> &Vec<Ident> {
        let types = self.types.as_ref().unwrap_or(&self.variants);
        assert_eq!(types.len(), self.variants.len());
        types
    }

    fn sort_by_selector(&mut self) {
        let len = self.selectors.len();
        if len <= 1 {
            return;
        }

        let prev = self.selectors.clone();
        self.selectors.sort_unstable();
        // Arbitrary max length.
        if len <= 20 && prev == self.selectors {
            return;
        }

        let old_variants = self.variants.clone();
        let old_types = self.types.clone();
        let new_idxs =
            prev.iter().map(|selector| self.selectors.iter().position(|s| s == selector).unwrap());
        for (old, new) in new_idxs.enumerate() {
            if old == new {
                continue;
            }

            self.variants[new] = old_variants[old].clone();
            if let Some(types) = self.types.as_mut() {
                types[new] = old_types.as_ref().unwrap()[old].clone();
            }
        }
    }
}

enum ToExpand<'a> {
    Functions(&'a [ItemFunction]),
    Errors(&'a [&'a ItemError]),
    Events(&'a [&'a ItemEvent]),
}

impl<'a> ToExpand<'a> {
    fn to_data(&self, expander: &CallLikeExpander<'_>) -> ExpandData {
        let &CallLikeExpander { cx, ref contract_name, .. } = expander;
        match self {
            Self::Functions(functions) => {
                let variants: Vec<_> =
                    functions.iter().map(|f| cx.overloaded_name(f.into()).0).collect();

                let types: Vec<_> = variants.iter().map(|name| cx.raw_call_name(name)).collect();

                ExpandData {
                    name: format_ident!("{contract_name}Calls"),
                    variants,
                    types: Some(types),
                    min_data_len: functions
                        .iter()
                        .map(|function| ty::params_base_data_size(cx, &function.parameters))
                        .min()
                        .unwrap(),
                    trait_: format_ident!("SolCall"),
                    selectors: functions.iter().map(|f| cx.function_selector(f)).collect(),
                }
            }

            Self::Errors(errors) => ExpandData {
                name: format_ident!("{contract_name}Errors"),
                variants: errors.iter().map(|error| error.name.0.clone()).collect(),
                types: None,
                min_data_len: errors
                    .iter()
                    .map(|error| ty::params_base_data_size(cx, &error.parameters))
                    .min()
                    .unwrap(),
                trait_: format_ident!("SolError"),
                selectors: errors.iter().map(|e| cx.error_selector(e)).collect(),
            },

            Self::Events(events) => {
                let variants: Vec<_> =
                    events.iter().map(|&event| cx.overloaded_name(event.into()).0).collect();

                ExpandData {
                    name: format_ident!("{contract_name}Events"),
                    variants,
                    types: None,
                    min_data_len: events
                        .iter()
                        .map(|event| ty::params_base_data_size(cx, &event.params()))
                        .min()
                        .unwrap(),
                    trait_: format_ident!("SolEvent"),
                    selectors: events.iter().map(|e| cx.event_selector(e)).collect(),
                }
            }
        }
    }
}

impl<'a> CallLikeExpander<'a> {
    fn expand(&self, to_expand: ToExpand<'_>, attrs: Vec<Attribute>) -> TokenStream {
        let data = &to_expand.to_data(self);

        let mut sorted_data = data.clone();
        sorted_data.sort_by_selector();
        #[cfg(debug_assertions)]
        for (i, sv) in sorted_data.variants.iter().enumerate() {
            let s = &sorted_data.selectors[i];

            let normal_pos = data.variants.iter().position(|v| v == sv).unwrap();
            let ns = &data.selectors[normal_pos];
            assert_eq!(s, ns);
        }

        if let ToExpand::Events(events) = to_expand {
            return self.expand_events(events, data, &sorted_data, attrs);
        }

        let def = self.generate_enum(data, &sorted_data, attrs);
        let ExpandData { name, variants, min_data_len, trait_, .. } = data;
        let types = data.types();
        let name_s = name.to_string();
        let count = data.variants.len();

        let sorted_variants = &sorted_data.variants;
        let sorted_types = sorted_data.types();

        quote! {
            #def

            #[automatically_derived]
            impl alloy_sol_types::SolInterface for #name {
                const NAME: &'static str = #name_s;
                const MIN_DATA_LENGTH: usize = #min_data_len;
                const COUNT: usize = #count;

                #[inline]
                fn selector(&self) -> [u8; 4] {
                    match self {#(
                        Self::#variants(_) => <#types as alloy_sol_types::#trait_>::SELECTOR,
                    )*}
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
                #[allow(unsafe_code, non_snake_case)]
                fn abi_decode_raw(
                    selector: [u8; 4],
                    data: &[u8],
                    validate: bool
                )-> alloy_sol_types::Result<Self> {
                    static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<#name>] = &[
                        #({
                            fn #sorted_variants(data: &[u8], validate: bool) -> alloy_sol_types::Result<#name> {
                                <#sorted_types as alloy_sol_types::#trait_>::abi_decode_raw(data, validate)
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
                    // SAFETY: `idx` is a valid index into `DECODE_SHIMS`.
                    (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
                }

                #[inline]
                fn abi_encoded_size(&self) -> usize {
                    match self {#(
                        Self::#variants(inner) =>
                            <#types as alloy_sol_types::#trait_>::abi_encoded_size(inner),
                    )*}
                }

                #[inline]
                fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                    match self {#(
                        Self::#variants(inner) =>
                            <#types as alloy_sol_types::#trait_>::abi_encode_raw(inner, out),
                    )*}
                }
            }
        }
    }

    fn expand_events(
        &self,
        events: &[&ItemEvent],
        data: &ExpandData,
        sorted_data: &ExpandData,
        attrs: Vec<Attribute>,
    ) -> TokenStream {
        let def = self.generate_enum(data, sorted_data, attrs);
        let ExpandData { name, trait_, .. } = data;
        let name_s = name.to_string();
        let count = data.variants.len();

        let has_anon = events.iter().any(|e| e.is_anonymous());
        let has_non_anon = events.iter().any(|e| !e.is_anonymous());
        assert!(has_anon || has_non_anon, "events shouldn't be empty");

        let e_name = |&e: &&ItemEvent| self.cx.overloaded_name(e.into());
        let err = quote! {
            alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                log: alloy_sol_types::private::Box::new(alloy_sol_types::private::LogData::new_unchecked(
                    topics.to_vec(),
                    data.to_vec().into(),
                )),
            })
        };
        let non_anon_impl = has_non_anon.then(|| {
            let variants = events.iter().filter(|e| !e.is_anonymous()).map(e_name);
            let ret = has_anon.then(|| quote!(return));
            let ret_err = (!has_anon).then_some(&err);
            quote! {
                match topics.first().copied() {
                    #(
                        Some(<#variants as alloy_sol_types::#trait_>::SIGNATURE_HASH) =>
                            #ret <#variants as alloy_sol_types::#trait_>::decode_raw_log(topics, data, validate)
                                .map(Self::#variants),
                    )*
                    _ => { #ret_err }
                }
            }
        });
        let anon_impl = has_anon.then(|| {
            let variants = events.iter().filter(|e| e.is_anonymous()).map(e_name);
            quote! {
                #(
                    if let Ok(res) = <#variants as alloy_sol_types::#trait_>::decode_raw_log(topics, data, validate) {
                        return Ok(Self::#variants(res));
                    }
                )*
                #err
            }
        });

        quote! {
            #def

            impl alloy_sol_types::SolEventInterface for #name {
                const NAME: &'static str = #name_s;
                const COUNT: usize = #count;

                fn decode_raw_log(topics: &[alloy_sol_types::Word], data: &[u8], validate: bool) -> alloy_sol_types::Result<Self> {
                    #non_anon_impl
                    #anon_impl
                }
            }
        }
    }

    fn generate_enum(
        &self,
        data: &ExpandData,
        sorted_data: &ExpandData,
        mut attrs: Vec<Attribute>,
    ) -> TokenStream {
        let ExpandData { name, variants, .. } = data;
        let types = data.types();

        let selectors = &sorted_data.selectors;

        let selector_len = selectors.first().unwrap().array.len();
        assert!(selectors.iter().all(|s| s.array.len() == selector_len));
        let selector_type = quote!([u8; #selector_len]);

        self.cx.type_derives(&mut attrs, types.iter().cloned().map(ast::Type::custom), false);

        let mut tokens = quote! {
            #(#attrs)*
            pub enum #name {
                #(#variants(#types),)*
            }

            #[automatically_derived]
            impl #name {
                /// All the selectors of this enum.
                ///
                /// Note that the selectors might not be in the same order as the variants.
                /// No guarantees are made about the order of the selectors.
                ///
                /// Prefer using `SolInterface` methods instead.
                // NOTE: This is currently sorted to allow for binary search in `SolInterface`.
                pub const SELECTORS: &'static [#selector_type] = &[#(#selectors),*];
            }
        };

        if self.extra_methods {
            let conversions =
                variants.iter().zip(types).map(|(v, t)| generate_variant_conversions(name, v, t));
            let methods = variants.iter().zip(types).map(generate_variant_methods);
            tokens.extend(conversions);
            tokens.extend(quote! {
                #[automatically_derived]
                impl #name {
                    #(#methods)*
                }
            });
        }

        tokens
    }
}

fn generate_variant_conversions(name: &Ident, variant: &Ident, ty: &Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<#ty> for #name {
            #[inline]
            fn from(value: #ty) -> Self {
                Self::#variant(value)
            }
        }

        #[automatically_derived]
        impl ::core::convert::TryFrom<#name> for #ty {
            type Error = #name;

            #[inline]
            fn try_from(value: #name) -> ::core::result::Result<Self, #name> {
                match value {
                    #name::#variant(value) => ::core::result::Result::Ok(value),
                    _ => ::core::result::Result::Err(value),
                }
            }
        }
    }
}

fn generate_variant_methods((variant, ty): (&Ident, &Ident)) -> TokenStream {
    let name_snake = snakify(&variant.to_string());

    let is_variant = format_ident!("is_{name_snake}");
    let is_variant_doc =
        format!("Returns `true` if `self` matches [`{variant}`](Self::{variant}).");

    let as_variant = format_ident!("as_{name_snake}");
    let as_variant_doc = format!(
        "Returns an immutable reference to the inner [`{ty}`] if `self` matches [`{variant}`](Self::{variant})."
    );

    let as_variant_mut = format_ident!("as_{name_snake}_mut");
    let as_variant_mut_doc = format!(
        "Returns a mutable reference to the inner [`{ty}`] if `self` matches [`{variant}`](Self::{variant})."
    );

    quote! {
        #[doc = #is_variant_doc]
        #[inline]
        pub const fn #is_variant(&self) -> bool {
            ::core::matches!(self, Self::#variant(_))
        }

        #[doc = #as_variant_doc]
        #[inline]
        pub const fn #as_variant(&self) -> ::core::option::Option<&#ty> {
            match self {
                Self::#variant(inner) => ::core::option::Option::Some(inner),
                _ => ::core::option::Option::None,
            }
        }

        #[doc = #as_variant_mut_doc]
        #[inline]
        pub fn #as_variant_mut(&mut self) -> ::core::option::Option<&mut #ty> {
            match self {
                Self::#variant(inner) => ::core::option::Option::Some(inner),
                _ => ::core::option::Option::None,
            }
        }
    }
}

fn call_builder_method(f: &ItemFunction, cx: &ExpCtxt<'_>) -> TokenStream {
    let name = cx.function_name(f);
    let call_name = cx.call_name(f);
    let param_names1 = f.parameters.names().enumerate().map(anon_name);
    let param_names2 = param_names1.clone();
    let param_tys = f.parameters.types().map(|ty| super::ty::expand_rust_type(ty, &cx.crates));
    let doc = format!("Creates a new call builder for the [`{name}`] function.");
    quote! {
        #[doc = #doc]
        pub fn #name(&self, #(#param_names1: #param_tys),*) -> alloy_contract::SolCallBuilder<N, T, &P, #call_name> {
            self.call_builder(&#call_name { #(#param_names2),* })
        }
    }
}

/// `heck` doesn't treat numbers as new words, and discards leading underscores.
fn snakify(s: &str) -> String {
    let leading_n = s.chars().take_while(|c| *c == '_').count();
    let (leading, s) = s.split_at(leading_n);
    let mut output: Vec<char> = leading.chars().chain(s.to_snake_case().chars()).collect();

    let mut num_starts = vec![];
    for (pos, c) in output.iter().enumerate() {
        if pos != 0
            && c.is_ascii_digit()
            && !output[pos - 1].is_ascii_digit()
            && !output[pos - 1].is_ascii_punctuation()
        {
            num_starts.push(pos);
        }
    }
    // need to do in reverse, because after inserting, all chars after the point of
    // insertion are off
    for i in num_starts.into_iter().rev() {
        output.insert(i, '_');
    }
    output.into_iter().collect()
}

// TODO(MSRV-1.66): Option::unzip
fn option_unzip<T, U>(opt: Option<(T, U)>) -> (Option<T>, Option<U>) {
    match opt {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    }
}
