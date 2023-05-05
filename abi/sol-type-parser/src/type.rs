use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use std::{
    fmt,
    num::{IntErrorKind, NonZeroU16},
};
use syn::{
    bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Paren},
    Ident, LitInt, Result, Token,
};

mod kw {
    syn::custom_keyword!(tuple);
}

#[derive(Clone)]
pub struct SolArray {
    ty: Box<SolDataType>,
    bracket_token: Bracket,
    size: Option<LitInt>,
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

    pub fn parse(input: ParseStream, ty: SolDataType) -> Result<Self> {
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
    types: Punctuated<SolDataType, Token![,]>,
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
        Ok(SolTuple {
            tuple_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            types: content.parse_terminated(SolDataType::parse, Token![,])?,
        })
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
pub enum SolDataType {
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

    /// `Some(size) => <size>`, `None => `
    Array(SolArray),
    /// `(tuple)? ( $($type),* )`
    Tuple(SolTuple),

    Other(Ident),
}

impl fmt::Debug for SolDataType {
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

impl fmt::Display for SolDataType {
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

impl Parse for SolDataType {
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
                        match parse_size(input, s)? {
                            None => Self::Other(ident),
                            Some(Some(size)) if size.get() > 32 => {
                                return Err(input.error("fixed bytes range is 1-32"));
                            }
                            Some(size) => Self::Bytes { span, size },
                        }
                    } else if let Some(s) = s.strip_prefix("int") {
                        match parse_size(input, s)? {
                            None => Self::Other(ident),
                            Some(Some(size)) if size.get() > 256 || size.get() % 8 != 0 => {
                                return Err(input.error("intX must be a multiple of 8 up to 256"));
                            }
                            Some(size) => Self::Int { span, size },
                        }
                    } else if let Some(s) = s.strip_prefix("uint") {
                        match parse_size(input, s)? {
                            None => Self::Other(ident),
                            Some(Some(size)) if size.get() > 256 || size.get() % 8 != 0 => {
                                return Err(input.error("uintX must be a multiple of 8 up to 256"));
                            }
                            Some(size) => Self::Uint { span, size },
                        }
                    } else {
                        Self::Other(ident)
                    }
                }
            }
        } else {
            return Err(input.error("expected solidity type"));
        };

        // while the next token is a bracket, parse an array size and nest the
        // candidate into an array
        while input.peek(Bracket) {
            candidate = Self::Array(SolArray::parse(input, candidate)?);
        }

        Ok(candidate)
    }
}

impl ToTokens for SolDataType {
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

impl SolDataType {
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

    pub fn is_non_primitive(&self) -> bool {
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
fn parse_size(input: ParseStream<'_>, s: &str) -> Result<Option<Option<NonZeroU16>>> {
    let opt = match s.parse::<NonZeroU16>() {
        Ok(size) => Some(Some(size)),
        Err(e) => match e.kind() {
            // bytes
            IntErrorKind::Empty => Some(None),
            // bytes_
            IntErrorKind::InvalidDigit => None,
            // bytesN where N == 0 || N > MAX
            _ => return Err(input.error(format_args!("invalid size: {e}"))),
        },
    };
    Ok(opt)
}
