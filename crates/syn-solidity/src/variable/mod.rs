use crate::{utils::tts_until_semi, VariableAttributes};

use super::{kw, SolIdent, Storage, Type};
use proc_macro2::{Span, TokenStream};
use std::fmt::{self, Write};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Attribute, Ident, Result, Token,
};

mod list;
pub use list::{FieldList, ParameterList, Parameters};

/// A variable declaration.
///
/// `<ty> [storage] <name>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDeclaration {
    /// The attributes of the variable.
    pub attrs: Vec<Attribute>,
    /// The type of the variable.
    pub ty: Type,
    /// The storage location of the variable, if any.
    pub storage: Option<Storage>,
    /// The name of the variable. This is always Some if parsed as part of
    /// [`Parameters`].
    pub name: Option<SolIdent>,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ty.fmt(f)?;
        if let Some(storage) = &self.storage {
            f.write_char(' ')?;
            storage.fmt(f)?;
        }
        if let Some(name) = &self.name {
            f.write_char(' ')?;
            name.fmt(f)?;
        }
        Ok(())
    }
}

impl Parse for VariableDeclaration {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Self::_parse(input, false)
    }
}

impl VariableDeclaration {
    pub const fn new(ty: Type) -> Self {
        Self {
            attrs: Vec::new(),
            ty,
            storage: None,
            name: None,
        }
    }

    pub fn span(&self) -> Span {
        let span = self.ty.span();
        match (&self.storage, &self.name) {
            (Some(storage), None) => span.join(storage.span()),
            (_, Some(name)) => span.join(name.span()),
            (None, None) => Some(span),
        }
        .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.ty.set_span(span);
        if let Some(storage) = &mut self.storage {
            storage.set_span(span);
        }
        if let Some(name) = &mut self.name {
            name.set_span(span);
        }
    }

    /// Formats `self` as an EIP-712 field: `<ty> <name>`
    pub fn fmt_eip712(&self, f: &mut impl Write) -> fmt::Result {
        write!(f, "{}", self.ty)?;
        if let Some(name) = &self.name {
            write!(f, " {}", name)?;
        }
        Ok(())
    }

    pub fn parse_for_struct(input: ParseStream<'_>) -> Result<Self> {
        Self::_parse(input, true)
    }

    fn _parse(input: ParseStream<'_>, for_struct: bool) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            ty: input.parse()?,
            storage: if input.peek(kw::memory)
                || input.peek(kw::storage)
                || input.peek(kw::calldata)
            {
                Some(input.parse()?)
            } else {
                None
            },
            // structs must have field names
            name: if for_struct || input.peek(Ident::peek_any) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct VariableDefinition {
    pub ty: Type,
    pub attributes: VariableAttributes,
    pub name: SolIdent,
    // TODO: Expr
    pub initializer: Option<(Token![=], TokenStream)>,
    pub semi_token: Token![;],
}

impl Parse for VariableDefinition {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            attributes: input.parse()?,
            name: input.parse()?,
            initializer: if input.peek(Token![=]) {
                Some((input.parse()?, tts_until_semi(input)))
            } else {
                None
            },
            semi_token: input.parse()?,
        })
    }
}

impl VariableDefinition {
    pub fn span(&self) -> Span {
        let span = self.ty.span();
        span.join(self.semi_token.span).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.ty.set_span(span);
        self.semi_token.span = span;
    }
}
