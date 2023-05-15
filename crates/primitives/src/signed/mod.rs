//! This module contains a 256-bit signed integer implementation.

/// Error types for signed integers.
mod errors;
pub use errors::{BigIntConversionError, ParseSignedError};

/// A simple [`Sign`] enum, for dealing with integer signs.
mod sign;
pub use sign::Sign;

/// Type aliases for signed integers whose bitsize is divisble by 8.
pub mod aliases;

/// Signed integer type wrapping a [`ruint::Uint`].
mod int;
pub use int::Signed;

/// Operation implementations.
mod ops;

/// Conversion implementations.
mod conversions;

/// Utility functions used in the signed integer implementation.
pub(crate) mod utils;
pub use utils::const_eq;
