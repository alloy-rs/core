//! [`ItemContract`] expansion.

use super::{ty, ExpCtxt};
use crate::{attr, utils::ExprArray};
use ast::{Item, ItemContract, ItemError, ItemEvent, ItemFunction, SolIdent};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
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
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);

    let bytecode = sol_attrs.bytecode.map(|lit| {
        let name = Ident::new("BYTECODE", lit.span());
        quote! {
            /// The creation / init code of the contract.
            pub static #name: ::alloy_sol_types::private::Bytes = ::alloy_sol_types::private::bytes!(#lit);
        }
    });
    let deployed_bytecode = sol_attrs.deployed_bytecode.map(|lit| {
        let name = Ident::new("DEPLOYED_BYTECODE", lit.span());
        quote! {
            /// The runtime bytecode of the contract.
            pub static #name: ::alloy_sol_types::private::Bytes = ::alloy_sol_types::private::bytes!(#lit);
        }
    });

    let mut functions = Vec::with_capacity(contract.body.len());
    let mut errors = Vec::with_capacity(contract.body.len());
    let mut events = Vec::with_capacity(contract.body.len());

    let mut item_tokens = TokenStream::new();
    let (mut mod_attrs, item_attrs): (Vec<_>, _) =
        attrs.into_iter().partition(|a| a.path().is_ident("doc"));
    mod_attrs.extend(item_attrs.iter().filter(|a| !a.path().is_ident("derive")).cloned());

    for item in body {
        match item {
            Item::Function(function) if function.name.is_some() => functions.push(function),
            Item::Error(error) => errors.push(error),
            Item::Event(event) => events.push(event),
            _ => {}
        }

        if item.attrs().is_none() || item_attrs.is_empty() {
            // avoid cloning item if we don't have to
            item_tokens.extend(cx.expand_item(item)?);
        } else {
            // prepend `item_attrs` to `item.attrs`
            let mut item = item.clone();
            item.attrs_mut().unwrap().splice(0..0, item_attrs.clone());
            item_tokens.extend(cx.expand_item(&item)?);
        }
    }

    let enum_expander = CallLikeExpander { cx, contract_name: name.clone(), extra_methods };

    let functions_enum = (!functions.is_empty()).then(|| {
        let mut attrs = item_attrs.clone();
        let doc_str = format!("Container for all the [`{name}`](self) function calls.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        enum_expander.expand(ToExpand::Functions(functions), attrs)
    });

    let errors_enum = (!errors.is_empty()).then(|| {
        let mut attrs = item_attrs.clone();
        let doc_str = format!("Container for all the [`{name}`](self) custom errors.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        enum_expander.expand(ToExpand::Errors(errors), attrs)
    });

    let events_enum = (!events.is_empty()).then(|| {
        let mut attrs = item_attrs;
        let doc_str = format!("Container for all the [`{name}`](self) events.");
        attrs.push(parse_quote!(#[doc = #doc_str]));
        enum_expander.expand(ToExpand::Events(events), attrs)
    });

    let mod_descr_doc = (docs && attr::docs_str(&mod_attrs).trim().is_empty())
        .then(|| attr::mk_doc("Module containing a contract's types and functions."));
    let mod_iface_doc = (docs && !attr::docs_str(&mod_attrs).contains("```solidity\n"))
        .then(|| attr::mk_doc(format!("\n\n```solidity\n{contract}\n```")));
    let tokens = quote! {
        #mod_descr_doc
        #(#mod_attrs)*
        #mod_iface_doc
        #[allow(non_camel_case_types, non_snake_case, clippy::style)]
        pub mod #name {
            use super::*;

            #bytecode
            #deployed_bytecode

            #item_tokens

            #functions_enum
            #errors_enum
            #events_enum
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
}

enum ToExpand<'a> {
    Functions(Vec<&'a ItemFunction>),
    Errors(Vec<&'a ItemError>),
    Events(Vec<&'a ItemEvent>),
}

impl<'a> ToExpand<'a> {
    fn to_data(&self, expander: &CallLikeExpander<'_>) -> ExpandData {
        let &CallLikeExpander { cx, ref contract_name, .. } = expander;
        match self {
            Self::Functions(functions) => {
                let variants: Vec<_> =
                    functions.iter().map(|&f| cx.overloaded_name(f.into()).0).collect();

                let types: Vec<_> = variants.iter().map(|name| cx.raw_call_name(name)).collect();

                let mut selectors: Vec<_> =
                    functions.iter().map(|f| cx.function_selector(f)).collect();
                selectors.sort_unstable();

                ExpandData {
                    name: format_ident!("{contract_name}Calls"),
                    variants,
                    types: Some(types),
                    min_data_len: functions
                        .iter()
                        .map(|function| ty::params_base_data_size(cx, &function.arguments))
                        .min()
                        .unwrap(),
                    trait_: Ident::new("SolCall", Span::call_site()),
                    selectors,
                }
            }

            Self::Errors(errors) => {
                let mut selectors: Vec<_> = errors.iter().map(|e| cx.error_selector(e)).collect();
                selectors.sort_unstable();

                ExpandData {
                    name: format_ident!("{contract_name}Errors"),
                    variants: errors.iter().map(|error| error.name.0.clone()).collect(),
                    types: None,
                    min_data_len: errors
                        .iter()
                        .map(|error| ty::params_base_data_size(cx, &error.parameters))
                        .min()
                        .unwrap(),
                    trait_: Ident::new("SolError", Span::call_site()),
                    selectors,
                }
            }

            Self::Events(events) => {
                let variants: Vec<_> =
                    events.iter().map(|&event| cx.overloaded_name(event.into()).0).collect();

                let mut selectors: Vec<_> = events.iter().map(|e| cx.event_selector(e)).collect();
                selectors.sort_unstable();

                ExpandData {
                    name: format_ident!("{contract_name}Events"),
                    variants,
                    types: None,
                    min_data_len: events
                        .iter()
                        .map(|event| ty::params_base_data_size(cx, &event.params()))
                        .min()
                        .unwrap(),
                    trait_: Ident::new("SolEvent", Span::call_site()),
                    selectors,
                }
            }
        }
    }
}

impl<'a> CallLikeExpander<'a> {
    fn expand(&self, to_expand: ToExpand<'_>, attrs: Vec<Attribute>) -> TokenStream {
        let data @ ExpandData { name, variants, min_data_len, trait_, .. } =
            &to_expand.to_data(self);
        let types = data.types();
        let name_s = name.to_string();
        let count = variants.len();
        let def = self.generate_enum(data, attrs);

        // TODO: SolInterface for events
        if matches!(to_expand, ToExpand::Events(_)) {
            return def;
        }

        quote! {
            #def

            #[automatically_derived]
            impl ::alloy_sol_types::SolInterface for #name {
                const NAME: &'static str = #name_s;
                const MIN_DATA_LENGTH: usize = #min_data_len;
                const COUNT: usize = #count;

                #[inline]
                fn selector(&self) -> [u8; 4] {
                    match self {#(
                        Self::#variants(_) => <#types as ::alloy_sol_types::#trait_>::SELECTOR,
                    )*}
                }

                #[inline]
                fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
                    Self::SELECTORS.get(i).copied()
                }

                #[inline]
                fn valid_selector(selector: [u8; 4]) -> bool {
                    ::core::matches!(selector, #(<#types as ::alloy_sol_types::#trait_>::SELECTOR)|*)
                }

                #[inline]
                fn abi_decode_raw(
                    selector: [u8; 4],
                    data: &[u8],
                    validate: bool
                )-> ::alloy_sol_types::Result<Self> {
                    match selector {
                        #(<#types as ::alloy_sol_types::#trait_>::SELECTOR => {
                            <#types as ::alloy_sol_types::#trait_>::abi_decode_raw(data, validate)
                                .map(Self::#variants)
                        })*
                        s => ::core::result::Result::Err(::alloy_sol_types::Error::unknown_selector(
                            <Self as ::alloy_sol_types::SolInterface>::NAME,
                            s,
                        )),
                    }
                }

                #[inline]
                fn abi_encoded_size(&self) -> usize {
                    match self {#(
                        Self::#variants(inner) =>
                            <#types as ::alloy_sol_types::#trait_>::abi_encoded_size(inner),
                    )*}
                }

                #[inline]
                fn abi_encode_raw(&self, out: &mut ::alloy_sol_types::private::Vec<u8>) {
                    match self {#(
                        Self::#variants(inner) =>
                            <#types as ::alloy_sol_types::#trait_>::abi_encode_raw(inner, out),
                    )*}
                }
            }
        }
    }

    fn generate_enum(&self, data: &ExpandData, mut attrs: Vec<Attribute>) -> TokenStream {
        let ExpandData { name, variants, selectors, .. } = data;
        let types = data.types();
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
                /// Note that the selectors might not be in the same order as the
                /// variants, as they are sorted instead of ordered by definition.
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
