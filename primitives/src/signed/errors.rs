use ruint::BaseConvertError;

/// The error type that is returned when parsing a signed integer.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum ParseSignedError {
    /// Error that occurs when an invalid digit is encountered while parsing.
    #[cfg_attr(feature = "std", error("Parsing Error: {0}"))]
    Ruint(ruint::ParseError),

    /// Error that occurs when the number is too large or too small (negative)
    /// and does not fit in the target signed integer.
    #[cfg_attr(feature = "std", error("number does not fit in the integer size"))]
    IntegerOverflow,
}

impl From<ruint::ParseError> for ParseSignedError {
    fn from(err: ruint::ParseError) -> Self {
        // these errors are redundant, so we coerce the more complex one to the
        // simpler one
        match err {
            ruint::ParseError::BaseConvertError(BaseConvertError::Overflow) => {
                Self::IntegerOverflow
            }
            _ => Self::Ruint(err),
        }
    }
}

/// The error type that is returned when conversion to or from a integer fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[cfg_attr(feature = "std", error("output of range integer conversion attempted"))]
pub struct BigIntConversionError;
