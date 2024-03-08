use crate::{Address, Bytes, B256};
use alloc::vec::Vec;

/// An Ethereum event log object.
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary))]
pub struct LogData {
    /// The indexed topic list.
    topics: Vec<B256>,
    /// The plain data.
    pub data: Bytes,
}

impl LogData {
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
        Self { topics: Vec::new(), data: Bytes::new() }
    }

    /// True if valid, false otherwise.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.topics.len() <= 4
    }

    /// Get the topic list.
    #[inline]
    pub fn topics(&self) -> &[B256] {
        &self.topics
    }

    /// Get the topic list, mutably. This gives access to the internal
    /// array, without allowing extension of that array.
    #[inline]
    pub fn topics_mut(&mut self) -> &mut [B256] {
        &mut self.topics
    }

    /// Get a mutable reference to the topic list. This allows creation of
    /// invalid logs.
    #[inline]
    pub fn topics_mut_unchecked(&mut self) -> &mut Vec<B256> {
        &mut self.topics
    }

    /// Set the topic list, without length-checking. This allows creation of
    /// invalid logs.
    #[inline]
    pub fn set_topics_unchecked(&mut self, topics: Vec<B256>) {
        self.topics = topics;
    }

    /// Set the topic list, truncating to 4 topics.
    #[inline]
    pub fn set_topics_truncating(&mut self, mut topics: Vec<B256>) {
        topics.truncate(4);
        self.set_topics_unchecked(topics);
    }

    /// Consumes the log data, returning the topic list and the data.
    #[inline]
    pub fn split(self) -> (Vec<B256>, Bytes) {
        (self.topics, self.data)
    }
}

/// A log consists of an address, and some log data.
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary))]
pub struct Log<T = LogData> {
    /// The address which emitted this log.
    pub address: Address,
    /// The log data.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data: T,
}

impl<T> core::ops::Deref for Log<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Log {
    /// Creates a new log.
    #[inline]
    pub fn new(address: Address, topics: Vec<B256>, data: Bytes) -> Option<Self> {
        LogData::new(topics, data).map(|data| Self { address, data })
    }

    /// Creates a new log.
    #[inline]
    pub fn new_unchecked(address: Address, topics: Vec<B256>, data: Bytes) -> Self {
        Self { address, data: LogData::new_unchecked(topics, data) }
    }

    /// Creates a new empty log.
    #[inline]
    pub const fn empty() -> Self {
        Self { address: Address::ZERO, data: LogData::empty() }
    }
}

#[cfg(feature = "rlp")]
impl alloy_rlp::Encodable for Log {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let payload_length =
            self.address.length() + self.data.data.length() + self.data.topics.length();

        alloy_rlp::Header { list: true, payload_length }.encode(out);
        self.address.encode(out);
        self.data.topics.encode(out);
        self.data.data.encode(out);
    }

    fn length(&self) -> usize {
        let payload_length =
            self.address.length() + self.data.data.length() + self.data.topics.length();
        payload_length + alloy_rlp::length_of_length(payload_length)
    }
}

#[cfg(feature = "rlp")]
impl alloy_rlp::Decodable for Log {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let h = alloy_rlp::Header::decode(buf)?;
        let pre = buf.len();

        let address = alloy_rlp::Decodable::decode(buf)?;
        let topics = alloy_rlp::Decodable::decode(buf)?;
        let data = alloy_rlp::Decodable::decode(buf)?;

        if h.payload_length != pre - buf.len() {
            return Err(alloy_rlp::Error::Custom("did not consume exact payload"));
        }

        Ok(Self { address, data: LogData { topics, data } })
    }
}
