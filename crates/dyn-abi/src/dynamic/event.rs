use crate::{DynSolType, DynSolValue, Error, Result};
use alloc::vec::Vec;
use alloy_primitives::{B256, IntoLogData, Log, LogData};

/// A dynamic ABI event.
///
/// This is a representation of a Solidity event, which can be used to decode
/// logs.
#[derive(Clone, Debug, PartialEq)]
pub struct DynSolEvent {
    /// The event signature hash, if any.
    pub(crate) topic_0: Option<B256>,
    /// The indexed types.
    pub(crate) indexed: Vec<DynSolType>,
    /// The un-indexed types.
    pub(crate) body: DynSolType,
}

impl DynSolEvent {
    /// Creates a new event, without length-checking the indexed, or ensuring
    /// the body is a tuple. This allows creation of invalid events.
    pub const fn new_unchecked(
        topic_0: Option<B256>,
        indexed: Vec<DynSolType>,
        body: DynSolType,
    ) -> Self {
        Self { topic_0, indexed, body }
    }

    /// Creates a new event.
    ///
    /// Checks that the indexed length is less than or equal to 4, and that the
    /// body is a tuple.
    pub fn new(topic_0: Option<B256>, indexed: Vec<DynSolType>, body: DynSolType) -> Option<Self> {
        let topics = indexed.len() + topic_0.is_some() as usize;
        if topics > 4 || body.as_tuple().is_none() {
            return None;
        }
        Some(Self::new_unchecked(topic_0, indexed, body))
    }

    /// True if anonymous.
    pub const fn is_anonymous(&self) -> bool {
        self.topic_0.is_none()
    }

    /// Decode the event from the given log info.
    pub fn decode_log_parts<I>(&self, topics: I, data: &[u8]) -> Result<DecodedEvent>
    where
        I: IntoIterator<Item = B256>,
    {
        let mut topics = topics.into_iter();
        let num_topics = self.indexed.len() + !self.is_anonymous() as usize;

        match topics.size_hint() {
            (n, Some(m)) if n == m && n != num_topics => {
                return Err(Error::TopicLengthMismatch { expected: num_topics, actual: n });
            }
            _ => {}
        }

        // skip event hash if not anonymous
        if !self.is_anonymous() {
            let t = topics.next();
            match t {
                Some(sig) => {
                    let expected = self.topic_0.expect("not anonymous");
                    if sig != expected {
                        return Err(Error::EventSignatureMismatch { expected, actual: sig });
                    }
                }
                None => return Err(Error::TopicLengthMismatch { expected: num_topics, actual: 0 }),
            }
        }

        let not_anonymous = !self.is_anonymous() as usize;
        let indexed = self
            .indexed
            .iter()
            .enumerate()
            .map(|(i, ty)| match topics.next() {
                Some(topic) => Ok(ty.decode_event_topic(topic)),
                // Ran out of topics: report the count we actually have (event hash + `i`
                // indexed topics consumed so far) rather than letting the body decode below
                // fail with a confusing `Overrun`.
                None => Err(Error::TopicLengthMismatch {
                    expected: num_topics,
                    actual: not_anonymous + i,
                }),
            })
            .collect::<Result<_>>()?;

        // Validate the topic count before decoding the body. A log whose topic count does not
        // match this event (e.g. an ERC-721 `Transfer` decoded against the ERC-20 `Transfer`
        // ABI, which shares the same `topic0`) must surface as `TopicLengthMismatch`, not as an
        // `Overrun` from trying to read body words that the extra topics displaced. The early
        // `size_hint` check above only fires for exact-size iterators, so this also covers
        // iterators (`filter`/`map`/streaming) that don't report an exact length.
        let remaining = topics.count();
        if remaining > 0 {
            return Err(Error::TopicLengthMismatch {
                expected: num_topics,
                actual: num_topics + remaining,
            });
        }

        let body = self.body.abi_decode_sequence(data)?.into_fixed_seq().expect("body is a tuple");

        Ok(DecodedEvent { selector: self.topic_0, indexed, body })
    }

    /// Decode the event from the given log info.
    pub fn decode_log_data(&self, log: &LogData) -> Result<DecodedEvent> {
        self.decode_log_parts(log.topics().iter().copied(), &log.data)
    }

    /// Get the selector for this event, if any.
    pub const fn topic_0(&self) -> Option<B256> {
        self.topic_0
    }

    /// Get the indexed types.
    pub fn indexed(&self) -> &[DynSolType] {
        &self.indexed
    }

    /// Get the un-indexed types.
    pub fn body(&self) -> &[DynSolType] {
        self.body.as_tuple().expect("body is a tuple")
    }
}

/// A decoded dynamic ABI event.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedEvent {
    /// The hashes event_signature (if any)
    #[doc(alias = "topic_0")]
    pub selector: Option<B256>,
    /// The indexed values, in order.
    pub indexed: Vec<DynSolValue>,
    /// The un-indexed values, in order.
    pub body: Vec<DynSolValue>,
}

impl DecodedEvent {
    /// True if anonymous. False if not.
    pub const fn is_anonymous(&self) -> bool {
        self.selector.is_none()
    }

    /// Re-encode the event into a [`LogData`]
    pub fn encode_log_data(&self) -> LogData {
        debug_assert!(
            self.indexed.len() + !self.is_anonymous() as usize <= 4,
            "too many indexed values"
        );

        LogData::new_unchecked(
            self.selector
                .iter()
                .copied()
                .chain(self.indexed.iter().flat_map(DynSolValue::as_word))
                .collect(),
            DynSolValue::encode_seq(&self.body).into(),
        )
    }

    /// Transform a [`Log`] containing this event into a [`Log`] containing
    /// [`LogData`].
    pub fn encode_log(log: Log<Self>) -> Log<LogData> {
        Log { address: log.address, data: log.data.encode_log_data() }
    }
}

impl IntoLogData for DecodedEvent {
    fn to_log_data(&self) -> LogData {
        self.encode_log_data()
    }

    fn into_log_data(self) -> LogData {
        self.encode_log_data()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloy_primitives::{U256, address, b256, bytes};

    #[test]
    fn it_decodes_a_simple_log() {
        let log = LogData::new_unchecked(vec![], U256::ZERO.to_be_bytes_vec().into());
        let event = DynSolEvent {
            topic_0: None,
            indexed: vec![],
            body: DynSolType::Tuple(vec![DynSolType::Uint(256)]),
        };
        event.decode_log_data(&log).unwrap();
    }

    #[test]
    fn it_decodes_logs_with_indexed_params() {
        let t0 = b256!("0xcf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7");
        let log: LogData = LogData::new_unchecked(
            vec![t0, b256!("0x0000000000000000000000000000000000000000000000000000000000012321")],
            bytes!(
                "
			    0000000000000000000000000000000000000000000000000000000000012345
			    0000000000000000000000000000000000000000000000000000000000054321
			    "
            ),
        );
        let event = DynSolEvent {
            topic_0: Some(t0),
            indexed: vec![DynSolType::Address],
            body: DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Address,
                DynSolType::Address,
            ])]),
        };

        let decoded = event.decode_log_data(&log).unwrap();
        assert_eq!(
            decoded.indexed,
            vec![DynSolValue::Address(address!("0x0000000000000000000000000000000000012321"))]
        );

        let encoded = decoded.encode_log_data();
        assert_eq!(encoded, log);
    }

    // ERC-20 and ERC-721 `Transfer` share the same `topic0`, so a topic-only filter matches
    // both. Decoding an ERC-721 transfer (4 topics, empty data) against the ERC-20 ABI used to
    // fail with a confusing `Overrun` when the topic iterator didn't report an exact size,
    // because the body was decoded before the topic count was validated. See alloy#2243.
    #[test]
    fn topic_count_is_validated_before_body() {
        let t0 = b256!("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
        // event Transfer(address indexed from, address indexed to, uint256 value)
        let event = DynSolEvent {
            topic_0: Some(t0),
            indexed: vec![DynSolType::Address, DynSolType::Address],
            body: DynSolType::Tuple(vec![DynSolType::Uint(256)]),
        };

        // ERC-721-style log: 4 topics (sig + from + to + tokenId), empty data.
        let topics = [
            t0,
            b256!("0x000000000000000000000000d9a442856c234a39a81a089c06451ebaa4306a72"),
            b256!("0x0000000000000000000000002a3dd3eb832af982ec71669e178424b10dca2ede"),
            b256!("0x0000000000000000000000000000000000000000000000000000000000000290"),
        ];

        // Exact-size iterator: caught by the up-front `size_hint` check.
        let err = event.decode_log_parts(topics.iter().copied(), &[]).unwrap_err();
        assert!(
            matches!(err, Error::TopicLengthMismatch { expected: 3, actual: 4 }),
            "exact iterator: {err:?}"
        );

        // Non-exact iterator (e.g. `filter`): previously returned `Overrun`.
        let err = event.decode_log_parts(topics.iter().copied().filter(|_| true), &[]).unwrap_err();
        assert!(
            matches!(err, Error::TopicLengthMismatch { expected: 3, actual: 4 }),
            "non-exact iterator: {err:?}"
        );
    }

    #[test]
    fn too_few_topics_is_a_length_mismatch() {
        let t0 = b256!("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
        let event = DynSolEvent {
            topic_0: Some(t0),
            indexed: vec![DynSolType::Address, DynSolType::Address],
            body: DynSolType::Tuple(vec![DynSolType::Uint(256)]),
        };

        // Only sig + one indexed topic, supplied via a non-exact iterator.
        let topics =
            [t0, b256!("0x000000000000000000000000d9a442856c234a39a81a089c06451ebaa4306a72")];
        let err = event.decode_log_parts(topics.iter().copied().filter(|_| true), &[]).unwrap_err();
        assert!(matches!(err, Error::TopicLengthMismatch { expected: 3, actual: 2 }), "{err:?}");
    }
}
