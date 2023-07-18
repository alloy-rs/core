use super::Item;
use crate::{kw, utils::DebugPunctuated, Modifier, SolIdent};
use proc_macro2::Span;
use std::{cmp::Ordering, fmt};
use syn::{
    braced,
    parse::{Lookahead1, Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Attribute, Error, Result, Token,
};

/// A contract, abstract contract, interface, or library definition:
/// `contract Foo is Bar("foo"), Baz { ... }`
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.contractDefinition>
#[derive(Clone)]
pub struct ItemContract {
    pub attrs: Vec<Attribute>,
    pub kind: ContractKind,
    pub name: SolIdent,
    pub inheritance: Option<Inheritance>,
    pub brace_token: Brace,
    pub body: Vec<Item>,
}

impl fmt::Debug for ItemContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Contract")
            .field("attrs", &self.attrs)
            .field("kind", &self.kind)
            .field("name", &self.name)
            .field("inheritance", &self.inheritance)
            .field("body", &self.body)
            .finish()
    }
}

impl Parse for ItemContract {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let kind;
        let content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            kind: {
                kind = input.parse()?;
                kind
            },
            name: input.parse()?,
            inheritance: {
                if input.peek(kw::is) {
                    if kind.is_library() {
                        return Err(input.error("libraries are not allowed to inherit"))
                    }
                    Some(input.parse()?)
                } else {
                    None
                }
            },
            brace_token: braced!(content in input),
            body: {
                let mut body = Vec::new();
                while !content.is_empty() {
                    let item: Item = content.parse()?;
                    if matches!(item, Item::Contract(_)) {
                        return Err(Error::new(item.span(), "cannot declare nested contracts"))
                    }
                    body.push(item);
                }
                body
            },
        })
    }
}

impl ItemContract {
    pub fn span(&self) -> Span {
        self.name.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.name.set_span(span);
    }

    /// Returns true if `self` is an abstract contract.
    pub fn is_abstract_contract(&self) -> bool {
        self.kind.is_abstract_contract()
    }

    /// Returns true if `self` is a contract.
    pub fn is_contract(&self) -> bool {
        self.kind.is_contract()
    }

    /// Returns true if `self` is an interface.
    pub fn is_interface(&self) -> bool {
        self.kind.is_interface()
    }

    /// Returns true if `self` is a library.
    pub fn is_library(&self) -> bool {
        self.kind.is_library()
    }
}

/// The kind of contract.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractKind {
    AbstractContract(kw::Abstract, kw::contract),
    Contract(kw::contract),
    Interface(kw::interface),
    Library(kw::library),
}

impl fmt::Debug for ContractKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_debug_str())
    }
}

impl fmt::Display for ContractKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialOrd for ContractKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ContractKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx().cmp(&other.idx())
    }
}

impl Parse for ContractKind {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::Abstract) {
            Ok(Self::AbstractContract(input.parse()?, input.parse()?))
        } else if lookahead.peek(kw::contract) {
            input.parse().map(Self::Contract)
        } else if lookahead.peek(kw::interface) {
            input.parse().map(Self::Interface)
        } else if lookahead.peek(kw::library) {
            input.parse().map(Self::Library)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ContractKind {
    pub fn peek(lookahead: &Lookahead1<'_>) -> bool {
        lookahead.peek(kw::Abstract)
            || lookahead.peek(kw::contract)
            || lookahead.peek(kw::interface)
            || lookahead.peek(kw::library)
    }

    pub fn span(self) -> Span {
        match self {
            Self::AbstractContract(kw_abstract, kw_contract) => {
                let span = kw_abstract.span;
                span.join(kw_contract.span).unwrap_or(span)
            }
            Self::Contract(kw) => kw.span,
            Self::Interface(kw) => kw.span,
            Self::Library(kw) => kw.span,
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::AbstractContract(kw_abstract, kw_contract) => {
                kw_abstract.span = span;
                kw_contract.span = span;
            }
            Self::Contract(kw) => kw.span = span,
            Self::Interface(kw) => kw.span = span,
            Self::Library(kw) => kw.span = span,
        }
    }

    /// Returns true if `self` is an abstract contract.
    pub fn is_abstract_contract(self) -> bool {
        matches!(self, Self::AbstractContract(..))
    }

    /// Returns true if `self` is a contract.
    pub fn is_contract(self) -> bool {
        matches!(self, Self::Contract(_))
    }

    /// Returns true if `self` is an interface.
    pub fn is_interface(self) -> bool {
        matches!(self, Self::Interface(_))
    }

    /// Returns true if `self` is a library.
    pub fn is_library(self) -> bool {
        matches!(self, Self::Library(_))
    }

    pub const fn as_debug_str(self) -> &'static str {
        match self {
            Self::AbstractContract(..) => "AbstractContract",
            Self::Contract(_) => "Contract",
            Self::Interface(_) => "Interface",
            Self::Library(_) => "Library",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AbstractContract(..) => "abstract contract",
            Self::Contract(_) => "contract",
            Self::Interface(_) => "interface",
            Self::Library(_) => "library",
        }
    }

    fn idx(&self) -> usize {
        match self {
            Self::AbstractContract(..) => 0,
            Self::Contract(_) => 1,
            Self::Interface(_) => 2,
            Self::Library(_) => 3,
        }
    }
}

/// A list of inheritance specifiers of an [`ItemContract`]:
/// `is ERC20("Token", "TKN"), Ownable`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.inheritanceSpecifier>
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Inheritance {
    pub is_token: kw::is,
    pub inheritance: Punctuated<Modifier, Token![,]>,
}

impl fmt::Debug for Inheritance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Inheritance")
            .field(DebugPunctuated::new(&self.inheritance))
            .finish()
    }
}

impl Parse for Inheritance {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let is_token = input.parse()?;
        let mut inheritance = Punctuated::new();
        loop {
            if input.is_empty() || input.peek(Brace) {
                break
            }
            inheritance.push_value(input.parse()?);
            if input.is_empty() || input.peek(Brace) {
                break
            }
            inheritance.push_punct(input.parse()?);
        }
        if inheritance.is_empty() {
            Err(input.parse::<SolIdent>().unwrap_err())
        } else {
            Ok(Self {
                is_token,
                inheritance,
            })
        }
    }
}

impl Inheritance {
    pub fn span(&self) -> Span {
        let span = self.is_token.span;
        self.inheritance
            .last()
            .and_then(|last| span.join(last.span()))
            .unwrap_or(span)
    }

    pub fn set_span(&mut self, span: Span) {
        self.is_token.span = span;
    }
}
