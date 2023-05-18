//! This module contains a 256-bit signed integer implementation.

/// Type aliases for signed integers whose bitsize is divisble by 8.
pub mod aliases;

/// Conversion implementations.
mod conversions;

/// Error types for signed integers.
mod errors;
pub use errors::{BigIntConversionError, ParseSignedError};

/// Signed integer type wrapping a [`ruint::Uint`].
mod int;
pub use int::Signed;

/// Operation implementations.
mod ops;

/// A simple [`Sign`] enum, for dealing with integer signs.
mod sign;
pub use sign::Sign;

/// Serde support.
#[cfg(feature = "serde")]
mod serde;

/// Utility functions used in the signed integer implementation.
pub(crate) mod utils;
pub use utils::const_eq;
