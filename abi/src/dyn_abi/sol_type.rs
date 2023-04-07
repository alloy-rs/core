use crate::{
    dyn_abi::{DynSolValue, DynToken},
    sol_type, AbiResult, SolType, Word,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Dynamic SolType. Equivalent to an enum wrapper around all implementers of
/// [`crate::SolType`]. This is used to represent solidity types that are not
/// known at compile time. It is used in conjunction with [`DynToken`] and
/// [`DynSolValue`] to allow for dynamic ABI encoding and decoding.
///
/// Users will generally want to instantiate via the [`crate::dyn_abi::parse`]
/// function. This will parse a string into a [`DynSolType`]. User-defined
/// types be instantiated directly.
///
/// # Example
/// ```
/// # use ethers_abi_enc::{DynSolType, DynToken, DynSolValue, AbiResult};
/// # use ethers_primitives::U256;
/// # pub fn main() -> AbiResult<()> {
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
pub enum DynSolType {
    /// Address
    Address,
    /// Dynamic bytes
    Bytes,
    /// Signed Integer
    Int(usize),
    /// Unsigned Integer
    Uint(usize),
    /// Boolean
    Bool,
    /// Dynamically sized array
    Array(Box<DynSolType>),
    /// String
    String,
    /// Fixed-size bytes, up to 32
    FixedBytes(usize),
    /// Fixed-sized array
    FixedArray(Box<DynSolType>, usize),
    /// Tuple
    Tuple(Vec<DynSolType>),
    /// User-defined struct
    CustomStruct {
        /// Name of the struct
        name: String,
        // TODO: names?
        /// Inner types
        tuple: Vec<DynSolType>,
    },
    /// User-defined value
    CustomValue {
        /// Name of the value type
        name: String,
    },
}

impl DynSolType {
    /// Dynamic tokenization
    pub fn tokenize(&self, value: DynSolValue) -> AbiResult<DynToken> {
        match (self, value) {
            (DynSolType::Address, DynSolValue::Address(val)) => {
                Ok(DynToken::Word(sol_type::Address::tokenize(val).inner()))
            }
            (DynSolType::Bool, DynSolValue::Bool(val)) => {
                Ok(DynToken::Word(sol_type::Bool::tokenize(val).inner()))
            }
            (DynSolType::Bytes, DynSolValue::Bytes(val)) => Ok(DynToken::PackedSeq(val)),
            (DynSolType::FixedBytes(len), DynSolValue::FixedBytes(word, size)) => {
                if size != *len {
                    return Err(crate::Error::custom_owned(format!(
                        "Size mismatch for FixedBytes. Got {}, expected {}",
                        size, len
                    )));
                }
                Ok(word.into())
            }
            (DynSolType::Int(_), DynSolValue::Int(word, _)) => Ok(word.into()),
            (DynSolType::Uint(_), DynSolValue::Uint(num, _)) => Ok(DynToken::Word(num.into())),
            (DynSolType::String, DynSolValue::String(buf)) => Ok(DynToken::PackedSeq(buf.into())),
            (DynSolType::Tuple(types), DynSolValue::Tuple(tokens)) => {
                let tokens = types
                    .iter()
                    .zip(tokens.into_iter())
                    .map(|(ty, token)| ty.tokenize(token))
                    .collect::<Result<_, _>>()?;

                Ok(DynToken::FixedSeq(tokens, types.len()))
            }
            (DynSolType::Array(t), DynSolValue::Array(values)) => {
                let contents: Vec<DynToken> = values
                    .into_iter()
                    .map(|val| t.tokenize(val))
                    .collect::<Result<_, _>>()?;
                let template = Box::new(contents.first().unwrap().clone());
                Ok(DynToken::DynSeq { contents, template })
            }
            (DynSolType::FixedArray(t, size), DynSolValue::FixedArray(tokens)) => {
                if *size != tokens.len() {
                    return Err(crate::Error::custom_owned(format!(
                        "Size mismatch for FixedArray. Got {}, expected {}",
                        tokens.len(),
                        size,
                    )));
                }
                Ok(DynToken::FixedSeq(
                    tokens
                        .into_iter()
                        .map(|token| t.tokenize(token))
                        .collect::<Result<_, _>>()?,
                    *size,
                ))
            }
            (DynSolType::CustomStruct { name, tuple }, DynSolValue::Tuple(tokens)) => {
                if tuple.len() != tokens.len() {
                    return Err(crate::Error::custom_owned(format!(
                        "Tuple length mismatch for {} . Got {}, expected {}",
                        name,
                        tokens.len(),
                        tuple.len(),
                    )));
                }
                let len = tuple.len();
                let tuple = tuple
                    .iter()
                    .zip(tokens.into_iter())
                    .map(|(ty, token)| ty.tokenize(token))
                    .collect::<Result<_, _>>()?;
                Ok(DynToken::FixedSeq(tuple, len))
            }
            (
                DynSolType::CustomStruct { name, tuple },
                DynSolValue::CustomStruct {
                    name: name_val,
                    tuple: tuple_val,
                },
            ) => {
                if name != &name_val {
                    return Err(crate::Error::custom_owned(std::format!(
                        "Name mismatch for {} . Got {}, expected {}",
                        name,
                        name_val,
                        name,
                    )));
                }
                if tuple.len() != tuple_val.len() {
                    return Err(crate::Error::custom_owned(std::format!(
                        "Tuple length mismatch for {} . Got {}, expected {}",
                        name,
                        tuple_val.len(),
                        tuple.len(),
                    )));
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
                if name != &name_val {
                    return Err(crate::Error::custom_owned(std::format!(
                        "Name mismatch for {} . Got {}, expected {}",
                        name,
                        name_val,
                        name,
                    )));
                }
                // A little hacky. A Custom value type is always encoded as a full 32-byte worc
                Ok(DynSolType::FixedBytes(32).tokenize(DynSolValue::FixedBytes(inner_val, 32))?)
            }
            _ => Err(crate::Error::Other(
                "Invalid type on dynamic tokenization".into(),
            )),
        }
    }

    /// Dynamic detokenization
    pub fn detokenize(&self, token: DynToken) -> AbiResult<DynSolValue> {
        match (self, token) {
            (DynSolType::Address, DynToken::Word(word)) => Ok(DynSolValue::Address(
                sol_type::Address::detokenize(word.into())?,
            )),
            (DynSolType::Bool, DynToken::Word(word)) => {
                Ok(DynSolValue::Bool(sol_type::Bool::detokenize(word.into())?))
            }
            (DynSolType::Bytes, DynToken::PackedSeq(buf)) => Ok(DynSolValue::Bytes(buf)),
            (DynSolType::FixedBytes(size), DynToken::Word(word)) => Ok(DynSolValue::FixedBytes(
                sol_type::FixedBytes::<32>::detokenize(word.into())?.into(),
                *size,
            )),
            // cheating here, but it's ok
            (DynSolType::Int(size), DynToken::Word(word)) => Ok(DynSolValue::Int(
                sol_type::FixedBytes::<32>::detokenize(word.into())?.into(),
                *size,
            )),
            (DynSolType::Uint(size), DynToken::Word(word)) => Ok(DynSolValue::Uint(
                sol_type::Uint::<256>::detokenize(word.into())?,
                *size,
            )),

            (DynSolType::String, DynToken::PackedSeq(buf)) => Ok(DynSolValue::String(
                sol_type::String::detokenize(buf.into())?,
            )),
            (DynSolType::Tuple(types), DynToken::FixedSeq(tokens, _)) => {
                if types.len() != tokens.len() {
                    return Err(crate::Error::custom(
                        "tuple length mismatch on dynamic detokenization",
                    ));
                }
                Ok(DynSolValue::Tuple(
                    types
                        .iter()
                        .zip(tokens.into_iter())
                        .map(|(t, w)| t.detokenize(w))
                        .collect::<Result<_, _>>()?,
                ))
            }
            (DynSolType::Array(t), DynToken::DynSeq { contents, .. }) => Ok(DynSolValue::Array(
                contents
                    .into_iter()
                    .map(|tok| t.detokenize(tok))
                    .collect::<Result<_, _>>()?,
            )),
            (DynSolType::FixedArray(t, size), DynToken::FixedSeq(tokens, _)) => {
                if *size != tokens.len() {
                    return Err(crate::Error::custom(
                        "array length mismatch on dynamic detokenization",
                    ));
                }
                Ok(DynSolValue::FixedArray(
                    tokens
                        .into_iter()
                        .map(|tok| t.detokenize(tok))
                        .collect::<Result<_, _>>()?,
                ))
            }
            (DynSolType::CustomStruct { name, tuple }, DynToken::FixedSeq(tokens, len)) => {
                if len != tokens.len() || len != tuple.len() {
                    return Err(crate::Error::custom(
                        "custom length mismatch on dynamic detokenization",
                    ));
                }
                let tuple = tuple
                    .iter()
                    .zip(tokens.into_iter())
                    .map(|(t, w)| t.detokenize(w))
                    .collect::<Result<_, _>>()?;

                Ok(DynSolValue::CustomStruct {
                    name: name.clone(),
                    tuple,
                })
            }
            (DynSolType::CustomValue { .. }, token) => DynSolType::FixedBytes(32).detokenize(token),
            _ => Err(crate::Error::custom(
                "mismatched types on dynamic detokenization",
            )),
        }
    }

    /// Instantiate an empty dyn token, to be decoded into
    pub(crate) fn empty_dyn_token(&self) -> DynToken {
        match self {
            DynSolType::Address => DynToken::Word(Word::default()),
            DynSolType::Bool => DynToken::Word(Word::default()),
            DynSolType::Bytes => DynToken::PackedSeq(Vec::new()),
            DynSolType::FixedBytes(_) => DynToken::Word(Word::default()),
            DynSolType::Int(_) => DynToken::Word(Word::default()),
            DynSolType::Uint(_) => DynToken::Word(Word::default()),
            DynSolType::String => DynToken::PackedSeq(Vec::new()),
            DynSolType::Tuple(types) => DynToken::FixedSeq(
                types.iter().map(|t| t.empty_dyn_token()).collect(),
                types.len(),
            ),
            DynSolType::Array(t) => DynToken::DynSeq {
                contents: vec![],
                template: Box::new(t.empty_dyn_token()),
            },
            DynSolType::FixedArray(t, size) => {
                DynToken::FixedSeq(vec![t.empty_dyn_token(); *size], *size)
            }
            DynSolType::CustomStruct { tuple, .. } => DynToken::FixedSeq(
                tuple.iter().map(|t| t.empty_dyn_token()).collect(),
                tuple.len(),
            ),
            DynSolType::CustomValue { .. } => DynToken::Word(Word::default()),
        }
    }

    /// Encode a single value. Fails if the value does not match this type
    pub fn encode_single(&self, value: DynSolValue) -> AbiResult<Vec<u8>> {
        let mut encoder = crate::encoder::Encoder::default();
        self.tokenize(value)?.encode_single(&mut encoder)?;
        Ok(encoder.into_bytes())
    }

    /// Decode a single value. Fails if the value does not match this type
    pub fn decode_single(&self, data: &[u8]) -> AbiResult<DynSolValue> {
        let mut decoder = crate::decoder::Decoder::new(data, false);
        let mut toks = self.empty_dyn_token();
        toks.decode_single_populate(&mut decoder)?;
        self.detokenize(toks)
    }

    /// Encode a sequence of values. Fails if the values do not match this
    /// type. Is a no-op if this type or the values are not a sequence.
    pub fn encode_sequence(&self, values: DynSolValue) -> AbiResult<Vec<u8>> {
        let mut encoder = crate::encoder::Encoder::default();
        self.tokenize(values)?.encode_sequence(&mut encoder)?;
        Ok(encoder.into_bytes())
    }

    /// Decode a sequence of values. Fails if the values do not match this type
    pub fn decode_sequence(&self, data: &[u8]) -> AbiResult<DynSolValue> {
        let mut decoder = crate::decoder::Decoder::new(data, false);
        let mut toks = self.empty_dyn_token();
        toks.decode_sequence_populate(&mut decoder)?;
        self.detokenize(toks)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use ethers_primitives::B160;

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
            .tokenize(DynSolValue::Address(B160::repeat_byte(0x01)))
            .unwrap();
        assert_eq!(token, DynToken::from(word1));

        let sol_type = DynSolType::FixedArray(Box::new(DynSolType::Address), 2);
        let token = sol_type
            .tokenize(DynSolValue::FixedArray(vec![
                B160::repeat_byte(0x01).into(),
                B160::repeat_byte(0x02).into(),
            ]))
            .unwrap();
        assert_eq!(
            token,
            DynToken::FixedSeq(vec![DynToken::Word(word1), DynToken::Word(word2)], 2)
        );
        let mut enc = crate::encoder::Encoder::default();
        token.encode_sequence(&mut enc).unwrap();
        assert_eq!(enc.finish(), vec![word1, word2]);
    }
}
