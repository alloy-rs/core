use alloy_sol_type_parser::Error as TypeParserError;
use core::fmt;

/// Dynamic ABI result type.
pub type DynAbiResult<T, E = DynAbiError> = core::result::Result<T, E>;

/// Error when parsing EIP-712 `encodeType` strings
///
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
#[derive(Debug, Clone, PartialEq)]
pub enum DynAbiError {
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
    MissingType(alloc::string::String),
    /// Detected circular dep during typegraph resolution.
    #[cfg(feature = "eip712")]
    CircularDependency(alloc::string::String),
    /// Invalid Property definition.
    #[cfg(feature = "eip712")]
    InvalidPropertyDefinition(alloc::string::String),

    /// Hex.
    HexError(hex::FromHexError),
    /// Type Str Error
    TypeParserError(TypeParserError),
}

impl From<TypeParserError> for DynAbiError {
    #[inline]
    fn from(e: TypeParserError) -> Self {
        Self::TypeParserError(e)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DynAbiError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        #[allow(unreachable_patterns)]
        match self {
            Self::HexError(e) => Some(e),
            Self::TypeParserError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for DynAbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "eip712")]
            Self::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch, expected: {expected:?}, actual: {actual}")
            }
            #[cfg(feature = "eip712")]
            Self::MissingType(name) => write!(f, "Missing type in type resolution: {name}"),
            #[cfg(feature = "eip712")]
            Self::CircularDependency(dep) => write!(f, "Circular dependency: {dep}"),
            #[cfg(feature = "eip712")]
            Self::InvalidPropertyDefinition(def) => {
                write!(f, "Invalid property definition: {def}")
            }

            Self::HexError(h) => h.fmt(f),
            Self::TypeParserError(e) => e.fmt(f),
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
    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn type_mismatch(expected: crate::DynSolType, actual: &serde_json::Value) -> Self {
        Self::TypeMismatch {
            expected,
            actual: actual.clone(),
        }
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn invalid_property_def(def: &str) -> Self {
        Self::InvalidPropertyDefinition(def.into())
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn missing_type(name: &str) -> Self {
        Self::MissingType(name.into())
    }

    #[cfg(feature = "eip712")]
    #[inline]
    pub(crate) fn circular_dependency(dep: &str) -> Self {
        Self::CircularDependency(dep.into())
    }
}
