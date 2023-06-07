use super::{kw, SolIdent, Storage, Type};
use proc_macro2::Span;
use quote::format_ident;
use std::{
    fmt::{self, Write},
    ops::{Deref, DerefMut},
};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, Result, Token,
};

/// A list of comma-separated [VariableDeclaration]s.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.parameterList>
pub type ParameterList = Parameters<syn::token::Comma>;

/// A list of semicolon-separated [VariableDeclaration]s.
pub type FieldList = Parameters<syn::token::Semi>;

/// A list of [VariableDeclaration]s, separated by `P`.
///
/// Currently, `P` can only be `Token![,]` or `Token![;]`.
///
/// It is recommended to use the type aliases where possible instead.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters<P>(Punctuated<VariableDeclaration, P>);

impl<P> Deref for Parameters<P> {
    type Target = Punctuated<VariableDeclaration, P>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P> DerefMut for Parameters<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P> fmt::Debug for Parameters<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Parameter list: fields names are set to `_{index}`
impl Parse for Parameters<Token![,]> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut list = input.parse_terminated(VariableDeclaration::parse, Token![,])?;

        // Set names for anonymous parameters
        for (i, var) in list.iter_mut().enumerate() {
            if var.name.is_none() {
                let mut ident = format_ident!("_{i}");
                ident.set_span(var.span());
                var.name = Some(SolIdent(ident));
            }
        }

        Ok(Self(list))
    }
}

/// Struct: enforce semicolon after each field and field name.
impl Parse for Parameters<Token![;]> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let this = input.parse_terminated(VariableDeclaration::parse_for_struct, Token![;])?;
        if this.is_empty() {
            Err(input.error("defining empty structs is disallowed"))
        } else if !this.trailing_punct() {
            Err(input.error("expected trailing semicolon"))
        } else {
            Ok(Self(this))
        }
    }
}

impl<P> IntoIterator for Parameters<P> {
    type IntoIter = <Punctuated<VariableDeclaration, P> as IntoIterator>::IntoIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, P> IntoIterator for &'a Parameters<P> {
    type IntoIter = syn::punctuated::Iter<'a, VariableDeclaration>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, P> IntoIterator for &'a mut Parameters<P> {
    type IntoIter = syn::punctuated::IterMut<'a, VariableDeclaration>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<P> Parameters<P> {
    pub const fn new() -> Self {
        Self(Punctuated::new())
    }

    pub fn eip712_signature(&self, mut name: String) -> String {
        name.reserve(2 + self.len() * 32);
        name.push('(');
        for (i, field) in self.iter().enumerate() {
            if i > 0 {
                name.push(',');
            }
            field.fmt_eip712(&mut name).unwrap();
        }
        name.push(')');
        name
    }

    pub fn names(
        &self,
    ) -> impl ExactSizeIterator<Item = Option<&SolIdent>> + DoubleEndedIterator + Clone {
        self.iter().map(|var| var.name.as_ref())
    }

    pub fn types(&self) -> impl ExactSizeIterator<Item = &Type> + DoubleEndedIterator + Clone {
        self.iter().map(|var| &var.ty)
    }

    pub fn types_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Type> + DoubleEndedIterator {
        self.iter_mut().map(|var| &mut var.ty)
    }

    pub fn type_strings(
        &self,
    ) -> impl ExactSizeIterator<Item = String> + DoubleEndedIterator + Clone + '_ {
        self.iter().map(|var| var.ty.to_string())
    }

    #[cfg(feature = "visit")]
    pub fn visit_types(&self, mut f: impl FnMut(&Type)) {
        self.types().for_each(|ty| ty.visit(&mut f))
    }

    #[cfg(feature = "visit-mut")]
    pub fn visit_types_mut(&mut self, mut f: impl FnMut(&mut Type)) {
        self.types_mut().for_each(|ty| ty.visit_mut(&mut f))
    }
}

/// A variable declaration.
///
/// `<ty> [storage] <name>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDeclaration {
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
                    return Err(Error::new(storage.span(), msg))
                }
                Some(storage)
            } else {
                None
            },
            // structs must have field names
            name: if for_struct || input.peek(Ident::peek_any) {
                Some(input.parse()?)
            } else {
                None
            },
        };
        Ok(this)
    }
}
