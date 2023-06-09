use alloy_primitives::B256;

use crate::{token::TokenSeq, SolType};

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
    type BodyTuple: SolType<TokenType = Self::BodyToken>;

    /// The [`TokenSeq`] type corresponding to the tuple.
    type BodyToken: TokenSeq;

    /// The event's ABI signature.
    const SIGNATURE: &'static str;

    /// The keccak256 hash of the event's ABI signature. For non-anonymous
    /// events, this will be the topic0 of the event.
    const SIGNATURE_HASH: [u8; 32];

    /// True if the event is anonymous.
    const ANONYMOUS: bool;

    /// The number of topics.
    const TOPICS_LEN: usize;

    /// Type of Topic 0. This is typically a [`B256`] representing the event's
    /// signature. However, for anonymous events, this may be some other type.
    /// If an anonymous event has no indexed parameters, this will be the unit
    /// type `()`.
    type Topic0: SolType;
    /// Type of Topic 1. If the event does not have an indexed parameter at this
    /// position, this will be the unit type `()`.
    type Topic1: SolType;
    /// Type of Topic 2. If the event does not have an indexed parameter at this
    /// position, this will be the unit type `()`.
    type Topic2: SolType;
    /// Type of Topic 3. If the event does not have an indexed parameter at this
    /// position, this will be the unit type `()`.
    type Topic3: SolType;

    /// Converts to the tuple type used for ABI encoding and decoding.
    fn body_to_rust(&self) -> <Self::BodyTuple as SolType>::RustType;

    fn from_rust(
        topic0: <Self::Topic0 as SolType>::RustType,
        topic1: <Self::Topic1 as SolType>::RustType,
        topic2: <Self::Topic2 as SolType>::RustType,
        topic3: <Self::Topic3 as SolType>::RustType,
        body: <Self::BodyTuple as SolType>::RustType,
    ) -> Self;

    /// The size of the encoded body data in bytes.
    fn body_size(&self) -> usize;

    fn topic_0(&self) -> <Self::Topic0 as SolType>::RustType;
    fn topic_1(&self) -> <Self::Topic1 as SolType>::RustType;
    fn topic_2(&self) -> <Self::Topic2 as SolType>::RustType;
    fn topic_3(&self) -> <Self::Topic3 as SolType>::RustType;
}

mod example {

    use alloy_primitives::{keccak256, U256};

    use crate::{sol_data, SolEvent, SolType};

    //  event MyEvent(bytes32 indexed a, uint256 b, string indexed c, bytes d);
    pub struct MyEvent {
        pub a: [u8; 32],
        pub b: U256,
        pub c: String,
        pub d: Vec<u8>,
    }

    impl SolEvent for MyEvent {
        type BodyTuple = (sol_data::Uint<256>, sol_data::String, sol_data::Bytes);
        type BodyToken = (
            <sol_data::Uint<256> as SolType>::TokenType,
            <sol_data::String as SolType>::TokenType,
            <sol_data::Bytes as SolType>::TokenType,
        );
        const SIGNATURE: &'static str = "MyEvent(bytes32,uint256,string,bytes)";
        const SIGNATURE_HASH: [u8; 32] = [0; 32]; // FIXME
        const ANONYMOUS: bool = false;
        const TOPICS_LEN: usize = 3;
        type Topic0 = sol_data::FixedBytes<32>; // Signature hash
        type Topic1 = sol_data::FixedBytes<32>; // Indexed bytes32 a
        type Topic2 = sol_data::FixedBytes<32>; // Hash of indexed string c
        type Topic3 = (); // no 4th topic

        fn body_to_rust(&self) -> <Self::BodyTuple as SolType>::RustType {
            (self.b, self.c.clone(), self.d.clone())
        }

        fn from_rust(
            topic0: <Self::Topic0 as SolType>::RustType,
            topic1: <Self::Topic1 as SolType>::RustType,
            topic2: <Self::Topic2 as SolType>::RustType,
            topic3: <Self::Topic3 as SolType>::RustType,
            body: <Self::BodyTuple as SolType>::RustType,
        ) -> Self {
            Self {
                a: topic1,
                b: body.0,
                c: body.1,
                d: body.2,
            }
        }

        fn body_size(&self) -> usize {
            0
            // FIXME: as data_size for error.
        }

        fn topic_0(&self) -> <Self::Topic0 as SolType>::RustType {
            Self::SIGNATURE_HASH
        }

        fn topic_1(&self) -> <Self::Topic1 as SolType>::RustType {
            self.a
        }

        fn topic_2(&self) -> <Self::Topic2 as SolType>::RustType {
            keccak256(self.c.as_bytes()).into()
        }

        fn topic_3(&self) -> <Self::Topic3 as SolType>::RustType {
            ()
        }
    }
}
