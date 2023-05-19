//! This module contains a 256-bit signed integer implementation.

/// Conversion implementations.
mod conversions;

/// Error types for signed integers.
mod errors;
pub use errors::{BigIntConversionError, ParseSignedError};

/// A simple [`Sign`] enum, for dealing with integer signs.
mod sign;
pub use sign::Sign;

/// Signed integer type wrapping a [`ruint::Uint`].
mod int;
pub use int::Signed;

/// Operation implementations.
mod ops;

/// Utility functions used in the signed integer implementation.
pub(crate) mod utils;
pub use utils::const_eq;
