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

impl TypeStem<'_> {
    /// Fallible conversion to root type.
    pub fn as_root(&self) -> Option<&RootType<'_>> {
        match self {
            Self::Root(root) => Some(root),
            _ => None,
        }
    }

    /// Fallible conversion to tuple type.
    pub fn as_tuple(&self) -> Option<&TupleSpecifier<'_>> {
        match self {
            Self::Tuple(tuple) => Some(tuple),
            _ => None,
        }
    }

    /// The full span of the type stem
    pub fn span(&self) -> &str {
        match self {
            Self::Root(root) => root.span(),
            Self::Tuple(tuple) => tuple.span(),
        }
    }

    pub(crate) fn try_basic_solidity(&self) -> Result<()> {
        match self {
            Self::Root(root) => root.try_basic_solidity(),
            Self::Tuple(tuple) => tuple.try_basic_solidity(),
        }
    }
}

impl<'a> TryFrom<&'a str> for TypeStem<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        if value.starts_with('(') || value.starts_with("tuple(") {
            Ok(Self::Tuple(value.try_into()?))
        } else {
            Ok(Self::Root(value.try_into()?))
        }
    }
}
