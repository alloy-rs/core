use crate::{Expr, SolIdent};
use proc_macro2::Span;
use std::fmt;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Paren},
    Result, Token,
};

#[derive(Clone)]
pub struct CallArgumentList {
    pub paren_token: Paren,
    /// The list of arguments. Can be named or unnamed.
    ///
    /// When empty, this is an empty unnamed list.
    pub list: CallArgumentListImpl,
}

impl fmt::Debug for CallArgumentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallArgumentList")
            .field("list", &self.list)
            .finish()
    }
}

impl Parse for CallArgumentList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            list: content.parse()?,
        })
    }
}

impl CallArgumentList {
    pub fn span(&self) -> Span {
        self.paren_token.span.join()
    }

    pub fn set_span(&mut self, span: Span) {
        self.paren_token = Paren(span);
    }
}

#[derive(Clone, Debug)]
pub enum CallArgumentListImpl {
    Unnamed(Punctuated<Expr, Token![,]>),
    Named(Brace, Punctuated<NamedArg, Token![,]>),
}

impl Parse for CallArgumentListImpl {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Brace) {
            let content;
            Ok(Self::Named(
                braced!(content in input),
                content.parse_terminated(NamedArg::parse, Token![,])?,
            ))
        } else {
            input
                .parse_terminated(Expr::parse, Token![,])
                .map(Self::Unnamed)
        }
    }
}

/// A named argument in an argument list: `foo: uint256(42)`
#[derive(Clone)]
pub struct NamedArg {
    pub name: SolIdent,
    pub colon_token: Token![:],
    pub arg: Expr,
}

impl fmt::Debug for NamedArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NamedArg")
            .field("name", &self.name)
            .field("arg", &self.arg)
            .finish()
    }
}

impl Parse for NamedArg {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            colon_token: input.parse()?,
            arg: input.parse()?,
        })
    }
}

impl NamedArg {
    pub fn span(&self) -> Span {
        let span = self.name.span();
        span.join(self.arg.span()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
        self.arg.set_span(span);
    }
}
