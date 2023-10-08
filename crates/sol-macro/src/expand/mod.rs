//! Functions which generate Rust code from the Solidity AST.

use crate::{
    attr::{self, SolAttrs},
    expand::ty::expand_rust_type,
    utils::{self, ExprArray},
};
use ast::{
    File, Item, ItemError, ItemEvent, ItemFunction, Parameters, SolIdent, SolPath, Spanned, Type,
    VariableDeclaration, Visit,
};
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, TokenStreamExt};
use std::{borrow::Borrow, collections::HashMap, fmt::Write};
use syn::{parse_quote, Attribute, Error, Result};

mod ty;
pub use ty::expand_type;

mod contract;
mod r#enum;
mod error;
mod event;
mod function;
mod r#struct;
mod udt;
mod var_def;

/// The limit for the number of times to resolve a type.
const RESOLVE_LIMIT: usize = 8;

/// The [`sol!`][crate::sol!] expansion implementation.
pub fn expand(ast: File) -> Result<TokenStream> {
    ExpCtxt::new(&ast).expand()
}

/// The expansion context.
struct ExpCtxt<'ast> {
    all_items: Vec<&'ast Item>,
    custom_types: HashMap<SolIdent, Type>,

    /// `name => item`
    overloaded_items: HashMap<String, Vec<OverloadedItem<'ast>>>,
    /// `signature => new_name`
    overloads: HashMap<String, String>,

    attrs: SolAttrs,
    ast: &'ast File,
}

// expand
impl<'ast> ExpCtxt<'ast> {
    fn new(ast: &'ast File) -> Self {
        Self {
            all_items: Vec::new(),
            custom_types: HashMap::new(),
            overloaded_items: HashMap::new(),
            overloads: HashMap::new(),
            attrs: SolAttrs::default(),
            ast,
        }
    }

    fn expand(mut self) -> Result<TokenStream> {
        let mut abort = false;
        let mut tokens = TokenStream::new();

        if let Err(e) = self.parse_file_attributes() {
            tokens.extend(e.into_compile_error());
        }

        self.visit_file(self.ast);

        if self.all_items.len() > 1 {
            self.resolve_custom_types();
            if self.mk_overloads_map().is_err() {
                abort = true;
            }
        }

        if abort {
            return Ok(tokens)
        }

        for item in &self.ast.items {
            // TODO: Dummy items
            let t = match self.expand_item(item) {
                Ok(t) => t,
                Err(e) => e.into_compile_error(),
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
            Item::Variable(var_def) => var_def::expand(self, var_def),
            Item::Import(_) | Item::Pragma(_) | Item::Using(_) => Ok(TokenStream::new()),
        }
    }
}

// resolve
impl<'ast> ExpCtxt<'ast> {
    fn parse_file_attributes(&mut self) -> Result<()> {
        let (attrs, others) = attr::SolAttrs::parse(&self.ast.attrs)?;
        self.attrs = attrs;

        let errs = others
            .iter()
            .map(|attr| Error::new_spanned(attr, "unexpected attribute"));
        utils::combine_errors(errs)
    }

    fn mk_types_map(&mut self) {
        let mut map = std::mem::take(&mut self.custom_types);
        map.reserve(self.all_items.len());
        for &item in &self.all_items {
            let (name, ty) = match item {
                Item::Contract(c) => (&c.name, c.as_type()),
                Item::Enum(e) => (&e.name, e.as_type()),
                Item::Struct(s) => (&s.name, s.as_type()),
                Item::Udt(u) => (&u.name, u.ty.clone()),
                _ => continue,
            };
            map.insert(name.clone(), ty);
        }
        self.custom_types = map;
    }

    fn resolve_custom_types(&mut self) {
        self.mk_types_map();
        // you won't get me this time, borrow checker
        // SAFETY: no data races, we don't modify the map while we're iterating
        // I think this is safe anyway
        let map_ref: &mut HashMap<SolIdent, Type> =
            unsafe { &mut *(&mut self.custom_types as *mut _) };
        for ty in map_ref.values_mut() {
            let mut i = 0;
            ty.visit_mut(|ty| {
                if i >= RESOLVE_LIMIT {
                    return
                }
                let ty @ Type::Custom(_) = ty else { return };
                let Type::Custom(name) = &*ty else {
                    unreachable!()
                };
                let Some(resolved) = self.try_custom_type(name) else {
                    return
                };
                ty.clone_from(resolved);
                i += 1;
            });
            if i >= RESOLVE_LIMIT {
                abort!(
                    ty.span(),
                    "failed to resolve types.\n\
                     This is likely due to an infinitely recursive type definition.\n\
                     If you believe this is a bug, please file an issue at \
                     https://github.com/alloy-rs/core/issues/new/choose"
                );
            }
        }
    }

    fn mk_overloads_map(&mut self) -> std::result::Result<(), ()> {
        let all_orig_names: Vec<_> = self
            .overloaded_items
            .values()
            .flatten()
            .filter_map(|f| f.name())
            .collect();
        let mut overloads_map = std::mem::take(&mut self.overloads);

        let mut failed = false;

        for functions in self.overloaded_items.values().filter(|fs| fs.len() >= 2) {
            // check for same parameters
            for (i, &a) in functions.iter().enumerate() {
                for &b in functions.iter().skip(i + 1) {
                    if a.eq_by_types(b) {
                        failed = true;
                        emit_error!(
                            a.span(),
                            "{} with same name and parameter types defined twice",
                            a.desc();

                            note = b.span() => "other declaration is here";
                        );
                    }
                }
            }

            for (i, &item) in functions.iter().enumerate() {
                let Some(old_name) = item.name() else {
                    continue
                };
                let new_name = format!("{old_name}_{i}");
                if let Some(other) = all_orig_names.iter().find(|x| x.0 == new_name) {
                    failed = true;
                    emit_error!(
                        old_name.span(),
                        "{} `{old_name}` is overloaded, \
                         but the generated name `{new_name}` is already in use",
                        item.desc();

                        note = other.span() => "other declaration is here";
                    )
                }

                overloads_map.insert(item.signature(self), new_name);
            }
        }

        if failed {
            return Err(())
        }

        self.overloads = overloads_map;
        Ok(())
    }
}

impl<'ast> Visit<'ast> for ExpCtxt<'ast> {
    fn visit_item(&mut self, item: &'ast Item) {
        self.all_items.push(item);
        ast::visit::visit_item(self, item);
    }

    fn visit_item_function(&mut self, function: &'ast ItemFunction) {
        if let Some(name) = &function.name {
            self.overloaded_items
                .entry(name.as_string())
                .or_default()
                .push(OverloadedItem::Function(function));
        }
        ast::visit::visit_item_function(self, function);
    }

    fn visit_item_event(&mut self, event: &'ast ItemEvent) {
        self.overloaded_items
            .entry(event.name.as_string())
            .or_default()
            .push(OverloadedItem::Event(event));
        ast::visit::visit_item_event(self, event);
    }
}

#[derive(Clone, Copy)]
enum OverloadedItem<'a> {
    Function(&'a ItemFunction),
    Event(&'a ItemEvent),
}

impl<'ast> From<&'ast ItemFunction> for OverloadedItem<'ast> {
    fn from(f: &'ast ItemFunction) -> Self {
        Self::Function(f)
    }
}

impl<'ast> From<&'ast ItemEvent> for OverloadedItem<'ast> {
    fn from(e: &'ast ItemEvent) -> Self {
        Self::Event(e)
    }
}

impl<'a> OverloadedItem<'a> {
    fn name(self) -> Option<&'a SolIdent> {
        match self {
            Self::Function(f) => f.name.as_ref(),
            Self::Event(e) => Some(&e.name),
        }
    }

    fn desc(&self) -> &'static str {
        match self {
            Self::Function(_) => "function",
            Self::Event(_) => "event",
        }
    }

    fn eq_by_types(self, other: Self) -> bool {
        match (self, other) {
            (Self::Function(a), Self::Function(b)) => a.arguments.types().eq(b.arguments.types()),
            (Self::Event(a), Self::Event(b)) => a.param_types().eq(b.param_types()),
            _ => false,
        }
    }

    fn span(self) -> Span {
        match self {
            Self::Function(f) => f.span(),
            Self::Event(e) => e.span(),
        }
    }

    fn signature(self, cx: &ExpCtxt<'a>) -> String {
        match self {
            Self::Function(f) => cx.function_signature(f),
            Self::Event(e) => cx.event_signature(e),
        }
    }
}

// utils
impl<'ast> ExpCtxt<'ast> {
    #[allow(dead_code)]
    fn item(&self, name: &SolPath) -> &Item {
        match self.try_item(name) {
            Some(item) => item,
            None => abort!(name.span(), "unresolved item: {}", name),
        }
    }

    fn try_item(&self, name: &SolPath) -> Option<&Item> {
        let name = name.last();
        self.all_items
            .iter()
            .copied()
            .find(|item| item.name() == Some(name))
    }

    fn custom_type(&self, name: &SolPath) -> &Type {
        match self.try_custom_type(name) {
            Some(item) => item,
            None => abort!(name.span(), "unresolved custom type: {}", name),
        }
    }

    fn try_custom_type(&self, name: &SolPath) -> Option<&Type> {
        self.custom_types.get(name.last())
    }

    /// Returns the name of the function, adjusted for overloads.
    fn function_name(&self, function: &ItemFunction) -> SolIdent {
        self.overloaded_name(function.into())
    }

    /// Returns the name of the given item, adjusted for overloads.
    ///
    /// Use `.into()` to convert from `&ItemFunction` or `&ItemEvent`.
    fn overloaded_name(&self, item: OverloadedItem<'ast>) -> SolIdent {
        let original_ident = item.name().expect("item has no name");
        let sig = item.signature(self);
        match self.overloads.get(&sig) {
            Some(name) => SolIdent::new_spanned(name, original_ident.span()),
            None => original_ident.clone(),
        }
    }

    /// Returns the name of the function's call Rust struct.
    fn call_name(&self, function: &ItemFunction) -> Ident {
        let function_name = self.function_name(function);
        self.raw_call_name(function_name)
    }

    /// Formats the given name as a function's call Rust struct name.
    fn raw_call_name(&self, function_name: impl quote::IdentFragment + std::fmt::Display) -> Ident {
        let mut new_ident = format_ident!("{function_name}Call");
        if let Some(span) = function_name.span() {
            new_ident.set_span(span);
        }
        new_ident
    }

    /// Returns the name of the function's return Rust struct.
    fn return_name(&self, function: &ItemFunction) -> Ident {
        let function_name = self.function_name(function);
        self.raw_return_name(function_name)
    }

    /// Formats the given name as a function's return Rust struct name.
    fn raw_return_name(
        &self,
        function_name: impl quote::IdentFragment + std::fmt::Display,
    ) -> Ident {
        let mut new_ident = format_ident!("{function_name}Return");
        if let Some(span) = function_name.span() {
            new_ident.set_span(span);
        }
        new_ident
    }

    fn function_signature(&self, function: &ItemFunction) -> String {
        self.signature(function.name().as_string(), &function.arguments)
    }

    fn function_selector(&self, function: &ItemFunction) -> ExprArray<u8, 4> {
        utils::selector(self.function_signature(function))
    }

    fn error_signature(&self, error: &ItemError) -> String {
        self.signature(error.name.as_string(), &error.parameters)
    }

    fn error_selector(&self, error: &ItemError) -> ExprArray<u8, 4> {
        utils::selector(self.error_signature(error))
    }

    #[allow(dead_code)]
    fn event_signature(&self, event: &ItemEvent) -> String {
        self.signature(event.name.as_string(), &event.params())
    }

    #[allow(dead_code)]
    fn event_selector(&self, event: &ItemEvent) -> ExprArray<u8, 32> {
        utils::event_selector(self.event_signature(event))
    }

    /// Formats the name and parameters of the function as a Solidity signature.
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

    /// Extends `attrs` with all possible derive attributes for the given type
    /// if `#[sol(all_derives)]` was passed.
    ///
    /// The following traits are only implemented on tuples of arity 12 or less:
    /// - [PartialEq](https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html)
    /// - [Eq](https://doc.rust-lang.org/stable/std/cmp/trait.Eq.html)
    /// - [PartialOrd](https://doc.rust-lang.org/stable/std/cmp/trait.PartialOrd.html)
    /// - [Ord](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html)
    /// - [Debug](https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html)
    /// - [Default](https://doc.rust-lang.org/stable/std/default/trait.Default.html)
    /// - [Hash](https://doc.rust-lang.org/stable/std/hash/trait.Hash.html)
    ///
    /// while the `Default` trait is only implemented on arrays of length 32 or
    /// less.
    ///
    /// Tuple reference: <https://doc.rust-lang.org/stable/std/primitive.tuple.html#trait-implementations-1>
    ///
    /// Array reference: <https://doc.rust-lang.org/stable/std/primitive.array.html>
    ///
    /// `derive_default` should be set to false when calling this for enums.
    fn derives<'a, I>(&self, attrs: &mut Vec<Attribute>, params: I, derive_default: bool)
    where
        I: IntoIterator<Item = &'a VariableDeclaration>,
    {
        self.type_derives(attrs, params.into_iter().map(|p| &p.ty), derive_default);
    }

    /// Implementation of [`derives`](Self::derives).
    fn type_derives<T, I>(&self, attrs: &mut Vec<Attribute>, types: I, mut derive_default: bool)
    where
        I: IntoIterator<Item = T>,
        T: Borrow<Type>,
    {
        let Some(true) = self.attrs.all_derives else {
            return
        };

        let mut derives = Vec::with_capacity(5);
        let mut derive_others = true;
        for ty in types {
            let ty = ty.borrow();
            derive_default = derive_default && ty::can_derive_default(self, ty);
            derive_others = derive_others && ty::can_derive_builtin_traits(self, ty);
        }
        if derive_default {
            derives.push("Default");
        }
        if derive_others {
            derives.extend(["Debug", "PartialEq", "Eq", "Hash"]);
        }
        let derives = derives.iter().map(|s| Ident::new(s, Span::call_site()));
        attrs.push(parse_quote! { #[derive(#(#derives),*)] });
    }

    /// Returns an error if any of the types in the parameters are unresolved.
    ///
    /// Provides a better error message than an `unwrap` or `expect` when we
    /// know beforehand that we will be needing types to be resolved.
    fn assert_resolved<'a, I>(&self, params: I) -> Result<()>
    where
        I: IntoIterator<Item = &'a VariableDeclaration>,
    {
        let mut errored = false;
        for param in params {
            param.ty.visit(|ty| {
                if let Type::Custom(name) = ty {
                    if !self.custom_types.contains_key(name.last()) {
                        let note = (!errored).then(|| {
                            errored = true;
                            "Custom types must be declared inside of the same scope they are referenced in,\n\
                             or \"imported\" as a UDT with `type ... is (...);`"
                        });
                        emit_error!(name.span(), "unresolved type"; help =? note);
                    }
                }
            });
        }
        Ok(())
    }
}

// helper functions

/// Expands a list of parameters into a list of struct fields.
fn expand_fields<P>(params: &Parameters<P>) -> impl Iterator<Item = TokenStream> + '_ {
    params.iter().enumerate().map(|(i, var)| {
        let name = anon_name((i, var.name.as_ref()));
        let ty = expand_rust_type(&var.ty);
        let attrs = &var.attrs;
        quote! {
            #(#attrs)*
            pub #name: #ty
        }
    })
}

/// Generates an anonymous name from an integer. Used in [`anon_name`].
#[inline]
pub fn generate_name(i: usize) -> Ident {
    format_ident!("_{i}")
}

/// Returns the name of a parameter, or a generated name if it is `None`.
fn anon_name<T: Into<Ident> + Clone>((i, name): (usize, Option<&T>)) -> Ident {
    match name {
        Some(name) => name.clone().into(),
        None => generate_name(i),
    }
}

/// Expands `From` impls for a list of types and the corresponding tuple.
fn expand_from_into_tuples<P>(name: &Ident, fields: &Parameters<P>) -> TokenStream {
    let names = fields.names().enumerate().map(anon_name);

    let names2 = names.clone();
    let idxs = (0..fields.len()).map(syn::Index::from);

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
                Self {
                    #(#names2: tuple.#idxs),*
                }
            }
        }
    }
}

/// Returns `(sol_tuple, rust_tuple)`
fn expand_tuple_types<'a, I: IntoIterator<Item = &'a Type>>(
    types: I,
) -> (TokenStream, TokenStream) {
    let mut sol = TokenStream::new();
    let mut rust = TokenStream::new();
    let comma = Punct::new(',', Spacing::Alone);
    for ty in types {
        ty::rec_expand_type(ty, &mut sol);
        sol.append(comma.clone());

        ty::rec_expand_rust_type(ty, &mut rust);
        rust.append(comma.clone());
    }
    let wrap_in_parens =
        |stream| TokenStream::from(TokenTree::Group(Group::new(Delimiter::Parenthesis, stream)));
    (wrap_in_parens(sol), wrap_in_parens(rust))
}
