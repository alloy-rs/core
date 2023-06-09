use alloy_primitives::FixedBytes;

use crate::{
    token::{TokenSeq, WordToken},
    Result, SolType,
};

use sealed::Sealed;

/// A `TopicList` represents the topics of a Solidity event. A topic list may
/// be 0-4 elements. Topics are included in log
///
/// This trait is sealed to prevent incorrect downstream implementations of
/// `TopicList` from being created.
pub trait TopicList: SolType + Sealed {
    /// The number of topics
    const COUNT: usize;

    /// Detokenize the topics into a tuple of rust types.
    ///
    /// This function accepts an iterator of `WordToken`
    fn detokenize<I, D>(topics: I) -> Self::RustType
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>;
}

impl TopicList for () {
    const COUNT: usize = 0;

    fn detokenize<I, D>(_topics: I) -> Self::RustType
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
    }
}

impl<T> TopicList for (T,)
where
    T: SolType<TokenType = WordToken>,
{
    const COUNT: usize = 1;

    fn detokenize<I, D>(topics: I) -> Self::RustType
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        let mut iter = topics.into_iter().map(Into::into);
        let topic0 = T::detokenize(iter.next().unwrap_or_default()).unwrap();

        (topic0,)
    }
}

impl<T, U> TopicList for (T, U)
where
    T: SolType<TokenType = WordToken>,
    U: SolType<TokenType = WordToken>,
{
    const COUNT: usize = 2;

    fn detokenize<I, D>(topics: I) -> Self::RustType
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        let mut iter: core::iter::Map<<I as IntoIterator>::IntoIter, _> =
            topics.into_iter().map(Into::into);
        let topic0 = T::detokenize(iter.next().unwrap_or_default()).unwrap();
        let topic1 = U::detokenize(iter.next().unwrap_or_default()).unwrap();
        (topic0, topic1)
    }
}

impl<T, U, V> TopicList for (T, U, V)
where
    T: SolType<TokenType = WordToken>,
    U: SolType<TokenType = WordToken>,
    V: SolType<TokenType = WordToken>,
{
    const COUNT: usize = 3;

    fn detokenize<I, D>(topics: I) -> Self::RustType
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        let mut iter = topics.into_iter().map(Into::into);
        let topic0 = T::detokenize(iter.next().unwrap_or_default()).unwrap();
        let topic1 = U::detokenize(iter.next().unwrap_or_default()).unwrap();
        let topic2 = V::detokenize(iter.next().unwrap_or_default()).unwrap();
        (topic0, topic1, topic2)
    }
}

impl<T, U, V, W> TopicList for (T, U, V, W)
where
    T: SolType<TokenType = WordToken>,
    U: SolType<TokenType = WordToken>,
    V: SolType<TokenType = WordToken>,
    W: SolType<TokenType = WordToken>,
{
    const COUNT: usize = 4;

    fn detokenize<I, D>(topics: I) -> Self::RustType
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        let mut iter = topics.into_iter().map(Into::into);
        let topic0 = T::detokenize(iter.next().unwrap_or_default()).unwrap();
        let topic1 = U::detokenize(iter.next().unwrap_or_default()).unwrap();
        let topic2 = V::detokenize(iter.next().unwrap_or_default()).unwrap();
        let topic3 = W::detokenize(iter.next().unwrap_or_default()).unwrap();
        (topic0, topic1, topic2, topic3)
    }
}

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
    /// These are ABI encoded and included in the log struct returned by the
    /// RPC node. Complex and dynamic indexed parameters are encoded according
    /// to [special rules] and then hashed
    ///
    /// [special rules]: https://docs.soliditylang.org/en/v0.8.18/abi-spec.html#indexed-event-encoding
    type TopicList: TopicList;

    /// The event's ABI signature. For anonymous events, this is unused, but is
    /// still present.
    const SIGNATURE: &'static str;

    /// The keccak256 hash of the event's ABI signature. For non-anonymous
    /// events, this will be the topic0 of the event. For anonymous events, this
    /// is unused, but is still present.
    ///
    /// Also called the event `selector`
    const SIGNATURE_HASH: FixedBytes<32>;

    /// True if the event is anonymous.
    const ANONYMOUS: bool;

    /// The number of topics.
    const TOPICS_LEN: usize;

    /// Decode the body of this event from the given data. The event body
    /// contains the non-indexed parameters.
    fn decode_body(data: &[u8], validate: bool) -> Result<<Self::DataTuple as SolType>::RustType> {
        <Self::DataTuple as SolType>::decode(data, validate)
    }

    /// Encode the body of this event.
    fn encode_data(&self) -> Vec<u8>;

    /// Decode the topics of this event from the given data. The topics contain
    /// the selector (for non-anonymous events) and indexed parameters.
    fn decode_topics<I, D>(topics: I) -> <Self::TopicList as SolType>::RustType
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        <Self::TopicList as TopicList>::detokenize(topics)
    }

    /// The size of the encoded body data in bytes.
    fn data_size(&self) -> usize;

    /// Convert decoded rust data to the event type.
    fn new(
        topics: <Self::TopicList as SolType>::RustType,
        body: <Self::DataTuple as SolType>::RustType,
    ) -> Self;

    /// Decode the event from the given log info.
    fn decode_log<I, D>(topics: I, body_data: &[u8], validate: bool) -> Result<Self>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        let topics = Self::decode_topics(topics);
        let body = Self::decode_body(body_data, validate)?;

        Ok(Self::new(topics, body))
    }
}

mod sealed {
    use super::*;

    pub trait Sealed {}
    impl Sealed for () {}
    impl<T> Sealed for (T,) where T: SolType<TokenType = WordToken> {}
    impl<T, U> Sealed for (T, U)
    where
        T: SolType<TokenType = WordToken>,
        U: SolType<TokenType = WordToken>,
    {
    }
    impl<T, U, V> Sealed for (T, U, V)
    where
        T: SolType<TokenType = WordToken>,
        U: SolType<TokenType = WordToken>,
        V: SolType<TokenType = WordToken>,
    {
    }
    impl<T, U, V, W> Sealed for (T, U, V, W)
    where
        T: SolType<TokenType = WordToken>,
        U: SolType<TokenType = WordToken>,
        V: SolType<TokenType = WordToken>,
        W: SolType<TokenType = WordToken>,
    {
    }
}

#[cfg(test)]
mod compile_test {
    use alloy_primitives::{FixedBytes, U256};

    use crate::{sol_data, SolEvent, SolType};

    #[allow(unreachable_pub, dead_code)]
    ///  event MyEvent(bytes32 indexed a, uint256 b, string indexed c, bytes d);
    struct MyEvent {
        /// bytes indexed a
        pub a: [u8; 32],
        /// uint256 b
        pub b: U256,
        /// string indexed c
        pub hash_c: [u8; 32],
        /// bytes d
        pub d: Vec<u8>,
    }

    impl SolEvent for MyEvent {
        type DataTuple = (sol_data::Uint<256>, sol_data::Bytes);
        ///
        type DataToken = (
            <sol_data::Uint<256> as SolType>::TokenType,
            <sol_data::Bytes as SolType>::TokenType,
        );

        // this is a, and keccak256(c)
        type TopicList = (
            sol_data::FixedBytes<32>,
            sol_data::FixedBytes<32>,
            sol_data::FixedBytes<32>,
        );

        const SIGNATURE: &'static str = "MyEvent(bytes32,uint256,string,bytes)";
        const SIGNATURE_HASH: FixedBytes<32> = FixedBytes([0; 32]); // FIXME: caluclate it
        const ANONYMOUS: bool = false;
        const TOPICS_LEN: usize = 3;

        fn data_size(&self) -> usize {
            0
            // FIXME: as data_size for error.
        }

        fn encode_data(&self) -> Vec<u8> {
            todo!()
        }

        fn new(
            topics: <Self::TopicList as SolType>::RustType,
            body: <Self::DataTuple as SolType>::RustType,
        ) -> Self {
            Self {
                a: topics.1,
                b: body.0,
                hash_c: topics.2,
                d: body.1,
            }
        }
    }
}
