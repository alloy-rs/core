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
    topics: Vec<B256>,
    /// The plain data.
    pub data: Bytes,
}

#[allow(clippy::missing_const_for_fn)]
impl Log {
    /// Creates a new log, without length-checking. This allows creation of
    /// invalid logs. May be safely used when the length of the topic list is
    /// known to be 4 or less.
    #[inline]
    pub fn new_unchecked(topics: Vec<B256>, data: Bytes) -> Self {
        Self { topics, data }
    }

    /// Creates a new log.
    #[inline]
    pub fn new(topics: Vec<B256>, data: Bytes) -> Option<Self> {
        let this = Self::new_unchecked(topics, data);
        this.is_valid().then_some(this)
    }

    /// Creates a new empty log.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            topics: Vec::new(),
            data: Bytes::new(),
        }
    }

    /// True if valid, false otherwise.
    pub fn is_valid(&self) -> bool {
        self.topics.len() <= 4
    }

    /// Get the topic list.
    pub fn topics(&self) -> &[B256] {
        &self.topics
    }

    /// Get the topic list, mutably. This gives access to the internal
    /// array, without allowing extension of that array.
    pub fn topics_mut(&mut self) -> &mut [B256] {
        &mut self.topics
    }

    /// Get a mutable reference to the topic list. This allows creation of
    /// invalid logs.
    pub fn topics_mut_unchecked(&mut self) -> &mut Vec<B256> {
        &mut self.topics
    }

    /// Set the topic list, without length-checking. This allows creation of
    /// invalid logs.
    pub fn set_topics_unchecked(&mut self, topics: Vec<B256>) {
        self.topics = topics;
    }

    /// Set the topic list, truncating to 4 topics.
    pub fn set_topics_truncating(&mut self, mut topics: Vec<B256>) {
        topics.truncate(4);
        self.set_topics_unchecked(topics);
    }
}
