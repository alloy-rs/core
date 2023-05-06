use super::{kw, SolIdent, Storage};
use crate::r#type::Type;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Error, Ident, Result,
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
        Self::_parse(input, false)
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

    pub fn parse_for_struct(input: ParseStream) -> Result<Self> {
        Self::_parse(input, true)
    }

    fn _parse(input: ParseStream, for_struct: bool) -> Result<Self> {
        let ty = input.parse::<Type>()?;
        let can_have_storage = ty.can_have_storage();
        let this = Self {
            ty,
            storage: if input.peek(kw::memory)
                || input.peek(kw::storage)
                || input.peek(kw::calldata)
            {
                let storage = input.parse::<Storage>()?;
                if for_struct || !can_have_storage {
                    let msg = if for_struct {
                        "data locations are not allowed in struct definitions"
                    } else {
                        "data location can only be specified for array, struct or mapping types"
                    };
                    return Err(Error::new(storage.span(), msg));
                }
                Some(storage)
            } else {
                None
            },
            name: if input.peek(Ident::peek_any) {
                Some(input.parse()?)
            } else {
                None
            },
        };
        Ok(this)
    }
}
