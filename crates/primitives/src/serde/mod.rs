//! Serde-related utilities for Alloy.

use crate::{B256, hex};
use serde::Serializer;

pub mod displayfromstr;

pub mod checksum;

mod optional;
pub use self::optional::*;

pub mod quantity;

/// Storage related helpers.
pub mod storage;
pub use storage::JsonStorageKey;

#[cfg(feature = "serde-json")]
pub mod ttd;
#[cfg(feature = "serde-json")]
pub use ttd::*;

#[cfg(feature = "serde-json")]
mod other;
#[cfg(feature = "serde-json")]
pub use other::{OtherFields, WithOtherFields};

/// Serialize a byte vec as a hex string _without_ the "0x" prefix.
///
/// This behaves the same as [`hex::encode`].
pub fn serialize_hex_string_no_prefix<S, T>(x: T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    s.serialize_str(&hex::encode(x.as_ref()))
}

/// Serialize a [B256] as a hex string _without_ the "0x" prefix.
pub fn serialize_b256_hex_string_no_prefix<S>(x: &B256, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_str(&format_args!("{x:x}"))
}
