use crate::{
    kw, Block, FunctionAttributes, ParameterList, Parameters, SolIdent, Type, VariableDefinition,
};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
    Attribute, Error, Result, Token,
};

/// A function, constructor, fallback, receive, or modifier definition:
/// `function helloWorld() external pure returns(string memory);`
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.functionDefinition>
#[derive(Clone)]
pub struct ItemFunction {
    /// The `syn` attributes of the function.
    pub attrs: Vec<Attribute>,
    pub kind: FunctionKind,
    pub name: Option<SolIdent>,
    pub paren_token: Paren,
    pub arguments: ParameterList,
    /// The Solidity attributes of the function.
    pub attributes: FunctionAttributes,
    /// The optional return types of the function.
    pub returns: Option<Returns>,
    pub body: FunctionBody,
}

impl fmt::Debug for ItemFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("attrs", &self.attrs)
            .field("kind", &self.kind)
            .field("name", &self.name)
            .field("arguments", &self.arguments)
            .field("attributes", &self.attributes)
            .field("returns", &self.returns)
            .finish()
    }
}

impl Parse for ItemFunction {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            kind: input.parse()?,
            name: input.call(SolIdent::parse_opt)?,
            paren_token: parenthesized!(content in input),
            arguments: content.parse()?,
            attributes: input.parse()?,
            returns: input.call(Returns::parse_opt)?,
            body: input.parse()?,
        })
    }
}

impl ItemFunction {
    pub fn new(kind: FunctionKind, name: Option<SolIdent>) -> Self {
        let span = name
            .as_ref()
            .map_or_else(|| kind.span(), |name| name.span());
        Self {
            attrs: Vec::new(),
            kind,
            name,
            paren_token: Paren(span),
            arguments: Parameters::new(),
            attributes: FunctionAttributes::new(),
            returns: None,
            body: FunctionBody::Empty(Token![;](span)),
        }
    }

    /// Creates a new function from a variable definition. The function will
    /// have the same name and the variable type's will be the return type.
    pub fn from_variable_definition(var: &VariableDefinition) -> Self {
        let name = var.name.clone();
        let span = name.span();
        let kind = FunctionKind::new_function(span);

        let mut function = ItemFunction::new(kind, Some(name));

        let mut returns = ParameterList::new();
        returns.push(var.as_declaration());
        let returns = Returns::new(span, returns);
        function.returns = Some(returns);

        function
            .attributes
            .0
            .extend(var.attributes.0.iter().cloned().map(Into::into));

        function
    }

    pub fn span(&self) -> Span {
        if let Some(name) = &self.name {
            name.span()
        } else {
            self.kind.span()
        }
    }

    pub fn set_span(&mut self, span: Span) {
        self.kind.set_span(span);
        if let Some(name) = &mut self.name {
            name.set_span(span);
        }
    }

    /// Returns the name of the function.
    ///
    /// # Panics
    ///
    /// Panics if the function has no name. This is the case when `kind` is not
    /// `Function`.
    pub fn name(&self) -> &SolIdent {
        match &self.name {
            Some(name) => name,
            None => panic!("function has no name: {self:?}"),
        }
    }

    /// Returns true if the function returns nothing.
    pub fn is_void(&self) -> bool {
        match &self.returns {
            None => true,
            Some(returns) => returns.returns.is_empty(),
        }
    }

    /// Returns true if the function has a body.
    pub fn has_implementation(&self) -> bool {
        matches!(self.body, FunctionBody::Block(_))
    }

    /// Returns the function's arguments tuple type.
    pub fn call_type(&self) -> Type {
        Type::Tuple(self.arguments.iter().map(|arg| arg.ty.clone()).collect())
    }

    /// Returns the function's return tuple type.
    pub fn return_type(&self) -> Option<Type> {
        self.returns.as_ref().map(|returns| {
            Type::Tuple(
                returns
                    .returns
                    .iter()
                    .map(|returns| returns.ty.clone())
                    .collect(),
            )
        })
    }
}

kw_enum! {
    /// The kind of function.
    pub enum FunctionKind {
        /// `constructor`
        Constructor(kw::constructor),
        /// `function`
        Function(kw::function),
        /// `fallback`
        Fallback(kw::fallback),
        /// `receive`
        Receive(kw::receive),
        /// `modifier`
        Modifier(kw::modifier),
    }
}

/// The `returns` attribute of a function.
#[derive(Clone)]
pub struct Returns {
    pub returns_token: kw::returns,
    pub paren_token: Paren,
    /// The returns of the function. This cannot be parsed empty.
    pub returns: ParameterList,
}

impl PartialEq for Returns {
    fn eq(&self, other: &Self) -> bool {
        self.returns == other.returns
    }
}

impl Eq for Returns {}

impl Hash for Returns {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.returns.hash(state);
    }
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
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let this = Self {
            returns_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            returns: content.parse()?,
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
    pub fn new(span: Span, returns: ParameterList) -> Self {
        Self {
            returns_token: kw::returns(span),
            paren_token: Paren(span),
            returns,
        }
    }

    pub fn span(&self) -> Span {
        let span = self.returns_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.returns_token.span = span;
        self.paren_token = Paren(span);
    }

    pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
        if input.peek(kw::returns) {
            input.parse().map(Some)
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
pub enum FunctionBody {
    /// A function body delimited by curly braces.
    Block(Block),
    /// A function without implementation.
    Empty(Token![;]),
}

impl Parse for FunctionBody {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(Token![;]) {
            input.parse().map(Self::Empty)
        } else {
            Err(lookahead.error())
        }
    }
}
