use alloc::string::String;
use core::fmt;

/// Type string parsing result
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// A type string parsing error.
#[derive(Clone, PartialEq, Eq)]
pub struct Error(Repr);

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Error").field(&self.0 .0).finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error {
    /// Instantiate a new error.
    pub fn new(s: impl fmt::Display) -> Self {
        Self::new_("", &s)
    }

    /// Instantiate a new parser error.
    pub(crate) fn parser(e: impl fmt::Display) -> Self {
        Self::new_(
            if cfg!(feature = "std") {
                "parser error:\n"
            } else {
                "parser error: "
            },
            &e,
        )
    }

    /// Instantiate an invalid type string error. Invalid type string errors are
    /// for type strings that are not valid type strings. E.g. "uint256))))[".
    pub fn invalid_type_string(ty: impl fmt::Display) -> Self {
        Self::new_("invalid type string: ", &ty)
    }

    /// Instantiate an invalid size error. Invalid size errors are for valid
    /// primitive types with invalid sizes. E.g. `"uint7"` or `"bytes1337"` or
    /// `"string[aaaaaa]"`.
    pub fn invalid_size(ty: impl fmt::Display) -> Self {
        Self::new_("invalid size for type: ", &ty)
    }

    #[inline(never)]
    #[cold]
    fn new_(s: &str, e: &dyn fmt::Display) -> Self {
        Self(Repr(format!("{s}{e}")))
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Repr(String);

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
