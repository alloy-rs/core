use crate::{kw, variable::VariableDefinition, SolIdent};
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Result, Token,
};

mod contract;
pub use contract::ItemContract;

mod r#enum;
pub use r#enum::ItemEnum;

mod error;
pub use error::ItemError;

mod event;
pub use event::{EventParameter, ItemEvent};

mod function;
pub use function::{FunctionKind, ItemFunction, Returns};

mod import;
pub use import::{
    ImportAlias, ImportAliases, ImportDirective, ImportGlob, ImportPath, ImportPlain,
};

mod pragma;
pub use pragma::{PragmaDirective, PragmaTokens};

mod r#struct;
pub use r#struct::ItemStruct;

mod udt;
pub use udt::ItemUdt;

mod using;
pub use using::{UserDefinableOperator, UsingDirective, UsingList, UsingListItem, UsingType};

/// An AST item. A more expanded version of a [Solidity source unit][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.sourceUnit
#[derive(Clone, Debug)]
pub enum Item {
    /// A contract, abstract contract, interface, or library definition:
    /// `contract Foo is Bar, Baz { ... }`
    Contract(ItemContract),

    /// An enum definition: `enum Foo { A, B, C }`
    Enum(ItemEnum),

    /// An error definition: `error Foo(uint256 a, uint256 b);`
    Error(ItemError),

    /// An event definition: `event Transfer(address indexed from, address
    /// indexed to, uint256 value);`
    Event(ItemEvent),

    /// A function, constructor, fallback, receive, or modifier definition:
    /// `function helloWorld() external pure returns(string memory);`
    Function(ItemFunction),

    /// An import directive: `import "foo.sol";`
    Import(ImportDirective),

    /// A pragma directive: `pragma solidity ^0.8.0;`
    Pragma(PragmaDirective),

    /// A struct definition: `struct Foo { uint256 bar; }`
    Struct(ItemStruct),

    /// A user-defined value type definition: `type Foo is uint256;`
    Udt(ItemUdt),

    /// A `using` directive: `using { A, B.mul as * } for uint256 global;`
    Using(UsingDirective),

    /// A state variable or constant definition: `uint256 constant FOO = 42;`
    Variable(VariableDefinition),
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;

        let lookahead = input.lookahead1();
        let mut item = if FunctionKind::peek(&lookahead) {
            input.parse().map(Self::Function)
        } else if lookahead.peek(Token![struct]) {
            input.parse().map(Self::Struct)
        } else if lookahead.peek(kw::event) {
            input.parse().map(Self::Event)
        } else if lookahead.peek(kw::error) {
            input.parse().map(Self::Error)
        } else if contract::ContractKind::peek(&lookahead) {
            input.parse().map(Self::Contract)
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Self::Enum)
        } else if lookahead.peek(Token![type]) {
            input.parse().map(Self::Udt)
        } else if lookahead.peek(kw::pragma) {
            input.parse().map(Self::Pragma)
        } else if lookahead.peek(kw::import) {
            input.parse().map(Self::Import)
        } else if lookahead.peek(kw::using) {
            input.parse().map(Self::Using)
        } else if crate::Type::peek(&lookahead) {
            input.parse().map(Self::Variable)
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
            Self::Contract(contract) => contract.span(),
            Self::Enum(enumm) => enumm.span(),
            Self::Error(error) => error.span(),
            Self::Event(event) => event.span(),
            Self::Function(function) => function.span(),
            Self::Import(import) => import.span(),
            Self::Pragma(pragma) => pragma.span(),
            Self::Struct(strukt) => strukt.span(),
            Self::Udt(udt) => udt.span(),
            Self::Using(using) => using.span(),
            Self::Variable(variable) => variable.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Contract(contract) => contract.set_span(span),
            Self::Enum(enumm) => enumm.set_span(span),
            Self::Error(error) => error.set_span(span),
            Self::Event(event) => event.set_span(span),
            Self::Function(function) => function.set_span(span),
            Self::Import(import) => import.set_span(span),
            Self::Pragma(pragma) => pragma.set_span(span),
            Self::Struct(strukt) => strukt.set_span(span),
            Self::Udt(udt) => udt.set_span(span),
            Self::Using(using) => using.set_span(span),
            Self::Variable(variable) => variable.set_span(span),
        }
    }

    pub fn name(&self) -> Option<&SolIdent> {
        match self {
            Self::Contract(ItemContract { name, .. })
            | Self::Enum(ItemEnum { name, .. })
            | Self::Error(ItemError { name, .. })
            | Self::Event(ItemEvent { name, .. })
            | Self::Function(ItemFunction {
                name: Some(name), ..
            })
            | Self::Struct(ItemStruct { name, .. })
            | Self::Udt(ItemUdt { name, .. }) => Some(name),
            _ => None,
        }
    }

    fn replace_attrs(&mut self, src: Vec<Attribute>) -> Vec<Attribute> {
        match self {
            Self::Contract(ItemContract { attrs, .. })
            | Self::Function(ItemFunction { attrs, .. })
            | Self::Enum(ItemEnum { attrs, .. })
            | Self::Error(ItemError { attrs, .. })
            | Self::Event(ItemEvent { attrs, .. })
            | Self::Struct(ItemStruct { attrs, .. })
            | Self::Udt(ItemUdt { attrs, .. }) => std::mem::replace(attrs, src),
            _ => vec![],
        }
    }
}
