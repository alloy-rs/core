use alloy_sol_type_str::Error as TypeStrError;

use core::fmt;

use alloc::string::String;

/// Dynamic ABI result type.
pub type DynAbiResult<T, E = DynAbiError> = core::result::Result<T, E>;

/// Error when parsing EIP-712 `encodeType` strings
///
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
#[derive(Debug, Clone, PartialEq)]
pub enum DynAbiError {
    /// Unknown type referenced from another type.
    MissingType(String),
    /// Hex.
    HexError(hex::FromHexError),
    /// Type String parsing issue.
    InvalidTypeString(TypeStrError),

    #[cfg(feature = "eip712")]
    /// Type mismatch during coercion.
    TypeMismatch {
        /// The expected type.
        expected: DynSolType,
        /// The actual type.
        actual: serde_json::Value,
    },
    /// Detected circular dep during typegraph resolution.
    #[cfg(feature = "eip712")]
    CircularDependency(String),
    /// Invalid Property definition.
    #[cfg(feature = "eip712")]
    InvalidPropertyDefinition(String),
}

#[cfg(feature = "std")]
impl std::error::Error for DynAbiError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::HexError(e) => Some(e),
            Self::InvalidTypeString(e) => Some(e),
            _ => None,
        }
    }
}

impl From<TypeStrError> for DynAbiError {
    fn from(e: TypeStrError) -> Self {
        Self::InvalidTypeString(e)
    }
}

impl fmt::Display for DynAbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynAbiError::MissingType(name) => write!(f, "Missing type in type resolution: {name}"),
            DynAbiError::InvalidTypeString(s) => write!(f, "Invalid type string: {}", s),
            DynAbiError::HexError(h) => h.fmt(f),

            #[cfg(feature = "eip712")]
            DynAbiError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch, expected: {expected:?}, actual: {actual}")
            }
            #[cfg(feature = "eip712")]
            DynAbiError::CircularDependency(dep) => write!(f, "Circular dependency: {dep}"),
            #[cfg(feature = "eip712")]
            DynAbiError::InvalidPropertyDefinition(def) => {
                write!(f, "Invalid property definition: {def}")
            }
        }
    }
}

impl From<hex::FromHexError> for DynAbiError {
    fn from(e: hex::FromHexError) -> Self {
        Self::HexError(e)
    }
}

#[allow(dead_code)]
impl DynAbiError {
    #[inline]
    pub(crate) fn missing_type(name: &str) -> DynAbiError {
        DynAbiError::MissingType(name.into())
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn type_mismatch(expected: DynSolType, actual: &serde_json::Value) -> DynAbiError {
        Self::TypeMismatch {
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
    pub(crate) fn circular_dependency(dep: &str) -> DynAbiError {
        DynAbiError::CircularDependency(dep.into())
    }
}
