use crate::{Error, Result, TypeStem};
use alloc::vec::Vec;
use core::num::NonZeroUsize;

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
    pub fn parse(span: &'a str) -> Result<Self> {
        let span = span.trim();
        let err = || Error::invalid_type_string(span);

        // i is the start of the array sizes
        let (i, is_tuple) = if let Some(i) = span.rfind(')') {
            // ')' is 1 byte
            (i + 1, true)
        } else {
            (span.find('[').unwrap_or(span.len()), false)
        };
        // spit_at_unchecked(i)
        let (l, r) = unsafe { (span.get_unchecked(..i), span.get_unchecked(i..)) };
        // avoids double check in `TypeStem::parse`
        let stem = if is_tuple {
            l.try_into().map(TypeStem::Tuple)
        } else {
            l.try_into().map(TypeStem::Root)
        }?;

        let mut sizes = vec![];
        let mut chars = r.char_indices();
        while let Some((i, next)) = chars.next() {
            match next {
                '[' => {
                    let mut j = 0;
                    let mut closed = false;
                    for (idx, c) in chars.by_ref() {
                        match c {
                            ']' => {
                                closed = true;
                                break
                            }
                            c if c.is_ascii_digit() => j = idx,
                            c if c.is_whitespace() => continue,
                            _ => return Err(err()),
                        }
                    }
                    if !closed {
                        return Err(err())
                    }
                    let size = if j == 0 {
                        None
                    } else {
                        // i and j are the index of '[' and the last digit respectively,
                        // '[' and ASCII digits are 1 byte
                        let s = unsafe { r.get_unchecked(i + 1..j + 1) };
                        // end is trimmed in the loop above
                        Some(s.trim_start().parse().map_err(|_| err())?)
                    };
                    sizes.push(size);
                }
                c if c.is_whitespace() => continue,
                _ => return Err(err()),
            }
        }

        Ok(Self { span, stem, sizes })
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
            TypeSpecifier::parse("uint256").unwrap(),
            TypeSpecifier {
                span: "uint256",
                stem: TypeStem::parse("uint256").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            TypeSpecifier::parse("uint256[2]").unwrap(),
            TypeSpecifier {
                span: "uint256[2]",
                stem: TypeStem::parse("uint256").unwrap(),
                sizes: vec![NonZeroUsize::new(2)],
            }
        );

        assert_eq!(
            TypeSpecifier::parse("uint256[2][]").unwrap(),
            TypeSpecifier {
                span: "uint256[2][]",
                stem: TypeStem::parse("uint256").unwrap(),
                sizes: vec![NonZeroUsize::new(2), None],
            }
        );

        assert_eq!(
            TypeSpecifier::parse("(uint256,uint256)").unwrap(),
            TypeSpecifier {
                span: "(uint256,uint256)",
                stem: TypeStem::Tuple(TupleSpecifier::parse("(uint256,uint256)").unwrap()),
                sizes: vec![],
            }
        );

        assert_eq!(
            TypeSpecifier::parse("(uint256,uint256)[2]").unwrap(),
            TypeSpecifier {
                span: "(uint256,uint256)[2]",
                stem: TypeStem::Tuple(TupleSpecifier::parse("(uint256,uint256)").unwrap()),
                sizes: vec![NonZeroUsize::new(2)],
            }
        );

        assert_eq!(
            TypeSpecifier::parse("MyStruct").unwrap(),
            TypeSpecifier {
                span: "MyStruct",
                stem: TypeStem::parse("MyStruct").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            TypeSpecifier::parse("MyStruct[2]").unwrap(),
            TypeSpecifier {
                span: "MyStruct[2]",
                stem: TypeStem::parse("MyStruct").unwrap(),
                sizes: vec![NonZeroUsize::new(2)],
            }
        );
    }

    #[test]
    fn a_type_named_tuple() {
        TypeSpecifier::parse("tuple").unwrap();
    }
}
