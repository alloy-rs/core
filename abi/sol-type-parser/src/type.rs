use std::fmt::Debug;

use proc_macro2::TokenStream;
use syn::{
    bracketed, parenthesized, parse::Parse, punctuated::Punctuated, Error, Ident, LitInt, Token,
};

use quote::{quote, ToTokens};

pub struct ArraySize {
    _bracket: syn::token::Bracket,
    size: Option<LitInt>,
}

impl Debug for ArraySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

pub struct SolTuple {
    _tup: Option<kw::tuple>,
    parenthesized: syn::token::Paren,
    inner: Punctuated<SolType, Token![,]>,
}

impl Debug for SolTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolTuple").finish()
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

pub enum SolType {
    Address,
    Array(Box<SolType>, ArraySize),
    Bool,
    Bytes,
    FixedBytes(LitInt),
    Function,
    Int(LitInt),
    String,
    Uint(LitInt),
    Tuple(SolTuple),
    Other(Ident),
}

impl std::fmt::Debug for SolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address => write!(f, "Address"),
            Self::Array(arg0, arg1) => f.debug_tuple("Array").field(arg0).field(arg1).finish(),
            Self::Bool => write!(f, "Bool"),
            Self::Bytes => write!(f, "Bytes"),
            Self::FixedBytes(arg0) => f
                .debug_tuple("FixedBytes")
                .field(&arg0.base10_digits())
                .finish(),
            Self::Function => write!(f, "Function"),
            Self::Int(arg0) => f.debug_tuple("Int").field(&arg0.base10_digits()).finish(),
            Self::String => write!(f, "String"),
            Self::Uint(arg0) => f.debug_tuple("Uint").field(&arg0.base10_digits()).finish(),
            Self::Tuple(arg0) => f.debug_tuple("Tuple").field(arg0).finish(),
            Self::Other(arg0) => f.debug_tuple("Other").field(arg0).finish(),
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
            SolType::Function => quote! { ::ethers_abi_enc::sol_type::Function },
            SolType::Int(size) => quote! { ::ethers_abi_enc::sol_type::Int<#size> },
            SolType::String => quote! { ::ethers_abi_enc::sol_type::String },
            SolType::Uint(size) => quote! { ::ethers_abi_enc::sol_type::Uint<#size> },
            SolType::Tuple(inner) => return inner.to_tokens(tokens),
            SolType::Other(ident) => {
                quote! { #ident }
            }
        };
        tokens.extend(expanded);
    }
}

mod kw {
    syn::custom_keyword!(tuple);
    syn::custom_keyword!(address);
    syn::custom_keyword!(bool);
    syn::custom_keyword!(bytes);
    syn::custom_keyword!(function);
    syn::custom_keyword!(string);
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
        } else if input.peek(kw::function) {
            let _ = input.parse::<kw::function>()?;
            Self::Function
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
