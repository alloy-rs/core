use alloc::string::{String, ToString};
use core::fmt;

/// Type string parsing result
pub type Result<T> = core::result::Result<T, Error>;

/// A type string parsing error.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Invalid type string, extra chars, or invalid structure.
    InvalidTypeString(String),
    /// Invalid size for a primitive type (intX, uintX, or bytesX).
    InvalidSize(String),
}

impl Error {
    /// Instantiate an invalid type string error. Invalid type string errors are
    /// for type strings that are not valid type strings. E.g. "uint256))))[".
    pub fn invalid_type_string(ty: impl ToString) -> Self {
        Self::InvalidTypeString(ty.to_string())
    }

    /// Instantiate an invalid size error. Invalid size errors are for valid
    /// primitive types with invalid sizes. E.g. `"uint7"` or `"bytes1337"` or
    /// `"string[aaaaaa]"`.
    pub fn invalid_size(ty: impl ToString) -> Self {
        Self::InvalidSize(ty.to_string())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTypeString(s) => write!(f, "Invalid type string: {}", s),
            Self::InvalidSize(ty) => write!(f, "Invalid size for type: {ty}"),
        }
    }
}
