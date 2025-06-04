//! [`ItemFunction`] expansion.

use super::{
    ExpCtxt, FieldKind, anon_name, expand_fields, expand_from_into_tuples, expand_tokenize,
    expand_tuple_types,
};
use alloy_sol_macro_input::{ContainsSolAttrs, mk_doc};
use ast::{FunctionKind, ItemFunction, Spanned};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Result;

/// Expands an [`ItemFunction`]:
///
/// ```ignore (pseudo-code)
/// pub struct #{name}Call {
///     #(pub #argument_name: #argument_type,)*
/// }
///
/// pub struct #{name}Return {
///     #(pub #return_name: #return_type,)*
/// }
///
/// impl SolCall for #{name}Call {
///     type Return = #{name}Return;
///     ...
/// }
/// ```
pub(super) fn expand(cx: &ExpCtxt<'_>, function: &ItemFunction) -> Result<TokenStream> {
    let ItemFunction { parameters, returns, name, kind, .. } = function;

    if matches!(kind, FunctionKind::Constructor(_)) {
        return expand_constructor(cx, function);
    }

    if name.is_none() {
        // ignore functions without names (modifiers...)
        return Ok(quote!());
    };

    let returns = returns.as_ref().map(|r| &r.returns).unwrap_or_default();

    cx.assert_resolved(parameters)?;
    if !returns.is_empty() {
        cx.assert_resolved(returns)?;
    }

    let (sol_attrs, mut call_attrs) = function.split_attrs()?;
    let mut return_attrs = call_attrs.clone();
    cx.derives(&mut call_attrs, parameters, true);
    if !returns.is_empty() {
        cx.derives(&mut return_attrs, returns, true);
    }
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);
    let abi = sol_attrs.abi.or(cx.attrs.abi).unwrap_or(false);

    let call_name = cx.call_name(function);
    let return_name = cx.return_name(function);

    let call_fields = expand_fields(parameters, cx);
    let return_fields = expand_fields(returns, cx);

    let call_tuple = expand_tuple_types(parameters.types(), cx).0;
    let return_tuple = expand_tuple_types(returns.types(), cx).0;

    let converts = expand_from_into_tuples(&call_name, parameters, cx, FieldKind::Deconstruct);
    let return_converts = expand_from_into_tuples(&return_name, returns, cx, FieldKind::Original);

    let signature = cx.function_signature(function);
    let selector = crate::utils::selector(&signature);
    let tokenize_impl = expand_tokenize(parameters, cx, FieldKind::Deconstruct);

    let call_doc = docs.then(|| {
        let selector = hex::encode_prefixed(selector.array.as_slice());
        mk_doc(format!(
            "Function with signature `{signature}` and selector `{selector}`.\n\
            ```solidity\n{function}\n```"
        ))
    });
    let return_doc = docs.then(|| {
        mk_doc(format!(
            "Container type for the return parameters of the [`{signature}`]({call_name}) function."
        ))
    });

    let abi: Option<TokenStream> = abi.then(|| {
        if_json! {
            let function = super::to_abi::generate(function, cx);
            quote! {
                #[automatically_derived]
                impl alloy_sol_types::JsonAbiExt for #call_name {
                    type Abi = alloy_sol_types::private::alloy_json_abi::Function;

                    #[inline]
                    fn abi() -> Self::Abi {
                        #function
                    }
                }
            }
        }
    });

    let call_struct = if parameters.is_empty() {
        quote! {
            pub struct #call_name;
        }
    } else if parameters.len() == 1 && parameters[0].name.is_none() {
        let ty = cx.expand_rust_type(&parameters[0].ty);
        quote! {
            pub struct #call_name(pub #ty);
        }
    } else {
        quote! {
            pub struct #call_name {
                #(#call_fields),*
            }
        }
    };

    let alloy_sol_types = &cx.crates.sol_types;

    let decode_sequence =
        quote!(<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data));

    // Determine whether the return type should directly yield result or the <name>Return struct.
    let is_single_return = returns.len() == 1;
    let return_type =
        if is_single_return { cx.expand_rust_type(&returns[0].ty) } else { quote!(#return_name) };
    let tokenize_returns_impl = if is_single_return {
        quote!()
    } else {
        let imp = expand_tokenize(returns, cx, FieldKind::Original);
        quote! {
            impl #return_name {
                fn _tokenize(&self) -> <#call_name as alloy_sol_types::SolCall>::ReturnToken<'_> {
                    #imp
                }
            }
        }
    };
    let tokenize_returns = if is_single_return {
        let ty = cx.expand_type(&returns[0].ty);
        quote! { (<#ty as alloy_sol_types::SolType>::tokenize(ret),) }
    } else {
        quote! { #return_name::_tokenize(ret) }
    };
    let decode_returns = if is_single_return {
        let name = anon_name((0, returns[0].name.as_ref()));
        quote! {
            #decode_sequence.map(|r| {
                let r: #return_name = r.into();
                r.#name
            })
        }
    } else {
        quote!(#decode_sequence.map(Into::into))
    };

    let decode_sequence_validate = quote!(
        <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
    );
    let decode_returns_validate = if is_single_return {
        let name = anon_name((0, returns[0].name.as_ref()));
        quote! {
            #decode_sequence_validate.map(|r| {
                let r: #return_name = r.into();
                r.#name
            })
        }
    } else {
        quote!(#decode_sequence_validate.map(Into::into))
    };

    let tokens = quote! {
        #(#call_attrs)*
        #call_doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        #[derive(Clone)]
        #call_struct

        #(#return_attrs)*
        #return_doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        #[derive(Clone)]
        pub struct #return_name {
            #(#return_fields),*
        }

        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
        const _: () = {
            use #alloy_sol_types as alloy_sol_types;

            { #converts }
            { #return_converts }

            #tokenize_returns_impl

            #[automatically_derived]
            impl alloy_sol_types::SolCall for #call_name {
                type Parameters<'a> = #call_tuple;
                type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;

                type Return = #return_type;

                type ReturnTuple<'a> = #return_tuple;
                type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;

                const SIGNATURE: &'static str = #signature;
                const SELECTOR: [u8; 4] = #selector;

                #[inline]
                fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    #tokenize_impl
                }

                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    #tokenize_returns
                }

                #[inline]
                fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                    #decode_returns
                }

                #[inline]
                fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                    #decode_returns_validate
                }
            }

            #abi
        };
    };
    Ok(tokens)
}

fn expand_constructor(cx: &ExpCtxt<'_>, constructor: &ItemFunction) -> Result<TokenStream> {
    let ItemFunction { parameters, .. } = constructor;

    let (sol_attrs, call_attrs) = constructor.split_attrs()?;
    let docs = sol_attrs.docs.or(cx.attrs.docs).unwrap_or(true);

    let alloy_sol_types = &cx.crates.sol_types;

    let call_name = format_ident!("constructorCall").with_span(constructor.kind.span());
    let call_fields = expand_fields(parameters, cx);
    let call_tuple = expand_tuple_types(parameters.types(), cx).0;
    let converts = expand_from_into_tuples(&call_name, parameters, cx, FieldKind::Original);
    let tokenize_impl = expand_tokenize(parameters, cx, FieldKind::Original);

    let call_doc = docs.then(|| {
        mk_doc(format!(
            "Constructor`.\n\
            ```solidity\n{constructor}\n```"
        ))
    });

    let tokens = quote! {
        #(#call_attrs)*
        #call_doc
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        #[derive(Clone)]
        pub struct #call_name {
            #(#call_fields),*
        }

        const _: () = {
            use #alloy_sol_types as alloy_sol_types;

            { #converts }

            #[automatically_derived]
            impl alloy_sol_types::SolConstructor for #call_name {
                type Parameters<'a> = #call_tuple;
                type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;

                #[inline]
                fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
                    tuple.into()
                }

                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    #tokenize_impl
                }
            }
        };
    };
    Ok(tokens)
}
