use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use syn::{
    bracketed, parenthesized, parse_macro_input, punctuated::Punctuated, Error, Ident, LitInt,
    Token,
};

use quote::{quote, ToTokens};

struct ArraySize {
    _bracket: syn::token::Bracket,
    size: Option<LitInt>,
}

impl syn::parse::Parse for ArraySize {
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

struct SolTuple {
    _tup: Option<kw::tuple>,
    parenthesized: syn::token::Paren,
    inner: Punctuated<SolType, Token![,]>,
}

impl syn::parse::Parse for SolTuple {
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

enum SolType {
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

impl syn::parse::Parse for SolType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let candidate = if input.peek(kw::address) {
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
                return Err(Error::new(ident.span(), "unknown sol type found"));
            }
        } else {
            return Err(Error::new(input.span(), "no candidate sol type found"));
        };

        if input.peek(syn::token::Bracket) {
            Ok(Self::Array(Box::new(candidate), input.parse()?))
        } else {
            Ok(candidate)
        }
    }
}

#[proc_macro]
pub fn sol(input: TS) -> TS {
    let s: SolType = parse_macro_input!(input);
    quote! {
        #s
    }
    .into()
}
