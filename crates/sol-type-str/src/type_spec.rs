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
/// - `struct MyContract.MyStruct[333]
/// - `enum MyContract.MyEnum[][][][][][]
/// - `MyValueType`
///
/// ## Example
///
/// ```
/// # use alloy_sol_type_str::TypeSpecifier;
/// # use core::num::NonZeroUsize;
/// # fn main() -> alloy_sol_type_str::Result<()> {
/// let spec = TypeSpecifier::try_from("uint256[2][]")?;
/// assert_eq!(spec.span(), "uint256[2][]");
/// assert_eq!(spec.stem.span(), "uint256");
/// // The sizes are in innermost-to-outermost order.
/// assert_eq!(spec.sizes.as_slice(), &[NonZeroUsize::new(2), None]);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSpecifier<'a> {
    /// A keyword that may precede the type specifier. These are primarily used
    /// in the ABI JSON `internalType` field. The valid keywords are `struct`,
    /// `enum`, and `contract`.
    pub keyword: Option<&'a str>,
    /// The full span of the specifier.
    pub span: &'a str,
    /// The contract name, if the type is a member of another contract.
    /// These types may be UDVTs, enums, or structs. If the type is qualified
    /// with a contract name, it will be of the format `Contract.Name`
    pub contract: Option<&'a str>,
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

        let (keyword, body) = match span.split_once(' ') {
            Some((kw, body)) if kw == "struct" || kw == "enum" || kw == "contract" => {
                (Some(kw), body)
            }
            Some(_) => return Err(Error::invalid_type_string(span)),
            _ => (None, span),
        };

        let (contract, body) = match body.split_once('.') {
            Some((contract, body)) => (Some(contract), body),
            None => (None, body),
        };

        // "contract A.B" is never a legal type string
        if keyword == Some("contract") && contract.is_some() {
            return Err(Error::invalid_type_string(span))
        }

        let mut root = body;
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
            let size = if s.is_empty() {
                None
            } else {
                Some(s.parse().map_err(|_| Error::invalid_type_string(span))?)
            };
            sizes.push(size);
        }

        sizes.reverse();
        Ok(Self {
            keyword,
            span,
            contract,
            stem: root.try_into()?,
            sizes,
        })
    }

    /// Returns the type stem as a string.
    #[inline]
    pub const fn span(&self) -> &'a str {
        self.span
    }

    /// Returns the type stem.
    pub const fn stem(&self) -> &TypeStem<'_> {
        &self.stem
    }

    /// Returns true if the type is a basic Solidity type.
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
                keyword: None,
                span: "uint256",
                contract: None,
                stem: TypeStem::try_from("uint256").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("uint256[2]").unwrap(),
            super::TypeSpecifier {
                keyword: None,
                span: "uint256[2]",
                contract: None,
                stem: TypeStem::try_from("uint256").unwrap(),
                sizes: vec![NonZeroUsize::new(2)],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("uint256[2][]").unwrap(),
            super::TypeSpecifier {
                keyword: None,
                span: "uint256[2][]",
                contract: None,
                stem: TypeStem::try_from("uint256").unwrap(),
                sizes: vec![NonZeroUsize::new(2), None],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("(uint256,uint256)").unwrap(),
            super::TypeSpecifier {
                keyword: None,
                span: "(uint256,uint256)",
                contract: None,
                stem: TypeStem::Tuple(TupleSpecifier::try_from("(uint256,uint256)").unwrap()),
                sizes: vec![],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("(uint256,uint256)[2]").unwrap(),
            super::TypeSpecifier {
                keyword: None,
                span: "(uint256,uint256)[2]",
                contract: None,
                stem: TypeStem::Tuple(TupleSpecifier::try_from("(uint256,uint256)").unwrap()),
                sizes: vec![NonZeroUsize::new(2)],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("MyStruct").unwrap(),
            super::TypeSpecifier {
                keyword: None,
                span: "MyStruct",
                contract: None,
                stem: TypeStem::try_from("MyStruct").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            super::TypeSpecifier::try_from("MyStruct[2]").unwrap(),
            super::TypeSpecifier {
                keyword: None,
                span: "MyStruct[2]",
                contract: None,
                stem: TypeStem::try_from("MyStruct").unwrap(),
                sizes: vec![NonZeroUsize::new(2)],
            }
        );
    }

    #[test]
    fn a_type_named_tuple() {
        TypeSpecifier::try_from("tuple").unwrap();
    }

    #[test]
    fn json_internal_type_compatibility() {
        assert_eq!(
            TypeSpecifier::try_from("enum A.B").unwrap(),
            TypeSpecifier {
                keyword: Some("enum"),
                span: "enum A.B",
                contract: Some("A"),
                stem: TypeStem::try_from("B").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            TypeSpecifier::try_from("struct A.B").unwrap(),
            TypeSpecifier {
                keyword: Some("struct"),
                span: "struct A.B",
                contract: Some("A"),
                stem: TypeStem::try_from("B").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            TypeSpecifier::try_from("contract B").unwrap(),
            TypeSpecifier {
                keyword: Some("contract"),
                span: "contract B",
                contract: None,
                stem: TypeStem::try_from("B").unwrap(),
                sizes: vec![],
            }
        );

        assert_eq!(
            TypeSpecifier::try_from("enum A.B[2]").unwrap(),
            TypeSpecifier {
                keyword: Some("enum"),
                span: "enum A.B[2]",
                contract: Some("A"),
                stem: TypeStem::try_from("B").unwrap(),
                sizes: vec![NonZeroUsize::new(2)],
            }
        );
    }

    #[test]
    fn json_internal_type_errors() {
        TypeSpecifier::try_from("enum A A").unwrap_err();
        TypeSpecifier::try_from("enum A.").unwrap_err();
        TypeSpecifier::try_from("struct A..A").unwrap_err();
        TypeSpecifier::try_from("contract A.A").unwrap_err();
        TypeSpecifier::try_from("notAKeyword A.B").unwrap_err();
    }
}
