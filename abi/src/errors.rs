// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::no_std_prelude::*;
use core::fmt;

/// ABI result type
pub type AbiResult<T> = core::result::Result<T, Error>;

/// ABI Encoding and Decoding errors.
#[derive(Debug)]
pub enum Error {
    /// A typecheck detected a word that does not match the data type
    TypeCheckFail {
        /// The Solidity type we failed to produce
        expected_type: Cow<'static, str>,
        /// Hex-encoded data
        data: String,
    },

    /// Overran deserialization buffer
    Overrun,

    /// Validation reserialization did not match input
    ReserMismatch,

    /// Hex error
    FromHexError(hex::FromHexError),

    /// Other errors.
    Other(Cow<'static, str>),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FromHexError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeCheckFail {
                expected_type,
                data,
            } => {
                write!(
                    f,
                    "Type check failed for \"{expected_type}\" with data: {data}",
                )
            }
            Self::Overrun => f.write_str("Buffer overrun while deserializing"),
            Self::ReserMismatch => f.write_str("Reserialization did not match original"),
            Self::FromHexError(e) => e.fmt(f),
            Self::Other(e) => f.write_str(e),
        }
    }
}

impl Error {
    /// Instantiates a new error with a static str
    pub fn custom(s: impl Into<Cow<'static, str>>) -> Self {
        Self::Other(s.into())
    }

    /// Instantiates a [`Error::TypeCheckFail`] with the provided data
    pub fn type_check_fail_sig(data: &[u8], signature: &'static str) -> Self {
        let expected_type = signature.split('(').next().unwrap_or(signature);
        Self::type_check_fail(data, expected_type)
    }

    /// Instantiates a [`Error::TypeCheckFail`] with the provided data
    pub fn type_check_fail(data: &[u8], expected_type: impl Into<Cow<'static, str>>) -> Self {
        Self::TypeCheckFail {
            expected_type: expected_type.into(),
            data: hex::encode(data),
        }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Self::FromHexError(value)
    }
}
