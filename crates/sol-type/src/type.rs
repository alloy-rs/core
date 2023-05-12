use crate::{common::kw, r#struct::Struct, udt::Udt};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::{IntErrorKind, NonZeroU16, NonZeroU64},
};
use syn::{
    bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Paren},
    Error, Ident, LitInt, Result, Token,
};

#[derive(Clone)]
pub struct SolArray {
    pub ty: Box<Type>,
    bracket_token: Bracket,
    pub size: Option<LitInt>,
}

impl PartialEq for SolArray {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.size == other.size
    }
}

impl Eq for SolArray {}

impl Hash for SolArray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.size.hash(state);
    }
}

impl fmt::Debug for SolArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SolArray")
            .field(&self.ty)
            .field(&self.size.as_ref().map(|s| s.base10_digits()))
            .finish()
    }
}

impl fmt::Display for SolArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ty.fmt(f)?;
        f.write_str("[")?;
        if let Some(s) = &self.size {
            f.write_str(s.base10_digits())?;
        }
        f.write_str("]")
    }
}

impl ToTokens for SolArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.ty;
        let span = self.span();
        let expanded = if let Some(size) = &self.size {
            quote_spanned! {span=>
                ::ethers_abi_enc::sol_data::FixedArray<#ty, #size>
            }
        } else {
            quote_spanned! {span=>
                ::ethers_abi_enc::sol_data::Array<#ty>
            }
        };
        tokens.extend(expanded);
    }
}

impl Parse for SolArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        Self::wrap(input, ty)
    }
}

impl SolArray {
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

    pub fn wrap(input: ParseStream, ty: Type) -> Result<Self> {
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

#[derive(Clone)]
pub struct SolTuple {
    tuple_token: Option<kw::tuple>,
    paren_token: Paren,
    pub types: Punctuated<Type, Token![,]>,
}

impl PartialEq for SolTuple {
    fn eq(&self, other: &Self) -> bool {
        self.types == other.types
    }
}

impl Eq for SolTuple {}

impl Hash for SolTuple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.types.hash(state);
    }
}

impl fmt::Debug for SolTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SolTuple").field(&self.types).finish()
    }
}

impl fmt::Display for SolTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        for (i, ty) in self.types.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            ty.fmt(f)?;
        }
        f.write_str(")")
    }
}

impl Parse for SolTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let this = Self {
            tuple_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            types: content.parse_terminated(Type::parse, Token![,])?,
        };
        match this.types.len() {
            0 => Err(Error::new(
                this.paren_token.span.join(),
                "empty tuples are not allowed",
            )),
            1 if !this.types.trailing_punct() => Err(Error::new(
                this.paren_token.span.close(),
                "single element tuples must have a trailing comma",
            )),
            _ => Ok(this),
        }
    }
}

impl ToTokens for SolTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token
            .surround(tokens, |tokens| self.types.to_tokens(tokens))
    }
}

impl FromIterator<Type> for SolTuple {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        SolTuple {
            tuple_token: None,
            paren_token: Paren::default(),
            types: {
                let mut types = iter.into_iter().collect::<Punctuated<_, _>>();
                // ensure trailing comma for single item tuple
                if !types.trailing_punct() && types.len() == 1 {
                    types.push_punct(Default::default())
                }
                types
            },
        }
    }
}

impl SolTuple {
    pub fn span(&self) -> Span {
        let span = self.paren_token.span.join();
        self.tuple_token
            .and_then(|tuple_token| tuple_token.span.join(span))
            .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        if let Some(tuple_token) = &mut self.tuple_token {
            *tuple_token = kw::tuple(span);
        }
        self.paren_token = Paren(span);
    }
}

/// A custom type that implements `ethers_abi_enc::SolidityType`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CustomType {
    /// A type that has not yet been resolved
    Unresolved(Ident),
    /// A user-defined type
    Udt(Box<Udt>),
    /// A struct
    Struct(Struct),
}

impl fmt::Display for CustomType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unresolved(ident) => ident.fmt(f),
            Self::Udt(udt) => udt.ty.fmt(f),
            Self::Struct(strukt) => strukt.fields.fmt_as_tuple(f),
        }
    }
}

impl CustomType {
    pub fn span(&self) -> Span {
        match self {
            Self::Unresolved(ident) => ident.span(),
            Self::Udt(ty) => ty.span(),
            Self::Struct(ty) => ty.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Unresolved(ident) => ident.set_span(span),
            Self::Udt(udt) => udt.set_span(span),
            Self::Struct(strukt) => strukt.set_span(span),
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            Self::Unresolved(ident) => ident,
            Self::Udt(udt) => &udt.name.0,
            Self::Struct(strukt) => &strukt.name.0,
        }
    }

    pub fn as_type(&self) -> Type {
        match self {
            Self::Unresolved(_) => Type::Custom(self.clone()),
            Self::Udt(udt) => udt.ty.clone(),
            Self::Struct(strukt) => strukt.ty(),
        }
    }
}

#[derive(Clone)]
pub enum Type {
    /// `address`
    Address(Span),
    /// `bool`
    Bool(Span),
    /// `string`
    String(Span),

    /// `Some(size) => bytes<size>`, `None => bytes`
    Bytes {
        span: Span,
        size: Option<NonZeroU16>,
    },
    /// `Some(size) => int<size>`, `None => int`
    Int {
        span: Span,
        size: Option<NonZeroU16>,
    },
    /// `Some(size) => uint<size>`, `None => uint`
    Uint {
        span: Span,
        size: Option<NonZeroU16>,
    },

    /// `Some(size) => <type>[<size>]`, `None => <type>[]`
    Array(SolArray),
    /// `(tuple)? ( $($type),* )`
    Tuple(SolTuple),

    /// A custom type, that may or may not be resolved to a Solidity type.
    Custom(CustomType),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Address(_), Self::Address(_)) => true,
            (Self::Bool(_), Self::Bool(_)) => true,
            (Self::String(_), Self::String(_)) => true,
            (Self::Bytes { size: a, .. }, Self::Bytes { size: b, .. }) => a == b,
            (Self::Int { size: a, .. }, Self::Int { size: b, .. }) => a == b,
            (Self::Uint { size: a, .. }, Self::Uint { size: b, .. }) => a == b,
            (Self::Tuple(a), Self::Tuple(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => a == b,
            (Self::Custom(a), Self::Custom(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Address(_) | Self::Bool(_) | Self::String(_) => {}
            Self::Bytes { size, .. } => size.hash(state),
            Self::Int { size, .. } => size.hash(state),
            Self::Uint { size, .. } => size.hash(state),
            Self::Tuple(tuple) => tuple.hash(state),
            Self::Array(array) => array.hash(state),
            Self::Custom(custom) => custom.hash(state),
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(_) => f.write_str("Address"),
            Self::Bool(_) => f.write_str("Bool"),
            Self::String(_) => f.write_str("String"),
            Self::Bytes { size, .. } => f.debug_tuple("Bytes").field(size).finish(),
            Self::Int { size, .. } => f.debug_tuple("Int").field(size).finish(),
            Self::Uint { size, .. } => f.debug_tuple("Uint").field(size).finish(),
            Self::Tuple(tuple) => tuple.fmt(f),
            Self::Array(array) => array.fmt(f),
            Self::Custom(custom) => custom.fmt(f),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(_) => f.write_str("address"),
            Self::Bool(_) => f.write_str("bool"),
            Self::String(_) => f.write_str("string"),
            Self::Bytes { size, .. } => write_opt(f, "bytes", *size),
            Self::Int { size, .. } => write_opt(f, "int", *size),
            Self::Uint { size, .. } => write_opt(f, "uint", *size),
            Self::Tuple(tuple) => tuple.fmt(f),
            Self::Array(array) => array.fmt(f),
            Self::Custom(custom) => custom.fmt(f),
        }
    }
}

impl Parse for Type {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut candidate = if input.peek(Paren) || input.peek(kw::tuple) {
            Self::Tuple(input.parse()?)
        } else if input.peek(Ident::peek_any) {
            let ident = input.call(Ident::parse_any)?;
            let span = ident.span();
            let s = ident.to_string();
            match s.as_str() {
                "address" => Self::Address(span),
                "bool" => Self::Bool(span),
                "string" => Self::String(span),
                s => {
                    if let Some(s) = s.strip_prefix("bytes") {
                        match parse_size(s, span)? {
                            None => Self::custom(ident),
                            Some(Some(size)) if size.get() > 32 => {
                                return Err(Error::new(span, "fixed bytes range is 1-32"));
                            }
                            Some(size) => Self::Bytes { span, size },
                        }
                    } else if let Some(s) = s.strip_prefix("int") {
                        match parse_size(s, span)? {
                            None => Self::custom(ident),
                            Some(Some(size)) if size.get() > 256 || size.get() % 8 != 0 => {
                                return Err(Error::new(
                                    span,
                                    "intX must be a multiple of 8 up to 256",
                                ));
                            }
                            Some(size) => Self::Int { span, size },
                        }
                    } else if let Some(s) = s.strip_prefix("uint") {
                        match parse_size(s, span)? {
                            None => Self::custom(ident),
                            Some(Some(size)) if size.get() > 256 || size.get() % 8 != 0 => {
                                return Err(Error::new(
                                    span,
                                    "uintX must be a multiple of 8 up to 256",
                                ));
                            }
                            Some(size) => Self::Uint { span, size },
                        }
                    } else {
                        Self::custom(ident)
                    }
                }
            }
        } else {
            return Err(input.error(
                "expected a Solidity type: \
                `address`, `bool`, `string`, `bytesN`, `intN`, `uintN`, a tuple, or a custom type name",
            ));
        };

        // while the next token is a bracket, parse an array size and nest the
        // candidate into an array
        while input.peek(Bracket) {
            candidate = Self::Array(SolArray::wrap(input, candidate)?);
        }

        Ok(candidate)
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expanded = match *self {
            Self::Address(span) => quote_spanned! {span=> ::ethers_abi_enc::sol_data::Address },
            Self::Bool(span) => quote_spanned! {span=> ::ethers_abi_enc::sol_data::Bool },
            Self::String(span) => quote_spanned! {span=> ::ethers_abi_enc::sol_data::String },

            Self::Bytes { span, size: None } => {
                quote_spanned! {span=> ::ethers_abi_enc::sol_data::Bytes }
            }
            Self::Bytes {
                span,
                size: Some(size),
            } => {
                let size = Literal::u16_unsuffixed(size.get());
                quote_spanned! {span=>
                    ::ethers_abi_enc::sol_data::FixedBytes<#size>
                }
            }

            Self::Int { span, size } => {
                let size = Literal::u16_unsuffixed(size.map(NonZeroU16::get).unwrap_or(256));
                quote_spanned! {span=>
                    ::ethers_abi_enc::sol_data::Int<#size>
                }
            }
            Self::Uint { span, size } => {
                let size = Literal::u16_unsuffixed(size.map(NonZeroU16::get).unwrap_or(256));
                quote_spanned! {span=>
                    ::ethers_abi_enc::sol_data::Uint<#size>
                }
            }

            Self::Tuple(ref tuple) => return tuple.to_tokens(tokens),
            Self::Array(ref array) => return array.to_tokens(tokens),
            Self::Custom(ref custom) => return custom.ident().to_tokens(tokens),
        };
        tokens.extend(expanded);
    }
}

impl Type {
    pub fn custom(ident: Ident) -> Self {
        Self::Custom(CustomType::Unresolved(ident))
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Address(span)
            | Self::Bool(span)
            | Self::String(span)
            | Self::Bytes { span, .. }
            | Self::Int { span, .. }
            | Self::Uint { span, .. } => *span,
            Self::Tuple(tuple) => tuple.span(),
            Self::Array(array) => array.span(),
            Self::Custom(custom) => custom.span(),
        }
    }

    pub fn set_span(&mut self, new_span: Span) {
        match self {
            Self::Address(span)
            | Self::Bool(span)
            | Self::String(span)
            | Self::Bytes { span, .. }
            | Self::Int { span, .. }
            | Self::Uint { span, .. } => *span = new_span,
            Self::Tuple(tuple) => tuple.set_span(new_span),
            Self::Array(array) => array.set_span(new_span),
            Self::Custom(custom) => custom.set_span(new_span),
        }
    }

    /// Returns whether a [Storage][crate::common::Storage] location can be specified for this type.
    pub fn can_have_storage(&self) -> bool {
        self.is_dynamic() || self.is_struct()
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(
            self,
            Self::String(_) | Self::Bytes { size: None, .. } | Self::Array(_)
        )
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Custom(..))
    }

    /// Returns the resolved type, which is the innermost type that is not `Custom`.
    ///
    /// Prefer using other methods which don't clone, like `data_size` or `Display::fmt`.
    pub fn resolved(&self) -> Self {
        match self {
            Self::Array(SolArray {
                ty,
                bracket_token,
                size,
            }) => Self::Array(SolArray {
                ty: ty.resolved().into(),
                bracket_token: *bracket_token,
                size: size.clone(),
            }),
            Self::Tuple(SolTuple {
                tuple_token,
                paren_token,
                types,
            }) => Self::Tuple(SolTuple {
                tuple_token: *tuple_token,
                paren_token: *paren_token,
                types: {
                    let mut types = types.clone();
                    for ty in &mut types {
                        *ty = ty.resolved();
                    }
                    types
                },
            }),
            Self::Custom(CustomType::Udt(udt)) => udt.ty.resolved(),
            Self::Custom(CustomType::Struct(strukt)) => strukt.ty().resolved(),
            s => s.clone(),
        }
    }

    /// Recursively calculates the base ABI-encoded size of `self` in bytes.
    ///
    /// That is, the minimum number of bytes required to encode `self` without
    /// any dynamic data.
    pub fn base_data_size(&self) -> usize {
        match self {
            // static types: 1 word
            Self::Address(_)
            | Self::Bool(_)
            | Self::Int { .. }
            | Self::Uint { .. }
            | Self::Bytes { size: Some(_), .. } => 32,

            // dynamic types: 1 offset word, 1 length word
            Self::String(_)
            | Self::Bytes { size: None, .. }
            | Self::Array(SolArray { size: None, .. }) => 64,

            // fixed array: size * encoded size
            Self::Array(SolArray {
                ty,
                size: Some(size),
                ..
            }) => ty.base_data_size() * size.base10_parse::<usize>().unwrap(),

            // tuple: sum of encoded sizes
            Self::Tuple(tuple) => tuple.types.iter().map(Type::base_data_size).sum(),

            Self::Custom(CustomType::Unresolved(ident)) => unreachable!("unresolved type: {ident}"),
            Self::Custom(custom) => custom.as_type().base_data_size(),
        }
    }

    /// Recursively calculates the ABI-encoded size of `self` in bytes.
    pub fn data_size(&self, field: TokenStream) -> TokenStream {
        match self {
            // static types: 1 word
            Self::Address(_)
            | Self::Bool(_)
            | Self::Int { .. }
            | Self::Uint { .. }
            | Self::Bytes { size: Some(_), .. } => self.base_data_size().into_token_stream(),

            // dynamic types: 1 offset word, 1 length word, length rounded up to word size
            Self::String(_) | Self::Bytes { size: None, .. } => {
                let base = self.base_data_size();
                quote!(#base + (#field.len() / 31) * 32)
            }
            Self::Array(SolArray { ty, size: None, .. }) => {
                let base = self.base_data_size();
                let inner_size = ty.data_size(field.clone());
                quote!(#base + #field.len() * (#inner_size))
            }

            // fixed array: size * encoded size
            Self::Array(SolArray {
                ty,
                size: Some(size),
                ..
            }) => {
                let base = self.base_data_size();
                let inner_size = ty.data_size(field);
                let len: usize = size.base10_parse().unwrap();
                quote!(#base + #len * (#inner_size))
            }

            // tuple: sum of encoded sizes
            Self::Tuple(tuple) => {
                let fields = tuple.types.iter().enumerate().map(|(i, ty)| {
                    let index = syn::Index::from(i);
                    let field_name = quote!(#field.#index);
                    ty.data_size(field_name)
                });
                quote!(0usize #(+ #fields)*)
            }

            Self::Custom(CustomType::Unresolved(ident)) => unreachable!("unresolved type: {ident}"),
            Self::Custom(CustomType::Struct(strukt)) => strukt.fields.data_size(Some(field)),
            Self::Custom(CustomType::Udt(udt)) => udt.ty.data_size(field),
        }
    }

    /// Asserts that this type is resolved, meaning no `CustomType::Unresolved` types are present.
    ///
    /// This is only necessary when expanding code for `SolCall` or similar traits, which require
    /// the fully resolved type to calculate the ABI signature and selector.
    pub fn assert_resolved(&self) {
        self.visit(&mut |ty| {
            if let Self::Custom(CustomType::Unresolved(ident)) = ty {
                panic!(
                    "missing necessary definition for type \"{ident}\"\n\
                     Custom types must be declared inside of the same scope they are referenced in,\n\
                     or \"imported\" as a UDT with `type {ident} is (...);`"
                )
            }
        });
    }

    /// Calls `f` on each type.
    pub fn visit(&self, f: &mut impl FnMut(&Self)) {
        match self {
            Self::Array(array) => array.ty.visit(f),
            Self::Tuple(tuple) => tuple.types.iter().for_each(|ty| ty.visit(f)),
            Self::Custom(CustomType::Struct(strukt)) => {
                strukt.fields.types().for_each(|ty| ty.visit(f))
            }
            Self::Custom(CustomType::Udt(udt)) => f(&udt.ty),
            ty => f(ty),
        }
    }

    /// Calls `f` on each type.
    pub fn visit_mut(&mut self, f: &mut impl FnMut(&mut Self)) {
        match self {
            Self::Array(array) => array.ty.visit_mut(f),
            Self::Tuple(tuple) => tuple.types.iter_mut().for_each(|ty| ty.visit_mut(f)),
            Self::Custom(CustomType::Struct(strukt)) => {
                strukt.fields.types_mut().for_each(|ty| ty.visit_mut(f))
            }
            Self::Custom(CustomType::Udt(udt)) => udt.ty.visit_mut(f),
            ty => f(ty),
        }
    }
}

fn write_opt(f: &mut fmt::Formatter<'_>, name: &str, size: Option<NonZeroU16>) -> fmt::Result {
    f.write_str(name)?;
    if let Some(size) = size {
        write!(f, "{size}")?;
    }
    Ok(())
}

// None => Custom
// Some(size) => size
fn parse_size(s: &str, span: Span) -> Result<Option<Option<NonZeroU16>>> {
    let opt = match s.parse::<NonZeroU16>() {
        Ok(size) => Some(Some(size)),
        Err(e) => match e.kind() {
            // bytes
            IntErrorKind::Empty => Some(None),
            // bytes_
            IntErrorKind::InvalidDigit => None,
            // bytesN where N == 0 || N > MAX
            _ => return Err(Error::new(span, format_args!("invalid size: {e}"))),
        },
    };
    Ok(opt)
}
