use crate::{spanned, str_parser, Error, Result, TypeSpecifier};
use alloc::vec::Vec;
use winnow::{
    combinator::{cut_err, delimited, opt, separated0},
    trace::trace,
    PResult, Parser,
};

/// A tuple specifier, with no array suffixes. Corresponds to a sequence of
/// types.
///
/// The internal types are all [`TypeSpecifier`], and may be arbitrarily
/// complex.
///
/// # Examples
///
/// ```
/// # use alloy_sol_type_parser::TupleSpecifier;
/// let spec = TupleSpecifier::try_from("(uint256,uint256)")?;
/// assert_eq!(spec.span(), "(uint256,uint256)");
/// assert_eq!(spec.types.len(), 2);
/// assert_eq!(spec.types[0].span(), "uint256");
///
/// // No array suffixes. Use `TypeSpecifier` instead.
/// assert!(
///    TupleSpecifier::try_from("(uint256,uint256)[]").is_err()
/// );
/// # Ok::<_, alloy_sol_type_parser::Error>(())
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
    #[inline]
    pub fn parse(input: &'a str) -> Result<Self> {
        Self::parser.parse(input).map_err(Error::parser)
    }

    pub(crate) fn parser(input: &mut &'a str) -> PResult<Self> {
        trace("TupleSpecifier", spanned(Self::parse_types))
            .parse_next(input)
            .map(|(span, types)| Self { span, types })
    }

    #[inline]
    fn parse_types(input: &mut &'a str) -> PResult<Vec<TypeSpecifier<'a>>> {
        if let Some(stripped) = input.strip_prefix("tuple") {
            *input = stripped;
        }
        trace(
            "tuple",
            delimited(
                str_parser("("),
                cut_err(separated0(TypeSpecifier::parser, str_parser(","))),
                (opt(","), cut_err(str_parser(")"))),
            ),
        )
        .parse_next(input)
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
        TupleSpecifier::try_from("bool,uint256))").unwrap_err();
    }

    #[test]
    fn extra_open_parents() {
        TupleSpecifier::try_from("(bool,uint256").unwrap_err();
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
        TupleSpecifier::try_from("bool,uint256").unwrap_err();
    }
}
