//! Functions which generate Rust code from the Solidity AST.

use crate::utils::ExprArray;
use ast::{
    File, Item, ItemError, ItemEvent, ItemFunction, Parameters, SolIdent, SolPath, Type,
    VariableDeclaration, Visit,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, IdentFragment};
use std::{collections::HashMap, fmt::Write, num::NonZeroU16};
use syn::{Error, Result};

mod attr;

mod ty;
pub use ty::expand_type;

mod contract;
mod r#enum;
mod error;
mod event;
mod function;
mod r#struct;
mod udt;

/// The limit for the number of times to resolve a type.
const RESOLVE_LIMIT: usize = 8;

/// The [`sol!`][crate::sol!] expansion implementation.
pub fn expand(ast: File) -> Result<TokenStream> {
    ExpCtxt::new(&ast).expand()
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
            Item::Contract(contract) => contract::expand(self, contract),
            Item::Enum(enumm) => r#enum::expand(self, enumm),
            Item::Error(error) => error::expand(self, error),
            Item::Event(event) => event::expand(self, event),
            Item::Function(function) => function::expand(self, function),
            Item::Struct(strukt) => r#struct::expand(self, strukt),
            Item::Udt(udt) => udt::expand(self, udt),
            Item::Variable(_) => {
                // TODO: Expand getter function for public variables
                Ok(TokenStream::new())
            }
            Item::Import(_) | Item::Pragma(_) | Item::Using(_) => Ok(TokenStream::new()),
        }
    }
}

// resolve
impl<'ast> ExpCtxt<'ast> {
    fn mk_types_map(&mut self) {
        let mut map = std::mem::take(&mut self.custom_types);
        map.reserve(self.all_items.len());
        for &item in &self.all_items {
            let (name, ty) = match item {
                Item::Enum(e) => (
                    &e.name,
                    Type::Uint(e.span(), Some(NonZeroU16::new(8).unwrap())),
                ),
                Item::Struct(s) => (&s.name, s.as_type()),
                Item::Udt(u) => (&u.name, u.ty.clone()),
                _ => continue,
            };
            map.insert(name.clone(), ty);
        }
        self.custom_types = map;
    }

    fn resolve_custom_types(&mut self) -> Result<()> {
        self.mk_types_map();
        // you won't get me this time, borrow checker
        // SAFETY: no data races, we don't modify the map while we're iterating
        // I think this is safe anyway
        let map_ref: &mut HashMap<SolIdent, Type> =
            unsafe { &mut *(&mut self.custom_types as *mut _) };
        let map = &self.custom_types;
        for ty in map_ref.values_mut() {
            let mut i = 0;
            ty.visit_mut(|ty| {
                if i >= RESOLVE_LIMIT {
                    return
                }
                let ty @ Type::Custom(_) = ty else { return };
                let Type::Custom(name) = &*ty else { unreachable!() };
                let Some(resolved) = map.get(name.last_tmp()) else { return };
                ty.clone_from(resolved);
                i += 1;
            });
            if i >= RESOLVE_LIMIT {
                let msg = "\
                    failed to resolve types.\n\
                    This is likely due to an infinitely recursive type definition.\n\
                    If you believe this is a bug, please file an issue at \
                    https://github.com/alloy-rs/core/issues/new/choose";
                return Err(Error::new(ty.span(), msg))
            }
        }
        Ok(())
    }

    fn mk_overloads_map(&mut self) -> Result<()> {
        let all_orig_names: Vec<SolIdent> = self
            .functions
            .values()
            .flatten()
            .map(|f| f.name().clone())
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
                        let mut err = syn::Error::new(a.span(), msg);

                        let msg = "other declaration is here";
                        let note = syn::Error::new(b.span(), msg);

                        err.combine(note);
                        errors.push(err);
                    }
                }
            }

            for (i, &function) in functions.iter().enumerate() {
                let old_name = function.name();
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

    fn get_item(&self, name: &SolPath) -> &Item {
        let name = name.last_tmp();
        match self.all_items.iter().find(|item| item.name() == Some(name)) {
            Some(item) => item,
            None => panic!("unresolved item: {name}"),
        }
    }

    fn custom_type(&self, name: &SolPath) -> &Type {
        match self.custom_types.get(name.last_tmp()) {
            Some(item) => item,
            None => panic!("unresolved item: {name}"),
        }
    }

    /// Returns the name of the function, adjusted for overloads.
    fn function_name(&self, function: &ItemFunction) -> String {
        let sig = self.function_signature(function);
        match self.function_overloads.get(&sig) {
            Some(name) => name.clone(),
            None => function.name().as_string(),
        }
    }

    /// Returns the name of the function, adjusted for overloads.
    fn function_name_ident(&self, function: &ItemFunction) -> SolIdent {
        let sig = self.function_signature(function);
        match self.function_overloads.get(&sig) {
            Some(name) => SolIdent::new_spanned(name, function.name().span()),
            None => function.name().clone(),
        }
    }

    fn raw_call_name(&self, function_name: impl IdentFragment + std::fmt::Display) -> Ident {
        format_ident!("{function_name}Call")
    }

    fn call_name(&self, function: &ItemFunction) -> Ident {
        let function_name = self.function_name(function);
        self.raw_call_name(function_name)
    }

    fn raw_return_name(&self, function_name: impl IdentFragment + std::fmt::Display) -> Ident {
        format_ident!("{function_name}Return")
    }

    fn return_name(&self, function: &ItemFunction) -> Ident {
        let function_name = self.function_name(function);
        self.raw_return_name(function_name)
    }

    fn signature<'a, I: IntoIterator<Item = &'a VariableDeclaration>>(
        &self,
        mut name: String,
        params: I,
    ) -> String {
        name.push('(');
        let mut first = true;
        for param in params {
            if !first {
                name.push(',');
            }
            write!(name, "{}", ty::TypePrinter::new(self, &param.ty)).unwrap();
            first = false;
        }
        name.push(')');
        name
    }

    fn function_signature(&self, function: &ItemFunction) -> String {
        self.signature(function.name().as_string(), &function.arguments)
    }

    fn function_selector(&self, function: &ItemFunction) -> ExprArray<u8, 4> {
        crate::utils::selector(self.function_signature(function))
    }

    fn error_signature(&self, error: &ItemError) -> String {
        self.signature(error.name.as_string(), &error.parameters)
    }

    fn error_selector(&self, error: &ItemError) -> ExprArray<u8, 4> {
        crate::utils::selector(self.error_signature(error))
    }

    #[allow(dead_code)]
    fn event_signature(&self, event: &ItemEvent) -> String {
        self.signature(event.name.as_string(), &event.params())
    }

    #[allow(dead_code)]
    fn event_selector(&self, event: &ItemEvent) -> ExprArray<u8, 32> {
        crate::utils::event_selector(self.event_signature(event))
    }

    /// Returns an error if any of the types in the parameters are unresolved.
    ///
    /// Provides a better error message than an `unwrap` or `expect` when we
    /// know beforehand that we will be needing types to be resolved.
    fn assert_resolved<'a, I: IntoIterator<Item = &'a VariableDeclaration>>(
        &self,
        params: I,
    ) -> Result<()> {
        let mut errors = Vec::new();
        for param in params {
            param.ty.visit(|ty| {
                if let Type::Custom(name) = ty {
                    if !self.custom_types.contains_key(name.last_tmp()) {
                        let e = syn::Error::new(name.span(), "unresolved type");
                        errors.push(e);
                    }
                }
            });
        }
        if errors.is_empty() {
            Ok(())
        } else {
            let mut e = crate::utils::combine_errors(errors).unwrap();
            let note =
                "Custom types must be declared inside of the same scope they are referenced in,\n\
                 or \"imported\" as a UDT with `type ... is (...);`";
            e.combine(Error::new(Span::call_site(), note));
            Err(e)
        }
    }
}

impl<'ast> Visit<'ast> for ExpCtxt<'ast> {
    fn visit_item(&mut self, item: &'ast Item) {
        self.all_items.push(item);
        ast::visit::visit_item(self, item);
    }

    fn visit_item_function(&mut self, function: &'ast ItemFunction) {
        self.functions
            .entry(function.name().as_string())
            .or_default()
            .push(function);
        ast::visit::visit_item_function(self, function);
    }
}

// helper functions
/// Expands a list of parameters into a list of struct fields.
///
/// See [`expand_field`].
fn expand_fields<P>(params: &Parameters<P>) -> impl Iterator<Item = TokenStream> + '_ {
    params
        .iter()
        .enumerate()
        .map(|(i, var)| expand_field(i, &var.ty, var.name.as_ref()))
}

/// Expands a single parameter into a struct field.
fn expand_field(i: usize, ty: &Type, name: Option<&SolIdent>) -> TokenStream {
    let name = anon_name((i, name));
    let ty = expand_type(ty);
    quote! {
        #name: <#ty as ::alloy_sol_types::SolType>::RustType
    }
}

/// Generates an anonymous name from an integer. Used in `anon_name`
#[inline]
pub fn generate_name(i: usize) -> Ident {
    format_ident!("_{}", i)
}

/// Returns the name of a parameter, or a generated name if it is `None`.
fn anon_name<T: Into<Ident> + Clone>((i, name): (usize, Option<&T>)) -> Ident {
    match name {
        Some(name) => name.clone().into(),
        None => generate_name(i),
    }
}

/// Expands `From` impls for an empty struct and the unit type.
fn expand_from_into_unit(name: &Ident) -> TokenStream {
    quote! {
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();

        impl From<()> for #name {
            #[inline]
            fn from((): ()) -> Self {
                Self {}
            }
        }

        impl From<#name> for () {
            #[inline]
            fn from(#name {}: #name) {}
        }

        impl ::alloy_sol_types::Encodable<()> for #name {
            #[inline]
            fn to_tokens(&self) {}
        }
    }
}

/// Expands `From` impls for a list of types and the corresponding tuple.
///
/// See [`expand_from_into_tuples`].
fn expand_from_into_tuples<P>(name: &Ident, fields: &Parameters<P>) -> TokenStream {
    if fields.is_empty() {
        return expand_from_into_unit(name)
    }

    let names = fields.names().enumerate().map(anon_name);

    let names2 = names.clone();
    let idxs = (0..fields.len()).map(syn::Index::from);

    let names3 = names.clone();
    let field_tys = fields.types().map(expand_type);

    let (sol_tuple, rust_tuple) = expand_tuple_types(fields.types());

    quote! {
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = #sol_tuple;
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = #rust_tuple;

        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<#name> for UnderlyingRustTuple<'_> {
            fn from(value: #name) -> Self {
                (#(value.#names,)*)
            }
        }

        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for #name {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                #name {
                    #(#names2: tuple.#idxs),*
                }
            }
        }

        impl ::alloy_sol_types::Encodable<UnderlyingSolTuple<'_>> for #name {
            fn to_tokens(&self) -> <UnderlyingSolTuple<'_> as ::alloy_sol_types::SolType>::TokenType<'_> {
                (#(
                    ::alloy_sol_types::Encodable::<#field_tys>::to_tokens(&self.#names3),
                )*)
            }
        }
    }
}

/// Returns
/// - `(#(#expanded,)*)`
/// - `(#(<#expanded as ::alloy_sol_types::SolType>::RustType,)*)`
fn expand_tuple_types<'a, I: IntoIterator<Item = &'a Type>>(
    types: I,
) -> (TokenStream, TokenStream) {
    let mut sol_tuple = TokenStream::new();
    let mut rust_tuple = TokenStream::new();
    for ty in types {
        let expanded = expand_type(ty);
        sol_tuple.extend(quote!(#expanded,));
        rust_tuple.extend(quote!(<#expanded as ::alloy_sol_types::SolType>::RustType,));
    }
    (quote!((#sol_tuple)), quote!((#rust_tuple)))
}
