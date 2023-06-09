use crate::{kw, SolIdent, Type};
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

mod contract;
pub use contract::ItemContract;

mod error;
pub use error::ItemError;

mod event;
pub use event::{EventParameter, ItemEvent};

mod function;
pub use function::{ItemFunction, Returns};

mod r#struct;
pub use r#struct::ItemStruct;

mod udt;
pub use udt::ItemUdt;

/// An AST item. A more expanded version of a [Solidity source unit][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.sourceUnit
#[derive(Clone, Debug)]
pub enum Item {
    /// A contract, abstract contract, interface, or library definition:
    /// `contract Foo is Bar, Baz { ... }`
    Contract(ItemContract),

    /// An error definition: `error Foo(uint256 a, uint256 b);`
    Error(ItemError),

    /// An event definition: `event Transfer(address indexed from, address
    /// indexed to, uint256 value);`
    Event(ItemEvent),

    /// A function definition: `function helloWorld() external pure
    /// returns(string memory);`
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
        } else if lookahead.peek(kw::event) {
            input.parse().map(Self::Event)
        } else if lookahead.peek(kw::error) {
            input.parse().map(Self::Error)
        } else if contract::ContractKind::peek(&lookahead) {
            input.parse().map(Self::Contract)
        } else if lookahead.peek(Token![type]) {
            input.parse().map(Self::Udt)
        } else {
            Err(lookahead.error())
        }?;

        attrs.extend(std::mem::take(item.attrs_mut()));
        *item.attrs_mut() = attrs;

        Ok(item)
    }
}

impl Item {
    pub fn span(&self) -> Span {
        match self {
            Self::Contract(contract) => contract.span(),
            Self::Error(error) => error.span(),
            Self::Event(event) => event.span(),
            Self::Function(function) => function.span(),
            Self::Struct(strukt) => strukt.span(),
            Self::Udt(udt) => udt.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Contract(contract) => contract.set_span(span),
            Self::Error(error) => error.set_span(span),
            Self::Event(event) => event.set_span(span),
            Self::Function(function) => function.set_span(span),
            Self::Struct(strukt) => strukt.set_span(span),
            Self::Udt(udt) => udt.set_span(span),
        }
    }

    pub fn is_contract(&self) -> bool {
        matches!(self, Self::Contract(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn is_event(&self) -> bool {
        matches!(self, Self::Event(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_udt(&self) -> bool {
        matches!(self, Self::Udt(_))
    }

    pub fn name(&self) -> &SolIdent {
        match self {
            Self::Contract(ItemContract { name, .. })
            | Self::Error(ItemError { name, .. })
            | Self::Event(ItemEvent { name, .. })
            | Self::Function(ItemFunction { name, .. })
            | Self::Struct(ItemStruct { name, .. })
            | Self::Udt(ItemUdt { name, .. }) => name,
        }
    }

    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Self::Contract(ItemContract { attrs, .. })
            | Self::Function(ItemFunction { attrs, .. })
            | Self::Error(ItemError { attrs, .. })
            | Self::Event(ItemEvent { attrs, .. })
            | Self::Struct(ItemStruct { attrs, .. })
            | Self::Udt(ItemUdt { attrs, .. }) => attrs,
        }
    }

    pub fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        match self {
            Self::Contract(ItemContract { attrs, .. })
            | Self::Function(ItemFunction { attrs, .. })
            | Self::Error(ItemError { attrs, .. })
            | Self::Event(ItemEvent { attrs, .. })
            | Self::Struct(ItemStruct { attrs, .. })
            | Self::Udt(ItemUdt { attrs, .. }) => attrs,
        }
    }

    pub fn as_type(&self) -> Option<Type> {
        match self {
            Self::Struct(strukt) => Some(strukt.as_type()),
            Self::Udt(udt) => Some(udt.ty.clone()),
            Self::Error(error) => Some(error.as_type()),
            Self::Event(event) => Some(event.as_type()),
            Self::Contract(_) | Self::Function(_) => None,
        }
    }
}
