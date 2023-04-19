use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt;
use syn::{
    bracketed, parenthesized, parse::Parse, punctuated::Punctuated, Error, Ident, LitInt, Token,
};

mod kw {
    syn::custom_keyword!(tuple);
    syn::custom_keyword!(address);
    syn::custom_keyword!(bool);
    syn::custom_keyword!(bytes);
    syn::custom_keyword!(function);
    syn::custom_keyword!(string);
}

#[derive(Clone)]
pub struct ArraySize {
    _bracket: syn::token::Bracket,
    size: Option<LitInt>,
}

impl fmt::Debug for ArraySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self
            .size
            .as_ref()
            .map(|s| s.base10_digits().to_owned())
            .unwrap_or_default();

        f.debug_struct("ArraySize").field("size", &size).finish()
    }
}

impl Parse for ArraySize {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Bracket) {
            let content;
            return Ok(Self {
                _bracket: bracketed!(content in input),
                size: content.parse()?,
            });
        }
        Err(Error::new(
            input.span(),
            "expected brackets for solidity array",
        ))
    }
}
#[derive(Clone)]
pub struct SolTuple {
    _tup: Option<kw::tuple>,
    parenthesized: syn::token::Paren,
    inner: Punctuated<SolType, Token![,]>,
}

impl fmt::Debug for SolTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SolTuple").finish()
    }
}

impl fmt::Display for SolTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        write!(f, ")")
    }
}

impl Parse for SolTuple {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        Ok(SolTuple {
            _tup: input.parse()?,
            parenthesized: parenthesized!(content in input),
            inner: content.parse_terminated(SolType::parse, Token![,])?,
        })
    }
}

impl ToTokens for SolTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.parenthesized.surround(tokens, |tokens| {
            self.inner.to_tokens(tokens);
        })
    }
}

#[derive(Clone)]
pub enum SolType {
    Address,
    Array(Box<SolType>, ArraySize),
    Bool,
    Bytes,
    FixedBytes(LitInt),
    Int(LitInt),
    String,
    Uint(LitInt),
    Tuple(SolTuple),
    Other(Ident),
}

impl fmt::Debug for SolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address => write!(f, "Address"),
            Self::Array(arg0, arg1) => f.debug_tuple("Array").field(arg0).field(arg1).finish(),
            Self::Bool => write!(f, "Bool"),
            Self::Bytes => write!(f, "Bytes"),
            Self::FixedBytes(arg0) => f
                .debug_tuple("FixedBytes")
                .field(&arg0.base10_digits())
                .finish(),
            Self::Int(arg0) => f.debug_tuple("Int").field(&arg0.base10_digits()).finish(),
            Self::String => write!(f, "String"),
            Self::Uint(arg0) => f.debug_tuple("Uint").field(&arg0.base10_digits()).finish(),
            Self::Tuple(arg0) => f.debug_tuple("Tuple").field(arg0).finish(),
            Self::Other(arg0) => f.debug_tuple("Other").field(arg0).finish(),
        }
    }
}

impl fmt::Display for SolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolType::Address => write!(f, "address"),
            SolType::Array(ty, size) => {
                write!(
                    f,
                    "{}[{}]",
                    ty,
                    size.size
                        .as_ref()
                        .map(|s| s.base10_digits())
                        .unwrap_or_default()
                )
            }
            SolType::Bool => write!(f, "bool"),
            SolType::Bytes => write!(f, "bytes"),
            SolType::FixedBytes(size) => write!(f, "bytes{}", size.base10_digits()),
            SolType::Int(size) => write!(f, "int{}", size.base10_digits()),
            SolType::String => write!(f, "string"),
            SolType::Uint(size) => write!(f, "uint{}", size.base10_digits()),
            SolType::Tuple(inner) => write!(f, "{}", inner),
            SolType::Other(name) => write!(f, "{}", name),
        }
    }
}

impl ToTokens for SolType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expanded = match self {
            SolType::Address => quote! { ::ethers_abi_enc::sol_type::Address },
            SolType::Array(inner, size) => {
                if let Some(size) = &size.size {
                    quote! {
                        ::ethers_abi_enc::sol_type::FixedArray<#inner, #size>
                    }
                } else {
                    quote! {
                        ::ethers_abi_enc::sol_type::Array<#inner>
                    }
                }
            }
            SolType::Bool => quote! { ::ethers_abi_enc::sol_type::Bool },
            SolType::Bytes => quote! { ::ethers_abi_enc::sol_type::Bytes },
            SolType::FixedBytes(size) => quote! {::ethers_abi_enc::sol_type::FixedBytes<#size>},
            SolType::Int(size) => quote! { ::ethers_abi_enc::sol_type::Int<#size> },
            SolType::String => quote! { ::ethers_abi_enc::sol_type::String },
            SolType::Uint(size) => quote! { ::ethers_abi_enc::sol_type::Uint<#size> },
            SolType::Tuple(inner) => return inner.to_tokens(tokens),
            SolType::Other(ident) => quote! { #ident },
        };
        tokens.extend(expanded);
    }
}

impl Parse for SolType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut candidate = if input.peek(kw::address) {
            let _ = input.parse::<kw::address>()?;
            Self::Address
        } else if input.peek(kw::bool) {
            let _ = input.parse::<kw::bool>()?;
            Self::Bool
        } else if input.peek(kw::bytes) {
            let _ = input.parse::<kw::bytes>()?;
            Self::Bytes
        } else if input.peek(kw::string) {
            let _ = input.parse::<kw::string>()?;
            Self::String
        } else if input.peek(syn::token::Paren) || input.peek(kw::tuple) {
            Self::Tuple(SolTuple::parse(input)?)
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let s = ident.to_string();
            if let Some(num) = s.strip_prefix("bytes") {
                let i = LitInt::new(num, ident.span());
                let parsed: usize = i.base10_parse()?;
                if parsed > 32 {
                    return Err(syn::Error::new(i.span(), "fixed bytes range is 1-32"));
                }
                Self::FixedBytes(i)
            } else if let Some(num) = s.strip_prefix("uint") {
                let i = LitInt::new(num, ident.span());
                let parsed: usize = i.base10_parse()?;
                if parsed > 256 || parsed % 8 != 0 {
                    return Err(syn::Error::new(
                        i.span(),
                        "uint must be a multiple of 8 up to 256",
                    ));
                }
                Self::Uint(i)
            } else if let Some(num) = s.strip_prefix("int") {
                let i = LitInt::new(num, ident.span());
                let parsed: usize = i.base10_parse()?;
                if parsed > 256 || parsed % 8 != 0 {
                    return Err(syn::Error::new(
                        i.span(),
                        "intX must be a multiple of 8 up to 256",
                    ));
                }
                Self::Int(i)
            } else {
                Self::Other(ident)
            }
        } else {
            return Err(Error::new(input.span(), "no candidate sol type found"));
        };

        // while the next token is a bracket, parse an array size and nest the
        // candidate into an array
        while input.peek(syn::token::Bracket) {
            candidate = Self::Array(Box::new(candidate), input.parse()?)
        }
        Ok(candidate)
    }
}

impl SolType {
    pub fn is_non_primitive(&self) -> bool {
        matches!(self, Self::Other(_))
    }
}
