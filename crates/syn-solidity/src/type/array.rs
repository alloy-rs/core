use crate::Type;
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU64,
};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    token::Bracket,
    LitInt, Result,
};

/// An array type.
#[derive(Clone)]
pub struct TypeArray {
    pub ty: Box<Type>,
    pub bracket_token: Bracket,
    pub size: Option<LitInt>,
}

impl PartialEq for TypeArray {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.size == other.size
    }
}

impl Eq for TypeArray {}

impl Hash for TypeArray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.size.hash(state);
    }
}

impl fmt::Debug for TypeArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TypeArray")
            .field(&self.ty)
            .field(&self.size.as_ref().map(|s| s.base10_digits()))
            .finish()
    }
}

impl fmt::Display for TypeArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ty.fmt(f)?;
        f.write_str("[")?;
        if let Some(s) = &self.size {
            f.write_str(s.base10_digits())?;
        }
        f.write_str("]")
    }
}

impl Parse for TypeArray {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ty = input.parse()?;
        Self::wrap(input, ty)
    }
}

impl TypeArray {
    pub fn span(&self) -> Span {
        let span = self.ty.span();
        span.join(self.bracket_token.span.join()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.ty.set_span(span);
        self.bracket_token = Bracket(span);
        if let Some(size) = &mut self.size {
            size.set_span(span);
        }
    }

    /// See [`Type::is_abi_dynamic`].
    pub fn is_abi_dynamic(&self) -> bool {
        match self.size {
            Some(_) => false,
            None => self.ty.is_abi_dynamic(),
        }
    }

    pub fn wrap(input: ParseStream<'_>, ty: Type) -> Result<Self> {
        let content;
        Ok(Self {
            ty: Box::new(ty),
            bracket_token: bracketed!(content in input),
            size: {
                let size = content.parse::<Option<syn::LitInt>>()?;
                // Validate the size
                if let Some(sz) = &size {
                    sz.base10_parse::<NonZeroU64>()?;
                }
                size
            },
        })
    }
}
