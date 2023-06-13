use crate::{
    token::{TokenSeq, WordToken},
    Result, SolType,
};
use alloc::vec::Vec;
use alloy_primitives::{FixedBytes, B256};

mod topic;
pub use topic::EventTopic;

mod topic_list;
pub use topic_list::TopicList;

/// Solidity event.
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity event
/// definition.
pub trait SolEvent: Sized {
    /// The underlying tuple type which represents this event's non-indexed
    /// parameters. These parameters are ABI encoded and included in the log
    /// body.
    ///
    /// If this event has no non-indexed parameters, this will be the unit type
    /// `()`.
    type DataTuple: SolType<TokenType = Self::DataToken>;

    /// The [`TokenSeq`] type corresponding to the tuple.
    type DataToken: TokenSeq;

    /// The underlying tuple type which represents this event's topics.
    ///
    /// These are ABI encoded and included in the log struct returned by the
    /// RPC node.
    ///
    /// See the [`TopicList`] trait for more details.
    type TopicList: TopicList;

    /// The event's ABI signature.
    ///
    /// For anonymous events, this is unused, but is still present.
    const SIGNATURE: &'static str;

    /// The event's ABI signature hash, or selector: `keccak256(SIGNATURE)`
    ///
    /// For non-anonymous events, this will be the first topic (`topic0`).
    /// For anonymous events, this is unused, but is still present.
    const SIGNATURE_HASH: FixedBytes<32>;

    /// Whether the event is anonymous.
    const ANONYMOUS: bool;

    /// Convert decoded rust data to the event type.
    fn new(
        topics: <Self::TopicList as SolType>::RustType,
        data: <Self::DataTuple as SolType>::RustType,
    ) -> Self;

    // TODO: avoid clones here
    /// The event's non-indexed parameters.
    fn body(&self) -> <Self::DataTuple as SolType>::RustType;
    // TODO: avoid clones here
    /// The event's topics.
    fn topics(&self) -> <Self::TopicList as SolType>::RustType;

    /// The size of the ABI-encoded dynamic data in bytes.
    fn encoded_size(&self) -> usize {
        // This avoids unnecessary clones.
        // TODO: also avoid necessary clones.
        if let Some(size) = <Self::DataTuple as SolType>::ENCODED_SIZE {
            return size
        }
        <Self::DataTuple>::encoded_size(&self.body())
    }

    /// ABI-encode the dynamic data of this event into the given buffer.
    fn encode_data_to(&self, out: &mut Vec<u8>) {
        let body = self.body();
        out.reserve(<Self::DataTuple>::encoded_size(&body));
        out.extend(<Self::DataTuple as SolType>::encode(&body));
    }

    /// Encode the topics of this event into the given buffer.
    ///
    /// # Errors
    ///
    /// This method should only fail if the buffer is too small.
    fn encode_topics_raw(&self, out: &mut [WordToken]) -> Result<()>;

    /// ABI-encode the dynamic data of this event.
    #[inline]
    fn encode_data(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.encoded_size());
        self.encode_data_to(&mut out);
        out
    }

    /// Encode the topics of this event.
    ///
    /// The returned vector will have length `Self::TopicList::COUNT`.
    #[inline]
    fn encode_topics(&self) -> Vec<WordToken> {
        let mut out = vec![WordToken(B256::ZERO); Self::TopicList::COUNT];
        self.encode_topics_raw(&mut out).unwrap();
        out
    }

    /// Encode the topics of this event into a fixed-size array.
    ///
    /// # Panics
    ///
    /// This method will panic if `LEN` is not equal to
    /// `Self::TopicList::COUNT`.
    #[inline]
    fn encode_topics_array<const LEN: usize>(&self) -> [WordToken; LEN] {
        // TODO: make this a compile-time error when `const` blocks are stable
        assert_eq!(LEN, Self::TopicList::COUNT, "topic list length mismatch");
        let mut out = [WordToken(B256::ZERO); LEN];
        self.encode_topics_raw(&mut out).unwrap();
        out
    }

    /// Decode the topics of this event from the given data.
    fn decode_topics<I, D>(topics: I) -> Result<<Self::TopicList as SolType>::RustType>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        <Self::TopicList as TopicList>::detokenize(topics)
    }

    /// Decode the dynamic data of this event from the given buffer.
    fn decode_data(data: &[u8], validate: bool) -> Result<<Self::DataTuple as SolType>::RustType> {
        <Self::DataTuple as SolType>::decode(data, validate)
    }

    /// Decode the event from the given log info.
    fn decode_log<I, D>(topics: I, data: &[u8], validate: bool) -> Result<Self>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        let topics = Self::decode_topics(topics)?;
        let body = Self::decode_data(data, validate)?;
        Ok(Self::new(topics, body))
    }
}
