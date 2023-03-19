// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Ethers-rs Team

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::no_std_prelude::Cow;
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
#[cfg(feature = "serde")]
use core::num;
#[cfg(feature = "std")]
use thiserror::Error;

/// ABI result type
pub type AbiResult<T> = core::result::Result<T, Error>;

/// ABI errors
#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug)]
pub enum Error {
    /// Invalid entity such as a bad function name.
    #[cfg_attr(feature = "std", error("Invalid name: {0}"))]
    InvalidName(String),
    /// Invalid data.
    #[cfg_attr(feature = "std", error("Invalid data"))]
    InvalidData,
    #[cfg_attr(feature = "std", error("Buffer overrun in deser"))]
    /// Overran deser buffer
    Overrun,
    #[cfg_attr(feature = "std", error("Extra data in deser buffer"))]
    /// Extra data in deser buffer
    ExtraData,
    /// Serialization error.
    #[cfg(feature = "full-serde")]
    #[error("Serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    /// Integer parsing error.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "std", error("Integer parsing error: {0}"))]
    ParseInt(#[cfg_attr(feature = "std", from)] num::ParseIntError),
    /// Hex string parsing error.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "std", error("Hex parsing error: {0}"))]
    Hex(#[cfg_attr(feature = "std", from)] hex::FromHexError),
    /// Other errors.
    #[cfg_attr(feature = "std", error("{0}"))]
    Other(Cow<'static, str>),
}

#[cfg(feature = "serde")]
impl From<uint::FromDecStrErr> for Error {
    fn from(err: uint::FromDecStrErr) -> Self {
        use uint::FromDecStrErr::*;
        match err {
            InvalidCharacter => Self::Other(Cow::Borrowed("Uint parse error: InvalidCharacter")),
            InvalidLength => Self::Other(Cow::Borrowed("Uint parse error: InvalidLength")),
        }
    }
}
