use crate::{DynSolValue, DynToken, Error, ResolveSolType, Result};
use alloc::vec::Vec;
use alloy_json_abi::Event;
use alloy_primitives::{Log, B256};
use alloy_sol_types::Decoder;

mod sealed {
    pub trait Sealed {}
    impl Sealed for alloy_json_abi::Event {}
}
use sealed::Sealed;

/// Provides event encoding and decoding for the [`Event`] type.
///
/// This trait is sealed and cannot be implemented for types outside of this
/// crate. It is implemented only for [`Event`].
pub trait EventExt: Sealed {
    /// Decodes the given log info according to this item's input types.
    ///
    /// The `topics` parameter is the list of indexed topics, and the `data`
    /// parameter is the non-indexed data.
    ///
    /// The first topic is skipped, unless the event is anonymous.
    ///
    /// For more details, see the [Solidity reference][ref].
    ///
    /// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#encoding-of-indexed-event-parameters
    ///
    /// # Errors
    ///
    /// This function will return an error if the decoded data does not match
    /// the expected input types.
    fn decode_log<I>(&self, topics: I, data: &[u8], validate: bool) -> Result<Vec<DynSolValue>>
    where
        I: IntoIterator<Item = B256>;

    /// Decodes the given log object according to this item's input types.
    ///
    /// See [`decode_log`](EventExt::decode_log).
    #[inline]
    fn decode_log_object(&self, log: &Log, validate: bool) -> Result<Vec<DynSolValue>> {
        self.decode_log(log.topics.iter().copied(), &log.data, validate)
    }
}

impl EventExt for Event {
    fn decode_log<I>(&self, topics: I, data: &[u8], validate: bool) -> Result<Vec<DynSolValue>>
    where
        I: IntoIterator<Item = B256>,
    {
        let mut topics = topics.into_iter();

        // early exit if the number of topics does not match
        let num_topics = self.num_topics();
        if validate {
            match topics.size_hint() {
                (n, Some(m)) if n == m && n != num_topics => {
                    return Err(Error::TopicLengthMismatch {
                        expected: num_topics,
                        actual: n,
                    })
                }
                _ => {}
            }
        }

        // skip event hash if not anonymous
        if !self.anonymous {
            if let Some(sig) = topics.next() {
                if validate {
                    let expected = self.selector();
                    if sig != expected {
                        return Err(Error::EventSignatureMismatch {
                            expected,
                            actual: sig,
                        })
                    }
                }
            } else if validate {
                return Err(Error::TopicLengthMismatch {
                    expected: num_topics,
                    actual: 0,
                })
            };
        }

        let mut values = Vec::with_capacity(self.inputs.len());
        let mut decoder = Decoder::new(data, validate);
        let mut actual_topic_count = !self.anonymous as usize;
        for param in &self.inputs {
            let ty = param.resolve()?;
            let value = if param.indexed {
                actual_topic_count += 1;
                match topics.next() {
                    Some(topic) => Ok(ty.decode_event_topic(topic)),
                    None => Err(Error::TopicLengthMismatch {
                        expected: num_topics,
                        actual: actual_topic_count - 1,
                    }),
                }
            } else {
                ty._decode(&mut decoder, DynToken::decode_single_populate)
            }?;
            values.push(value);
        }

        if validate {
            let remaining = topics.count();
            if remaining > 0 {
                return Err(Error::TopicLengthMismatch {
                    expected: num_topics,
                    actual: num_topics + remaining,
                })
            }
        }

        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_json_abi::EventParam;
    use alloy_primitives::{address, b256, bytes, hex, keccak256, Signed};

    #[test]
    fn empty() {
        let mut event = Event {
            name: "MyEvent".into(),
            inputs: vec![],
            anonymous: false,
        };

        // skips over hash
        let values = event.decode_log(None, &[], false).unwrap();
        assert!(values.is_empty());

        // but if we validate, we get an error
        let err = event.decode_log(None, &[], true).unwrap_err();
        assert_eq!(
            err,
            Error::TopicLengthMismatch {
                expected: 1,
                actual: 0
            }
        );

        let values = event
            .decode_log(Some(keccak256("MyEvent()")), &[], true)
            .unwrap();
        assert!(values.is_empty());

        event.anonymous = true;
        let values = event.decode_log(None, &[], false).unwrap();
        assert!(values.is_empty());
        let values = event.decode_log(None, &[], true).unwrap();
        assert!(values.is_empty());
    }

    // https://github.com/rust-ethereum/ethabi/blob/b1710adc18f5b771d2d2519c87248b1ba9430778/ethabi/src/event.rs#L192
    #[test]
    fn test_decoding_event() {
        let event = Event {
            name: "foo".into(),
            inputs: vec![
                EventParam {
                    ty: "int256".into(),
                    indexed: false,
                    ..Default::default()
                },
                EventParam {
                    ty: "int256".into(),
                    indexed: true,
                    ..Default::default()
                },
                EventParam {
                    ty: "address".into(),
                    indexed: false,
                    ..Default::default()
                },
                EventParam {
                    ty: "address".into(),
                    indexed: true,
                    ..Default::default()
                },
                EventParam {
                    ty: "string".into(),
                    indexed: true,
                    ..Default::default()
                },
                EventParam {
                    ty: "int256[]".into(),
                    indexed: true,
                    ..Default::default()
                },
                EventParam {
                    ty: "address[5]".into(),
                    indexed: true,
                    ..Default::default()
                },
            ],
            anonymous: false,
        };

        let result = event
            .decode_log(
                [
                    b256!("0000000000000000000000000000000000000000000000000000000000000000"),
                    b256!("0000000000000000000000000000000000000000000000000000000000000002"),
                    b256!("0000000000000000000000001111111111111111111111111111111111111111"),
                    b256!("00000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                    b256!("00000000000000000bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
                    b256!("00000000000000000ccccccccccccccccccccccccccccccccccccccccccccccc"),
                ],
                &hex!(
                    "
                    0000000000000000000000000000000000000000000000000000000000000003
                    0000000000000000000000002222222222222222222222222222222222222222
                "
                ),
                false,
            )
            .unwrap();

        assert_eq!(
            result,
            [
                DynSolValue::Int(
                    Signed::from_be_bytes(hex!(
                        "0000000000000000000000000000000000000000000000000000000000000003"
                    )),
                    256
                ),
                DynSolValue::Int(
                    Signed::from_be_bytes(hex!(
                        "0000000000000000000000000000000000000000000000000000000000000002"
                    )),
                    256
                ),
                DynSolValue::Address(address!("2222222222222222222222222222222222222222")),
                DynSolValue::Address(address!("1111111111111111111111111111111111111111")),
                DynSolValue::FixedBytes(
                    b256!("00000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                    32
                ),
                DynSolValue::FixedBytes(
                    b256!("00000000000000000bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
                    32
                ),
                DynSolValue::FixedBytes(
                    b256!("00000000000000000ccccccccccccccccccccccccccccccccccccccccccccccc"),
                    32
                ),
            ]
        );
    }

    #[test]
    fn parse_log_whole() {
        let correct_event = Event {
            name: "Test".into(),
            inputs: vec![
                EventParam {
                    ty: "(address,address)".into(),
                    indexed: false,
                    ..Default::default()
                },
                EventParam {
                    ty: "address".into(),
                    indexed: true,
                    ..Default::default()
                },
            ],
            anonymous: false,
        };
        // swap indexed params
        let mut wrong_event = correct_event.clone();
        wrong_event.inputs[0].indexed = true;
        wrong_event.inputs[1].indexed = false;

        let log = Log::new(
            vec![
                b256!("cf74b4e62f836eeedcd6f92120ffb5afea90e6fa490d36f8b81075e2a7de0cf7"),
                b256!("0000000000000000000000000000000000000000000000000000000000012321"),
            ],
            bytes!(
                "
			0000000000000000000000000000000000000000000000000000000000012345
			0000000000000000000000000000000000000000000000000000000000054321
			"
            ),
        );

        wrong_event.decode_log_object(&log, false).unwrap();
        // TODO: How do we verify here?
        // wrong_event.decode_log_object(&log, true).unwrap_err();
        correct_event.decode_log_object(&log, false).unwrap();
        correct_event.decode_log_object(&log, true).unwrap();
    }
}
