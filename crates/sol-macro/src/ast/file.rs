use super::{
    item::{Function, Udt},
    CustomType, Item, SolIdent, Type,
};
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Result,
};

/// The limit for the number of times to resolve a type.
const RESOLVE_LIMIT: usize = 16;

/// A Solidity file. The root of the AST.
#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

impl Parse for File {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        if items.is_empty() {
            let message = "\
                expected at least one of: \
                `type`, `struct`, `function`, `error`, Solidity type";
            return Err(input.error(message))
        }

        let mut this = Self { items };
        if this.items.len() > 1 {
            this.resolve_custom_types()?;
            this.resolve_function_overloads()?;
        }
        Ok(this)
    }
}

impl File {
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
        let mut map = HashMap::with_capacity(self.items.len());
        for item in &self.items {
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
        for item in &mut self.items {
            match item {
                Item::Udt(Udt { ty, .. }) | Item::Type(ty) => ty.visit_mut(&mut f),

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

    /// Resolves all [Function] overloads by appending the index at the end of
    /// the name.
    fn resolve_function_overloads(&mut self) -> Result<()> {
        let all_orig_names: Vec<SolIdent> = self.functions().map(|f| f.name.clone()).collect();
        let mut all_functions_map = HashMap::with_capacity(self.items.len());
        for function in self.functions_mut() {
            all_functions_map
                .entry(function.name.as_string())
                .or_insert_with(Vec::new)
                .push(function);
        }

        // Report all errors at the end.
        // This is OK even if we mutate the functions in the loop, because we
        // will return an error at the end anyway.
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
                let span = function.name.span();
                let old_name = function.name.0.unraw();
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
                function.name.0 = Ident::new(&new_name, span);
            }
        }

        if errors.is_empty() {
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

    fn functions(&self) -> impl Iterator<Item = &Function> {
        self.items.iter().filter_map(|item| match item {
            Item::Function(function) => Some(function),
            _ => None,
        })
    }

    fn functions_mut(&mut self) -> impl Iterator<Item = &mut Function> {
        self.items.iter_mut().filter_map(|item| match item {
            Item::Function(function) => Some(function),
            _ => None,
        })
    }
}
