use super::{ident::SolPath, kw};
use proc_macro2::{Span, TokenStream};
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
    Result, Token,
};

mod function;
pub use function::{FunctionAttribute, FunctionAttributes};

mod variable;
pub use variable::{VariableAttribute, VariableAttributes};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Visibility {
    External(kw::external),
    Public(kw::public),
    Internal(kw::internal),
    Private(kw::private),
}

impl fmt::Debug for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_debug_str())
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Parse for Visibility {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::external) {
            Ok(Self::External(input.parse()?))
        } else if lookahead.peek(kw::public) {
            Ok(Self::Public(input.parse()?))
        } else if lookahead.peek(kw::internal) {
            Ok(Self::Internal(input.parse()?))
        } else if lookahead.peek(kw::private) {
            Ok(Self::Private(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Visibility {
    pub fn span(&self) -> Span {
        match self {
            Self::External(kw) => kw.span(),
            Self::Public(kw) => kw.span(),
            Self::Internal(kw) => kw.span(),
            Self::Private(kw) => kw.span(),
        }
    }

    pub const fn as_debug_str(&self) -> &'static str {
        match self {
            Self::External(_) => "External",
            Self::Public(_) => "Public",
            Self::Internal(_) => "Internal",
            Self::Private(_) => "Private",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::External(_) => "external",
            Self::Public(_) => "public",
            Self::Internal(_) => "internal",
            Self::Private(_) => "private",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Mutability {
    Pure(kw::pure),
    View(kw::view),
    Constant(kw::constant),
    Payable(kw::payable),
}

impl fmt::Debug for Mutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_debug_str())
    }
}

impl fmt::Display for Mutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Parse for Mutability {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::pure) {
            Ok(Self::Pure(input.parse()?))
        } else if lookahead.peek(kw::view) {
            Ok(Self::View(input.parse()?))
        } else if lookahead.peek(kw::constant) {
            Ok(Self::Constant(input.parse()?))
        } else if lookahead.peek(kw::payable) {
            Ok(Self::Payable(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Mutability {
    pub fn span(&self) -> Span {
        match self {
            Self::Pure(kw) => kw.span(),
            Self::View(kw) => kw.span(),
            Self::Constant(kw) => kw.span(),
            Self::Payable(kw) => kw.span(),
        }
    }

    pub const fn as_debug_str(&self) -> &'static str {
        match self {
            Self::Pure(_) => "Pure",
            Self::View(_) => "View",
            Self::Constant(_) => "Constant",
            Self::Payable(_) => "Payable",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Pure(_) => "pure",
            Self::View(_) => "view",
            Self::Constant(_) => "constant",
            Self::Payable(_) => "payable",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Override {
    override_token: kw::Override,
    paren_token: Option<Paren>,
    paths: Punctuated<SolPath, Token![,]>,
}

impl fmt::Debug for Override {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Override").field(&self.paths).finish()
    }
}

impl fmt::Display for Override {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("override")?;
        if self.paren_token.is_some() {
            f.write_str("(")?;
            for (i, path) in self.paths.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                path.fmt(f)?;
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

impl Parse for Override {
    fn parse(input: ParseStream) -> Result<Self> {
        let override_token = input.parse()?;
        let this = if input.peek(Paren) {
            let content;
            Self {
                override_token,
                paren_token: Some(parenthesized!(content in input)),
                paths: content.parse_terminated(SolPath::parse, Token![,])?,
            }
        } else {
            Self {
                override_token,
                paren_token: None,
                paths: Default::default(),
            }
        };
        Ok(this)
    }
}

impl Override {
    pub fn span(&self) -> Span {
        let span = self.override_token.span;
        match &self.paren_token {
            Some(paren_token) => span.join(paren_token.span.join()).unwrap_or(span),
            None => span,
        }
    }
}

#[derive(Clone)]
pub struct Modifier {
    name: SolPath,
    paren_token: Paren,
    // TODO: Expr
    arguments: Punctuated<TokenStream, Token![,]>,
}

impl PartialEq for Modifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Modifier {}

impl Hash for Modifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Debug for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Modifier")
            .field("name", &self.name)
            .field("arguments", &self.arguments)
            .finish()
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            name: input.parse()?,
            paren_token: parenthesized!(content in input),
            arguments: content.parse_terminated(TokenStream::parse, Token![,])?,
        })
    }
}

impl Modifier {
    pub fn span(&self) -> Span {
        let span = self.name.span();
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }
}
