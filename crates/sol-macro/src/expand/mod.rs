//! Functions which generate Rust code from the Solidity AST.

use ast::{
    CustomType, File, Item, ItemError, ItemFunction, ItemStruct, ItemUdt, Parameters, SolIdent,
    Type, VariableDeclaration,
};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::{collections::HashMap, num::NonZeroU16};
use syn::{Result, Token};

/// The limit for the number of times to resolve a type.
const RESOLVE_LIMIT: usize = 16;

/// The [`sol!`][crate::sol!] expansion implementation.
pub fn expand(ast: File) -> TokenStream {
    Context::new(ast)
        .expand()
        .unwrap_or_else(|e| e.into_compile_error())
}

pub fn expand_type(ty: &Type) -> TokenStream {
    let mut tokens = TokenStream::new();
    rec_expand_type(ty, &mut tokens);
    tokens
}

fn rec_expand_type(ty: &Type, tokens: &mut TokenStream) {
    let tts = match *ty {
        Type::Address(span, _) => quote_spanned! {span=>
            ::ethers_sol_types::sol_data::Address
        },
        Type::Bool(span) => quote_spanned! {span=> ::ethers_sol_types::sol_data::Bool },
        Type::String(span) => quote_spanned! {span=> ::ethers_sol_types::sol_data::String },

        Type::Bytes { span, size: None } => {
            quote_spanned! {span=> ::ethers_sol_types::sol_data::Bytes }
        }
        Type::Bytes {
            span,
            size: Some(size),
        } => {
            let size = Literal::u16_unsuffixed(size.get());
            quote_spanned! {span=>
                ::ethers_sol_types::sol_data::FixedBytes<#size>
            }
        }

        Type::Int { span, size } => {
            let size = Literal::u16_unsuffixed(size.map(NonZeroU16::get).unwrap_or(256));
            quote_spanned! {span=>
                ::ethers_sol_types::sol_data::Int<#size>
            }
        }
        Type::Uint { span, size } => {
            let size = Literal::u16_unsuffixed(size.map(NonZeroU16::get).unwrap_or(256));
            quote_spanned! {span=>
                ::ethers_sol_types::sol_data::Uint<#size>
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
                    ::ethers_sol_types::sol_data::FixedArray<#ty, #size>
                }
            } else {
                quote_spanned! {span=>
                    ::ethers_sol_types::sol_data::Array<#ty>
                }
            }
        }
        Type::Custom(ref custom) => return custom.ident().to_tokens(tokens),
    };
    tokens.extend(tts);
}

fn expand_var(var: &VariableDeclaration) -> TokenStream {
    let VariableDeclaration { ty, name, .. } = var;
    let ty = expand_type(ty);
    quote! {
        #name: <#ty as ::ethers_sol_types::SolType>::RustType
    }
}

struct Context {
    /// `function.signature() => new_name`
    overloads_map: HashMap<String, String>,
    ast: File,
}

// expand
impl Context {
    fn new(ast: File) -> Self {
        Self {
            overloads_map: HashMap::new(),
            ast,
        }
    }

    fn expand(mut self) -> Result<TokenStream> {
        if self.ast.items.len() > 1 {
            self.resolve_custom_types()?;
            self.resolve_function_overloads()?;
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
            Item::Error(error) => self.expand_error(error),
            Item::Function(function) => self.expand_function(function),
            Item::Struct(s) => self.expand_struct(s),
            Item::Udt(udt) => self.expand_udt(udt),
        }
    }

    fn expand_error(&self, error: &ItemError) -> Result<TokenStream> {
        let ItemError {
            fields,
            name,
            attrs,
            ..
        } = error;

        fields.assert_resolved();

        let signature = fields.signature(name.as_string());
        let selector = crate::utils::selector(&signature);

        let size = fields.data_size(None);

        let converts = from_into_tuples(&name.0, fields);
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
                impl ::ethers_sol_types::SolError for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

                    const SIGNATURE: &'static str = #signature;
                    const SELECTOR: [u8; 4] = #selector;

                    fn to_rust(&self) -> <Self::Tuple as ::ethers_sol_types::SolType>::RustType {
                        self.clone().into()
                    }

                    fn from_rust(tuple: <Self::Tuple as ::ethers_sol_types::SolType>::RustType) -> Self {
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
        let call_name = format_ident!("{}Call", function_name);
        let mut tokens = self.expand_call(function, &call_name, &function.arguments)?;

        if let Some(ret) = &function.returns {
            assert!(!ret.returns.is_empty());
            let return_name = format_ident!("{}Return", function_name);
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
        params.assert_resolved();

        let fields = params.iter().map(expand_var);

        let signature = params.signature(function.name.as_string());
        let selector = crate::utils::selector(&signature);

        let size = params.data_size(None);

        let converts = from_into_tuples(call_name, params);

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
                impl ::ethers_sol_types::SolCall for #call_name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

                    const SIGNATURE: &'static str = #signature;
                    const SELECTOR: [u8; 4] = #selector;

                    fn to_rust(&self) -> <Self::Tuple as ::ethers_sol_types::SolType>::RustType {
                        self.clone().into()
                    }

                    fn from_rust(tuple: <Self::Tuple as ::ethers_sol_types::SolType>::RustType) -> Self {
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

    /// Returns the name of the function, adjusted for overloads.
    fn function_name(&self, function: &ItemFunction) -> String {
        match self.overloads_map.get(&function.signature()) {
            Some(name) => name.clone(),
            None => function.name.as_string(),
        }
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
        let encode_type_impl = if fields.iter().any(|f| f.ty.is_struct()) {
            quote! {
                {
                    let mut encoded = String::from(#encoded_type);
                    #(
                        if let Some(s) = <#props_tys as ::ethers_sol_types::SolType>::eip712_encode_type() {
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
                quote!(<#ty as ::ethers_sol_types::SolType>::eip712_data_word(&self.#name).0.to_vec())
            }
            _ => quote! {
                [#(
                    <#props_tys as ::ethers_sol_types::SolType>::eip712_data_word(&self.#props).0,
                )*].concat()
            },
        };

        let attrs = attrs.iter();
        let convert = from_into_tuples(&name.0, fields);
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
                use ::ethers_sol_types::no_std_prelude::*;

                #convert

                #[automatically_derived]
                impl ::ethers_sol_types::SolStruct for #name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

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
            ::ethers_sol_types::define_udt! {
                #(#attrs)*
                #name,
                underlying: #ty,
            }
        };
        Ok(tokens)
    }
}

// resolve
impl Context {
    /// Resolves custom types in the order they were defined.
    fn resolve_custom_types(&mut self) -> Result<()> {
        let types = self.custom_type_map();
        if types.is_empty() {
            return Ok(())
        }
        for _i in 0..RESOLVE_LIMIT {
            let mut any = false;
            self.visit_types(|ty| {
                let Type::Custom(ty @ CustomType::Unresolved(_)) = ty else { return };
                let Some(resolved) = types.get(ty.ident()) else { return };
                let old_span = ty.span();
                ty.clone_from(resolved);
                ty.set_span(old_span);
                any = true;
            });
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
        Err(syn::Error::new(proc_macro2::Span::call_site(), msg))
    }

    /// Constructs a map of custom types' names to their definitions.
    fn custom_type_map(&self) -> HashMap<Ident, CustomType> {
        let mut map = HashMap::with_capacity(self.ast.items.len());
        for item in &self.ast.items {
            let (name, ty) = match item {
                Item::Udt(udt) => (&udt.name.0, CustomType::Udt(udt.clone().into())),
                Item::Struct(strukt) => (&strukt.name.0, CustomType::Struct(strukt.clone())),
                _ => continue,
            };
            map.insert(name.clone(), ty);
        }
        map
    }

    /// Visits all [Type]s in the input.
    fn visit_types(&mut self, mut f: impl FnMut(&mut Type)) {
        for item in &mut self.ast.items {
            match item {
                Item::Udt(ItemUdt { ty, .. }) => ty.visit_mut(&mut f),

                Item::Struct(strukt) => {
                    for field in &mut strukt.fields {
                        field.ty.visit_mut(&mut f);
                    }
                }
                Item::Function(function) => {
                    for arg in &mut function.arguments {
                        arg.ty.visit_mut(&mut f);
                    }
                    if let Some(returns) = &mut function.returns {
                        for ret in &mut returns.returns {
                            ret.ty.visit_mut(&mut f);
                        }
                    }
                }
                Item::Error(error) => {
                    for field in &mut error.fields {
                        field.ty.visit_mut(&mut f);
                    }
                }
            }
        }
    }

    /// Resolves all [ItemFunction] overloads by appending the index at the end
    /// of the name.
    fn resolve_function_overloads(&mut self) -> Result<()> {
        let all_orig_names: Vec<SolIdent> = self.functions().map(|f| f.name.clone()).collect();
        let mut overloads_map = std::mem::take(&mut self.overloads_map);

        let mut all_functions_map = HashMap::with_capacity(self.ast.items.len());
        for function in self.functions_mut() {
            all_functions_map
                .entry(function.name.as_string())
                .or_insert_with(Vec::new)
                .push(function);
        }

        // Report all errors at the end.
        let mut errors = Vec::new();

        for functions in all_functions_map.values_mut().filter(|fs| fs.len() >= 2) {
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

            for (i, function) in functions.iter_mut().enumerate() {
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

                overloads_map.insert(function.signature(), new_name);
            }
        }

        if errors.is_empty() {
            self.overloads_map = overloads_map;
            Ok(())
        } else {
            Err(errors
                .into_iter()
                .reduce(|mut a, b| {
                    a.combine(b);
                    a
                })
                .unwrap())
        }
    }

    fn functions(&self) -> impl Iterator<Item = &ItemFunction> {
        self.ast.items.iter().filter_map(|item| match item {
            Item::Function(function) => Some(function),
            _ => None,
        })
    }

    fn functions_mut(&mut self) -> impl Iterator<Item = &mut ItemFunction> {
        self.ast.items.iter_mut().filter_map(|item| match item {
            Item::Function(function) => Some(function),
            _ => None,
        })
    }
}

/// Expands `From` impls for a list of types and the corresponding tuple.
fn from_into_tuples<P>(name: &Ident, fields: &Parameters<P>) -> TokenStream {
    let names = fields.names();
    let names2 = names.clone();
    let idxs = (0..fields.len()).map(syn::Index::from);

    let tys = fields.types().map(expand_type);
    let tys2 = tys.clone();

    quote! {
        type UnderlyingSolTuple = (#(#tys,)*);
        type UnderlyingRustTuple = (#(<#tys2 as ::ethers_sol_types::SolType>::RustType,)*);

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
