use crate::{kw, utils::tts_until_semi, SolIdent, Spanned};
use proc_macro2::{Span, TokenStream};
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

/// A pragma directive: `pragma solidity ^0.8.0;`
#[derive(Clone)]
pub struct PragmaDirective {
    pub pragma_token: kw::pragma,
    pub tokens: PragmaTokens,
    pub semi_token: Token![;],
}

impl fmt::Display for PragmaDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pragma {};", self.tokens)
    }
}

impl fmt::Debug for PragmaDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PragmaDirective").field(&self.tokens).finish()
    }
}

impl Parse for PragmaDirective {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            pragma_token: input.parse()?,
            tokens: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Spanned for PragmaDirective {
    fn span(&self) -> Span {
        let span = self.pragma_token.span;
        span.join(self.semi_token.span).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.pragma_token.span = span;
        self.tokens.set_span(span);
        self.semi_token.span = span;
    }
}

#[derive(Clone, Debug)]
pub enum PragmaTokens {
    Version(kw::solidity, TokenStream),
    Abicoder(kw::abicoder, SolIdent),
    Experimental(kw::experimental, SolIdent),
    Verbatim(TokenStream),
}

impl fmt::Display for PragmaTokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Version(_, version) => write!(f, "solidity {version}"),
            Self::Abicoder(_, ident) => write!(f, "abicoder {ident}"),
            Self::Experimental(_, ident) => write!(f, "experimental {ident}"),
            Self::Verbatim(tokens) => tokens.fmt(f),
        }
    }
}

impl Parse for PragmaTokens {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(kw::solidity) {
            let solidity = input.parse()?;
            let version = tts_until_semi(input);
            Ok(Self::Version(solidity, version))
        } else if input.peek(kw::abicoder) {
            let abicoder = input.parse()?;
            let ident = input.parse()?;
            Ok(Self::Abicoder(abicoder, ident))
        } else if input.peek(kw::experimental) {
            let experimental = input.parse()?;
            let ident = input.parse()?;
            Ok(Self::Experimental(experimental, ident))
        } else {
            Ok(Self::Verbatim(tts_until_semi(input)))
        }
    }
}

impl Spanned for PragmaTokens {
    fn span(&self) -> Span {
        match self {
            Self::Version(solidity, version) => {
                let span = solidity.span;
                span.join(version.span()).unwrap_or(span)
            }
            Self::Abicoder(abicoder, ident) => {
                let span = abicoder.span;
                span.join(ident.span()).unwrap_or(span)
            }
            Self::Experimental(experimental, ident) => {
                let span = experimental.span;
                span.join(ident.span()).unwrap_or(span)
            }
            Self::Verbatim(tokens) => tokens.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Version(solidity, _version) => {
                solidity.span = span;
            }
            Self::Abicoder(abicoder, ident) => {
                abicoder.span = span;
                ident.set_span(span);
            }
            Self::Experimental(experimental, ident) => {
                experimental.span = span;
                ident.set_span(span);
            }
            Self::Verbatim(_tokens) => {}
        }
    }
}
