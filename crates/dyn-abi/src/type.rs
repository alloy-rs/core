use crate::{
    eip712::coerce::{self, coerce_custom_struct, coerce_custom_value},
    no_std_prelude::*,
    DynAbiError, DynSolValue, DynToken, Result, SolType, Word,
};
use alloy_sol_types::sol_data;

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructProp {
    name: String,
    ty: DynSolType,
}

/// A Dynamic SolType. Equivalent to an enum wrapper around all implementers of
/// [`crate::SolType`]. This is used to represent Solidity types that are not
/// known at compile time. It is used in conjunction with [`DynToken`] and
/// [`DynSolValue`] to allow for dynamic ABI encoding and decoding.
///
/// Users will generally want to instantiate via the [`std::str::FromStr`] impl
/// on [`DynSolType`]. This will parse a string into a [`DynSolType`].
/// User-defined types can be instantiated directly.
///
/// # Example
/// ```
/// # use alloy_dyn_abi::{DynSolType, DynSolValue, Result};
/// # use alloy_primitives::U256;
/// # pub fn main() -> Result<()> {
/// let my_type = DynSolType::Uint(256);
/// let my_data: DynSolValue = U256::from(183).into();
///
/// let encoded = my_type.encode_single(my_data.clone())?;
/// let decoded = my_type.decode_single(&encoded)?;
///
/// assert_eq!(decoded, my_data);
///
/// let my_type = DynSolType::Array(Box::new(DynSolType::Uint(256)));
/// let my_data = DynSolValue::Array(vec![my_data.clone()]);
///
/// let encoded = my_type.encode_single(my_data.clone())?;
/// let decoded = my_type.decode_single(&encoded)?;
///
/// assert_eq!(decoded, my_data);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynSolType {
    /// Address.
    Address,
    /// Dynamic bytes.
    Bytes,
    /// Signed Integer.
    Int(usize),
    /// Unsigned Integer.
    Uint(usize),
    /// Boolean.
    Bool,
    /// Dynamically sized array.
    Array(Box<DynSolType>),
    /// String.
    String,
    /// Fixed-size bytes, up to 32.
    FixedBytes(usize),
    /// Fixed-sized array.
    FixedArray(Box<DynSolType>, usize),
    /// Tuple.
    Tuple(Vec<DynSolType>),
    /// User-defined struct.
    CustomStruct {
        /// Name of the struct.
        name: String,
        /// Prop names.
        prop_names: Vec<String>,
        /// Inner types.
        tuple: Vec<DynSolType>,
    },
    /// User-defined value.
    CustomValue {
        /// Name of the value type.
        name: String,
    },
}

impl DynSolType {
    /// Dynamic tokenization.
    pub fn tokenize(&self, value: DynSolValue) -> Result<DynToken<'_>> {
        match (self, value) {
            (DynSolType::Address, DynSolValue::Address(val)) => {
                Ok(DynToken::Word(sol_data::Address::tokenize(&val).0))
            }
            (DynSolType::Bool, DynSolValue::Bool(val)) => {
                Ok(DynToken::Word(sol_data::Bool::tokenize(&val).0))
            }
            (DynSolType::Bytes, DynSolValue::Bytes(val)) => Ok(DynToken::PackedSeq(val)),
            (DynSolType::FixedBytes(len), DynSolValue::FixedBytes(word, size)) => {
                if size != *len {
                    return Err(crate::Error::custom(format!(
                        "Size mismatch for FixedBytes. Got {size}, expected {len}",
                    )))
                }
                Ok(word.into())
            }
            (DynSolType::Int(_), DynSolValue::Int(word, _)) => {
                Ok(DynToken::Word(word.to_be_bytes().into()))
            }
            (DynSolType::Uint(_), DynSolValue::Uint(num, _)) => {
                Ok(DynToken::Word(num.to_be_bytes().into()))
            }
            (DynSolType::String, DynSolValue::String(buf)) => Ok(DynToken::PackedSeq(buf.into())),
            (DynSolType::Tuple(types), DynSolValue::Tuple(tokens)) => {
                let tokens = types
                    .iter()
                    .zip(tokens)
                    .map(|(ty, token)| ty.tokenize(token))
                    .collect::<Result<_, _>>()?;

                Ok(DynToken::FixedSeq(tokens, types.len()))
            }
            (DynSolType::Array(t), DynSolValue::Array(values)) => {
                let contents: Vec<_> = values
                    .into_iter()
                    .map(|val| t.tokenize(val))
                    .collect::<Result<_, _>>()?;
                let template = Box::new(contents.first().unwrap().clone());
                Ok(DynToken::DynSeq {
                    contents: contents.into(),
                    template,
                })
            }
            (DynSolType::FixedArray(t, size), DynSolValue::FixedArray(tokens)) => {
                if *size != tokens.len() {
                    return Err(crate::Error::custom(format!(
                        "Size mismatch for FixedArray. Got {}, expected {}",
                        tokens.len(),
                        size,
                    )))
                }
                Ok(DynToken::FixedSeq(
                    tokens
                        .into_iter()
                        .map(|token| t.tokenize(token))
                        .collect::<Result<_, _>>()?,
                    *size,
                ))
            }
            (DynSolType::CustomStruct { name, tuple, .. }, DynSolValue::Tuple(tokens)) => {
                if tuple.len() != tokens.len() {
                    return Err(crate::Error::custom(format!(
                        "Tuple length mismatch for {}. Got {}, expected {}",
                        name,
                        tokens.len(),
                        tuple.len(),
                    )))
                }
                let len = tuple.len();
                let tuple = tuple
                    .iter()
                    .zip(tokens)
                    .map(|(ty, token)| ty.tokenize(token))
                    .collect::<Result<_, _>>()?;
                Ok(DynToken::FixedSeq(tuple, len))
            }
            (
                DynSolType::CustomStruct {
                    name,
                    tuple,
                    prop_names,
                },
                DynSolValue::CustomStruct {
                    name: name_val,
                    tuple: tuple_val,
                    prop_names: prop_names_val,
                },
            ) => {
                if *name != name_val {
                    return Err(crate::Error::custom(format!(
                        "Name mismatch for {name}. Got {name_val}, expected {name}",
                    )))
                }
                if *prop_names != prop_names_val {
                    return Err(crate::Error::custom(format!(
                        "Prop names mismatch for {name}. Got {prop_names_val:?}, expected {prop_names:?}",
                    )));
                }
                if tuple.len() != tuple_val.len() {
                    return Err(crate::Error::custom(format!(
                        "Tuple length mismatch for {}. Got {}, expected {}",
                        name,
                        tuple_val.len(),
                        tuple.len(),
                    )))
                }
                let len = tuple.len();
                let tuple = tuple
                    .iter()
                    .zip(tuple_val.into_iter())
                    .map(|(ty, token)| ty.tokenize(token))
                    .collect::<Result<_, _>>()?;
                Ok(DynToken::FixedSeq(tuple, len))
            }
            (
                DynSolType::CustomValue { name },
                DynSolValue::CustomValue {
                    name: name_val,
                    inner: inner_val,
                },
            ) => {
                if *name != name_val {
                    return Err(crate::Error::custom(format!(
                        "Name mismatch for {}. Got {}, expected {}",
                        name, name_val, name,
                    )))
                }
                // A little hacky. A Custom value type is always encoded as a full 32-byte word
                Ok(DynSolType::FixedBytes(32).tokenize(DynSolValue::FixedBytes(inner_val, 32))?)
            }
            _ => Err(crate::Error::custom("Invalid type on dynamic tokenization")),
        }
    }

    /// Dynamic detokenization.
    #[allow(clippy::unnecessary_to_owned)] // https://github.com/rust-lang/rust-clippy/issues/8148
    pub fn detokenize(&self, token: DynToken<'_>) -> Result<DynSolValue> {
        match (self, token) {
            (DynSolType::Address, DynToken::Word(word)) => Ok(DynSolValue::Address(
                sol_data::Address::detokenize(word.into()),
            )),
            (DynSolType::Bool, DynToken::Word(word)) => {
                Ok(DynSolValue::Bool(sol_data::Bool::detokenize(word.into())))
            }
            (DynSolType::Bytes, DynToken::PackedSeq(buf)) => Ok(DynSolValue::Bytes(buf)),
            (DynSolType::FixedBytes(size), DynToken::Word(word)) => Ok(DynSolValue::FixedBytes(
                sol_data::FixedBytes::<32>::detokenize(word.into()).into(),
                *size,
            )),
            // cheating here, but it's ok
            (DynSolType::Int(size), DynToken::Word(word)) => Ok(DynSolValue::Int(
                sol_data::Int::<256>::detokenize(word.into()),
                *size,
            )),
            (DynSolType::Uint(size), DynToken::Word(word)) => Ok(DynSolValue::Uint(
                sol_data::Uint::<256>::detokenize(word.into()),
                *size,
            )),

            (DynSolType::String, DynToken::PackedSeq(buf)) => Ok(DynSolValue::String(
                sol_data::String::detokenize(buf.as_slice().into()),
            )),
            (DynSolType::Tuple(types), DynToken::FixedSeq(tokens, _)) => {
                if types.len() != tokens.len() {
                    return Err(crate::Error::custom(
                        "tuple length mismatch on dynamic detokenization",
                    ))
                }
                Ok(DynSolValue::Tuple(
                    types
                        .iter()
                        .zip(tokens.into_owned())
                        .map(|(t, w)| t.detokenize(w))
                        .collect::<Result<_, _>>()?,
                ))
            }
            (DynSolType::Array(t), DynToken::DynSeq { contents, .. }) => Ok(DynSolValue::Array(
                contents
                    .into_owned()
                    .into_iter()
                    .map(|tok| t.detokenize(tok))
                    .collect::<Result<_, _>>()?,
            )),
            (DynSolType::FixedArray(t, size), DynToken::FixedSeq(tokens, _)) => {
                if *size != tokens.len() {
                    return Err(crate::Error::custom(
                        "array length mismatch on dynamic detokenization",
                    ))
                }
                Ok(DynSolValue::FixedArray(
                    tokens
                        .into_owned()
                        .into_iter()
                        .map(|tok| t.detokenize(tok))
                        .collect::<Result<_, _>>()?,
                ))
            }
            (
                DynSolType::CustomStruct {
                    name,
                    tuple,
                    prop_names,
                },
                DynToken::FixedSeq(tokens, len),
            ) => {
                if len != tokens.len() || len != tuple.len() {
                    return Err(crate::Error::custom(
                        "custom length mismatch on dynamic detokenization",
                    ))
                }
                let tuple = tuple
                    .iter()
                    .zip(tokens.into_owned())
                    .map(|(t, w)| t.detokenize(w))
                    .collect::<Result<_, _>>()?;

                Ok(DynSolValue::CustomStruct {
                    name: name.clone(),
                    prop_names: prop_names.clone(),
                    tuple,
                })
            }
            (DynSolType::CustomValue { .. }, token) => DynSolType::FixedBytes(32).detokenize(token),
            _ => Err(crate::Error::custom(
                "mismatched types on dynamic detokenization",
            )),
        }
    }

    /// Coerce a json value to a sol value via this type.
    pub fn coerce(&self, value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
        match self {
            DynSolType::Address => coerce::address(value),
            DynSolType::Bytes => coerce::bytes(value),
            DynSolType::Int(n) => coerce::int(*n, value),
            DynSolType::Uint(n) => coerce::uint(*n, value),
            DynSolType::Bool => coerce::bool(value),
            DynSolType::Array(inner) => coerce::array(inner, value),
            DynSolType::String => coerce::string(value),
            DynSolType::FixedBytes(n) => coerce::fixed_bytes(*n, value),
            DynSolType::FixedArray(inner, n) => coerce::fixed_array(inner, *n, value),
            DynSolType::Tuple(inner) => coerce::tuple(inner, value),
            DynSolType::CustomStruct {
                name,
                prop_names,
                tuple,
            } => coerce_custom_struct(name, prop_names, tuple, value),
            DynSolType::CustomValue { name } => coerce_custom_value(name, value),
        }
    }

    /// Instantiate an empty dyn token, to be decoded into.
    pub(crate) fn empty_dyn_token(&self) -> DynToken<'_> {
        match self {
            DynSolType::Address => DynToken::Word(Word::ZERO),
            DynSolType::Bool => DynToken::Word(Word::ZERO),
            DynSolType::Bytes => DynToken::PackedSeq(Vec::new()),
            DynSolType::FixedBytes(_) => DynToken::Word(Word::ZERO),
            DynSolType::Int(_) => DynToken::Word(Word::ZERO),
            DynSolType::Uint(_) => DynToken::Word(Word::ZERO),
            DynSolType::String => DynToken::PackedSeq(Vec::new()),
            DynSolType::Tuple(types) => DynToken::FixedSeq(
                types.iter().map(|t| t.empty_dyn_token()).collect(),
                types.len(),
            ),
            DynSolType::Array(t) => DynToken::DynSeq {
                contents: Default::default(),
                template: Box::new(t.empty_dyn_token()),
            },
            DynSolType::FixedArray(t, size) => {
                DynToken::FixedSeq(vec![t.empty_dyn_token(); *size].into(), *size)
            }
            DynSolType::CustomStruct { tuple, .. } => DynToken::FixedSeq(
                tuple.iter().map(|t| t.empty_dyn_token()).collect(),
                tuple.len(),
            ),
            DynSolType::CustomValue { .. } => DynToken::Word(Word::ZERO),
        }
    }

    /// Encode a single value. Fails if the value does not match this type.
    pub fn encode_single(&self, value: DynSolValue) -> Result<Vec<u8>> {
        let mut encoder = crate::Encoder::default();
        self.tokenize(value)?.encode_single(&mut encoder)?;
        Ok(encoder.into_bytes())
    }

    /// Decode a single value. Fails if the value does not match this type.
    pub fn decode_single(&self, data: &[u8]) -> Result<DynSolValue> {
        let mut decoder = crate::Decoder::new(data, false);
        let mut token = self.empty_dyn_token();
        token.decode_single_populate(&mut decoder)?;
        self.detokenize(token)
    }

    /// Encode a sequence of values. Fails if the values do not match this
    /// type. Is a no-op if this type or the values are not a sequence.
    pub fn encode_sequence(&self, values: DynSolValue) -> Result<Vec<u8>> {
        let mut encoder = crate::Encoder::default();
        self.tokenize(values)?.encode_sequence(&mut encoder)?;
        Ok(encoder.into_bytes())
    }

    /// Decode a sequence of values. Fails if the values do not match this type.
    pub fn decode_sequence(&self, data: &[u8]) -> Result<DynSolValue> {
        let mut decoder = crate::Decoder::new(data, false);
        let mut token = self.empty_dyn_token();
        token.decode_sequence_populate(&mut decoder)?;
        self.detokenize(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use alloy_primitives::Address;
    use serde_json::json;

    #[test]
    fn dynamically_encodes() {
        let word1 = "0000000000000000000000000101010101010101010101010101010101010101"
            .parse()
            .unwrap();
        let word2 = "0000000000000000000000000202020202020202020202020202020202020202"
            .parse()
            .unwrap();

        let sol_type = DynSolType::Address;
        let token = sol_type
            .tokenize(DynSolValue::Address(Address::repeat_byte(0x01)))
            .unwrap();
        assert_eq!(token, DynToken::from(word1));

        let sol_type = DynSolType::FixedArray(Box::new(DynSolType::Address), 2);
        let token = sol_type
            .tokenize(DynSolValue::FixedArray(vec![
                Address::repeat_byte(0x01).into(),
                Address::repeat_byte(0x02).into(),
            ]))
            .unwrap();
        assert_eq!(
            token,
            DynToken::FixedSeq(vec![DynToken::Word(word1), DynToken::Word(word2)].into(), 2)
        );
        let mut enc = crate::Encoder::default();
        token.encode_sequence(&mut enc).unwrap();
        assert_eq!(enc.finish(), vec![word1, word2]);
    }

    #[test]
    fn it_coerces() {
        let j = json!({
            "message": {
                "contents": "Hello, Bob!",
                "attachedMoneyInEth": 4.2,
                "from": {
                    "name": "Cow",
                    "wallets": [
                        "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
                        "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF",
                    ]
                },
                "to": [{
                    "name": "Bob",
                    "wallets": [
                        "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
                        "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57",
                        "0xB0B0b0b0b0b0B000000000000000000000000000",
                    ]
                }]
            }
        });

        let ty = DynSolType::CustomStruct {
            name: "Message".to_owned(),
            prop_names: vec!["contents".to_string(), "from".to_string(), "to".to_string()],
            tuple: vec![
                DynSolType::String,
                DynSolType::CustomStruct {
                    name: "Person".to_owned(),
                    prop_names: vec!["name".to_string(), "wallets".to_string()],
                    tuple: vec![
                        DynSolType::String,
                        DynSolType::Array(Box::new(DynSolType::Address)),
                    ],
                },
                DynSolType::Array(Box::new(DynSolType::CustomStruct {
                    name: "Person".to_owned(),
                    prop_names: vec!["name".to_string(), "wallets".to_string()],
                    tuple: vec![
                        DynSolType::String,
                        DynSolType::Array(Box::new(DynSolType::Address)),
                    ],
                })),
            ],
        };
        let top = j.as_object().unwrap().get("message").unwrap();

        assert_eq!(
            ty.coerce(top).unwrap(),
            DynSolValue::CustomStruct {
                name: "Message".to_owned(),
                prop_names: vec!["contents".to_string(), "from".to_string(), "to".to_string()],
                tuple: vec![
                    DynSolValue::String("Hello, Bob!".to_string()),
                    DynSolValue::CustomStruct {
                        name: "Person".to_owned(),
                        prop_names: vec!["name".to_string(), "wallets".to_string()],
                        tuple: vec![
                            DynSolValue::String("Cow".to_string()),
                            vec![
                                DynSolValue::Address(
                                    "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF"
                                        .parse()
                                        .unwrap()
                                ),
                            ]
                            .into()
                        ]
                    },
                    vec![DynSolValue::CustomStruct {
                        name: "Person".to_owned(),
                        prop_names: vec!["name".to_string(), "wallets".to_string()],
                        tuple: vec![
                            DynSolValue::String("Bob".to_string()),
                            vec![
                                DynSolValue::Address(
                                    "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xB0B0b0b0b0b0B000000000000000000000000000"
                                        .parse()
                                        .unwrap()
                                ),
                            ]
                            .into()
                        ]
                    }]
                    .into()
                ]
            }
        )
    }
}
