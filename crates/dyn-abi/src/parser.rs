//! Contains utilities for parsing Solidity types.
//!
//! This is a simple representation of Solidity type grammar.

use crate::{no_std_prelude::*, DynAbiError, DynSolType};
use core::fmt;

/// A root type, with no array suffixes. Corresponds to a single, non-sequence
/// type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RootType<'a>(&'a str);

impl fmt::Display for RootType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl RootType<'_> {
    /// The string underlying this type. The type name.
    pub const fn as_str(&self) -> &str {
        self.0
    }

    /// An identifier in Solidity has to start with a letter, a dollar-sign or
    /// an underscore and may additionally contain numbers after the first
    /// symbol.
    ///
    /// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.Identifier>
    fn legal_identifier(&self) -> Result<(), DynAbiError> {
        if self.0.is_empty() {
            return Err(DynAbiError::invalid_type_string(self.0))
        }

        match self.0.chars().next().unwrap() {
            'a'..='z' | 'A'..='Z' | '_' | '$' => {}
            _ => return Err(DynAbiError::invalid_type_string(self.0)),
        }

        if self.0.chars().skip(1).all(char::is_alphanumeric) {
            Ok(())
        } else {
            Err(DynAbiError::invalid_type_string(self.0))
        }
    }

    /// True if the type is a basic Solidity type.
    pub fn try_basic_solidity(&self) -> Result<(), DynAbiError> {
        let type_name = self.0;
        match type_name {
            "address" | "bool" | "string" | "bytes" => Ok(()),
            _ => {
                if type_name.starts_with("bytes") {
                    if let Some(len) = type_name.strip_prefix("bytes") {
                        if let Ok(len) = len.parse::<usize>() {
                            if len <= 32 {
                                return Ok(())
                            }
                            return Err(DynAbiError::invalid_size(type_name))
                        }
                    }
                }
                if type_name.starts_with("uint") {
                    if let Some(len) = type_name.strip_prefix("uint") {
                        if len.is_empty() {
                            return Ok(())
                        }
                        if let Ok(len) = len.parse::<usize>() {
                            if len <= 256 && len % 8 == 0 {
                                return Ok(())
                            }
                            return Err(DynAbiError::invalid_size(type_name))
                        }
                    }
                }
                if type_name.starts_with("int") {
                    if let Some(len) = type_name.strip_prefix("int") {
                        if len.is_empty() {
                            return Ok(())
                        }
                        if let Ok(len) = len.parse::<usize>() {
                            if len <= 256 && len % 8 == 0 {
                                return Ok(())
                            }
                            return Err(DynAbiError::invalid_size(type_name))
                        }
                    }
                }
                Err(DynAbiError::invalid_type_string(type_name))
            }
        }
    }

    /// Resolve the type string into a basic Solidity type.
    pub fn resolve_basic_solidity(&self) -> Result<DynSolType, DynAbiError> {
        self.try_basic_solidity()?;

        let s = self.0;
        if let Some(s) = s.strip_prefix("int") {
            let len = s.trim().parse::<usize>().unwrap_or(256);
            return Ok(DynSolType::Int(len))
        }

        if let Some(s) = s.strip_prefix("uint") {
            let len = s.trim().parse::<usize>().unwrap_or(256);
            return Ok(DynSolType::Uint(len))
        }

        match s {
            "address" => return Ok(DynSolType::Address),
            "bool" => return Ok(DynSolType::Bool),
            "string" => return Ok(DynSolType::String),
            "bytes" => return Ok(DynSolType::Bytes),
            _ => {}
        }

        // This block must come after the match statement, as the `bytes`
        // prefix is shared between two types
        if let Some(s) = s.strip_prefix("bytes") {
            return Ok(DynSolType::FixedBytes(
                s.trim().parse().map_err(|_| DynAbiError::invalid_size(s))?,
            ))
        }
        Err(DynAbiError::missing_type(s))
    }
}

impl<'a> TryFrom<&'a str> for RootType<'a> {
    type Error = DynAbiError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let s = Self(value.trim());
        s.legal_identifier()?;
        Ok(s)
    }
}

impl Borrow<str> for RootType<'_> {
    fn borrow(&self) -> &str {
        self.0
    }
}

impl AsRef<str> for RootType<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// A tuple specifier, with no array suffixes. Corresponds to a sequence of
/// types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleSpecifier<'a> {
    /// The full span of the tuple specifier.
    pub span: &'a str,
    /// The internal types.
    pub types: Vec<TypeSpecifier<'a>>,
}

impl TupleSpecifier<'_> {
    /// True if the type is a basic Solidity type.
    pub fn try_basic_solidity(&self) -> Result<(), DynAbiError> {
        for ty in &self.types {
            ty.try_basic_solidity()?;
        }
        Ok(())
    }

    /// Resolve the type string into a basic Solidity type if possible.
    pub fn resolve_basic_solidity(&self) -> Result<DynSolType, DynAbiError> {
        let tuple = self
            .types
            .iter()
            .map(|ty| ty.resolve_basic_solidity())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DynSolType::Tuple(tuple))
    }
}

impl<'a> TryFrom<&'a str> for TupleSpecifier<'a> {
    type Error = DynAbiError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // flexible for `a, b` or `(a, b)`, or `tuple(a, b)`
        // or any missing parenthesis
        let value = value.trim();
        let value = value.strip_suffix(')').unwrap_or(value);
        let value = value.strip_prefix("tuple").unwrap_or(value);
        let value = value.strip_prefix('(').unwrap_or(value);

        let mut types = vec![];
        let mut start = 0;
        let mut depth = 0;
        for (i, c) in value.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 0 => {
                    types.push(value[start..i].try_into()?);
                    start = i + 1;
                }
                _ => {}
            }
        }
        types.push(value[start..].try_into()?);
        Ok(Self { span: value, types })
    }
}

/// This is the stem of a Solidity array type. It is either a root type, or a
/// tuple type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeStem<'a> {
    /// Root type.
    Root(RootType<'a>),
    /// Tuple type.
    Tuple(TupleSpecifier<'a>),
}

impl TypeStem<'_> {
    pub(crate) fn try_basic_solidity(&self) -> Result<(), DynAbiError> {
        match self {
            Self::Root(root) => root.try_basic_solidity(),
            Self::Tuple(tuple) => tuple.try_basic_solidity(),
        }
    }

    pub(crate) fn resolve_basic_solidity(&self) -> Result<DynSolType, DynAbiError> {
        match self {
            Self::Root(root) => root.resolve_basic_solidity(),
            Self::Tuple(tuple) => tuple.resolve_basic_solidity(),
        }
    }
}

impl<'a> TryFrom<&'a str> for TypeStem<'a> {
    type Error = DynAbiError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.starts_with('(') || value.starts_with("tuple") {
            Ok(Self::Tuple(value.try_into()?))
        } else {
            Ok(Self::Root(value.try_into()?))
        }
    }
}

/// Represents a type-name which consists of an identifier and optional array
/// sizes
///
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.typeName>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSpecifier<'a> {
    /// The full span of the specifier.
    pub span: &'a str,
    /// The type stem, which is either a root type or a tuple type.
    pub root: TypeStem<'a>,
    /// Array sizes, in innermost-to-outermost order. If the size is `None`,
    /// then the array is dynamic. If the size is `Some`, then the array is
    /// fixed-size. If the vec is empty, then the type is not an array.
    pub sizes: Vec<Option<usize>>,
}

impl AsRef<str> for TypeSpecifier<'_> {
    fn as_ref(&self) -> &str {
        self.span
    }
}

impl TypeSpecifier<'_> {
    /// True if the type is a basic Solidity type.
    pub fn try_basic_solidity(&self) -> Result<(), DynAbiError> {
        self.root.try_basic_solidity()
    }

    /// Resolve the type string into a basic Solidity type if possible.
    pub fn resolve_basic_solidity(&self) -> Result<DynSolType, DynAbiError> {
        let ty = self.root.resolve_basic_solidity()?;
        Ok(self.sizes.iter().fold(ty, |acc, item| match item {
            Some(size) => DynSolType::FixedArray(Box::new(acc), *size),
            _ => DynSolType::Array(Box::new(acc)),
        }))
    }
}

impl<'a> TryFrom<&'a str> for TypeSpecifier<'a> {
    type Error = DynAbiError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let mut root = value;
        let mut sizes = vec![];

        // an iterator of string slices split by `[`
        for s in root.rsplit('[') {
            // we've reached a root tuple so we need to include the closing
            // paren
            if s.contains(')') {
                let idx = value.rfind(')').unwrap();
                root = &value[..=idx];
                break
            }
            // we've reached a root type that is not a tuple or array
            if !s.contains(']') {
                root = s;
                break
            }

            let s = s
                .trim()
                .strip_suffix(']')
                .ok_or_else(|| DynAbiError::invalid_type_string(value))?;

            if s.is_empty() {
                sizes.push(None);
            } else {
                sizes.push(Some(
                    s.parse()
                        .map_err(|_| DynAbiError::invalid_type_string(value))?,
                ));
            }
        }

        sizes.reverse();
        Ok(Self {
            span: value,
            root: root.try_into()?,
            sizes,
        })
    }
}

pub(crate) fn parse(s: &str) -> Result<DynSolType, DynAbiError> {
    let s = s.trim();
    let ty = TypeSpecifier::try_from(s)?;

    ty.resolve_basic_solidity()
}

impl core::str::FromStr for DynSolType {
    type Err = DynAbiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_parses_solidity_types() {
        assert_eq!(parse("uint256").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("uint8").unwrap(), DynSolType::Uint(8));
        assert_eq!(parse("uint").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("address").unwrap(), DynSolType::Address);
        assert_eq!(parse("bool").unwrap(), DynSolType::Bool);
        assert_eq!(parse("string").unwrap(), DynSolType::String);
        assert_eq!(parse("bytes").unwrap(), DynSolType::Bytes);
        assert_eq!(parse("bytes32").unwrap(), DynSolType::FixedBytes(32));
        assert_eq!(
            parse("uint256[]").unwrap(),
            DynSolType::Array(Box::new(DynSolType::Uint(256)))
        );
        assert_eq!(
            parse("uint256[2]").unwrap(),
            DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2)
        );
        assert_eq!(
            parse("uint256[2][3]").unwrap(),
            DynSolType::FixedArray(
                Box::new(DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2)),
                3
            )
        );
        assert_eq!(
            parse("uint256[][][]").unwrap(),
            DynSolType::Array(Box::new(DynSolType::Array(Box::new(DynSolType::Array(
                Box::new(DynSolType::Uint(256))
            )))))
        );
        assert_eq!(
            parse("(uint256,uint256)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])
        );
        assert_eq!(
            parse("(uint256,uint256)[2]").unwrap(),
            DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![
                    DynSolType::Uint(256),
                    DynSolType::Uint(256)
                ])),
                2
            )
        );
        assert_eq!(
            parse(r#"tuple(address,bytes, (bool, (string, uint256)[][3]))[2]"#),
            Ok(DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![
                    DynSolType::Address,
                    DynSolType::Bytes,
                    DynSolType::Tuple(vec![
                        DynSolType::Bool,
                        DynSolType::FixedArray(
                            Box::new(DynSolType::Array(Box::new(DynSolType::Tuple(vec![
                                DynSolType::String,
                                DynSolType::Uint(256)
                            ])))),
                            3
                        ),
                    ]),
                ])),
                2
            ))
        );
    }

    #[test]
    fn try_basic_solidity() {
        assert_eq!(
            TypeSpecifier::try_from("uint256")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("uint256[]")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("(uint256,uint256)")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("(uint256,uint256)[2]")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("tuple(uint256,uint256)")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from(r#"tuple(address,bytes, (bool, (string, uint256)[][3]))[2]"#)
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
    }

    #[test]
    fn not_basic_solidity() {
        assert_eq!(
            TypeSpecifier::try_from("MyStruct")
                .unwrap()
                .try_basic_solidity(),
            Err(DynAbiError::invalid_type_string("MyStruct"))
        );
    }
}
