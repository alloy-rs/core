use std::fmt::Debug;

use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use syn::{
    braced, bracketed, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated,
    token::Struct, Error, Ident, Index, LitInt, Token,
};

use quote::{quote, ToTokens};

struct ArraySize {
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

struct SolTuple {
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

struct SolStructField {
    ty: SolType,
    name: Ident,
}

impl Parse for SolStructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(SolStructField {
            ty: input.parse()?,
            name: input.parse()?,
        })
    }
}

impl ToTokens for SolStructField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let ty = &self.ty;
        tokens.extend(quote! {
            #name: <#ty as ::ethers_abi_enc::SolType>::RustType
        });
    }
}

struct SolStructDef {
    _struct: Struct,
    name: Ident,
    _brace: syn::token::Brace,
    fields: Punctuated<SolStructField, Token![;]>,
}

impl Parse for SolStructDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        Ok(SolStructDef {
            _struct: input.parse()?,
            name: input.parse()?,
            _brace: braced!(content in input),
            fields: content.parse_terminated(SolStructField::parse, Token![;])?,
        })
    }
}

impl SolStructDef {
    fn rust_name_alias(&self) -> Ident {
        Ident::new(&format!("{}Rust", self.name), self.name.span())
    }

    fn token_alias(&self) -> Ident {
        Ident::new(&format!("{}Token", self.name), self.name.span())
    }

    fn expand_aliases(&self) -> TokenStream {
        let rust_name = self.rust_name_alias();
        let sol_doc = format!("A type containg info for the Solidity type {}", &self.name);
        let token_alias = self.token_alias();
        let sol_name = &self.name;
        let field_ty = self.fields.iter().map(|f| &f.ty);
        let field_ty2 = field_ty.clone();
        let field_ty3 = field_ty.clone();

        quote! {
            #[doc = #sol_doc]
            #[derive(Debug, Clone, Copy, Default)]
            pub struct #sol_name(
                #(::core::marker::PhantomData<#field_ty>),*,
                ::core::marker::PhantomData<#rust_name>,
            );

            impl #sol_name {
                /// Instantiate at runtime for dynamic encoding
                pub fn new() -> Self {
                    Self::default()
                }
            }

            type UnderlyingTuple = (#(#field_ty2),*);

            pub type #token_alias = <UnderlyingTuple as ::ethers_abi_enc::SolType>::TokenType;

            type UnderlyingRustTuple = (#(<#field_ty3 as ::ethers_abi_enc::SolType>::RustType),*);
        }
    }

    fn expand_from(&self) -> TokenStream {
        let rust_name = self.rust_name_alias();
        let field_names = self.fields.iter().map(|f| &f.name);

        let (f_no, f_name): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| (Index::from(i), &f.name))
            .unzip();

        quote! {
            impl Into<UnderlyingRustTuple> for #rust_name {
                fn into(self) -> UnderlyingRustTuple {
                    (#(self.#field_names),*)
                }
            }

            impl From<UnderlyingRustTuple> for #rust_name {
                fn from(tuple: UnderlyingRustTuple) -> Self {
                    #rust_name {
                        #(#f_name: tuple.#f_no),*
                    }
                }
            }
        }
    }

    fn expand_detokenize(&self) -> TokenStream {
        let rust_name = self.rust_name_alias();

        let (field_nos, field_names): (Vec<_>, Vec<_>) = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| (Index::from(i), &f.name))
            .unzip();

        quote! {
            fn detokenize(token: Self::TokenType) -> ::ethers_abi_enc::AbiResult<#rust_name> {
                let tuple = <UnderlyingTuple as ::ethers_abi_enc::SolType>::detokenize(token)?;
                Ok(#rust_name {
                    #(#field_names: tuple.#field_nos),*
                })
            }
        }
    }

    fn expand_tokenize(&self) -> TokenStream {
        quote! {
            fn tokenize<Borrower>(rust: Borrower) -> Self::TokenType
            where
                Borrower: ::std::borrow::Borrow<Self::RustType>
            {
                let tuple: UnderlyingRustTuple = rust.borrow().clone().into();
                <UnderlyingTuple as ::ethers_abi_enc::SolType>::tokenize(tuple)
            }
        }
    }
}

impl ToTokens for SolStructDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let rust_name = self.rust_name_alias();
        let rust_name_str = rust_name.to_string();
        let sol_name = &self.name;

        let token_alias = Ident::new(&format!("{}Token", sol_name), sol_name.span());

        let mod_name = Ident::new(&format!("__{}", sol_name), sol_name.span());

        let doc = format!(
            "A rust struct that represents the Solidity type {}",
            sol_name
        );
        let fields = self.fields.iter();

        let aliases = self.expand_aliases();
        let from = self.expand_from();
        let detokenize = self.expand_detokenize();
        let tokenize = self.expand_tokenize();

        tokens.extend(quote! {
            pub use #mod_name::{#sol_name, #rust_name, #token_alias};
            #[allow(non_snake_case)]
            mod #mod_name {
                use super::*;

                #aliases

                #[doc = #doc]
                #[derive(Debug, Clone, PartialEq)]
                pub struct #rust_name {
                    #(pub #fields),*
                }

                #from

                impl ::ethers_abi_enc::SolType for #sol_name {
                    type RustType = #rust_name;
                    type TokenType = #token_alias;

                    fn sol_type_name() -> ::std::string::String {
                        #rust_name_str.to_owned()
                    }
                    fn is_dynamic() -> bool {
                        <UnderlyingTuple as ::ethers_abi_enc::SolType>::is_dynamic()
                    }
                    fn type_check(token: &Self::TokenType) -> ::ethers_abi_enc::AbiResult<()> {
                        <UnderlyingTuple as ::ethers_abi_enc::SolType>::type_check(token)
                    }

                    #detokenize
                    #tokenize

                    fn encode_packed_to<Borrower>(target: &mut Vec<u8>, rust: Borrower)
                    where
                        Borrower: ::std::borrow::Borrow<Self::RustType>
                    {
                        let tuple: UnderlyingRustTuple = rust.borrow().clone().into();
                        <UnderlyingTuple as ::ethers_abi_enc::SolType>::encode_packed_to(target, tuple)
                    }
                }
            }
        });
    }
}

enum SolInput {
    SolType(SolType),
    SolStructDef(SolStructDef),
}

impl Parse for SolInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Struct) {
            Ok(SolInput::SolStructDef(input.parse()?))
        } else {
            Ok(SolInput::SolType(input.parse()?))
        }
    }
}

impl ToTokens for SolInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            SolInput::SolType(ty) => ty.to_tokens(tokens),
            SolInput::SolStructDef(def) => def.to_tokens(tokens),
        }
    }
}

#[proc_macro]
pub fn sol(input: TS) -> TS {
    let s: SolInput = parse_macro_input!(input);
    quote! {
        #s
    }
    .into()
}
