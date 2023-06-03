use crate::ast::{kw, SolIdent, Type};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

/// A user-defined value type definition.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.userDefinedValueTypeDefinition>
#[derive(Clone)]
pub struct Udt {
    pub attrs: Vec<Attribute>,
    pub type_token: Token![type],
    pub name: SolIdent,
    pub is_token: kw::is,
    pub ty: Type,
    pub semi_token: Token![;],
}

impl fmt::Debug for Udt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Udt")
            .field("name", &self.name)
            .field("ty", &self.ty)
            .finish()
    }
}

impl PartialEq for Udt {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ty == other.ty
    }
}

impl Eq for Udt {}

impl Hash for Udt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ty.hash(state);
    }
}

impl Parse for Udt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let this = Self {
            attrs: input.call(Attribute::parse_outer)?,
            type_token: input.parse()?,
            name: input.parse()?,
            is_token: input.parse()?,
            ty: input.parse()?,
            semi_token: input.parse()?,
        };

        // Solidity doesn't allow this, and it would cause ambiguity in `CustomType`
        let mut has_struct = this.ty.is_struct();
        this.ty.visit(&mut |ty| {
            has_struct |= ty.is_struct();
        });
        if has_struct {
            return Err(syn::Error::new(
                this.ty.span(),
                "the underlying type for a user defined value type has to be an elementary value type",
            ));
        }

        Ok(this)
    }
}

impl ToTokens for Udt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name, ty, attrs, ..
        } = self;
        tokens.extend(quote! {
            ::ethers_sol_types::define_udt! {
                #(#attrs)*
                #name,
                underlying: #ty,
            }
        });
    }
}

impl Udt {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.type_token.span = span;
        self.name.set_span(span);
        self.is_token.span = span;
        self.ty.set_span(span);
        self.semi_token.span = span;
    }
}
