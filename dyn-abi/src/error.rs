use crate::{no_std_prelude::*, DynSolType};

/// Error when parsing EIP-712 `encodeType` strings
///
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
#[derive(Debug, Clone, PartialEq)]
pub enum DynAbiError {
    /// Type mismatch during coercion
    TypeMismatch {
        /// The expected type
        expected: DynSolType,
        /// The actual type
        actual: serde_json::Value,
    },
    /// Invalid size for a primitive type (intX, uintX, or bytesX)
    InvalidSize(String),
    /// Invalid type string, extra chars, or invalid structure
    InvalidTypeString(String),
    /// Unknown type referenced from another type
    MissingType(String),
    /// Detected circular dep during typegraph resolution
    CircularDependency(String),
    /// Invalid Property definition
    InvalidPropertyDefinition(String),
    /// Hex
    HexError(hex::FromHexError),
}

#[cfg(feature = "std")]
impl std::error::Error for DynAbiError {}

impl fmt::Display for DynAbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynAbiError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch, expected: {expected:?}, actual: {actual}")
            }
            DynAbiError::InvalidSize(ty) => write!(f, "Invalid size for type: {ty}"),
            DynAbiError::InvalidTypeString(ty) => write!(f, "Invalid type string: {ty}"),
            DynAbiError::MissingType(name) => write!(f, "Missing type in type resolution: {name}"),
            DynAbiError::CircularDependency(dep) => write!(f, "Circular dependency: {dep}"),
            DynAbiError::InvalidPropertyDefinition(def) => {
                write!(f, "Invalid property definition: {def}")
            }
            DynAbiError::HexError(h) => h.fmt(f),
        }
    }
}

impl From<hex::FromHexError> for DynAbiError {
    fn from(e: hex::FromHexError) -> Self {
        DynAbiError::HexError(e)
    }
}

impl DynAbiError {
    pub(crate) fn type_mismatch(expected: DynSolType, actual: &serde_json::Value) -> DynAbiError {
        DynAbiError::TypeMismatch {
            expected,
            actual: actual.clone(),
        }
    }

    pub(crate) fn invalid_property_def(def: impl ToString) -> DynAbiError {
        DynAbiError::InvalidPropertyDefinition(def.to_string())
    }

    pub(crate) fn invalid_size(ty: impl ToString) -> DynAbiError {
        DynAbiError::InvalidSize(ty.to_string())
    }

    pub(crate) fn invalid_type_string(ty: impl ToString) -> DynAbiError {
        DynAbiError::InvalidTypeString(ty.to_string())
    }

    pub(crate) fn missing_type(name: impl ToString) -> DynAbiError {
        DynAbiError::MissingType(name.to_string())
    }

    pub(crate) fn circular_dependency(dep: impl ToString) -> DynAbiError {
        DynAbiError::CircularDependency(dep.to_string())
    }
}
