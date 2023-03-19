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

/// ABI Encoding and Decoding errors.
#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug)]
pub enum Error {
    /// Invalid data.
    #[cfg_attr(feature = "std", error("Invalid data"))]
    InvalidData,
    #[cfg_attr(feature = "std", error("Buffer overrun in deser"))]
    /// Overran deser buffer
    Overrun,
    #[cfg_attr(feature = "std", error("Reserialization did not match original"))]
    /// Validation reserialization did not match input
    ReserMismatch,
    /// Other errors.
    #[cfg_attr(feature = "std", error("{0}"))]
    Other(Cow<'static, str>),
}
