use crate::{no_std_prelude::*, Decoder, DynSolValue, Error, Result, Word};
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
        /// The type template of the dynamic sequence.
        /// This is used only when decoding. It indicates what the token type
        /// of the sequence is. During tokenization of data, the type of the
        /// contents is known, so this is not needed.
        template: Option<Box<DynToken<'a>>>,
    },
    /// A packed sequence (string or bytes).
    PackedSeq(&'a [u8]),
}

impl<T: Into<Word>> From<T> for DynToken<'_> {
    fn from(value: T) -> Self {
        Self::Word(value.into())
    }
}

impl<'a> PartialEq<DynToken<'a>> for DynToken<'_> {
    fn eq(&self, other: &DynToken<'_>) -> bool {
        match (self, other) {
            (Self::Word(l0), DynToken::Word(r0)) => l0 == r0,
            (Self::FixedSeq(l0, l1), DynToken::FixedSeq(r0, r1)) => l0 == r0 && l1 == r1,
            (
                Self::DynSeq {
                    contents: l_contents,
                    ..
                },
                DynToken::DynSeq {
                    contents: r_contents,
                    ..
                },
            ) => l_contents == r_contents,
            (Self::PackedSeq(l0), DynToken::PackedSeq(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for DynToken<'_> {}

impl<'a> DynToken<'a> {
    /// Instantiate a DynToken from a fixed sequence of values.
    pub fn from_fixed_seq(seq: &'a [DynSolValue]) -> Self {
        let tokens = seq.iter().map(|v| v.tokenize()).collect::<Vec<_>>();
        Self::FixedSeq(Cow::Owned(tokens), seq.len())
    }

    /// Instantiate a DynToken from a dynamic sequence of values.
    pub fn from_dyn_seq(seq: &'a [DynSolValue]) -> Self {
        let tokens = seq.iter().map(|v| v.tokenize()).collect::<Vec<_>>();
        Self::DynSeq {
            contents: Cow::Owned(tokens),
            template: None,
        }
    }

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

    /// Fallible cast into a sequence, dynamic or fixed-size
    pub fn as_token_seq(&self) -> Option<&[DynToken<'a>]> {
        match self {
            Self::FixedSeq(tokens, _) => Some(tokens),
            Self::DynSeq { contents, .. } => Some(contents),
            _ => None,
        }
    }

    /// Fallible cast into a packed sequence.
    #[inline]
    pub const fn as_packed_seq(&self) -> Option<&[u8]> {
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
    #[inline]
    pub(crate) fn decode_populate(&mut self, dec: &mut Decoder<'a>) -> Result<()> {
        let dynamic = self.is_dynamic();
        match self {
            Self::Word(w) => *w = WordToken::decode_from(dec)?.0,
            Self::FixedSeq(_, _) => {
                let mut child = if dynamic {
                    dec.take_indirection()?
                } else {
                    dec.raw_child()
                };

                self.decode_sequence_populate(&mut child)?;

                if !dynamic {
                    dec.take_offset(child);
                }
            }
            Self::DynSeq { contents, template } => {
                let mut child = dec.take_indirection()?;
                let size = child.take_u32()? as usize;
                // This appears to be an unclarity in the solidity spec. The
                // spec specifies that offsets are relative to the beginning of
                // `enc(X)`. But known-good test vectors have it relative to the
                // word AFTER the array size
                let mut child = child.raw_child();

                let mut new_tokens: Vec<_> = Vec::with_capacity(size);
                // This expect is safe because this is only invoked after
                // `empty_dyn_token()` which always sets template
                let t = template
                    .take()
                    .expect("No template. This is an alloy bug. Please report it.");
                new_tokens.resize(size, *t);

                new_tokens
                    .iter_mut()
                    .for_each(|t| t.decode_populate(&mut child).unwrap());

                *contents = new_tokens.into();
            }
            Self::PackedSeq(buf) => *buf = PackedSeqToken::decode_from(dec)?.0,
        }
        Ok(())
    }

    /// Decode a sequence from the decoder, populating the data by consuming
    /// decoder words.
    #[inline]
    pub(crate) fn decode_sequence_populate(&mut self, dec: &mut Decoder<'a>) -> Result<()> {
        match self {
            Self::FixedSeq(buf, size) => {
                for item in buf.to_mut().iter_mut().take(*size) {
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

    /// Decode a single item of this type, as a sequence of length 1.
    #[inline]
    pub(crate) fn decode_single_populate(&mut self, dec: &mut Decoder<'a>) -> Result<()> {
        // This is what
        // `Self::FixedSeq(vec![self.clone()], 1).decode_populate()`
        // would do, so we skip the allocation.
        self.decode_populate(dec)
    }
}
