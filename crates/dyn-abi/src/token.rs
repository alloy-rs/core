use crate::{no_std_prelude::*, Decoder, Encoder, Error, Result, Word};
use alloc::borrow::Cow;
use alloy_sol_types::token::{PackedSeqToken, TokenType, WordToken};

/// A dynamic token. Equivalent to an enum over all types implementing
/// [`alloy_sol_types::TokenType`]
// NOTE: do not derive `Hash` for this type. The derived version is not
// compatible with the current `PartialEq` implementation. If manually
// implementing `Hash`, ignore the `template` prop in the `DynSeq` variant
#[derive(Debug, Clone)]
pub enum DynToken<'a> {
    /// A single word.
    Word(Word),
    /// A Fixed Sequence.
    FixedSeq(Cow<'a, [DynToken<'a>]>, usize),
    /// A dynamic-length sequence.
    DynSeq {
        /// The contents of the dynamic sequence.
        contents: Cow<'a, [DynToken<'a>]>,
        /// The type of the dynamic sequence.
        template: Box<DynToken<'a>>,
    },
    /// A packed sequence (string or bytes).
    PackedSeq(Vec<u8>),
}

impl<'a> PartialEq<DynToken<'a>> for DynToken<'a> {
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

impl Eq for DynToken<'_> {}

impl From<Word> for DynToken<'_> {
    fn from(value: Word) -> Self {
        Self::Word(value)
    }
}

impl<'a> DynToken<'a> {
    /// Attempt to cast to a word.
    #[inline]
    pub const fn as_word(&self) -> Option<Word> {
        match self {
            Self::Word(word) => Some(*word),
            _ => None,
        }
    }

    /// Fallible cast into a fixed sequence.
    #[inline]
    pub fn as_fixed_seq(&self) -> Option<(&[DynToken<'a>], usize)> {
        match self {
            Self::FixedSeq(tokens, size) => Some((tokens, *size)),
            _ => None,
        }
    }

    /// Fallible cast into a dynamic sequence.
    #[inline]
    pub fn as_dynamic_seq(&self) -> Option<&[DynToken<'a>]> {
        match self {
            Self::DynSeq { contents, .. } => Some(contents),
            _ => None,
        }
    }

    /// Fallible cast into a packed sequence.
    #[inline]
    pub fn as_packed_seq(&self) -> Option<&[u8]> {
        match self {
            Self::PackedSeq(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// True if the type is dynamic, else false.
    #[inline]
    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Word(_) => false,
            Self::FixedSeq(inner, _) => inner.iter().any(|i| i.is_dynamic()),
            Self::DynSeq { .. } => true,
            Self::PackedSeq(_) => true,
        }
    }

    /// Decodes from a decoder, populating the structure with the decoded data.
    pub fn decode_populate(&mut self, dec: &mut Decoder<'_>) -> Result<()> {
        let dynamic = self.is_dynamic();
        match self {
            Self::Word(w) => *w = WordToken::decode_from(dec)?.0,
            Self::FixedSeq(tokens, size) => {
                let mut child = if dynamic {
                    dec.take_indirection()?
                } else {
                    dec.raw_child()
                };
                for token in tokens.to_mut().iter_mut().take(*size) {
                    token.decode_populate(&mut child)?;
                }
            }
            Self::DynSeq { contents, template } => {
                let mut child = dec.take_indirection()?;
                let size = child.take_u32()? as usize;
                let mut new_tokens = Vec::with_capacity(size);
                for _ in 0..size {
                    let mut t = (**template).clone();
                    t.decode_populate(&mut child)?;
                    new_tokens.push(t);
                }
                *contents = new_tokens.into();
            }
            Self::PackedSeq(buf) => *buf = PackedSeqToken::decode_from(dec)?.into_vec(),
        }
        Ok(())
    }

    /// Returns the number of words this type uses in the head of the ABI blob.
    #[inline]
    pub fn head_words(&self) -> usize {
        match self {
            Self::Word(_) => 1,
            Self::FixedSeq(tokens, _) => {
                if self.is_dynamic() {
                    1
                } else {
                    tokens.iter().map(Self::head_words).sum()
                }
            }
            Self::DynSeq { .. } => 1,
            Self::PackedSeq(_) => 1,
        }
    }

    /// Returns the number of words this type uses in the tail of the ABI blob.
    #[inline]
    pub fn tail_words(&self) -> usize {
        match self {
            Self::Word(_) => 0,
            Self::FixedSeq(_, size) => self.is_dynamic() as usize * *size,
            Self::DynSeq { contents, .. } => {
                1 + contents.iter().map(Self::tail_words).sum::<usize>()
            }
            Self::PackedSeq(buf) => 1 + (buf.len() + 31) / 32,
        }
    }

    /// Append this data to the head of an in-progress blob via the encoder.
    pub fn head_append(&self, enc: &mut Encoder) {
        match self {
            Self::Word(word) => enc.append_word(*word),
            Self::FixedSeq(tokens, _) => {
                if self.is_dynamic() {
                    enc.append_indirection();
                } else {
                    tokens.iter().for_each(|inner| inner.head_append(enc))
                }
            }
            Self::DynSeq { .. } => enc.append_indirection(),
            Self::PackedSeq(_) => enc.append_indirection(),
        }
    }

    /// Append this data to the tail of an in-progress blob via the encoder.
    #[inline]
    pub fn tail_append(&self, enc: &mut Encoder) {
        match self {
            Self::Word(_) => {}
            Self::FixedSeq(_, _) => {
                if self.is_dynamic() {
                    self.encode_sequence(enc).expect("known to be sequence");
                }
            }
            Self::DynSeq { contents, .. } => {
                enc.append_seq_len(contents);
                self.encode_sequence(enc).expect("known to be sequence");
            }
            Self::PackedSeq(buf) => enc.append_packed_seq(buf),
        }
    }

    /// Encode this data, if it is a sequence. Error otherwise.
    pub(crate) fn encode_sequence(&self, enc: &mut Encoder) -> Result<()> {
        match self {
            Self::FixedSeq(tokens, _) => {
                let head_words = tokens.iter().map(Self::head_words).sum::<usize>();
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
            Self::DynSeq { contents, .. } => {
                let head_words = contents.iter().map(Self::head_words).sum::<usize>();
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
    /// decoder words.
    pub(crate) fn decode_sequence_populate(&mut self, dec: &mut Decoder<'_>) -> Result<()> {
        match self {
            Self::FixedSeq(buf, _) => {
                for item in buf.to_mut().iter_mut() {
                    item.decode_populate(dec)?;
                }
                Ok(())
            }
            Self::DynSeq { .. } => self.decode_populate(dec),
            _ => Err(Error::custom(
                "Called decode_sequence_populate on non-sequence token",
            )),
        }
    }

    /// Encode a single item of this type, as a sequence of length 1.
    #[inline]
    pub(crate) fn encode_single(&'a self, enc: &mut Encoder) -> Result<()> {
        Self::FixedSeq(core::slice::from_ref(self).into(), 1).encode_sequence(enc)
    }

    /// Decode a single item of this type, as a sequence of length 1.
    #[inline]
    pub(crate) fn decode_single_populate(&mut self, dec: &mut Decoder<'_>) -> Result<()> {
        // This is what `Self::FixedSeq(vec![self.clone()], 1).decode_populate()`
        // would do, so we skip the allocation.
        self.decode_populate(dec)
    }
}
