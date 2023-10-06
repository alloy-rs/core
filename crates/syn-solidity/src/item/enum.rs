use crate::{utils::DebugPunctuated, SolIdent, Spanned, Type};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::{fmt, num::NonZeroU16};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Attribute, Result, Token,
};

/// An enum definition: `enum Foo { A, B, C }`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.enumDefinition>
#[derive(Clone)]
pub struct ItemEnum {
    pub attrs: Vec<Attribute>,
    pub enum_token: Token![enum],
    pub name: SolIdent,
    pub brace_token: Brace,
    pub variants: Punctuated<Variant, Token![,]>,
}

impl fmt::Display for ItemEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enum {} {{ ", self.name)?;
        for (i, variant) in self.variants.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            variant.fmt(f)?;
        }
        f.write_str(" }")
    }
}

impl fmt::Debug for ItemEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemEnum")
            .field("attrs", &self.attrs)
            .field("name", &self.name)
            .field("variants", DebugPunctuated::new(&self.variants))
            .finish()
    }
}

impl Parse for ItemEnum {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            enum_token: input.parse()?,
            name: input.parse()?,
            brace_token: braced!(content in input),
            variants: content.parse_terminated(Variant::parse, Token![,])?,
        })
    }
}

impl Spanned for ItemEnum {
    fn span(&self) -> Span {
        self.name.span()
    }

    fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
    }
}

impl ItemEnum {
    pub fn as_type(&self) -> Type {
        Type::Uint(self.span(), Some(NonZeroU16::new(8).unwrap()))
    }
}

/// An enum variant.
#[derive(Clone, Debug)]
pub struct Variant {
    pub attrs: Vec<Attribute>,

    /// Name of the variant.
    pub ident: SolIdent,
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ident.fmt(f)
    }
}

impl Parse for Variant {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            ident: input.parse()?,
        })
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.ident.to_tokens(tokens);
    }
}

impl Spanned for Variant {
    fn span(&self) -> Span {
        self.ident.span()
    }

    fn set_span(&mut self, span: Span) {
        self.ident.set_span(span);
    }
}
