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
    InvalidSize(Cow<'static, str>),
    /// Invalid type string, extra chars, or invalid structure
    InvalidTypeString(Cow<'static, str>),
    /// Unknown type referenced from another type
    MissingType(Cow<'static, str>),
    /// Detected circular dep during typegraph resolution
    CircularDependency(Cow<'static, str>),
    /// Invalid Property definition
    InvalidPropertyDefinition(Cow<'static, str>),
    /// Hex
    HexError(hex::FromHexError),
}

impl core::fmt::Display for DynAbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynAbiError::TypeMismatch { expected, actual } => write!(
                f,
                "Type mismatch, expected: {:?}, actual: {}",
                expected, actual
            ),
            DynAbiError::InvalidSize(ty) => write!(f, "Invalid size for type: {}", ty),
            DynAbiError::InvalidTypeString(ty) => write!(f, "Invalid type string: {}", ty),
            DynAbiError::MissingType(name) => {
                write!(f, "Missing type in type resolution: {}", name)
            }
            DynAbiError::CircularDependency(dep) => write!(f, "Circular dependency: {}", dep),
            DynAbiError::InvalidPropertyDefinition(def) => {
                write!(f, "Invalid property definition: {}", def)
            }
            DynAbiError::HexError(h) => write!(f, "Hex error: {}", h),
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

    pub(crate) fn invalid_property_def(def: impl Borrow<str>) -> DynAbiError {
        DynAbiError::InvalidPropertyDefinition(def.borrow().to_owned().into())
    }

    pub(crate) fn invalid_size(ty: impl Borrow<str>) -> DynAbiError {
        DynAbiError::InvalidSize(ty.borrow().to_owned().into())
    }

    pub(crate) fn invalid_type_string(ty: impl Borrow<str>) -> DynAbiError {
        DynAbiError::InvalidTypeString(ty.borrow().to_owned().into())
    }

    pub(crate) fn missing_type(name: impl Borrow<str>) -> DynAbiError {
        DynAbiError::MissingType(name.borrow().to_owned().into())
    }

    pub(crate) fn circular_dependency(dep: impl Borrow<str>) -> DynAbiError {
        DynAbiError::CircularDependency(dep.borrow().to_owned().into())
    }
}
