use crate::{no_std_prelude::*, AbiResult, Decoder, Encoder, Error, Word};
use ethers_abi_enc::{PackedSeqToken, TokenType, WordToken};

// TODO: try to remove duplicated encoding/decoding logic

/// A dynamic token. Equivalent to an enum over all types implementing
/// [`crate::TokenType`]
// NOTE: do not derive `Hash` for this type. The derived version is not
// compatible with the current `PartialEq` implementation. If manually
// implementing `Hash`, ignore the `template` prop in the `DynSeq` variant
#[derive(Debug, Clone)]
pub enum DynToken {
    /// A single word
    Word(Word),
    /// A Fixed Sequence
    FixedSeq(Vec<DynToken>, usize),
    /// A dynamic-length sequence
    DynSeq {
        /// The contents of the dynamic sequence
        contents: Vec<DynToken>,
        /// The type of the dynamic sequence
        template: Box<DynToken>,
    },
    /// A packed sequence (string or bytes)
    PackedSeq(Vec<u8>),
}

impl PartialEq<DynToken> for DynToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Word(l0), Self::Word(r0)) => l0 == r0,
            (Self::FixedSeq(l0, l1), Self::FixedSeq(r0, r1)) => l0 == r0 && l1 == r1,
            (
                Self::DynSeq {
                    contents: l_contents,
                    ..
                },
                Self::DynSeq {
                    contents: r_contents,
                    ..
                },
            ) => l_contents == r_contents,
            (Self::PackedSeq(l0), Self::PackedSeq(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for DynToken {}

impl From<Word> for DynToken {
    fn from(value: Word) -> Self {
        Self::Word(value)
    }
}

impl DynToken {
    /// Attempt to cast to a word.
    pub const fn as_word(&self) -> Option<Word> {
        match self {
            Self::Word(word) => Some(*word),
            _ => None,
        }
    }

    /// Fallible cast into a fixed sequence.
    pub fn as_fixed_seq(&self) -> Option<(&[DynToken], usize)> {
        match self {
            Self::FixedSeq(toks, size) => Some((toks.as_slice(), *size)),
            _ => None,
        }
    }

    /// Fallible cast into a dynamic sequence.
    pub fn as_dynamic_seq(&self) -> Option<&[DynToken]> {
        match self {
            Self::DynSeq { contents, .. } => Some(contents.as_slice()),
            _ => None,
        }
    }

    /// Fallible cast into a packed sequence.
    pub fn as_packed_seq(&self) -> Option<&[u8]> {
        match self {
            Self::PackedSeq(bytes) => Some(bytes.as_slice()),
            _ => None,
        }
    }

    /// True if the type is dynamic, else false
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Word(_) => false,
            Self::FixedSeq(inner, _) => inner.iter().any(|i| i.is_dynamic()),
            Self::DynSeq { .. } => true,
            Self::PackedSeq(_) => true,
        }
    }

    /// Decodes from a decoder, populating the structure with the decoded data
    pub fn decode_populate(&mut self, dec: &mut Decoder<'_>) -> AbiResult<()> {
        let dynamic = self.is_dynamic();
        match self {
            DynToken::Word(w) => *w = WordToken::decode_from(dec)?.inner(),
            DynToken::FixedSeq(toks, size) => {
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
                let size = child.take_u32()? as usize;
                let mut new_toks = Vec::with_capacity(size);
                for _ in 0..size {
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

    /// Returns the number of words this type uses in the head of the ABI blob
    pub fn head_words(&self) -> usize {
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

    /// Returns the number of words this type uses in the tail of the ABI blob
    pub fn tail_words(&self) -> usize {
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

    /// Append this data to the head of an in-progress blob via the encoder
    pub fn head_append(&self, enc: &mut Encoder) {
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
            DynToken::PackedSeq(_) => enc.append_indirection(),
        }
    }

    /// Append this data to the tail of an in-progress blob via the encoder
    pub fn tail_append(&self, enc: &mut Encoder) {
        match self {
            DynToken::Word(_) => {}
            DynToken::FixedSeq(_, _) => {
                if self.is_dynamic() {
                    self.encode_sequence(enc).expect("known to be sequence");
                }
            }
            DynToken::DynSeq { contents, .. } => {
                enc.append_seq_len(contents);
                self.encode_sequence(enc).expect("known to be sequence");
            }
            DynToken::PackedSeq(buf) => enc.append_packed_seq(buf),
        }
    }

    /// Encode this data, if it is a sequence. Error otherwise
    pub(crate) fn encode_sequence(&self, enc: &mut Encoder) -> AbiResult<()> {
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
            _ => {
                return Err(Error::custom(
                    "Called encode_sequence on non-sequence token",
                ))
            }
        }
        Ok(())
    }

    /// Decode a sequence from the decoder, populating the data by consuming
    /// decoder words
    pub(crate) fn decode_sequence_populate(&mut self, dec: &mut Decoder<'_>) -> AbiResult<()> {
        match self {
            DynToken::FixedSeq(buf, _) => {
                for item in buf.iter_mut() {
                    item.decode_populate(dec)?;
                }
                Ok(())
            }
            DynToken::DynSeq { .. } => self.decode_populate(dec),
            _ => Err(Error::custom(
                "Called decode_sequence_populate on non-sequence token",
            )),
        }
    }

    /// Encode a single item of this type, as a sequence of length 1
    pub(crate) fn encode_single(&self, enc: &mut Encoder) -> AbiResult<()> {
        DynToken::FixedSeq(vec![self.clone()], 1).encode_sequence(enc)
    }

    /// Decode a single item of this type, as a sequence of length 1
    pub(crate) fn decode_single_populate(&mut self, dec: &mut Decoder<'_>) -> AbiResult<()> {
        let mut tok = DynToken::FixedSeq(vec![self.clone()], 1);
        tok.decode_sequence_populate(dec)?;
        if let DynToken::FixedSeq(mut toks, _) = tok {
            *self = toks.drain(..).next().unwrap();
        } else {
            unreachable!()
        }
        Ok(())
    }
}
