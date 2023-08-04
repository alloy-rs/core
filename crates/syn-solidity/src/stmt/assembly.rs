use crate::{kw, LitStr, YulBlock};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Result, Token,
};

#[derive(Clone)]
pub struct StmtAssembly {
    pub assembly_token: kw::assembly,
    pub literal: Option<LitStr>,
    pub flags: Option<AssemblyFlags>,
    pub block: YulBlock,
}

impl fmt::Debug for StmtAssembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StmtAssembly")
            .field("literal", &self.literal)
            .field("flags", &self.flags)
            .field("block", &self.block)
            .finish()
    }
}

impl Parse for StmtAssembly {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            assembly_token: input.parse()?,
            literal: input.call(LitStr::parse_opt)?,
            flags: input.call(AssemblyFlags::parse_opt)?,
            block: input.parse()?,
        })
    }
}

impl StmtAssembly {
    pub fn span(&self) -> Span {
        let span = self.assembly_token.span;
        span.join(self.block.span()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.assembly_token.span = span;
        self.block.set_span(span);
    }
}

#[derive(Clone)]
pub struct AssemblyFlags {
    pub paren_token: Paren,
    pub strings: Punctuated<LitStr, Token![,]>,
}

impl fmt::Debug for AssemblyFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssemblyFlags")
            .field("strings", &self.strings)
            .finish()
    }
}

impl Parse for AssemblyFlags {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            paren_token: syn::parenthesized!(content in input),
            strings: content.parse_terminated(LitStr::parse, Token![,])?,
        })
    }
}

impl AssemblyFlags {
    pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
        if input.peek(Paren) {
            input.parse().map(Some)
        } else {
            Ok(None)
        }
    }
}
