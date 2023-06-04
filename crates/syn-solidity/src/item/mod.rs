use super::kw;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

mod error;
pub use error::ItemError;

mod function;
pub use function::ItemFunction;

mod r#struct;
pub use r#struct::ItemStruct;

mod udt;
pub use udt::ItemUdt;

/// An AST item. A more expanded version of a [Solidity source unit][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.sourceUnit
#[derive(Debug)]
pub enum Item {
    /// An error definition: `error Foo(uint256 a, uint256 b);`
    Error(ItemError),

    /// A function definition:
    /// `function helloWorld() external pure returns(string memory);`
    Function(ItemFunction),

    /// A struct definition: `struct Foo { uint256 bar; }`
    Struct(ItemStruct),

    /// A user-defined value type definition: `type Foo is uint256;`
    Udt(ItemUdt),
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;

        let lookahead = input.lookahead1();
        let mut item = if lookahead.peek(kw::function) {
            input.parse().map(Self::Function)
        } else if lookahead.peek(Token![struct]) {
            input.parse().map(Self::Struct)
        } else if lookahead.peek(kw::error) {
            input.parse().map(Self::Error)
        } else if lookahead.peek(Token![type]) {
            input.parse().map(Self::Udt)
        } else {
            Err(lookahead.error())
        }?;

        attrs.extend(item.replace_attrs(Vec::new()));
        item.replace_attrs(attrs);

        Ok(item)
    }
}

impl Item {
    pub fn span(&self) -> Span {
        match self {
            Self::Error(error) => error.span(),
            Self::Function(function) => function.span(),
            Self::Struct(strukt) => strukt.span(),
            Self::Udt(udt) => udt.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Error(error) => error.set_span(span),
            Self::Function(function) => function.set_span(span),
            Self::Struct(strukt) => strukt.set_span(span),
            Self::Udt(udt) => udt.set_span(span),
        }
    }

    fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute> {
        match self {
            Self::Struct(ItemStruct { attrs, .. })
            | Self::Function(ItemFunction { attrs, .. })
            | Self::Error(ItemError { attrs, .. })
            | Self::Udt(ItemUdt { attrs, .. }) => std::mem::replace(attrs, new),
        }
    }
}
