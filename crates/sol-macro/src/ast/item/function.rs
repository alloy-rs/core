use crate::ast::{kw, FunctionAttributes, Parameters, Returns, SolIdent, SolTuple, Type};
use proc_macro2::Span;
use std::fmt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
    Attribute, Result, Token,
};

/// A function definition.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.functionDefinition>
pub struct Function {
    pub attrs: Vec<Attribute>,
    pub function_token: kw::function,
    /// The original name of the function, before any overload renaming.
    pub original_name: SolIdent,
    /// The name of the function, after any overload renaming.
    pub name: SolIdent,
    pub paren_token: Paren,
    pub arguments: Parameters<Token![,]>,
    pub attributes: FunctionAttributes,
    pub returns: Option<Returns>,
    pub semi_token: Token![;],
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
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        fn parse_check_brace<T: Parse>(input: ParseStream<'_>) -> Result<T> {
            if input.peek(Brace) {
                Err(input.error("functions cannot have an implementation"))
            } else {
                input.parse()
            }
        }

        let name: SolIdent;
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            function_token: input.parse()?,
            original_name: {
                name = input.parse()?;
                name.clone()
            },
            name,
            paren_token: parenthesized!(content in input),
            arguments: content.parse()?,
            attributes: parse_check_brace(input)?,
            returns: if input.peek(kw::returns) {
                Some(input.parse()?)
            } else {
                None
            },
            semi_token: parse_check_brace(input)?,
        })
    }
}

impl Function {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn call_type(&self) -> Type {
        let mut args = self
            .arguments
            .iter()
            .map(|arg| arg.ty.clone())
            .collect::<SolTuple>();
        // ensure trailing comma for single item tuple
        if !args.types.trailing_punct() && args.types.len() == 1 {
            args.types.push_punct(Default::default())
        }
        Type::Tuple(args)
    }
}
