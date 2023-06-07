use super::{kw, utils::DebugPunctuated, SolPath};
use proc_macro2::{Span, TokenStream};
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Result, Token,
};

mod function;
pub use function::{FunctionAttribute, FunctionAttributes};

mod variable;
pub use variable::{VariableAttribute, VariableAttributes};

kw_enum! {
    /// A storage location.
    pub enum Storage {
        /// `memory`
        Memory(kw::memory),
        /// `storage`
        Storage(kw::storage),
        /// `calldata`
        Calldata(kw::calldata),
    }
}

kw_enum! {
    /// A visibility attribute.
    pub enum Visibility {
        /// `external`
        External(kw::external),
        /// `public`
        Public(kw::public),
        /// `internal`
        Internal(kw::internal),
        /// `private`
        Private(kw::private),
    }
}

kw_enum! {
    /// A mutability attribute.
    pub enum Mutability {
        /// `pure`
        Pure(kw::pure),
        /// `view`
        View(kw::view),
        /// `constant`
        Constant(kw::constant),
        /// `payable`
        Payable(kw::payable),
    }
}

/// The `override` attribute.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Override {
    pub override_token: kw::Override,
    pub paren_token: Option<Paren>,
    pub paths: Punctuated<SolPath, Token![,]>,
}

impl fmt::Debug for Override {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Override")
            .field(DebugPunctuated::new(&self.paths))
            .finish()
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
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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
        self.paren_token
            .and_then(|paren_token| span.join(paren_token.span.join()))
            .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.override_token.span = span;
        if let Some(paren_token) = &mut self.paren_token {
            *paren_token = Paren(span);
        }
    }
}

/// A modifier invocation, or an inheritance specifier.
#[derive(Clone)]
pub struct Modifier {
    pub name: SolPath,
    pub paren_token: Paren,
    // TODO: Expr
    pub arguments: Punctuated<TokenStream, Token![,]>,
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
            .field("arguments", DebugPunctuated::new(&self.arguments))
            .finish()
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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

    pub fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
        self.paren_token = Paren(span);
    }
}
