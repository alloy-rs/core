use crate::{SolIdent, Spanned, Type, VariableDeclaration};
use proc_macro2::Span;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
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

impl Default for &ParameterList {
    #[inline]
    fn default() -> Self {
        const NEW: &ParameterList = &ParameterList::new();
        NEW
    }
}

impl Default for &FieldList {
    #[inline]
    fn default() -> Self {
        const NEW: &FieldList = &FieldList::new();
        NEW
    }
}

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

/// Parameter list
impl Parse for ParameterList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        input
            .parse_terminated(VariableDeclaration::parse, Token![,])
            .map(Self)
    }
}

/// Struct: enforce semicolon after each field and field name.
impl Parse for FieldList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let this = input.parse_terminated(VariableDeclaration::parse_with_name, Token![;])?;
        if this.is_empty() {
            Err(input.error("defining empty structs is disallowed"))
        } else if !this.trailing_punct() {
            Err(input.error("expected trailing semicolon"))
        } else {
            Ok(Self(this))
        }
    }
}

impl<P> Spanned for Parameters<P> {
    fn span(&self) -> Span {
        crate::utils::join_spans(&self.0)
    }

    fn set_span(&mut self, span: Span) {
        crate::utils::set_spans(&mut self.0, span);
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

impl<P: Default> FromIterator<VariableDeclaration> for Parameters<P> {
    fn from_iter<T: IntoIterator<Item = VariableDeclaration>>(iter: T) -> Self {
        Self(Punctuated::from_iter(iter))
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

    pub fn types_and_names(
        &self,
    ) -> impl ExactSizeIterator<Item = (&Type, Option<&SolIdent>)> + DoubleEndedIterator {
        self.iter().map(|p| (&p.ty, p.name.as_ref()))
    }

    pub fn type_strings(
        &self,
    ) -> impl ExactSizeIterator<Item = String> + DoubleEndedIterator + Clone + '_ {
        self.iter().map(|var| var.ty.to_string())
    }

    #[cfg(feature = "visit")]
    pub fn visit_types(&self, mut f: impl FnMut(&Type)) {
        for ty in self.types() {
            ty.visit(&mut f);
        }
    }

    #[cfg(feature = "visit-mut")]
    pub fn visit_types_mut(&mut self, mut f: impl FnMut(&mut Type)) {
        for ty in self.types_mut() {
            ty.visit_mut(&mut f);
        }
    }
}
