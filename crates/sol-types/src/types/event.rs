use alloy_primitives::FixedBytes;

use crate::{
    token::{TokenSeq, WordToken},
    Result, SolType, Word,
};

trait TopicList: SolType + sealed::Sealed {
    const COUNT: usize;

    fn detokenize<'a>(topics: impl IntoIterator<Item = &'a WordToken>) -> Self::RustType;
}

impl TopicList for () {
    const COUNT: usize = 0;

    fn detokenize<'a>(topics: impl IntoIterator<Item = &'a WordToken>) -> Self::RustType {
        ()
    }
}

impl<T> TopicList for (T,)
where
    T: SolType<TokenType = WordToken>,
{
    const COUNT: usize = 1;

    fn detokenize<'a>(topics: impl IntoIterator<Item = &'a WordToken>) -> Self::RustType {
        let mut iter = topics.into_iter().copied();
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

    fn detokenize<'a>(topics: impl IntoIterator<Item = &'a WordToken>) -> Self::RustType {
        let mut iter = topics.into_iter().copied();
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

    fn detokenize<'a>(topics: impl IntoIterator<Item = &'a WordToken>) -> Self::RustType {
        let mut iter = topics.into_iter().copied();
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

    fn detokenize<'a>(topics: impl IntoIterator<Item = &'a WordToken>) -> Self::RustType {
        let mut iter = topics.into_iter().copied();
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
    /// The underlying tuple type which represents this event's non-indexed and
    /// dynamically-sized event parameters. These parameters are ABI encoded
    /// and included in the log body.
    ///
    /// If this event has no non-indexed and no dynamically-sized parameters,
    /// this will be the unit type `()`.
    type DataTuple: SolType<TokenType = Self::DataToken>;

    /// The [`TokenSeq`] type corresponding to the tuple.
    type DataToken: TokenSeq;

    /// The underlying tuple type which represents this event's topics.
    /// These are ABI encoded and included in the log structs
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

    /// Decode the body of this event from the given data.
    fn decode_body(data: &[u8], validate: bool) -> Result<<Self::DataTuple as SolType>::RustType> {
        <Self::DataTuple as SolType>::decode(data, validate)
    }

    /// Encode the body of this event.
    fn encode_body(&self) -> Vec<u8>;

    /// Decode the topics of this event from the given data.
    fn decode_topics<'a>(
        topics: impl IntoIterator<Item = &'a WordToken>,
    ) -> <Self::TopicList as SolType>::RustType {
        <Self::TopicList as TopicList>::detokenize(topics)
    }

    /// The size of the encoded body data in bytes.
    fn body_size(&self) -> usize;

    /// Convert decoded rust data to the event type.
    fn new(
        topics: <Self::TopicList as SolType>::RustType,
        body: <Self::DataTuple as SolType>::RustType,
    ) -> Self;

    fn decode_log<'a>(
        topics: impl IntoIterator<Item = &'a WordToken>,
        body_data: &[u8],
        validate: bool,
    ) -> Result<Self> {
        let topics = Self::decode_topics(topics);
        let body = Self::decode_body(body_data, validate)?;

        Ok(Self::new(topics, body))
    }
}

mod example {
    use alloy_primitives::{FixedBytes, U256};

    use crate::{sol_data, SolEvent, SolType};

    //  event MyEvent(bytes32 indexed a, uint256 b, string indexed c, bytes d);
    pub struct MyEvent {
        pub a: [u8; 32],
        pub b: U256,
        pub c: String,
        pub d: Vec<u8>,
    }

    impl SolEvent for MyEvent {
        type DataTuple = (sol_data::Uint<256>, sol_data::String, sol_data::Bytes);
        type DataToken = (
            <sol_data::Uint<256> as SolType>::TokenType,
            <sol_data::String as SolType>::TokenType,
            <sol_data::Bytes as SolType>::TokenType,
        );

        // this is a, and keccak256(c)
        type TopicList = (sol_data::FixedBytes<32>, sol_data::FixedBytes<32>);

        const SIGNATURE: &'static str = "MyEvent(bytes32,uint256,string,bytes)";
        const SIGNATURE_HASH: FixedBytes<32> = FixedBytes([0; 32]); // FIXME: caluclate it
        const ANONYMOUS: bool = false;
        const TOPICS_LEN: usize = 3;

        fn body_size(&self) -> usize {
            0
            // FIXME: as data_size for error.
        }

        fn encode_body(&self) -> Vec<u8> {
            todo!()
        }

        fn new(
            topics: <Self::TopicList as SolType>::RustType,
            body: <Self::DataTuple as SolType>::RustType,
        ) -> Self {
            Self {
                a: topics.1,
                b: body.0,
                c: body.1,
                d: body.2,
            }
        }
    }
}

mod sealed {
    use super::*;

    pub(crate) trait Sealed {}
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
