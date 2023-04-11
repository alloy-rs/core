// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;

/// ABI result type
pub type AbiResult<T> = core::result::Result<T, Error>;

/// ABI Encoding and Decoding errors.
#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug)]
pub enum Error {
    /// A typecheck detected a word that does not match the data type
    #[cfg_attr(
        feature = "std",
        error("Type check failed. Expected {expected_type} but {data} is not valid for that type")
    )]
    TypeCheckFail {
        /// Hex-encoded data
        data: Cow<'static, str>,
        /// The Solidity type we failed to produce
        expected_type: Cow<'static, str>,
    },
    #[cfg_attr(feature = "std", error("Buffer overrun in deser"))]
    /// Overran deser buffer
    Overrun,
    #[cfg_attr(feature = "std", error("Reserialization did not match original"))]
    /// Validation reserialization did not match input
    ReserMismatch,
    /// FromHex
    #[cfg_attr(feature = "std", error("{0}"))]
    FromHexError(hex::FromHexError),
    /// Other errors.
    #[cfg_attr(feature = "std", error("{0}"))]
    Other(Cow<'static, str>),
}

impl Error {
    /// Instantiates a new error with a static str
    pub fn custom(s: &'static str) -> Self {
        Self::Other(s.into())
    }

    /// Instantiates a new error with a string
    pub fn custom_owned(s: alloc::string::String) -> Self {
        Self::Other(s.into())
    }

    /// Instantiates a [`Error::TypeCheckFail`] with the provided data
    pub fn type_check_fail(
        data: impl Into<Cow<'static, str>>,
        expected_type: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::TypeCheckFail {
            data: data.into(),
            expected_type: expected_type.into(),
        }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Self::FromHexError(value)
    }
}
