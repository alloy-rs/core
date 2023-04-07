//! This module contains a 256-bit signed integer implementation.

/// Error types for signed integers
mod errors;
pub use errors::{BigIntConversionError, ParseSignedError};

/// Sign type
mod sign;
pub use sign::Sign;

/// Signed integer type
mod generic;
pub use generic::{const_eq, Signed};

/// 128-bit signed integer type.
pub type I128 = Signed<128, 2>;

/// 160-bit signed integer type.
pub type I160 = Signed<160, 3>;

/// 192-bit signed integer type.
pub type I192 = Signed<192, 3>;

/// 256-bit signed integer type.
pub type I256 = Signed<256, 4>;
