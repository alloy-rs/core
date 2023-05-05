use crate::{
    common::kw, error::Error, function::Function, r#struct::Struct, r#type::Type, udt::Udt,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

/// Entry point for the `sol` proc-macro.
pub struct Input {
    attrs: Vec<Attribute>,
    kind: SolInputKind,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            kind: input.parse()?,
        })
    }
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self.kind {
            SolInputKind::Udt(udt) => udt.to_tokens(tokens, &self.attrs),
            SolInputKind::Struct(strukt) => strukt.to_tokens(tokens, &self.attrs),
            SolInputKind::Function(function) => function.to_tokens(tokens, &self.attrs),
            SolInputKind::Error(error) => error.to_tokens(tokens, &self.attrs),
            SolInputKind::Type(ty) => {
                if !self.attrs.is_empty() {
                    tokens.extend(quote_spanned! {ty.span()=>
                        compile_error!("attributes are not allowed on type aliases")
                    });
                }
                ty.to_tokens(tokens)
            }
        }
    }
}

enum SolInputKind {
    Udt(Udt),
    Struct(Struct),
    Function(Function),
    Error(Error),
    Type(Type),
}

impl Parse for SolInputKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let this = if input.peek(Token![type]) {
            Self::Udt(input.parse()?)
        } else if input.peek(Token![struct]) {
            Self::Struct(input.parse()?)
        } else if input.peek(kw::function) {
            Self::Function(input.parse()?)
        } else if input.peek(kw::error) {
            Self::Error(input.parse()?)
        } else {
            Self::Type(input.parse()?)
        };
        Ok(this)
    }
}
