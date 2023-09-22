use crate::{spanned, str_parser, Error, Result, TypeStem};
use alloc::vec::Vec;
use core::num::NonZeroUsize;
use winnow::{
    ascii::digit0,
    combinator::{cut_err, delimited, opt, repeat},
    trace::trace,
    PResult, Parser,
};

/// Represents a type-name. Consists of an identifier and optional array sizes.
///
/// A type specifier has a stem, which is [`TypeStem`] representing either a
/// [`RootType`] or a [`TupleSpecifier`], and a list of array sizes. The array
/// sizes are in innermost-to-outermost order. An empty array size vec indicates
/// that the specified type is not an array
///
/// Type specifier examples:
/// - `uint256`
/// - `uint256[2]`
/// - `uint256[2][]`
/// - `(uint256,uint256)`
/// - `(uint256,uint256)[2]`
/// - `MyStruct`
/// - `MyStruct[2]`
///
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.typeName>
///
/// [`RootType`]: crate::RootType
/// [`TupleSpecifier`]: crate::TupleSpecifier
///
/// ## Compatibility with JSON ABI
///
/// This type supports the `internalType` semantics for JSON-ABI compatibility.
///
/// Examples of valid JSON ABI internal types:
/// - `contract MyContract`
/// - `struct MyStruct`
/// - `enum MyEnum`
/// - `struct MyContract.MyStruct\[333\]`
/// - `enum MyContract.MyEnum[][][][][][]`
/// - `MyValueType`
///
/// # Examples
///
/// ```
/// # use alloy_sol_type_parser::TypeSpecifier;
/// # use core::num::NonZeroUsize;
/// let spec = TypeSpecifier::parse("uint256[2][]")?;
/// assert_eq!(spec.span(), "uint256[2][]");
/// assert_eq!(spec.stem.span(), "uint256");
/// // The sizes are in innermost-to-outermost order.
/// assert_eq!(spec.sizes.as_slice(), &[NonZeroUsize::new(2), None]);
/// # Ok::<_, alloy_sol_type_parser::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSpecifier<'a> {
    /// The full span of the specifier.
    pub span: &'a str,
    /// The type stem, which is either a root type or a tuple type.
    pub stem: TypeStem<'a>,
    /// Array sizes, in innermost-to-outermost order. If the size is `None`,
    /// then the array is dynamic. If the size is `Some`, then the array is
    /// fixed-size. If the vec is empty, then the type is not an array.
    pub sizes: Vec<Option<NonZeroUsize>>,
}

impl<'a> TryFrom<&'a str> for TypeSpecifier<'a> {
    type Error = Error;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self> {
        Self::parse(s)
    }
}

impl AsRef<str> for TypeSpecifier<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.span()
    }
}

impl<'a> TypeSpecifier<'a> {
    /// Parse a type specifier from a string.
    #[inline]
    pub fn parse(input: &'a str) -> Result<Self> {
        Self::parser.parse(input).map_err(Error::parser)
    }

    pub(crate) fn parser(input: &mut &'a str) -> PResult<Self> {
        trace(
            "TypeSpecifier",
            spanned(|input: &mut &'a str| {
                let stem = TypeStem::parser(input)?;
                let sizes = if input.starts_with('[') {
                    repeat(
                        1..,
                        delimited(
                            str_parser("["),
                            opt(cut_err(digit0).parse_to()),
                            cut_err(str_parser("]")),
                        ),
                    )
                    .parse_next(input)?
                } else {
                    Vec::new()
                };
                Ok((stem, sizes))
            }),
        )
        .parse_next(input)
        .map(|(span, (stem, sizes))| Self { span, stem, sizes })
    }

    /// Returns the type stem as a string.
    #[inline]
    pub const fn span(&self) -> &'a str {
        self.span
    }

    /// Returns the type stem.
    #[inline]
    pub const fn stem(&self) -> &TypeStem<'_> {
        &self.stem
    }

    /// Returns true if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(&self) -> Result<()> {
        self.stem.try_basic_solidity()
    }

    /// Returns true if this type is an array.
    #[inline]
    pub fn is_array(&self) -> bool {
        !self.sizes.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TupleSpecifier;

    #[test]
    fn parse_test() {
        assert_eq!(
            TypeSpecifier::parse("uint256"),
            Ok(TypeSpecifier {
                span: "uint256",
                stem: TypeStem::parse("uint256").unwrap(),
                sizes: vec![],
            })
        );

        assert_eq!(
            TypeSpecifier::parse("uint256[2]"),
            Ok(TypeSpecifier {
                span: "uint256[2]",
                stem: TypeStem::parse("uint256").unwrap(),
                sizes: vec![NonZeroUsize::new(2)],
            })
        );

        assert_eq!(
            TypeSpecifier::parse("uint256[2][]"),
            Ok(TypeSpecifier {
                span: "uint256[2][]",
                stem: TypeStem::parse("uint256").unwrap(),
                sizes: vec![NonZeroUsize::new(2), None],
            })
        );

        assert_eq!(
            TypeSpecifier::parse("(uint256,uint256)"),
            Ok(TypeSpecifier {
                span: "(uint256,uint256)",
                stem: TypeStem::Tuple(TupleSpecifier::parse("(uint256,uint256)").unwrap()),
                sizes: vec![],
            })
        );

        assert_eq!(
            TypeSpecifier::parse("(uint256,uint256)[2]"),
            Ok(TypeSpecifier {
                span: "(uint256,uint256)[2]",
                stem: TypeStem::Tuple(TupleSpecifier::parse("(uint256,uint256)").unwrap()),
                sizes: vec![NonZeroUsize::new(2)],
            })
        );

        assert_eq!(
            TypeSpecifier::parse("MyStruct"),
            Ok(TypeSpecifier {
                span: "MyStruct",
                stem: TypeStem::parse("MyStruct").unwrap(),
                sizes: vec![],
            })
        );

        assert_eq!(
            TypeSpecifier::parse("MyStruct[2]"),
            Ok(TypeSpecifier {
                span: "MyStruct[2]",
                stem: TypeStem::parse("MyStruct").unwrap(),
                sizes: vec![NonZeroUsize::new(2)],
            })
        );
    }

    #[test]
    fn a_type_named_tuple() {
        TypeSpecifier::parse("tuple").unwrap();
    }
}
