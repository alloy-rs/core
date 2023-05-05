use super::{kw, SolIdent, Storage};
use crate::r#type::Type;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Ident, Result,
};

/// `<ty> [storage] <name>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDeclaration {
    pub ty: Type,
    pub storage: Option<Storage>,
    pub name: Option<SolIdent>,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ty.fmt(f)?;
        if let Some(name) = &self.name {
            f.write_str(" ")?;
            name.fmt(f)?;
        }
        Ok(())
    }
}

impl Parse for VariableDeclaration {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            storage: if input.peek(kw::memory)
                || input.peek(kw::storage)
                || input.peek(kw::calldata)
            {
                Some(input.parse()?)
            } else {
                None
            },
            name: if input.peek(Ident::peek_any) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl ToTokens for VariableDeclaration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ty, name, .. } = self;
        tokens.extend(quote! {
            #name: <#ty as ::ethers_abi_enc::SolType>::RustType
        });
    }
}

impl VariableDeclaration {
    pub fn span(&self) -> Span {
        let span = self.ty.span();
        match (&self.storage, &self.name) {
            (Some(storage), None) => span.join(storage.span()),
            (_, Some(name)) => span.join(name.span()),
            (None, None) => Some(span),
        }
        .unwrap_or(span)
    }
}
