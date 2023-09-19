use crate::{token::WordToken, Result, SolType, Word};
use alloc::vec::Vec;

/// Solidity enum. This is always a wrapper around a [`u8`].
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity error
/// definition.
pub trait SolEnum: Sized + Copy + Into<u8> + TryFrom<u8, Error = crate::Error> {
    /// The number of variants in the enum.
    ///
    /// This is generally between 1 and 256 inclusive.
    const COUNT: usize;

    /// Tokenize the enum.
    #[inline]
    fn tokenize(self) -> WordToken {
        WordToken(Word::with_last_byte(self.into()))
    }

    /// ABI decode the enum from the given buffer.
    #[inline]
    fn decode(data: &[u8], validate: bool) -> Result<Self> {
        <crate::sol_data::Uint<8> as SolType>::decode(data, validate).and_then(Self::try_from)
    }

    /// ABI encode the enum into the given buffer.
    #[inline]
    fn encode_raw(self, out: &mut Vec<u8>) {
        out.extend(self.tokenize().0);
    }

    /// ABI encode the enum.
    #[inline]
    fn encode(self) -> Vec<u8> {
        self.tokenize().0.to_vec()
    }
}
