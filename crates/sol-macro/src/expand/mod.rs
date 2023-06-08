//! Functions which generate Rust code from the Solidity AST.

use ast::{
    File, Item, ItemContract, ItemError, ItemFunction, ItemStruct, ItemUdt, Parameters, SolIdent,
    Type, VariableDeclaration, Visit,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, IdentFragment};
use std::{collections::HashMap, fmt::Write};
use syn::{parse_quote, Attribute, Error, Result, Token};

mod attr;

mod r#type;
pub use r#type::expand_type;
use r#type::TypePrinter;

/// The limit for the number of times to resolve a type.
const RESOLVE_LIMIT: usize = 16;

/// The [`sol!`][crate::sol!] expansion implementation.
pub fn expand(ast: File) -> Result<TokenStream> {
    ExpCtxt::new(&ast).expand()
}

fn expand_var(var: &VariableDeclaration) -> TokenStream {
    let VariableDeclaration { ty, name, .. } = var;
    let ty = expand_type(ty);
    quote! {
        #name: <#ty as ::alloy_sol_types::SolType>::RustType
    }
}

struct ExpCtxt<'ast> {
    all_items: Vec<&'ast Item>,
    custom_types: HashMap<SolIdent, Type>,

    /// `name => functions`
    functions: HashMap<String, Vec<&'ast ItemFunction>>,
    /// `function_signature => new_name`
    function_overloads: HashMap<String, String>,

    ast: &'ast File,
}

// expand
impl<'ast> ExpCtxt<'ast> {
    fn new(ast: &'ast File) -> Self {
        Self {
            all_items: Vec::new(),
            custom_types: HashMap::new(),
            functions: HashMap::new(),
            function_overloads: HashMap::new(),
            ast,
        }
    }

    fn expand(mut self) -> Result<TokenStream> {
        self.visit_file(self.ast);
        if self.all_items.len() > 1 {
            self.resolve_custom_types()?;
            self.mk_overloads_map()?;
        }

        let mut tokens = TokenStream::new();
        for item in &self.ast.items {
            let t = match self.expand_item(item) {
                Ok(t) => t,
                Err(e) => {
                    // TODO: Dummy items
                    e.into_compile_error()
                }
            };
            tokens.extend(t);
        }
        Ok(tokens)
    }

    fn expand_item(&self, item: &Item) -> Result<TokenStream> {
        match item {
            Item::Contract(contract) => self.expand_contract(contract),
            Item::Error(error) => self.expand_error(error),
            Item::Function(function) => self.expand_function(function),
            Item::Struct(s) => self.expand_struct(s),
            Item::Udt(udt) => self.expand_udt(udt),
        }
    }

    fn expand_contract(&self, contract: &ItemContract) -> Result<TokenStream> {
        let ItemContract {
            attrs, name, body, ..
        } = contract;

        let mut functions = Vec::with_capacity(contract.body.len());
        let mut errors = Vec::with_capacity(contract.body.len());
        let mut item_tokens = TokenStream::new();
        let d_attrs: Vec<Attribute> = attr::derives(attrs).cloned().collect();
        for item in body {
            match item {
                Item::Function(function) => functions.push(function),
                Item::Error(error) => errors.push(error),
                _ => {}
            }
            item_tokens.extend(quote!(#(#d_attrs)*));
            item_tokens.extend(self.expand_item(item)?);
        }

        let functions_enum = if functions.len() > 1 {
            let mut attrs = d_attrs.clone();
            let doc_str = format!("Container for all the [`{name}`] function calls.");
            attrs.push(parse_quote!(#[doc = #doc_str]));
            Some(self.expand_functions_enum(name, functions, &attrs))
        } else {
            None
        };

        let errors_enum = if errors.len() > 1 {
            let mut attrs = d_attrs;
            let doc_str = format!("Container for all the [`{name}`] custom errors.");
            attrs.push(parse_quote!(#[doc = #doc_str]));
            Some(self.expand_errors_enum(name, errors, &attrs))
        } else {
            None
        };

        let mod_attrs = attr::docs(attrs);
        let tokens = quote! {
            #(#mod_attrs)*
            #[allow(non_camel_case_types, non_snake_case, clippy::style)]
            pub mod #name {
                #item_tokens
                #functions_enum
                #errors_enum
            }
        };
        Ok(tokens)
    }

    fn expand_functions_enum(
        &self,
        name: &SolIdent,
        functions: Vec<&ItemFunction>,
        attrs: &[Attribute],
    ) -> TokenStream {
        let name = format_ident!("{name}Calls");
        let variants: Vec<_> = functions
            .iter()
            .map(|f| self.function_name_ident(f).0)
            .collect();
        let types: Vec<_> = variants.iter().map(|name| self.call_name(name)).collect();
        let min_data_len = functions
            .iter()
            .map(|function| self.min_data_size(&function.arguments))
            .max()
            .unwrap();
        let trt = Ident::new("SolCall", Span::call_site());
        self.expand_call_like_enum(name, &variants, &types, min_data_len, trt, attrs)
    }

    fn expand_errors_enum(
        &self,
        name: &SolIdent,
        errors: Vec<&ItemError>,
        attrs: &[Attribute],
    ) -> TokenStream {
        let name = format_ident!("{name}Errors");
        let variants: Vec<_> = errors.iter().map(|error| error.name.0.clone()).collect();
        let min_data_len = errors
            .iter()
            .map(|error| self.min_data_size(&error.fields))
            .max()
            .unwrap();
        let trt = Ident::new("SolError", Span::call_site());
        self.expand_call_like_enum(name, &variants, &variants, min_data_len, trt, attrs)
    }

    fn expand_call_like_enum(
        &self,
        name: Ident,
        variants: &[Ident],
        types: &[Ident],
        min_data_len: usize,
        trt: Ident,
        attrs: &[Attribute],
    ) -> TokenStream {
        assert_eq!(variants.len(), types.len());
        let name_s = name.to_string();
        let count = variants.len();
        let min_data_len = min_data_len.min(4);
        quote! {
            #(#attrs)*
            pub enum #name {#(
                #variants(#types),
            )*}

            // TODO: Implement these functions using traits?
            #[automatically_derived]
            impl #name {
                /// The number of variants.
                pub const COUNT: usize = #count;

                // no decode_raw is possible because we need the selector to know which variant to
                // decode into

                /// ABI-decodes the given data into one of the variants of `self`.
                pub fn decode(data: &[u8], validate: bool) -> ::alloy_sol_types::Result<Self> {
                    if data.len() >= #min_data_len {
                        // TODO: Replace with `data.split_array_ref` once it's stable
                        let (selector, data) = data.split_at(4);
                        let selector: &[u8; 4] =
                            ::core::convert::TryInto::try_into(selector).expect("unreachable");
                        match *selector {
                            #(<#types as ::alloy_sol_types::#trt>::SELECTOR => {
                                return <#types as ::alloy_sol_types::#trt>::decode_raw(data, validate)
                                    .map(Self::#variants)
                            })*
                            _ => {}
                        }
                    }
                    ::core::result::Result::Err(::alloy_sol_types::Error::type_check_fail(
                        data,
                        #name_s,
                    ))
                }

                /// ABI-encodes `self` into the given buffer.
                pub fn encode_raw(&self, out: &mut Vec<u8>) {
                    match self {#(
                        Self::#variants(inner) =>
                            <#types as ::alloy_sol_types::#trt>::encode_raw(inner, out),
                    )*}
                }

                /// ABI-encodes `self` into the given buffer.
                #[inline]
                pub fn encode(&self) -> Vec<u8> {
                    match self {#(
                        Self::#variants(inner) =>
                            <#types as ::alloy_sol_types::#trt>::encode(inner),
                    )*}
                }
            }

            #(
                #[automatically_derived]
                impl From<#types> for #name {
                    fn from(value: #types) -> Self {
                        Self::#variants(value)
                    }
                }
            )*
        }
    }

    fn expand_error(&self, error: &ItemError) -> Result<TokenStream> {
        let ItemError {
            fields,
            name,
            attrs,
            ..
        } = error;
        self.assert_resolved(fields)?;

        let signature = self.signature(name.as_string(), fields);
        let selector = crate::utils::selector(&signature);

        let size = self.params_data_size(fields, None);

        let converts = expand_from_into_tuples(&name.0, fields);
        let fields = fields.iter().map(expand_var);
        let tokens = quote! {
            #(#attrs)*
            #[allow(non_camel_case_types, non_snake_case)]
            #[derive(Clone)]
            pub struct #name {
                #(pub #fields,)*
            }

            #[allow(non_camel_case_types, non_snake_case, clippy::style)]
            const _: () = {
                #converts

                #[automatically_derived]
                impl ::alloy_sol_types::SolError for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::alloy_sol_types::SolType>::TokenType;

                    const SIGNATURE: &'static str = #signature;
                    const SELECTOR: [u8; 4] = #selector;

                    fn to_rust(&self) -> <Self::Tuple as ::alloy_sol_types::SolType>::RustType {
                        self.clone().into()
                    }

                    fn from_rust(tuple: <Self::Tuple as ::alloy_sol_types::SolType>::RustType) -> Self {
                        tuple.into()
                    }

                    fn data_size(&self) -> usize {
                        #size
                    }
                }
            };
        };
        Ok(tokens)
    }

    fn expand_function(&self, function: &ItemFunction) -> Result<TokenStream> {
        let function_name = self.function_name(function);
        let call_name = self.call_name(function_name.clone());
        let mut tokens = self.expand_call(function, &call_name, &function.arguments)?;

        if let Some(ret) = &function.returns {
            assert!(!ret.returns.is_empty());
            let return_name = self.return_name(function_name);
            let ret = self.expand_call(function, &return_name, &ret.returns)?;
            tokens.extend(ret);
        }

        Ok(tokens)
    }

    fn expand_call(
        &self,
        function: &ItemFunction,
        call_name: &Ident,
        params: &Parameters<Token![,]>,
    ) -> Result<TokenStream> {
        self.assert_resolved(params)?;

        let fields = params.iter().map(expand_var);

        let signature = self.signature(function.name.as_string(), params);
        let selector = crate::utils::selector(&signature);

        let size = self.params_data_size(params, None);

        let converts = expand_from_into_tuples(call_name, params);

        let attrs = &function.attrs;
        let tokens = quote! {
            #(#attrs)*
            #[allow(non_camel_case_types, non_snake_case)]
            #[derive(Clone)]
            pub struct #call_name {
                #(pub #fields,)*
            }

            #[allow(non_camel_case_types, non_snake_case, clippy::style)]
            const _: () = {
                #converts

                #[automatically_derived]
                impl ::alloy_sol_types::SolCall for #call_name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::alloy_sol_types::SolType>::TokenType;

                    const SIGNATURE: &'static str = #signature;
                    const SELECTOR: [u8; 4] = #selector;

                    fn to_rust(&self) -> <Self::Tuple as ::alloy_sol_types::SolType>::RustType {
                        self.clone().into()
                    }

                    fn from_rust(tuple: <Self::Tuple as ::alloy_sol_types::SolType>::RustType) -> Self {
                        tuple.into()
                    }

                    fn data_size(&self) -> usize {
                        #size
                    }
                }
            };
        };
        Ok(tokens)
    }

    fn expand_struct(&self, s: &ItemStruct) -> Result<TokenStream> {
        let ItemStruct {
            name,
            fields,
            attrs,
            ..
        } = s;

        let (f_ty, f_name): (Vec<_>, Vec<_>) = s
            .fields
            .iter()
            .map(|f| (f.ty.to_string(), f.name.as_ref().unwrap().to_string()))
            .unzip();

        let props_tys: Vec<_> = fields.iter().map(|f| expand_type(&f.ty)).collect();
        let props = fields.iter().map(|f| &f.name);

        let encoded_type = fields.eip712_signature(name.to_string());
        let encode_type_impl = if fields.iter().any(|f| f.ty.is_custom()) {
            quote! {
                {
                    let mut encoded = String::from(#encoded_type);
                    #(
                        if let Some(s) = <#props_tys as ::alloy_sol_types::SolType>::eip712_encode_type() {
                            encoded.push_str(&s);
                        }
                    )*
                    encoded
                }
            }
        } else {
            quote!(#encoded_type)
        };

        let encode_data_impl = match fields.len() {
            0 => unreachable!(),
            1 => {
                let VariableDeclaration { ty, name, .. } = fields.first().unwrap();
                let ty = expand_type(ty);
                quote!(<#ty as ::alloy_sol_types::SolType>::eip712_data_word(&self.#name).0.to_vec())
            }
            _ => quote! {
                [#(
                    <#props_tys as ::alloy_sol_types::SolType>::eip712_data_word(&self.#props).0,
                )*].concat()
            },
        };

        let attrs = attrs.iter();
        let convert = expand_from_into_tuples(&name.0, fields);
        let name_s = name.to_string();
        let fields = fields.iter().map(expand_var);
        let tokens = quote! {
            #(#attrs)*
            #[allow(non_camel_case_types, non_snake_case)]
            #[derive(Clone)]
            pub struct #name {
                #(pub #fields),*
            }

            #[allow(non_camel_case_types, non_snake_case, clippy::style)]
            const _: () = {
                use ::alloy_sol_types::no_std_prelude::*;

                #convert

                #[automatically_derived]
                impl ::alloy_sol_types::SolStruct for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::alloy_sol_types::SolType>::TokenType;

                    const NAME: &'static str = #name_s;

                    const FIELDS: &'static [(&'static str, &'static str)] = &[
                        #((#f_ty, #f_name)),*
                    ];

                    fn to_rust(&self) -> UnderlyingRustTuple {
                        self.clone().into()
                    }

                    fn from_rust(tuple: UnderlyingRustTuple) -> Self {
                        tuple.into()
                    }

                    fn eip712_encode_type() -> Cow<'static, str> {
                        #encode_type_impl.into()
                    }

                    fn eip712_encode_data(&self) -> Vec<u8> {
                        #encode_data_impl
                    }
                }
            };
        };
        Ok(tokens)
    }

    fn expand_udt(&self, udt: &ItemUdt) -> Result<TokenStream> {
        let ItemUdt {
            name, ty, attrs, ..
        } = udt;
        let ty = expand_type(ty);
        let tokens = quote! {
            ::alloy_sol_types::define_udt! {
                #(#attrs)*
                #name,
                underlying: #ty,
            }
        };
        Ok(tokens)
    }
}

// resolve
impl<'ast> ExpCtxt<'ast> {
    fn mk_types_map(&mut self) {
        let mut map = std::mem::take(&mut self.custom_types);
        map.reserve(self.all_items.len());
        for &item in &self.all_items {
            if let Some(ty) = item.as_type() {
                map.insert(item.name().clone(), ty);
            }
        }
        self.custom_types = map;
    }

    fn resolve_custom_types(&mut self) -> Result<()> {
        self.mk_types_map();
        for _i in 0..RESOLVE_LIMIT {
            let mut any = false;
            // you won't get me this time, borrow checker
            let map_ref: &mut HashMap<SolIdent, Type> =
                unsafe { &mut *(&mut self.custom_types as *mut _) };
            for ty in map_ref.values_mut() {
                ty.visit_mut(|ty| {
                    let ty @ Type::Custom(_) = ty else { return };
                    let Type::Custom(name) = &*ty else { unreachable!() };
                    let Some(resolved) = self.custom_types.get(name.last_tmp()) else { return };
                    ty.clone_from(resolved);
                    any = true;
                });
            }
            if !any {
                // done
                return Ok(())
            }
        }

        let msg = format!(
            "failed to resolve types after {RESOLVE_LIMIT} iterations.\n\
             This is likely due to an infinitely recursive type definition.\n\
             If you believe this is a bug, please file an issue at \
             https://github.com/ethers-rs/core/issues/new/choose"
        );
        Err(Error::new(Span::call_site(), msg))
    }

    fn mk_overloads_map(&mut self) -> Result<()> {
        let all_orig_names: Vec<SolIdent> = self
            .functions
            .values()
            .flatten()
            .map(|f| f.name.clone())
            .collect();
        let mut overloads_map = std::mem::take(&mut self.function_overloads);

        // report all errors at the end
        let mut errors = Vec::new();

        for functions in self.functions.values().filter(|fs| fs.len() >= 2) {
            // check for same parameters
            for (i, a) in functions.iter().enumerate() {
                for b in functions.iter().skip(i + 1) {
                    if a.arguments.types().eq(b.arguments.types()) {
                        let msg = "function with same name and parameter types defined twice";
                        let mut err = syn::Error::new(a.name.span(), msg);

                        let msg = "other declaration is here";
                        let note = syn::Error::new(b.name.span(), msg);

                        err.combine(note);
                        errors.push(err);
                    }
                }
            }

            for (i, &function) in functions.iter().enumerate() {
                let old_name = &function.name;
                let new_name = format!("{old_name}_{i}");
                if let Some(other) = all_orig_names.iter().find(|x| x.0 == new_name) {
                    let msg = format!(
                        "function `{old_name}` is overloaded, \
                         but the generated name `{new_name}` is already in use"
                    );
                    let mut err = syn::Error::new(old_name.span(), msg);

                    let msg = "other declaration is here";
                    let note = syn::Error::new(other.span(), msg);

                    err.combine(note);
                    errors.push(err);
                }

                overloads_map.insert(self.function_signature(function), new_name);
            }
        }

        if errors.is_empty() {
            self.function_overloads = overloads_map;
            Ok(())
        } else {
            Err(crate::utils::combine_errors(errors).unwrap())
        }
    }

    /// Returns the name of the function, adjusted for overloads.
    fn function_name(&self, function: &ItemFunction) -> String {
        let sig = self.function_signature(function);
        match self.function_overloads.get(&sig) {
            Some(name) => name.clone(),
            None => function.name.as_string(),
        }
    }

    /// Returns the name of the function, adjusted for overloads.
    fn function_name_ident(&self, function: &ItemFunction) -> SolIdent {
        let sig = self.function_signature(function);
        match self.function_overloads.get(&sig) {
            Some(name) => SolIdent::new_spanned(name, function.name.span()),
            None => function.name.clone(),
        }
    }

    fn call_name(&self, function_name: impl IdentFragment + std::fmt::Display) -> Ident {
        format_ident!("{function_name}Call")
    }

    fn return_name(&self, function_name: impl IdentFragment + std::fmt::Display) -> Ident {
        format_ident!("{function_name}Return")
    }

    fn signature<P>(&self, mut name: String, params: &Parameters<P>) -> String {
        name.reserve(2 + params.len() * 16);
        name.push('(');
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                name.push(',');
            }
            write!(name, "{}", TypePrinter::new(self, &param.ty)).unwrap();
        }
        name.push(')');
        name
    }

    fn function_signature(&self, function: &ItemFunction) -> String {
        self.signature(function.name.as_string(), &function.arguments)
    }

    /// Returns an error if any of the types in the parameters are unresolved.
    ///
    /// Provides a better error message than an `unwrap` or `expect` when we
    /// know beforehand that we will be needing types to be resolved.
    fn assert_resolved<P>(&self, params: &Parameters<P>) -> Result<()> {
        let mut errors = Vec::new();
        params.visit_types(|ty| {
            if let Type::Custom(name) = ty {
                if !self.custom_types.contains_key(name.last_tmp()) {
                    let e = syn::Error::new(name.span(), "unresolved type");
                    errors.push(e);
                }
            }
        });
        if errors.is_empty() {
            Ok(())
        } else {
            let mut e = crate::utils::combine_errors(errors).unwrap();
            let note =
                "Custom types must be declared inside of the same scope they are referenced in,\n\
                 or \"imported\" as a UDT with `type {ident} is (...);`";
            e.combine(Error::new(Span::call_site(), note));
            Err(e)
        }
    }

    fn params_data_size<P>(&self, list: &Parameters<P>, base: Option<TokenStream>) -> TokenStream {
        let base = base.unwrap_or_else(|| quote!(self));
        let sizes = list.iter().map(|var| {
            let field = var.name.as_ref().unwrap();
            self.type_data_size(&var.ty, quote!(#base.#field))
        });
        quote!(0usize #( + #sizes)*)
    }
}

impl<'ast> Visit<'ast> for ExpCtxt<'ast> {
    fn visit_item(&mut self, item: &'ast Item) {
        self.all_items.push(item);
        ast::visit::visit_item(self, item);
    }

    fn visit_item_function(&mut self, function: &'ast ItemFunction) {
        self.functions
            .entry(function.name.as_string())
            .or_default()
            .push(function);
        ast::visit::visit_item_function(self, function);
    }
}

/// Expands `From` impls for a list of types and the corresponding tuple.
fn expand_from_into_tuples<P>(name: &Ident, fields: &Parameters<P>) -> TokenStream {
    let names = fields.names();
    let names2 = names.clone();
    let idxs = (0..fields.len()).map(syn::Index::from);

    let tys = fields.types().map(expand_type);
    let tys2 = tys.clone();

    quote! {
        type UnderlyingSolTuple = (#(#tys,)*);
        type UnderlyingRustTuple = (#(<#tys2 as ::alloy_sol_types::SolType>::RustType,)*);

        #[automatically_derived]
        impl ::core::convert::From<#name> for UnderlyingRustTuple {
            fn from(value: #name) -> Self {
                (#(value.#names,)*)
            }
        }

        #[automatically_derived]
        impl ::core::convert::From<UnderlyingRustTuple> for #name {
            fn from(tuple: UnderlyingRustTuple) -> Self {
                #name {
                    #(#names2: tuple.#idxs),*
                }
            }
        }
    }
}
