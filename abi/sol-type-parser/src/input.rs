use std::collections::HashMap;

use crate::{
    common::kw,
    error::Error,
    function::Function,
    r#struct::Struct,
    r#type::{CustomType, Type},
    udt::Udt,
};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote_spanned, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

/// The limit for the number of times to resolve a type.
const RESOLVE_LIMIT: usize = 16;

/// Entry point for the `sol` proc-macro.
#[derive(Debug)]
pub struct Input {
    inputs: Vec<SingleInput>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut parsed_inputs = Vec::new();
        while !input.is_empty() {
            parsed_inputs.push(input.parse()?);
        }
        if parsed_inputs.is_empty() {
            let message = "\
                expected at least one of: \
                `type`, `struct`, `function`, `error`, Solidity type";
            return Err(input.error(message));
        }

        let mut this = Self {
            inputs: parsed_inputs,
        };
        if this.inputs.len() > 1 {
            this.resolve();
        }
        Ok(this)
    }
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        assert!(!self.inputs.is_empty());
        for input in &self.inputs {
            input.to_tokens(tokens);
        }
    }
}

impl Input {
    fn resolve(&mut self) {
        let types = self.custom_type_map();
        if types.is_empty() {
            return;
        }
        // Resolves types in the order they were defined.
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
                return;
            }
        }
        panic!(
            "failed to resolve types after {RESOLVE_LIMIT} iterations.\n\
             This is likely due to an infinitely recursive type definition.\n\
             If you believe this is a bug, please file an issue at \
             https://github.com/ethers-rs/core/issues/new/choose"
        );
    }

    /// Constructs a map of custom types' names to their definitions.
    fn custom_type_map(&self) -> HashMap<Ident, CustomType> {
        let mut map = HashMap::with_capacity(self.inputs.len());
        for s in &self.inputs {
            let (name, ty) = match &s.kind {
                InputKind::Udt(udt) => (&udt.name.0, CustomType::Udt(udt.clone().into())),
                InputKind::Struct(strukt) => (&strukt.name.0, CustomType::Struct(strukt.clone())),
                _ => continue,
            };
            map.insert(name.clone(), ty);
        }
        map
    }

    fn visit_types(&mut self, mut f: impl FnMut(&mut Type)) {
        for input in &mut self.inputs {
            match &mut input.kind {
                InputKind::Udt(Udt { ty, .. }) | InputKind::Type(ty) => ty.visit_mut(&mut f),

                InputKind::Struct(strukt) => {
                    for field in &mut strukt.fields {
                        field.ty.visit_mut(&mut f);
                    }
                }
                InputKind::Function(function) => {
                    for arg in &mut function.arguments {
                        arg.ty.visit_mut(&mut f);
                    }
                    if let Some(returns) = &mut function.returns {
                        for ret in &mut returns.returns {
                            ret.ty.visit_mut(&mut f);
                        }
                    }
                }
                InputKind::Error(error) => {
                    for field in &mut error.fields {
                        field.ty.visit_mut(&mut f);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct SingleInput {
    attrs: Vec<Attribute>,
    kind: InputKind,
}

impl Parse for SingleInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            kind: input.parse()?,
        })
    }
}

impl ToTokens for SingleInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self.kind {
            InputKind::Udt(udt) => udt.to_tokens(tokens, &self.attrs),
            InputKind::Struct(strukt) => strukt.to_tokens(tokens, &self.attrs),
            InputKind::Function(function) => function.to_tokens(tokens, &self.attrs),
            InputKind::Error(error) => error.to_tokens(tokens, &self.attrs),
            InputKind::Type(ty) => {
                if !self.attrs.is_empty() {
                    tokens.extend(quote_spanned! {ty.span()=>
                        compile_error!("attributes are not allowed on types")
                    });
                }
                ty.to_tokens(tokens)
            }
        }
    }
}

#[derive(Debug)]
enum InputKind {
    Udt(Udt),
    Struct(Struct),
    Function(Function),
    Error(Error),
    Type(Type),
}

impl Parse for InputKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let this = if lookahead.peek(Token![type]) {
            Self::Udt(input.parse()?)
        } else if lookahead.peek(Token![struct]) {
            Self::Struct(input.parse()?)
        } else if lookahead.peek(kw::function) {
            Self::Function(input.parse()?)
        } else if lookahead.peek(kw::error) {
            Self::Error(input.parse()?)
        } else if lookahead.peek(kw::tuple)
            || lookahead.peek(syn::token::Paren)
            || lookahead.peek(Ident::peek_any)
        {
            Self::Type(input.parse()?)
        } else {
            return Err(lookahead.error());
        };
        Ok(this)
    }
}
