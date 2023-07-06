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
/// # use alloy_sol_type_str::TupleSpecifier;
/// # fn main() -> alloy_sol_type_str::Result<()> {
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

impl AsRef<str> for TupleSpecifier<'_> {
    fn as_ref(&self) -> &str {
        self.span
    }
}

impl TupleSpecifier<'_> {
    /// The full span of the tuple specifier.
    pub fn span(&self) -> &str {
        self.span
    }

    /// True if the type is a basic Solidity type.
    pub fn try_basic_solidity(&self) -> Result<()> {
        for ty in &self.types {
            ty.try_basic_solidity()?;
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for TupleSpecifier<'a> {
    type Error = Error;

    fn try_from(span: &'a str) -> Result<Self> {
        // flexible for `(a, b)`, or `tuple(a, b)`
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
                    types.push(value[start..i].try_into()?);
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extra_close_parens() {
        let test_str = "bool,uint256))";
        assert_eq!(
            TupleSpecifier::try_from(test_str),
            Err(crate::Error::invalid_type_string(test_str).into())
        );
    }

    #[test]
    fn extra_open_parents() {
        let test_str = "(bool,uint256";
        assert_eq!(
            TupleSpecifier::try_from(test_str),
            Err(Error::invalid_type_string(test_str).into())
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
            Err(crate::Error::invalid_type_string(test_str).into())
        );
    }
}
