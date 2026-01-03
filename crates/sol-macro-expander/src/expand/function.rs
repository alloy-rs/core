//! [`ItemFunction`] expansion.

use super::{ExpCtxt, FieldKind, anon_name, expand_fields, expand_tokenize, expand_tuple_types};
use crate::codegen::{CallCodegen, ReturnInfo, gen_from_into_tuple};
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
    }

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

    let (call_names, (call_sol, call_rust)): (Vec<_>, (Vec<_>, Vec<_>)) = parameters
        .iter()
        .enumerate()
        .map(|(i, p)| (anon_name((i, p.name.as_ref())), (cx.expand_type(&p.ty), cx.expand_rust_type(&p.ty))))
        .unzip();
    let converts = gen_from_into_tuple(&call_name, &call_names, &call_sol, &call_rust, FieldKind::Deconstruct.into_layout(parameters));

    let (ret_names, (ret_sol, ret_rust)): (Vec<_>, (Vec<_>, Vec<_>)) = returns
        .iter()
        .enumerate()
        .map(|(i, p)| (anon_name((i, p.name.as_ref())), (cx.expand_type(&p.ty), cx.expand_rust_type(&p.ty))))
        .unzip();
    let return_converts = gen_from_into_tuple(&return_name, &ret_names, &ret_sol, &ret_rust, FieldKind::Original.into_layout(returns));

    let signature = cx.function_signature(function);
    let selector = crate::utils::calc_selector(&signature);
    let tokenize_impl = expand_tokenize(parameters, cx, FieldKind::Deconstruct);

    let call_doc = docs.then(|| {
        let selector = hex::encode_prefixed(selector.as_slice());
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

    // Determine whether the return type should directly yield result or the <name>Return struct.
    let is_single_return = returns.len() == 1;
    let return_info = if returns.is_empty() {
        ReturnInfo::Empty { return_name: return_name.clone() }
    } else if is_single_return {
        ReturnInfo::Single {
            sol_type: cx.expand_type(&returns[0].ty),
            rust_type: cx.expand_rust_type(&returns[0].ty),
            field_name: anon_name((0, returns[0].name.as_ref())),
            return_name: return_name.clone(),
        }
    } else {
        ReturnInfo::Multiple { return_name: return_name.clone() }
    };

    // For non-single returns, we need a helper method on the return struct
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

    let call_impl = CallCodegen { call_tuple, return_tuple, tokenize_impl, return_info }
        .expand(&call_name, &signature);

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

            #call_impl

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
    let (param_names, (sol_types, rust_types)): (Vec<_>, (Vec<_>, Vec<_>)) = parameters
        .iter()
        .enumerate()
        .map(|(i, p)| (anon_name((i, p.name.as_ref())), (cx.expand_type(&p.ty), cx.expand_rust_type(&p.ty))))
        .unzip();
    let converts = gen_from_into_tuple(&call_name, &param_names, &sol_types, &rust_types, FieldKind::Original.into_layout(parameters));
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
