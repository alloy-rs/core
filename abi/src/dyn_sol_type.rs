use core::{ops::Deref, str::FromStr};

use ethers_primitives::{B160, U256};

use crate::{
    decoder::Decoder, encoder::Encoder, sol_type, AbiResult, Error, FixedSeqToken, PackedSeqToken,
    SolType, TokenType, Word, WordToken,
};

pub struct Parenthesized<'a> {
    inner: &'a str,
}

impl<'a> FromStr for Parenthesized<'a> {
    type Err = ();

    fn from_str(s: &'a str) -> Result<Parenthesized<'a>, Self::Err> {
        s.split_once('(')
            .and_then(|(_, inner)| inner.rsplit_once(')'))
            .map(|(inner, _)| Self { inner })
            .ok_or(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Wraps all implementers of sol_type::SolType
pub enum DynSolType {
    Address,
    Bool,
    Bytes,
    FixedBytes(usize),
    Int(usize),
    Uint(usize),
    Function,
    String,
    Tuple(Vec<DynSolType>),
    Array(Box<DynSolType>),
    FixedArray(Box<DynSolType>, usize),
    CustomStruct {
        name: String,
        tuple: Vec<DynSolType>,
    },
    CustomValue {
        name: String,
        inner: Box<DynSolType>,
    },
}

impl FromStr for DynSolType {
    type Err = crate::Error;

    fn from_str(s: &str) -> AbiResult<Self> {
        match s {
            "address" => Ok(Self::Address),
            "bool" => Ok(Self::Bool),
            "bytes" => Ok(Self::Bytes),
            "function" => Ok(Self::Function),
            "string" => Ok(Self::String),
            _ => todo!(),
        }
    }
}

impl DynSolType {
    /// Dynamic tokenization
    pub fn tokenize(&self, value: SolValue) -> AbiResult<DynToken> {
        match (self, value) {
            (DynSolType::Address, SolValue::Address(val)) => {
                Ok(DynToken::Word(sol_type::Address::tokenize(val).inner()))
            }
            (DynSolType::Bool, SolValue::Bool(val)) => {
                Ok(DynToken::Word(sol_type::Bool::tokenize(val).inner()))
            }
            (DynSolType::Bytes, SolValue::Bytes(val)) => Ok(DynToken::PackedSeq(val)),
            (DynSolType::FixedBytes(len), SolValue::FixedBytes(word, size)) => {
                if size != *len {
                    return Err(crate::Error::custom_owned(format!(
                        "Size mismatch for FixedBytes. Got {}, expected {}",
                        size, len
                    )));
                }
                Ok(word.into())
            }
            (DynSolType::Int(_), SolValue::Int(word, _)) => Ok(word.into()),
            (DynSolType::Uint(_), SolValue::Uint(num, _)) => Ok(DynToken::Word(num.into())),
            (DynSolType::Function, SolValue::Function(word)) => {
                Ok(DynToken::Word(sol_type::Function::tokenize(word).inner()))
            }
            (DynSolType::String, SolValue::String(buf)) => Ok(DynToken::PackedSeq(buf.into())),
            (DynSolType::Tuple(types), SolValue::Tuple(tokens)) => {
                let tokens = types
                    .iter()
                    .zip(tokens.into_iter())
                    .map(|(ty, token)| ty.tokenize(token))
                    .collect::<Result<_, _>>()?;

                Ok(DynToken::FixedSeq(tokens, types.len()))
            }
            (DynSolType::Array(t), SolValue::Array(values)) => {
                let contents: Vec<DynToken> = values
                    .into_iter()
                    .map(|val| t.tokenize(val))
                    .collect::<Result<_, _>>()?;
                let template = Box::new(contents.first().unwrap().clone());
                Ok(DynToken::DynSeq { contents, template })
            }
            (DynSolType::FixedArray(t, size), SolValue::FixedArray(tokens)) => {
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
            (DynSolType::CustomStruct { name, tuple }, SolValue::Tuple(tokens)) => {
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
                SolValue::CustomStruct {
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
                DynSolType::CustomValue { name, inner },
                SolValue::CustomValue {
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
                Ok(inner.tokenize(*inner_val)?)
            }
            (DynSolType::CustomValue { inner, .. }, value) => inner.tokenize(value),

            _ => Err(crate::Error::Other(
                "Invalid type on dynamic tokenization".into(),
            )),
        }
    }

    /// Dynamic detokenization
    pub fn detokenize(&self, token: DynToken) -> AbiResult<SolValue> {
        match (self, token) {
            (DynSolType::Address, DynToken::Word(word)) => Ok(SolValue::Address(
                sol_type::Address::detokenize(word.into())?,
            )),
            (DynSolType::Bool, DynToken::Word(word)) => {
                Ok(SolValue::Bool(sol_type::Bool::detokenize(word.into())?))
            }
            (DynSolType::Bytes, DynToken::PackedSeq(buf)) => Ok(SolValue::Bytes(buf)),
            (DynSolType::FixedBytes(size), DynToken::Word(word)) => Ok(SolValue::FixedBytes(
                sol_type::FixedBytes::<32>::detokenize(word.into())?.into(),
                *size,
            )),
            // cheating here, but it's ok
            (DynSolType::Int(size), DynToken::Word(word)) => Ok(SolValue::Int(
                sol_type::FixedBytes::<32>::detokenize(word.into())?.into(),
                *size,
            )),
            (DynSolType::Uint(size), DynToken::Word(word)) => Ok(SolValue::Uint(
                sol_type::Uint::<256>::detokenize(word.into())?,
                *size,
            )),
            (DynSolType::Function, DynToken::Word(word)) => Ok(SolValue::Function(
                sol_type::Function::detokenize(word.into())?,
            )),
            (DynSolType::String, DynToken::PackedSeq(buf)) => {
                Ok(SolValue::String(sol_type::String::detokenize(buf.into())?))
            }
            (DynSolType::Tuple(types), DynToken::FixedSeq(tokens, _)) => {
                if types.len() != tokens.len() {
                    return Err(crate::Error::custom(
                        "tuple length mismatch on dynamic detokenization",
                    ));
                }
                Ok(SolValue::Tuple(
                    types
                        .iter()
                        .zip(tokens.into_iter())
                        .map(|(t, w)| t.detokenize(w))
                        .collect::<Result<_, _>>()?,
                ))
            }
            (DynSolType::Array(t), DynToken::DynSeq { contents, .. }) => Ok(SolValue::Array(
                contents
                    .into_iter()
                    .map(|tok| t.detokenize(tok))
                    .collect::<Result<_, _>>()?,
            )),
            (DynSolType::FixedArray(t, size), DynToken::FixedSeq(tokens, count)) => {
                if *size != tokens.len() {
                    return Err(crate::Error::custom(
                        "array length mismatch on dynamic detokenization",
                    ));
                }
                Ok(SolValue::FixedArray(
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

                Ok(SolValue::CustomStruct {
                    name: name.clone(),
                    tuple,
                })
            }
            (DynSolType::CustomValue { name, inner }, token) => inner.detokenize(token),
            _ => Err(crate::Error::custom(
                "mismatched types on dynamic detokenization",
            )),
        }
    }

    /// Instantiate an empty dyn token, to be decoded into
    pub fn empty_dyn_token(&self) -> DynToken {
        match self {
            DynSolType::Address => DynToken::Word(Word::default()),
            DynSolType::Bool => DynToken::Word(Word::default()),
            DynSolType::Bytes => DynToken::PackedSeq(Vec::new()),
            DynSolType::FixedBytes(_) => DynToken::Word(Word::default()),
            DynSolType::Int(_) => DynToken::Word(Word::default()),
            DynSolType::Uint(_) => DynToken::Word(Word::default()),
            DynSolType::Function => DynToken::Word(Word::default()),
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
            DynSolType::CustomValue { inner, .. } => inner.empty_dyn_token(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolValue {
    Address(B160),
    Bool(bool),
    Bytes(Vec<u8>),
    FixedBytes(Word, usize),
    Int(Word, usize),
    Uint(U256, usize),
    Function((B160, [u8; 4])),
    String(String),
    Tuple(Vec<SolValue>),
    Array(Vec<SolValue>),
    FixedArray(Vec<SolValue>),
    CustomStruct { name: String, tuple: Vec<SolValue> },
    CustomValue { name: String, inner: Box<SolValue> },
}

impl From<B160> for SolValue {
    fn from(value: B160) -> Self {
        Self::Address(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DynToken {
    Word(Word),
    FixedSeq(Vec<DynToken>, usize),
    DynSeq {
        contents: Vec<DynToken>,
        template: Box<DynToken>,
    },
    PackedSeq(Vec<u8>),
}

impl From<Word> for DynToken {
    fn from(value: Word) -> Self {
        Self::Word(value.into())
    }
}

impl DynToken {
    fn is_dynamic(&self) -> bool {
        match self {
            Self::Word(_) => false,
            Self::FixedSeq(inner, _) => inner.iter().any(|i| i.is_dynamic()),
            Self::DynSeq { .. } => true,
            Self::PackedSeq(_) => true,
        }
    }

    fn decode_populate(&mut self, dec: &mut Decoder) -> AbiResult<()> {
        let dynamic = self.is_dynamic();
        match self {
            DynToken::Word(w) => *w = WordToken::decode_from(dec)?.inner(),
            DynToken::FixedSeq(toks, size) => {
                // todo try to remove this duplication?
                let mut child = if dynamic {
                    dec.take_indirection()?
                } else {
                    dec.raw_child()
                };
                for tok in toks.iter_mut().take(*size) {
                    tok.decode_populate(&mut child)?;
                }
            }
            DynToken::DynSeq { contents, template } => {
                let mut child = dec.take_indirection()?;
                let size = dec.take_u32()? as usize;

                let mut new_toks = Vec::with_capacity(size);
                for tok in 0..size {
                    let mut t = (**template).clone();
                    t.decode_populate(&mut child)?;
                    new_toks.push(t);
                }
                *contents = new_toks;
            }
            DynToken::PackedSeq(buf) => *buf = PackedSeqToken::decode_from(dec)?.take_vec(),
        }
        Ok(())
    }

    fn head_words(&self) -> usize {
        match self {
            DynToken::Word(_) => 1,
            DynToken::FixedSeq(tokens, _) => {
                if self.is_dynamic() {
                    1
                } else {
                    tokens.iter().map(DynToken::head_words).sum()
                }
            }
            DynToken::DynSeq { .. } => 1,
            DynToken::PackedSeq(_) => 1,
        }
    }

    fn tail_words(&self) -> usize {
        match self {
            DynToken::Word(_) => 0,
            DynToken::FixedSeq(_, size) => {
                if self.is_dynamic() {
                    *size
                } else {
                    0
                }
            }
            DynToken::DynSeq { contents, .. } => {
                1 + contents.iter().map(DynToken::tail_words).sum::<usize>()
            }
            DynToken::PackedSeq(buf) => 1 + (buf.len() + 31) / 32,
        }
    }

    fn head_append(&self, enc: &mut Encoder) {
        match self {
            DynToken::Word(word) => enc.append_word(*word),
            DynToken::FixedSeq(tokens, _) => {
                if self.is_dynamic() {
                    enc.append_indirection();
                } else {
                    tokens.iter().for_each(|inner| inner.head_append(enc))
                }
            }
            DynToken::DynSeq { .. } => enc.append_indirection(),
            DynToken::PackedSeq(buf) => enc.append_indirection(),
        }
    }

    fn tail_append(&self, enc: &mut Encoder) {
        match self {
            DynToken::Word(_) => {}
            DynToken::FixedSeq(_, _) => {
                if self.is_dynamic() {
                    self.encode_sequence(enc);
                }
            }
            DynToken::DynSeq { contents, .. } => {
                enc.append_seq_len(contents);
                self.encode_sequence(enc);
            }
            DynToken::PackedSeq(buf) => enc.append_packed_seq(buf),
        }
    }

    fn encode_sequence(&self, enc: &mut Encoder) {
        match self {
            DynToken::FixedSeq(tokens, _) => {
                let head_words = tokens.iter().map(DynToken::head_words).sum::<usize>();
                enc.push_offset(head_words as u32);
                for t in tokens.iter() {
                    t.head_append(enc);
                    enc.bump_offset(t.tail_words() as u32);
                }
                for t in tokens.iter() {
                    t.tail_append(enc);
                }
                enc.pop_offset();
            }
            DynToken::DynSeq { contents, .. } => {
                let head_words = contents.iter().map(DynToken::head_words).sum::<usize>();
                enc.push_offset(head_words as u32);
                for t in contents.iter() {
                    t.head_append(enc);
                    enc.bump_offset(t.tail_words() as u32);
                }
                for t in contents.iter() {
                    t.tail_append(enc);
                }
                enc.pop_offset();
            }
            _ => {}
        }
    }

    fn decode_sequence_populate(&mut self, dec: &mut Decoder) -> AbiResult<()> {
        match self {
            DynToken::FixedSeq(buf, size) => {
                for item in buf.iter_mut() {
                    item.decode_populate(dec)?;
                }
                Ok(())
            }
            DynToken::DynSeq { .. } => self.decode_populate(dec),
            _ => Err(Error::custom(
                "Called decode_sequence_populate on non-sequence",
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_encodes() {
        let word1 = "0000000000000000000000000101010101010101010101010101010101010101"
            .parse()
            .unwrap();
        let word2 = "0000000000000000000000000202020202020202020202020202020202020202"
            .parse()
            .unwrap();

        let sol_type = DynSolType::Address;
        let token = sol_type
            .tokenize(SolValue::Address(B160::repeat_byte(0x01)))
            .unwrap();
        assert_eq!(token, DynToken::from(word1));

        let sol_type = DynSolType::FixedArray(Box::new(DynSolType::Address), 2);
        let token = sol_type
            .tokenize(SolValue::FixedArray(vec![
                B160::repeat_byte(0x01).into(),
                B160::repeat_byte(0x02).into(),
            ]))
            .unwrap();
        assert_eq!(
            token,
            DynToken::FixedSeq(vec![DynToken::Word(word1), DynToken::Word(word2)], 2)
        );
        let mut enc = Encoder::default();
        token.encode_sequence(&mut enc);
        assert_eq!(enc.finish(), vec![word1, word2]);
    }
}
