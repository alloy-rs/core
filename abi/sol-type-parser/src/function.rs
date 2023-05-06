use crate::common::{kw, FunctionAttributes, SolIdent, VariableDeclaration};
use proc_macro2::{Span, TokenStream};
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Attribute, Error, Result, Token,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Returns {
    returns_token: kw::returns,
    paren_token: Paren,
    returns: Punctuated<VariableDeclaration, Token![,]>,
}

impl fmt::Debug for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Returns").field(&self.returns).finish()
    }
}

impl fmt::Display for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("returns (")?;
        for (i, r) in self.returns.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{r}")?;
        }
        f.write_str(")")
    }
}

impl Parse for Returns {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let this = Self {
            returns_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            returns: content.parse_terminated(VariableDeclaration::parse, Token![,])?,
        };
        if this.returns.is_empty() {
            Err(Error::new(
                this.paren_token.span.join(),
                "expected at least one return type",
            ))
        } else {
            Ok(this)
        }
    }
}

impl Returns {
    #[allow(dead_code)]
    pub fn span(&self) -> Span {
        let span = self.returns_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }
}

pub struct Function {
    _function_token: kw::function,
    name: SolIdent,
    _paren_token: Paren,
    arguments: Punctuated<VariableDeclaration, Token![,]>,
    attributes: FunctionAttributes,
    returns: Option<Returns>,
    _semi_token: Token![;],
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("arguments", &self.arguments)
            .field("attributes", &self.attributes)
            .field("returns", &self.returns)
            .finish()
    }
}

impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _function_token: input.parse()?,
            name: input.parse()?,
            _paren_token: parenthesized!(content in input),
            arguments: content.parse_terminated(VariableDeclaration::parse, Token![,])?,
            attributes: input.parse()?,
            returns: if input.peek(kw::returns) {
                Some(input.parse()?)
            } else {
                None
            },
            _semi_token: input.parse()?,
        })
    }
}

impl Function {
    pub fn to_tokens(&self, _tokens: &mut TokenStream, _attrs: &[Attribute]) {
        let _ = &self.name;
        let _ = &self.arguments;
        let _ = self.arguments.iter().map(|x| x.span());
        // TODO
    }
}
