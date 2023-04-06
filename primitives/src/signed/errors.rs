/// The error type that is returned when parsing a 256-bit signed integer.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum ParseI256Error {
    /// Error that occurs when an invalid digit is encountered while parsing.
    #[cfg_attr(feature = "std", error("Parsing Error: {0}"))]
    Ruint(ruint::ParseError),

    /// Error that occurs when the number is too large or too small (negative)
    /// and does not fit in a 256-bit signed integer.
    #[cfg_attr(feature = "std", error("number does not fit in 256-bit integer"))]
    IntegerOverflow,
}

impl From<ruint::ParseError> for ParseI256Error {
    fn from(err: ruint::ParseError) -> Self {
        Self::Ruint(err)
    }
}

/// The error type that is returned when conversion to or from a 256-bit integer fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[cfg_attr(feature = "std", error("output of range integer conversion attempted"))]
pub struct TryFromBigIntError;
