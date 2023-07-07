use crate::{Error, Result, RootType, TupleSpecifier};

/// This is the stem of a Solidity array type. It is either a root type, or a
/// tuple type.
///
/// ## Example
///
/// ```
/// # use alloy_sol_type_str::{TypeStem, RootType, TupleSpecifier};
/// # fn main() -> alloy_sol_type_str::Result<()> {
/// let stem = TypeStem::try_from("uint256")?;
/// assert_eq!(stem.span(), "uint256");
/// assert!(matches!(stem, TypeStem::Root(_)));
/// assert_eq!(stem.as_root(), Some(&RootType::try_from("uint256").unwrap()));
///
/// let stem = TypeStem::try_from("(uint256,bool)")?;
/// assert_eq!(stem.span(), "(uint256,bool)");
/// assert!(matches!(stem, TypeStem::Tuple(_)));
/// assert_eq!(
///     stem.as_tuple(),
///     Some(&TupleSpecifier::try_from("(uint256,bool)").unwrap())
/// );
/// # Ok(())
/// # }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeStem<'a> {
    /// Root type.
    Root(RootType<'a>),
    /// Tuple type.
    Tuple(TupleSpecifier<'a>),
}

impl<'a> TryFrom<&'a str> for TypeStem<'a> {
    type Error = Error;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self> {
        Self::parse(value)
    }
}

impl AsRef<str> for TypeStem<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.span()
    }
}

impl<'a> TypeStem<'a> {
    /// Parse a type stem from a string.
    pub fn parse(s: &'a str) -> Result<Self> {
        if s.starts_with('(') || s.starts_with("tuple") {
            s.try_into().map(Self::Tuple)
        } else {
            s.try_into().map(Self::Root)
        }
    }

    /// Fallible conversion to root type.
    #[inline]
    pub fn as_root(&self) -> Option<&RootType<'_>> {
        match self {
            Self::Root(root) => Some(root),
            _ => None,
        }
    }

    /// Fallible conversion to tuple type.
    #[inline]
    pub fn as_tuple(&self) -> Option<&TupleSpecifier<'_>> {
        match self {
            Self::Tuple(tuple) => Some(tuple),
            _ => None,
        }
    }

    /// The full span of the type stem
    #[inline]
    pub const fn span(&self) -> &str {
        match self {
            Self::Root(root) => root.span(),
            Self::Tuple(tuple) => tuple.span(),
        }
    }

    #[inline]
    pub(crate) fn try_basic_solidity(&self) -> Result<()> {
        match self {
            Self::Root(root) => root.try_basic_solidity(),
            Self::Tuple(tuple) => tuple.try_basic_solidity(),
        }
    }
}
