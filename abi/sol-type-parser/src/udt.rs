use crate::{
    common::{kw, SolIdent},
    r#type::Type,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

#[derive(Clone)]
pub struct Udt {
    type_token: Token![type],
    pub name: SolIdent,
    is: kw::is,
    pub ty: Type,
    semi_token: Token![;],
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
    fn parse(input: ParseStream) -> Result<Self> {
        let this = Self {
            type_token: input.parse()?,
            name: input.parse()?,
            is: input.parse()?,
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

impl Udt {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.type_token = Token![type](span);
        self.name.set_span(span);
        self.is = kw::is(span);
        self.ty.set_span(span);
        self.semi_token = Token![;](span);
    }

    pub fn to_tokens(&self, tokens: &mut TokenStream, attrs: &[Attribute]) {
        let Self { name, ty, .. } = self;
        tokens.extend(quote! {
            ::ethers_abi_enc::define_udt! {
                #(#attrs)*
                #name,
                underlying: #ty,
            }
        });
    }
}
