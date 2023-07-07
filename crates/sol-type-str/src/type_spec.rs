use crate::{Error, Result, TypeStem};

use alloc::vec::Vec;

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
/// ## Example
///
/// ```
/// # use alloy_sol_type_str::TypeSpecifier;
/// # fn main() -> alloy_sol_type_str::Result<()> {
/// let spec = TypeSpecifier::try_from("uint256[2][]")?;
/// assert_eq!(spec.span(), "uint256[2][]");
/// assert_eq!(spec.stem.span(), "uint256");
/// // The sizes are in innermost-to-outermost order.
/// assert_eq!(spec.sizes.as_slice(), &[Some(2), None]);
/// # Ok(())
/// # }
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
    pub sizes: Vec<Option<usize>>,
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
    pub fn parse(span: &'a str) -> Result<Self> {
        let span = span.trim();

        let mut root = span;
        let mut sizes = vec![];

        // an iterator of string slices split by `[`
        for s in root.rsplit('[') {
            // we've reached a root tuple so we need to include the closing
            // paren
            if s.contains(')') {
                let idx = span.rfind(')').unwrap();
                root = &span[..=idx];
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
                .ok_or_else(|| Error::invalid_type_string(span))?;

            if s.is_empty() {
                sizes.push(None);
            } else {
                sizes.push(Some(
                    s.parse().map_err(|_| Error::invalid_type_string(span))?,
                ));
            }
        }

        sizes.reverse();
        Ok(Self {
            span,
            stem: root.try_into()?,
            sizes,
        })
    }

    /// The full span of the specifier.
    #[inline]
    pub const fn span(&self) -> &str {
        self.span
    }

    /// True if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(&self) -> Result<()> {
        self.stem.try_basic_solidity()
    }
}

#[cfg(test)]
mod test {

    use crate::TupleSpecifier;

    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(
            super::TypeSpecifier::try_from("uint256").unwrap(),
            super::TypeSpecifier {
                span: "uint256",
                stem: TypeStem::try_from("uint256").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("uint256[2]").unwrap(),
            super::TypeSpecifier {
                span: "uint256[2]",
                stem: TypeStem::try_from("uint256").unwrap(),
                sizes: vec![Some(2)],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("uint256[2][]").unwrap(),
            super::TypeSpecifier {
                span: "uint256[2][]",
                stem: TypeStem::try_from("uint256").unwrap(),
                sizes: vec![Some(2), None],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("(uint256,uint256)").unwrap(),
            super::TypeSpecifier {
                span: "(uint256,uint256)",
                stem: TypeStem::Tuple(TupleSpecifier::try_from("(uint256,uint256)").unwrap()),
                sizes: vec![],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("(uint256,uint256)[2]").unwrap(),
            super::TypeSpecifier {
                span: "(uint256,uint256)[2]",
                stem: TypeStem::Tuple(TupleSpecifier::try_from("(uint256,uint256)").unwrap()),
                sizes: vec![Some(2)],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("MyStruct").unwrap(),
            super::TypeSpecifier {
                span: "MyStruct",
                stem: TypeStem::try_from("MyStruct").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("MyStruct[2]").unwrap(),
            super::TypeSpecifier {
                span: "MyStruct[2]",
                stem: TypeStem::try_from("MyStruct").unwrap(),
                sizes: vec![Some(2)],
            }
        );
    }
}
