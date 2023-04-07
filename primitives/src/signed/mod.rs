//! This module contains a 256-bit signed integer implementation.

/// Error types for signed integers
mod errors;
pub use errors::{ParseSignedError, TryFromBigIntError};

/// Sign type
mod sign;
pub use sign::Sign;

/// Signed integer type
mod generic;
pub use generic::{const_eq, Signed};

/// I256 signed integer type.
pub type I256 = Signed<256, 4>;

/// I192 signed integer type.
pub type I192 = Signed<192, 3>;

/// I128 signed integer type.
pub type I128 = Signed<128, 2>;
