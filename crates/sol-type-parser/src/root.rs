use crate::{is_valid_identifier, Error, Result};
use core::fmt;

/// A root type, with no array suffixes. Corresponds to a single, non-sequence
/// type. This is the most basic type specifier.
///
/// # Examples
///
/// ```
/// # use alloy_sol_type_parser::RootType;
/// let root_type = RootType::try_from("uint256")?;
/// assert_eq!(root_type.span(), "uint256");
///
/// // Allows unknown types
/// assert_eq!(
///     RootType::try_from("MyStruct")?.span(),
///     "MyStruct",
/// );
///
/// // No sequences
/// assert!(
///     RootType::try_from("uint256[2]").is_err()
/// );
///
/// // No tuples
/// assert!(
///    RootType::try_from("(uint256,uint256)").is_err()
/// );
/// # Ok::<_, alloy_sol_type_parser::Error>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RootType<'a>(&'a str);

impl<'a> TryFrom<&'a str> for RootType<'a> {
    type Error = Error;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self> {
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
    pub fn parse(value: &'a str) -> Result<Self> {
        if is_valid_identifier(value) {
            Ok(Self(value))
        } else {
            Err(Error::invalid_type_string(value))
        }
    }

    /// The string underlying this type. The type name.
    #[inline]
    pub const fn span(self) -> &'a str {
        self.0
    }

    /// Returns `Ok(())` if the type is a basic Solidity type.
    #[inline]
    pub fn try_basic_solidity(self) -> Result<()> {
        match self.0 {
            "address" | "bool" | "string" | "bytes" | "uint" | "int" => Ok(()),
            name => {
                if let Some(sz) = name.strip_prefix("bytes") {
                    if let Ok(sz) = sz.parse::<usize>() {
                        if sz != 0 && sz <= 32 {
                            return Ok(())
                        }
                    }
                    return Err(Error::invalid_size(name))
                }

                // fast path both integer types
                let s = name.strip_prefix('u').unwrap_or(name);

                if let Some(sz) = s.strip_prefix("int") {
                    if let Ok(sz) = sz.parse::<usize>() {
                        if sz != 0 && sz <= 256 && sz % 8 == 0 {
                            return Ok(())
                        }
                    }
                    Err(Error::invalid_size(name))
                } else {
                    Err(Error::invalid_type_string(name))
                }
            }
        }
    }
}
