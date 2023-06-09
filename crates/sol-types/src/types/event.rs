use crate::{
    token::{TokenSeq, WordToken},
    Error, Result, SolType,
};
use alloc::{borrow::Cow, vec::Vec};
use alloy_primitives::FixedBytes;
use sealed::Sealed;

/// A `TopicList` represents the topics of a Solidity event. A topic list may
/// be 0-4 elements. Topics are included in log
///
/// This trait is sealed to prevent incorrect downstream implementations of
/// `TopicList` from being created.
pub trait TopicList: SolType + Sealed {
    /// The number of topics.
    const COUNT: usize;

    /// Detokenize the topics into a tuple of rust types.
    ///
    /// This function accepts an iterator of `WordToken`.
    fn detokenize<I, D>(topics: I) -> Result<Self::RustType>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! impl_for_tuples {
    ($($c:literal => $($t:ident),*;)+) => {$(
        impl<$($t: SolType<TokenType = WordToken>,)*> sealed::Sealed for ($($t,)*) {}
        impl<$($t: SolType<TokenType = WordToken>,)*> TopicList for ($($t,)*) {
            const COUNT: usize = $c;

            fn detokenize<I, D>(topics: I) -> Result<Self::RustType>
            where
                I: IntoIterator<Item = D>,
                D: Into<WordToken>
            {
                let err = || Error::Other(Cow::Borrowed("topic list length mismatch"));
                let mut iter = topics.into_iter().map(Into::into);
                Ok(($(
                    iter.next().ok_or_else(err).map(<$t>::detokenize)?,
                )*))
            }
        }
    )+};
}

impl sealed::Sealed for () {}
impl TopicList for () {
    const COUNT: usize = 0;

    #[inline]
    fn detokenize<I, D>(_: I) -> Result<Self::RustType>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        Ok(())
    }
}

impl_for_tuples! {
    1 => T;
    2 => T, U;
    3 => T, U, V;
    4 => T, U, V, W;
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
    ///
    /// These are ABI encoded and included in the log struct returned by the
    /// RPC node.
    ///
    /// Complex and dynamic indexed parameters are encoded according to [special
    /// rules] and then hashed.
    ///
    /// [special rules]: https://docs.soliditylang.org/en/v0.8.18/abi-spec.html#indexed-event-encoding
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

    /// The number of topics.
    const TOPICS_LEN: usize;

    /// Convert decoded rust data to the event type.
    fn new(
        topics: <Self::TopicList as SolType>::RustType,
        body: <Self::DataTuple as SolType>::RustType,
    ) -> Self;

    /// The size of the ABI-encoded dynamic data in bytes.
    fn data_size(&self) -> usize;

    /// ABI-encode the dynamic data of this event into the given buffer.
    fn encode_data_raw(&self, out: &mut Vec<u8>);

    /// ABI-encode the dynamic data of this event.
    #[inline]
    fn encode_data(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.data_size());
        self.encode_data_raw(&mut out);
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

    /// Decode the dynamic data of this event from the given .
    fn decode_data(data: &[u8], validate: bool) -> Result<<Self::DataTuple as SolType>::RustType> {
        <Self::DataTuple as SolType>::decode(data, validate)
    }

    /// Decode the event from the given log info.
    fn decode_log<I, D>(topics: I, body_data: &[u8], validate: bool) -> Result<Self>
    where
        I: IntoIterator<Item = D>,
        D: Into<WordToken>,
    {
        let topics = Self::decode_topics(topics)?;
        let body = Self::decode_data(body_data, validate)?;
        Ok(Self::new(topics, body))
    }
}

#[cfg(test)]
#[allow(clippy::all, unused)]
mod compile_test {
    use super::*;
    use crate::{sol_data, SolEvent, SolType};
    use alloy_primitives::{FixedBytes, U256};
    use hex_literal::hex;

    // event MyEvent(bytes32 indexed a, uint256 b, string indexed c, bytes d);
    struct MyEvent {
        a: [u8; 32],
        b: U256,
        c: String,
        d: Vec<u8>,
    }

    impl SolEvent for MyEvent {
        type DataTuple = (sol_data::Uint<256>, sol_data::String, sol_data::Bytes);
        type DataToken = (
            <sol_data::Uint<256> as SolType>::TokenType,
            <sol_data::String as SolType>::TokenType,
            <sol_data::Bytes as SolType>::TokenType,
        );

        // this is `a`, and `keccak256(c)`
        type TopicList = (sol_data::FixedBytes<32>, sol_data::FixedBytes<32>);

        const SIGNATURE: &'static str = "MyEvent(bytes32,uint256,string,bytes)";
        const SIGNATURE_HASH: FixedBytes<32> = FixedBytes(hex!(
            "a0361149ae231b28afd460baabadd0a64949fcaed9a1d488cbf2747a9a8be6dd"
        ));
        const ANONYMOUS: bool = false;
        const TOPICS_LEN: usize = 3;

        fn data_size(&self) -> usize {
            // abi encoded length of: b + c + d
            32 + (64 + (self.c.len() / 31) * 32) + (64 + (self.d.len() / 31) * 32)
        }

        fn encode_data_raw(&self, out: &mut Vec<u8>) {
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
