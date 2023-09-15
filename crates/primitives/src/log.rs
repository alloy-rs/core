use crate::{Bytes, B256};
use alloc::vec::Vec;

/// An Ethereum event log object.
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "arbitrary",
    derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct Log {
    /// The indexed topic list.
    pub topics: Vec<B256>,
    /// The plain data.
    pub data: Bytes,
}

#[allow(clippy::missing_const_for_fn)]
impl Log {
    /// Creates a new log.
    #[inline]
    pub fn new(topics: Vec<B256>, data: Bytes) -> Self {
        Self { topics, data }
    }

    /// Creates a new empty log.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            topics: Vec::new(),
            data: Bytes::new(),
        }
    }
}
