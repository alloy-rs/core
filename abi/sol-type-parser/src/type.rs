use crate::common::kw;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::{IntErrorKind, NonZeroU16},
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
    ty: Box<Type>,
    bracket_token: Bracket,
    size: Option<LitInt>,
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

impl SolArray {
    pub fn span(&self) -> Span {
        let span = self.ty.span();
        span.join(self.bracket_token.span.join()).unwrap_or(span)
    }

    pub fn parse(input: ParseStream, ty: Type) -> Result<Self> {
        let content;
        Ok(SolArray {
            ty: Box::new(ty),
            bracket_token: bracketed!(content in input),
            size: content.parse()?,
        })
    }
}

#[derive(Clone)]
pub struct SolTuple {
    tuple_token: Option<kw::tuple>,
    paren_token: Paren,
    types: Punctuated<Type, Token![,]>,
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
        f.debug_struct("SolTuple").finish()
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
        let this = SolTuple {
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

impl SolTuple {
    pub fn span(&self) -> Span {
        let span = self.paren_token.span.join();
        self.tuple_token
            .and_then(|tuple| tuple.span.join(span))
            .unwrap_or(span)
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

    /// Rust Ident assumed to be a `ethers_abi_enc::SolidityType` implementor.
    Other(Ident),
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
            (Self::Other(a), Self::Other(b)) => a == b,
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
            Self::Bytes { size, .. } => {
                size.hash(state);
            }
            Self::Int { size, .. } => {
                size.hash(state);
            }
            Self::Uint { size, .. } => {
                size.hash(state);
            }
            Self::Tuple(tuple) => {
                tuple.hash(state);
            }
            Self::Array(array) => {
                array.hash(state);
            }
            Self::Other(name) => {
                name.hash(state);
            }
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
            Self::Other(name) => name.fmt(f),
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
            Self::Other(name) => name.fmt(f),
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
                            None => Self::Other(ident),
                            Some(Some(size)) if size.get() > 32 => {
                                return Err(Error::new(span, "fixed bytes range is 1-32"));
                            }
                            Some(size) => Self::Bytes { span, size },
                        }
                    } else if let Some(s) = s.strip_prefix("int") {
                        match parse_size(s, span)? {
                            None => Self::Other(ident),
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
                            None => Self::Other(ident),
                            Some(Some(size)) if size.get() > 256 || size.get() % 8 != 0 => {
                                return Err(Error::new(
                                    span,
                                    "uintX must be a multiple of 8 up to 256",
                                ));
                            }
                            Some(size) => Self::Uint { span, size },
                        }
                    } else {
                        Self::Other(ident)
                    }
                }
            }
        } else {
            return Err(input.error( "expected a Solidity type: `address`, `bool`, `string`, `bytesN`, `intN`, `uintN`, or a tuple"));
        };

        // while the next token is a bracket, parse an array size and nest the
        // candidate into an array
        while input.peek(Bracket) {
            candidate = Self::Array(SolArray::parse(input, candidate)?);
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

            Self::Bytes { span, size } => match size {
                Some(size) => {
                    let size = Literal::u16_unsuffixed(size.get());
                    quote_spanned! {span=>
                        ::ethers_abi_enc::sol_data::FixedBytes<#size>
                    }
                }
                None => quote_spanned! {span=>
                    ::ethers_abi_enc::sol_data::Bytes
                },
            },
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
            Self::Other(ref ident) => return ident.to_tokens(tokens),
        };
        tokens.extend(expanded);
    }
}

impl Type {
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
            Self::Other(ident) => ident.span(),
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
        matches!(self, Self::Other(_))
    }
}

fn write_opt(f: &mut fmt::Formatter<'_>, name: &str, size: Option<NonZeroU16>) -> fmt::Result {
    f.write_str(name)?;
    if let Some(size) = size {
        write!(f, "{size}")?;
    }
    Ok(())
}

// None => Other
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
