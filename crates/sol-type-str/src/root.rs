use core::fmt;

use crate::{error::Error, Result};

/// A root type, with no array suffixes. Corresponds to a single, non-sequence
/// type. This is the most basic type specifier.
///
/// Examples:
///
/// ```
/// # use alloy_sol_type_str::RootType;
/// # fn main() -> alloy_sol_type_str::Result<()> {
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
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RootType<'a>(&'a str);

impl fmt::Display for RootType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl RootType<'_> {
    /// The full span of the root type.
    pub const fn span(&self) -> &str {
        self.0
    }

    /// The string underlying this type. The type name.
    pub const fn as_str(&self) -> &str {
        self.0
    }

    /// An identifier in Solidity has to start with a letter, a dollar-sign or
    /// an underscore and may additionally contain numbers after the first
    /// symbol.
    ///
    /// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.Identifier>
    pub fn legal_identifier(&self) -> Result<()> {
        if self.0.is_empty() {
            return Err(Error::invalid_type_string(self.0))
        }

        match self.0.chars().next().unwrap() {
            'a'..='z' | 'A'..='Z' | '_' | '$' => {}
            _ => return Err(Error::invalid_type_string(self.0)),
        }

        if self.0.chars().skip(1).all(char::is_alphanumeric) {
            Ok(())
        } else {
            Err(Error::invalid_type_string(self.0))
        }
    }

    /// Ok(()) if the type is a basic Solidity type, otherwise an error that
    /// indicates the source of the parser failure.
    pub fn try_basic_solidity(&self) -> Result<()> {
        let type_name = self.0;
        match type_name {
            "address" | "bool" | "string" | "bytes" => Ok(()),
            _ => {
                if type_name.starts_with("bytes") {
                    if let Some(len) = type_name.strip_prefix("bytes") {
                        if let Ok(len) = len.parse::<usize>() {
                            if len <= 32 {
                                return Ok(())
                            }
                            return Err(Error::invalid_size(type_name))
                        }
                    }
                }
                if type_name.starts_with("uint") {
                    if let Some(len) = type_name.strip_prefix("uint") {
                        if len.is_empty() {
                            return Ok(())
                        }
                        if let Ok(len) = len.parse::<usize>() {
                            if len <= 256 && len % 8 == 0 {
                                return Ok(())
                            }
                            return Err(Error::invalid_size(type_name))
                        }
                    }
                }
                if type_name.starts_with("int") {
                    if let Some(len) = type_name.strip_prefix("int") {
                        if len.is_empty() {
                            return Ok(())
                        }
                        if let Ok(len) = len.parse::<usize>() {
                            if len <= 256 && len % 8 == 0 {
                                return Ok(())
                            }
                            return Err(Error::invalid_size(type_name))
                        }
                    }
                }
                Err(Error::invalid_type_string(type_name))
            }
        }
    }

    /// Returns true if the type is a basic Solidity type, otherwise false.
    pub fn is_basic_solidity(&self) -> bool {
        self.try_basic_solidity().is_ok()
    }
}

impl<'a> TryFrom<&'a str> for RootType<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        let s = Self(value.trim());
        s.legal_identifier()?;
        Ok(s)
    }
}

impl core::borrow::Borrow<str> for RootType<'_> {
    fn borrow(&self) -> &str {
        self.0
    }
}

impl AsRef<str> for RootType<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
