use core::fmt;

/// RLP result type.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// RLP error type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Numeric Overflow.
    Overflow,
    /// Leading zero disallowed.
    LeadingZero,
    /// Overran input while decoding.
    InputTooShort,
    /// Expected single byte, but got invalid value.
    NonCanonicalSingleByte,
    /// Expected size, but got invalid value.
    NonCanonicalSize,
    /// Expected a payload of a specific size, got an unexpected size.
    UnexpectedLength,
    /// Expected another type, got a string instead.
    UnexpectedString,
    /// Expected another type, got a list instead.
    UnexpectedList,
    /// Got an unexpected number of items in a list.
    ListLengthMismatch {
        /// Expected length.
        expected: usize,
        /// Actual length.
        got: usize,
    },
    /// Custom Err.
    Custom(&'static str),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Overflow => f.write_str("overflow"),
            Error::LeadingZero => f.write_str("leading zero"),
            Error::InputTooShort => f.write_str("input too short"),
            Error::NonCanonicalSingleByte => f.write_str("non-canonical single byte"),
            Error::NonCanonicalSize => f.write_str("non-canonical size"),
            Error::UnexpectedLength => f.write_str("unexpected length"),
            Error::UnexpectedString => f.write_str("unexpected string"),
            Error::UnexpectedList => f.write_str("unexpected list"),
            Error::ListLengthMismatch { got, expected } => {
                write!(f, "unexpected list length (got {got}, expected {expected})")
            }
            Error::Custom(err) => f.write_str(err),
        }
    }
}
