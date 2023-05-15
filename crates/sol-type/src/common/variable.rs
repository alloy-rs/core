use super::{keccak256, kw, SolIdent, Storage};
use crate::r#type::{CustomType, Type};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
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
    fn parse(input: ParseStream) -> Result<Self> {
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
    fn parse(input: ParseStream) -> Result<Self> {
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

impl<P: ToTokens> ToTokens for Parameters<P> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
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
    pub fn new() -> Self {
        Self(Punctuated::new())
    }

    pub fn sig_and_sel(&self, name: String) -> (String, [u8; 4]) {
        let sig = self.signature(name);
        let sel = keccak256(sig.as_bytes())[..4].try_into().unwrap();
        (sig, sel)
    }

    pub fn signature(&self, mut name: String) -> String {
        name.reserve(2 + self.len() * 16);
        self.fmt_as_tuple(&mut name).unwrap();
        name
    }

    pub fn fmt_as_tuple(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_char('(')?;
        for (i, var) in self.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            write!(f, "{}", var.ty)?;
        }
        f.write_char(')')
    }

    pub fn selector(&self, name: String) -> [u8; 4] {
        let signature = self.signature(name);
        keccak256(signature.as_bytes())[..4].try_into().unwrap()
    }

    pub fn eip712_signature(&self, mut name: String) -> String {
        name.reserve(2 + self.len() * 32);
        name.push('(');
        for (i, field) in self.iter().enumerate() {
            if i > 0 {
                name.push(',');
            }
            write!(name, "{field}").unwrap();
        }
        name.push(')');
        name
    }

    pub fn names(
        &self,
    ) -> impl ExactSizeIterator<Item = &Option<SolIdent>> + DoubleEndedIterator + Clone {
        self.iter().map(|var| &var.name)
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

    pub fn data_size(&self, base: Option<TokenStream>) -> TokenStream {
        let base = base.unwrap_or_else(|| quote!(self));
        let sizes = self.iter().map(|var| {
            let field = var.name.as_ref().unwrap();
            var.ty.data_size(quote!(#base.#field))
        });
        quote!(0usize #( + #sizes)*)
    }

    pub fn assert_resolved(&self) {
        self.iter().for_each(|var| var.ty.assert_resolved())
    }
}

/// `<ty> [storage] <name>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDeclaration {
    /// The type of the variable.
    pub ty: Type,
    /// The storage location, if any, of the variable.
    pub storage: Option<Storage>,
    /// The name of the variable. This is always Some if parsed as part of
    /// [`Parameters`].
    pub name: Option<SolIdent>,
}

/// Formats as an EIP-712 field: `<ty> <name>`
impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // important: Custom structs are encoded dynamically at run time.
        // TODO: We can probably avoid doing this if we have all the resolved types.
        match &self.ty {
            Type::Custom(CustomType::Struct(s)) => s.name.fmt(f),
            Type::Custom(other) => other.fmt(f),
            ty => ty.fmt(f),
        }?;
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
    pub fn new(ty: Type) -> Self {
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
