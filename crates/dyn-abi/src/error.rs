use alloc::{borrow::Cow, string::String};
use alloy_sol_type_parser::Error as TypeParserError;
use alloy_sol_types::Error as SolTypesError;
use core::fmt;
use hex::FromHexError;

/// Dynamic ABI result type.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Error when parsing EIP-712 `encodeType` strings
///
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Unknown type referenced from another type.
    #[cfg(feature = "eip712")]
    MissingType(String),
    /// Detected circular dep during typegraph resolution.
    #[cfg(feature = "eip712")]
    CircularDependency(String),
    /// Invalid property definition.
    #[cfg(feature = "eip712")]
    InvalidPropertyDefinition(String),

    /// Type mismatch during encoding or coercion.
    TypeMismatch {
        /// The expected type.
        expected: String,
        /// The actual type.
        actual: String,
    },
    /// Length mismatch during encoding.
    EncodeLengthMismatch {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },

    /// [`hex`] error.
    Hex(hex::FromHexError),
    /// [`alloy_sol_type_parser`] error.
    TypeParser(TypeParserError),
    /// [`alloy_sol_types`] error.
    SolTypes(SolTypesError),
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        Self::Hex(e)
    }
}

impl From<SolTypesError> for Error {
    fn from(e: SolTypesError) -> Self {
        Self::SolTypes(e)
    }
}

impl From<TypeParserError> for Error {
    fn from(e: TypeParserError) -> Self {
        Self::TypeParser(e)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Hex(e) => Some(e),
            Self::TypeParser(e) => Some(e),
            Self::SolTypes(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "eip712")]
            Self::MissingType(name) => write!(f, "missing type in type resolution: {name}"),
            #[cfg(feature = "eip712")]
            Self::CircularDependency(dep) => write!(f, "circular dependency: {dep}"),
            #[cfg(feature = "eip712")]
            Self::InvalidPropertyDefinition(def) => write!(f, "invalid property definition: {def}"),

            Self::TypeMismatch { expected, actual } => write!(
                f,
                "type mismatch, expected type {expected:?}, got value with type {actual:?}",
            ),
            &Self::EncodeLengthMismatch { expected, actual } => write!(
                f,
                "encode length mismatch, expected {expected} types, got {actual}",
            ),

            Self::Hex(e) => e.fmt(f),
            Self::TypeParser(e) => e.fmt(f),
            Self::SolTypes(e) => e.fmt(f),
        }
    }
}

#[allow(dead_code)]
impl Error {
    /// Instantiates a new error with a static str.
    #[inline]
    pub fn custom(s: impl Into<Cow<'static, str>>) -> Self {
        Self::SolTypes(SolTypesError::custom(s))
    }

    #[cfg(feature = "eip712")]
    pub(crate) fn type_mismatch(expected: &crate::DynSolType, actual: &serde_json::Value) -> Self {
        Self::TypeMismatch {
            expected: expected.sol_type_name().into_owned(),
            actual: actual.to_string(),
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
