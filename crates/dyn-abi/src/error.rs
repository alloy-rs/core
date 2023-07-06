use alloc::string::String;
use core::fmt;

/// Dynamic ABI result type.
pub type DynAbiResult<T, E = DynAbiError> = core::result::Result<T, E>;

/// Error when parsing EIP-712 `encodeType` strings
///
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
#[derive(Debug, Clone, PartialEq)]
pub enum DynAbiError {
    /// Invalid size for a primitive type (intX, uintX, or bytesX).
    InvalidSize(String),
    /// Invalid type string, extra chars, or invalid structure.
    InvalidTypeString(String),

    /// Type mismatch during coercion.
    #[cfg(feature = "eip712")]
    TypeMismatch {
        /// The expected type.
        expected: crate::DynSolType,
        /// The actual type.
        actual: serde_json::Value,
    },
    /// Unknown type referenced from another type.
    #[cfg(feature = "eip712")]
    MissingType(String),
    /// Detected circular dep during typegraph resolution.
    #[cfg(feature = "eip712")]
    CircularDependency(String),
    /// Invalid Property definition.
    #[cfg(feature = "eip712")]
    InvalidPropertyDefinition(String),

    /// Hex.
    HexError(hex::FromHexError),
}

#[cfg(feature = "std")]
impl std::error::Error for DynAbiError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::HexError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for DynAbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynAbiError::InvalidSize(ty) => write!(f, "Invalid size for type: {ty}"),
            DynAbiError::InvalidTypeString(ty) => write!(f, "Invalid type string: {ty}"),

            #[cfg(feature = "eip712")]
            DynAbiError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch, expected: {expected:?}, actual: {actual}")
            }
            #[cfg(feature = "eip712")]
            DynAbiError::MissingType(name) => write!(f, "Missing type in type resolution: {name}"),
            #[cfg(feature = "eip712")]
            DynAbiError::CircularDependency(dep) => write!(f, "Circular dependency: {dep}"),
            #[cfg(feature = "eip712")]
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

#[allow(dead_code)]
impl DynAbiError {
    #[inline]
    pub(crate) fn invalid_size(ty: &str) -> DynAbiError {
        DynAbiError::InvalidSize(ty.into())
    }

    #[inline]
    pub(crate) fn invalid_type_string(ty: &str) -> DynAbiError {
        DynAbiError::InvalidTypeString(ty.into())
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn type_mismatch(
        expected: crate::DynSolType,
        actual: &serde_json::Value,
    ) -> DynAbiError {
        DynAbiError::TypeMismatch {
            expected,
            actual: actual.clone(),
        }
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn invalid_property_def(def: &str) -> DynAbiError {
        DynAbiError::InvalidPropertyDefinition(def.into())
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn missing_type(name: &str) -> DynAbiError {
        DynAbiError::MissingType(name.into())
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn circular_dependency(dep: &str) -> DynAbiError {
        DynAbiError::CircularDependency(dep.into())
    }
}
