//! Functions which generate Rust code from the Solidity AST.

use ast::{
    File, Item, ItemFunction, Parameters, SolIdent, SolPath, Type, VariableDeclaration, Visit,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, IdentFragment};
use std::{collections::HashMap, fmt::Write};
use syn::{Error, Result};

mod attr;

mod r#type;
pub use r#type::expand_type;
use r#type::TypePrinter;

mod contract;
mod error;
mod event;
mod function;
mod r#struct;
mod udt;

/// The limit for the number of times to resolve a type.
const RESOLVE_LIMIT: usize = 16;

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
            Item::Error(error) => error::expand(self, error),
            Item::Event(event) => event::expand(self, event),
            Item::Function(function) => function::expand(self, function),
            Item::Struct(strukt) => r#struct::expand(self, strukt),
            Item::Udt(udt) => udt::expand(self, udt),
        }
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

    fn get_item(&self, name: &SolPath) -> &Item {
        let name = name.last_tmp();
        match self.all_items.iter().find(|item| item.name() == name) {
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
            write!(name, "{}", TypePrinter::new(self, &param.ty)).unwrap();
            first = false;
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
            .entry(function.name.as_string())
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

/// Returns the name of a parameter, or a generated name if it is `None`.
fn anon_name<T: Into<Ident> + Clone>((i, name): (usize, Option<&T>)) -> Ident {
    match name {
        Some(name) => name.clone().into(),
        None => format_ident!("_{i}"),
    }
}

/// Expands `From` impls for a list of types and the corresponding tuple.
///
/// See [`expand_from_into_tuples`].
fn expand_from_into_tuples<P>(name: &Ident, fields: &Parameters<P>) -> TokenStream {
    let names = fields.names().enumerate().map(anon_name);
    let names2 = names.clone();
    let idxs = (0..fields.len()).map(syn::Index::from);

    let (sol_tuple, rust_tuple) = expand_tuple_types(fields.types());
    quote! {
        #[doc(hidden)]
        type UnderlyingSolTuple = #sol_tuple;
        #[doc(hidden)]
        type UnderlyingRustTuple = #rust_tuple;

        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<#name> for UnderlyingRustTuple {
            fn from(value: #name) -> Self {
                (#(value.#names,)*)
            }
        }

        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple> for #name {
            fn from(tuple: UnderlyingRustTuple) -> Self {
                #name {
                    #(#names2: tuple.#idxs),*
                }
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
