use crate::{Error, Result, TypeSpecifier};

use alloc::vec::Vec;

/// A tuple specifier, with no array suffixes. Corresponds to a sequence of
/// types.
///
/// The internal types are all [`TypeSpecifier`], and may be arbitrarily
/// complex.
///
/// ## Example
/// ```
/// # use alloy_sol_type_parser::TupleSpecifier;
/// # fn main() -> alloy_sol_type_parser::Result<()> {
/// let spec = TupleSpecifier::try_from("(uint256,uint256)")?;
/// assert_eq!(spec.span(), "(uint256,uint256)");
/// assert_eq!(spec.types.len(), 2);
/// assert_eq!(spec.types[0].span(), "uint256");
///
/// // No array suffixes. Use `TypeSpecifier` instead.
/// assert!(
///    TupleSpecifier::try_from("(uint256,uint256)[]").is_err()
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleSpecifier<'a> {
    /// The full span of the tuple specifier.
    pub span: &'a str,
    /// The internal types.
    pub types: Vec<TypeSpecifier<'a>>,
}

impl<'a> TryFrom<&'a str> for TupleSpecifier<'a> {
    type Error = Error;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self> {
        Self::parse(value)
    }
}

impl AsRef<str> for TupleSpecifier<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.span()
    }
}

impl<'a> TupleSpecifier<'a> {
    /// Parse a tuple specifier from a string.
    pub fn parse(span: &'a str) -> Result<Self> {
        // flexible for or `(a, b)`, or `tuple(a, b)`
        // or any missing parenthesis
        let span = span.trim();

        // if we strip a trailing paren we MUST strip a leading paren
        let value = if let Some(val) = span.strip_suffix(')') {
            val.strip_prefix("tuple")
                .unwrap_or(val)
                .strip_prefix('(')
                .ok_or_else(|| Error::invalid_type_string(span))?
        } else {
            return Err(Error::invalid_type_string(span))
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
                        .ok_or_else(|| Error::invalid_type_string(value))?;
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
            return Err(Error::invalid_type_string(value))
        }

        // handle trailing commas in tuples
        let candidate = value[start..].trim();
        if !candidate.is_empty() {
            types.push(candidate.try_into()?);
        }
        Ok(Self { span, types })
    }

    /// Returns the tuple specifier as a string.
    #[inline]
    pub const fn span(&self) -> &'a str {
        self.span
    }

    /// Returns true if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(&self) -> Result<()> {
        self.types
            .iter()
            .try_for_each(TypeSpecifier::try_basic_solidity)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extra_close_parens() {
        let test_str = "bool,uint256))";
        assert_eq!(
            TupleSpecifier::try_from(test_str),
            Err(crate::Error::invalid_type_string(test_str))
        );
    }

    #[test]
    fn extra_open_parents() {
        let test_str = "(bool,uint256";
        assert_eq!(
            TupleSpecifier::try_from(test_str),
            Err(Error::invalid_type_string(test_str))
        );
    }

    #[test]
    fn nested_tuples() {
        assert_eq!(
            TupleSpecifier::try_from("(bool,(uint256,uint256))").unwrap(),
            TupleSpecifier {
                span: "(bool,(uint256,uint256))",
                types: vec![
                    TypeSpecifier::try_from("bool").unwrap(),
                    TypeSpecifier::try_from("(uint256,uint256)").unwrap(),
                ]
            }
        );
        assert_eq!(
            TupleSpecifier::try_from("(((bool),),)").unwrap(),
            TupleSpecifier {
                span: "(((bool),),)",
                types: vec![TypeSpecifier::try_from("((bool),)").unwrap()]
            }
        );
    }

    #[test]
    fn does_not_parse_missing_parens() {
        let test_str = "bool,uint256";
        assert_eq!(
            TupleSpecifier::try_from(test_str),
            Err(crate::Error::invalid_type_string(test_str))
        );
    }
}
