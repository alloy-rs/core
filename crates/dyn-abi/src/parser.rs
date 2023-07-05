//! Contains utilities for parsing Solidity types.
//!
//! This is a simple representation of Solidity type grammar.

use crate::{DynAbiError, DynAbiResult, DynSolType};
use alloc::{boxed::Box, vec::Vec};
use core::{fmt, num::NonZeroUsize};

/// Returns `true` if the given character is valid at the start of a Solidity
/// identfier.
#[inline]
pub const fn is_id_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '$')
}

/// Returns `true` if the given character is valid in a Solidity identfier.
#[inline]
pub const fn is_id_continue(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$')
}

/// An identifier in Solidity has to start with a letter, a dollar-sign or
/// an underscore and may additionally contain numbers after the first
/// symbol.
///
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.Identifier>
#[inline]
pub fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        is_id_start(first) && chars.all(is_id_continue)
    } else {
        false
    }
}

/// A root type, with no array suffixes. Corresponds to a single, non-sequence
/// type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RootType<'a>(&'a str);

impl<'a> TryFrom<&'a str> for RootType<'a> {
    type Error = DynAbiError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl AsRef<str> for RootType<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for RootType<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> RootType<'a> {
    /// Parse a root type from a string.
    #[inline]
    pub fn parse(value: &'a str) -> DynAbiResult<Self> {
        if is_valid_identifier(value) {
            Ok(Self(value))
        } else {
            Err(DynAbiError::invalid_type_string(value))
        }
    }

    /// The string underlying this type. The type name.
    #[inline]
    pub const fn as_str(self) -> &'a str {
        self.0
    }

    /// Returns true if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(self) -> DynAbiResult<()> {
        self.resolve_basic_solidity().map(drop)
    }

    /// Resolve the type string into a basic Solidity type.
    pub fn resolve_basic_solidity(self) -> DynAbiResult<DynSolType> {
        let type_name = self.0;
        match type_name {
            "address" => Ok(DynSolType::Address),
            "bool" => Ok(DynSolType::Bool),
            "string" => Ok(DynSolType::String),
            "bytes" => Ok(DynSolType::Bytes),
            "uint" => Ok(DynSolType::Uint(256)),
            "int" => Ok(DynSolType::Int(256)),
            _ => {
                if let Some(sz) = type_name.strip_prefix("bytes") {
                    if let Ok(sz) = sz.parse::<usize>() {
                        return if sz != 0 && sz <= 32 {
                            Ok(DynSolType::FixedBytes(sz))
                        } else {
                            Err(DynAbiError::invalid_size(type_name))
                        }
                    }
                }

                // fast path both integer types
                let (s, is_uint) = if let Some(s) = type_name.strip_prefix('u') {
                    (s, true)
                } else {
                    (type_name, false)
                };
                if let Some(sz) = s.strip_prefix("int") {
                    if let Ok(sz) = sz.parse::<usize>() {
                        return if sz != 0 && sz <= 256 && sz % 8 == 0 {
                            if is_uint {
                                Ok(DynSolType::Uint(sz))
                            } else {
                                Ok(DynSolType::Int(sz))
                            }
                        } else {
                            Err(DynAbiError::invalid_size(type_name))
                        }
                    }
                }

                Err(DynAbiError::invalid_type_string(type_name))
            }
        }
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

impl<'a> TryFrom<&'a str> for TupleSpecifier<'a> {
    type Error = DynAbiError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl AsRef<str> for TupleSpecifier<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TupleSpecifier<'a> {
    /// Parse a tuple specifier from a string.
    pub fn parse(value: &'a str) -> DynAbiResult<Self> {
        // flexible for `a, b` or `(a, b)`, or `tuple(a, b)`
        // or any missing parenthesis
        let value = value.trim();

        // if we strip a trailing paren we MUST strip a leading paren
        let value = if let Some(val) = value.strip_suffix(')') {
            val.strip_prefix("tuple")
                .unwrap_or(val)
                .strip_prefix('(')
                .ok_or_else(|| DynAbiError::invalid_type_string(value))?
        } else {
            value
        };

        // passes over nested tuples
        let mut types: Vec<TypeSpecifier<'_>> = vec![];
        let mut start = 0;
        let mut depth: usize = 0;
        for (i, c) in value.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    // handle extra closing paren
                    depth = depth
                        .checked_sub(1)
                        .ok_or_else(|| DynAbiError::invalid_type_string(value))?;
                }
                ',' if depth == 0 => {
                    // SAFETY: `char_indices` always returns a valid char boundary
                    let v = unsafe { value.get_unchecked(start..i) };
                    types.push(v.try_into()?);
                    start = i + 1;
                }
                _ => {}
            }
        }

        // handle extra open paren
        if depth != 0 {
            return Err(DynAbiError::invalid_type_string(value))
        }

        // handle trailing commas in tuples
        let candidate = value[start..].trim();
        if !candidate.is_empty() {
            types.push(candidate.try_into()?);
        }
        Ok(Self { span: value, types })
    }

    /// Returns the tuple specifier as a string.
    #[inline]
    pub const fn as_str(&self) -> &'a str {
        self.span
    }

    /// Returns true if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(&self) -> DynAbiResult<()> {
        self.types
            .iter()
            .try_for_each(TypeSpecifier::try_basic_solidity)
    }

    /// Resolve the type string into a basic Solidity type if possible.
    #[inline]
    pub fn resolve_basic_solidity(&self) -> DynAbiResult<DynSolType> {
        self.types
            .iter()
            .map(TypeSpecifier::resolve_basic_solidity)
            .collect::<Result<Vec<_>, _>>()
            .map(DynSolType::Tuple)
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

impl<'a> TryFrom<&'a str> for TypeStem<'a> {
    type Error = DynAbiError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl AsRef<str> for TypeStem<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TypeStem<'a> {
    /// Parse a type stem from a string.
    pub fn parse(s: &'a str) -> DynAbiResult<Self> {
        if s.starts_with('(') || s.starts_with("tuple") {
            s.try_into().map(Self::Tuple)
        } else {
            s.try_into().map(Self::Root)
        }
    }

    /// Returns the type stem as a string.
    #[inline]
    pub const fn as_str(&self) -> &'a str {
        match self {
            Self::Root(root) => root.as_str(),
            Self::Tuple(tuple) => tuple.as_str(),
        }
    }

    /// Returns true if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(&self) -> Result<(), DynAbiError> {
        match self {
            Self::Root(root) => root.try_basic_solidity(),
            Self::Tuple(tuple) => tuple.try_basic_solidity(),
        }
    }

    /// Resolve the type string into a basic Solidity type if possible.
    #[inline]
    pub fn resolve_basic_solidity(&self) -> Result<DynSolType, DynAbiError> {
        match self {
            Self::Root(root) => root.resolve_basic_solidity(),
            Self::Tuple(tuple) => tuple.resolve_basic_solidity(),
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
    pub sizes: Vec<Option<NonZeroUsize>>,
}

impl<'a> TryFrom<&'a str> for TypeSpecifier<'a> {
    type Error = DynAbiError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

impl AsRef<str> for TypeSpecifier<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TypeSpecifier<'a> {
    /// Parse a type specifier from a string.
    pub fn parse(value: &'a str) -> Result<Self, DynAbiError> {
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

    /// Returns the type stem as a string.
    #[inline]
    pub const fn as_str(&self) -> &'a str {
        self.span
    }

    /// Returns true if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(&self) -> Result<(), DynAbiError> {
        self.root.try_basic_solidity()
    }

    /// Resolve the type string into a basic Solidity type if possible.
    pub fn resolve_basic_solidity(&self) -> Result<DynSolType, DynAbiError> {
        let ty = self.root.resolve_basic_solidity()?;
        Ok(self.wrap_type(ty))
    }

    #[inline]
    pub(crate) fn wrap_type(&self, mut ty: DynSolType) -> DynSolType {
        for size in &self.sizes {
            ty = match size {
                Some(size) => DynSolType::FixedArray(Box::new(ty), size.get()),
                None => DynSolType::Array(Box::new(ty)),
            };
        }
        ty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Result<DynSolType, DynAbiError> {
        s.parse()
    }

    #[test]
    fn extra_close_parens() {
        let test_str = "bool,uint256))";
        assert_eq!(
            parse(test_str),
            Err(DynAbiError::invalid_type_string(test_str))
        );
    }

    #[test]
    fn extra_open_parents() {
        let test_str = "(bool,uint256";
        assert_eq!(
            parse(test_str),
            Err(DynAbiError::invalid_type_string(test_str))
        );
    }

    #[test]
    fn it_parses_tuples() {
        assert_eq!(
            parse("(bool,)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Bool])
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
    }

    #[test]
    fn nested_tuples() {
        assert_eq!(
            parse("(bool,(uint256,uint256))").unwrap(),
            DynSolType::Tuple(vec![
                DynSolType::Bool,
                DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])
            ])
        );
        assert_eq!(
            parse("(((bool),),)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Bool
            ])])])
        );
    }

    #[test]
    fn empty_tuples() {
        assert_eq!(parse("()").unwrap(), DynSolType::Tuple(vec![]));
        assert_eq!(
            parse("((),())").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Tuple(vec![]), DynSolType::Tuple(vec![])])
        );
        assert_eq!(
            parse("((()))"),
            Ok(DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Tuple(vec![])
            ])]))
        );
    }

    #[test]
    fn it_parses_simple_types() {
        assert_eq!(parse("uint256").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("uint8").unwrap(), DynSolType::Uint(8));
        assert_eq!(parse("uint").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("address").unwrap(), DynSolType::Address);
        assert_eq!(parse("bool").unwrap(), DynSolType::Bool);
        assert_eq!(parse("string").unwrap(), DynSolType::String);
        assert_eq!(parse("bytes").unwrap(), DynSolType::Bytes);
        assert_eq!(parse("bytes32").unwrap(), DynSolType::FixedBytes(32));
    }

    #[test]
    fn it_parses_complex_solidity_types() {
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
