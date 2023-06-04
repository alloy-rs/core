use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::fmt;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Error, Result,
};

mod path;
pub use path::SolPath;

const KEYWORDS: &[&str] = &[
    "abstract",
    "address",
    "anonymous",
    "as",
    "assembly",
    "bool",
    "break",
    "byte",
    "bytes",
    "bytes1",
    "bytes2",
    "bytes3",
    "bytes4",
    "bytes5",
    "bytes6",
    "bytes7",
    "bytes8",
    "bytes9",
    "bytes10",
    "bytes11",
    "bytes12",
    "bytes13",
    "bytes14",
    "bytes15",
    "bytes16",
    "bytes17",
    "bytes18",
    "bytes19",
    "bytes20",
    "bytes21",
    "bytes22",
    "bytes23",
    "bytes24",
    "bytes25",
    "bytes26",
    "bytes27",
    "bytes28",
    "bytes29",
    "bytes30",
    "bytes31",
    "bytes32",
    "calldata",
    "catch",
    "constant",
    "constructor",
    "continue",
    "contract",
    "delete",
    "do",
    "else",
    "emit",
    "enum",
    "event",
    "external",
    "fallback",
    "false",
    "for",
    "function",
    "if",
    "immutable",
    "import",
    "indexed",
    "int",
    "int8",
    "int16",
    "int24",
    "int32",
    "int40",
    "int48",
    "int56",
    "int64",
    "int72",
    "int80",
    "int88",
    "int96",
    "int104",
    "int112",
    "int120",
    "int128",
    "int136",
    "int144",
    "int152",
    "int160",
    "int168",
    "int176",
    "int184",
    "int192",
    "int200",
    "int208",
    "int216",
    "int224",
    "int232",
    "int240",
    "int248",
    "int256",
    "interface",
    "internal",
    "is",
    "let",
    "library",
    "mapping",
    "memory",
    "modifier",
    "new",
    "override",
    "payable",
    "pragma",
    "private",
    "public",
    "pure",
    "receive",
    "return",
    "returns",
    "storage",
    "string",
    "struct",
    "this",
    "throw",
    "true",
    "try",
    "type",
    "uint",
    "uint8",
    "uint16",
    "uint24",
    "uint32",
    "uint40",
    "uint48",
    "uint56",
    "uint64",
    "uint72",
    "uint80",
    "uint88",
    "uint96",
    "uint104",
    "uint112",
    "uint120",
    "uint128",
    "uint136",
    "uint144",
    "uint152",
    "uint160",
    "uint168",
    "uint176",
    "uint184",
    "uint192",
    "uint200",
    "uint208",
    "uint216",
    "uint224",
    "uint232",
    "uint240",
    "uint248",
    "uint256",
    "unchecked",
    "using",
    "view",
    "virtual",
    "while",
];

/// Returns true if `c` is valid as a first character of an identifier.
fn is_id_start(c: char) -> bool {
    matches!(c, '$' | 'A'..='Z' | '_' | 'a'..='z')
}

/// Returns true if `c` is valid as a non-first character of an identifier.
fn is_id_continue(c: char) -> bool {
    matches!(c, '$' | '0'..='9' | 'A'..='Z' | '_' | 'a'..='z')
}

/// Returns true if `s` is a valid Solidity identifier.
fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().map_or(false, |start| {
        is_id_start(start) && chars.all(is_id_continue)
    })
}

/// A Solidity identifier.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SolIdent(pub Ident);

impl fmt::Display for SolIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for SolIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sol")?;
        self.0.fmt(f)
    }
}

impl Parse for SolIdent {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ident = input.call(Ident::parse_any)?;
        let s = ident.to_string();
        let s = s.as_str();

        if KEYWORDS.contains(&s) {
            Err(Error::new(
                ident.span(),
                "expected identifier, found reserved keyword",
            ))
        } else if !is_ident(s.strip_prefix("r#").unwrap_or(s)) {
            Err(Error::new(ident.span(), "invalid identifier"))
        } else {
            Ok(Self(ident))
        }
    }
}

impl ToTokens for SolIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl SolIdent {
    pub fn span(&self) -> Span {
        self.0.span()
    }

    pub fn set_span(&mut self, span: Span) {
        self.0.set_span(span);
    }

    /// Returns the identifier as a string, without the `r#` prefix if present.
    pub fn as_string(&self) -> String {
        let mut s = self.0.to_string();
        if s.starts_with("r#") {
            s = s[2..].to_string();
        }
        s
    }
}
