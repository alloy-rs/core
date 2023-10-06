use crate::{kw, FunctionAttributes, ParameterList, Returns, Spanned};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Result,
};

/// A function type: `function() returns (string memory)`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.functionTypeName>
#[derive(Clone)]
pub struct TypeFunction {
    pub function_token: kw::function,
    pub paren_token: Paren,
    pub arguments: ParameterList,
    /// The Solidity attributes of the function.
    pub attributes: FunctionAttributes,
    /// The optional return types of the function.
    pub returns: Option<Returns>,
}

impl PartialEq for TypeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.arguments == other.arguments && self.returns == other.returns
    }
}

impl Eq for TypeFunction {}

impl Hash for TypeFunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.arguments.hash(state);
        self.returns.hash(state);
    }
}

impl fmt::Display for TypeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("function (")?;
        self.arguments.fmt(f)?;
        f.write_str(")")?;

        for attr in &self.attributes.0 {
            write!(f, " {attr}")?;
        }

        if let Some(returns) = &self.returns {
            write!(f, " {returns}")?;
        }

        Ok(())
    }
}

impl fmt::Debug for TypeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypeFunction")
            .field("arguments", &self.arguments)
            .field("attributes", &self.attributes)
            .field("returns", &self.returns)
            .finish()
    }
}

impl Parse for TypeFunction {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            function_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            arguments: content.parse()?,
            attributes: input.parse()?,
            returns: input.call(Returns::parse_opt)?,
        })
    }
}

impl Spanned for TypeFunction {
    fn span(&self) -> Span {
        self.function_token.span
    }

    fn set_span(&mut self, span: Span) {
        self.function_token.span = span;
    }
}
