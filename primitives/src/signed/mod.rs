//! This module contains a 256-bit signed integer implementation.

/// Error types for signed integers
mod errors;
pub use errors::{BigIntConversionError, ParseSignedError};

/// Sign type
mod sign;
pub use sign::Sign;

/// Type aliases for signed integers whose bitsize is divisble by 8
pub mod aliases;

/// Signed integer type
mod generic;
pub use generic::{const_eq, Signed};
