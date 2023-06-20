use crate::{no_std_prelude::*, token::TokenSeq, Result, SolType};

/// Solidity call (a tuple with a selector).
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity function
/// definition.
pub trait SolCall: Sized {
    /// The underlying tuple type which represents this type's members.
    ///
    /// If this type has no arguments, this will be the unit type `()`.
    type Tuple<'a>: SolType<TokenType<'a> = Self::Token<'a>>;

    /// The corresponding [TokenSeq] type.
    type Token<'a>: TokenSeq<'a>;

    /// The function's ABI signature.
    const SIGNATURE: &'static str;

    /// The function selector: `keccak256(SIGNATURE)[0..4]`
    const SELECTOR: [u8; 4];

    // TODO: avoid clones here
    /// Converts to the tuple type used for ABI encoding and decoding.
    fn to_rust<'a>(&self) -> <Self::Tuple<'a> as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding and decoding.
    fn from_rust(tuple: <Self::Tuple<'_> as SolType>::RustType) -> Self;

    /// Tokenize the call's arguments.
    fn tokenize(&self) -> Self::Token<'_>;

    /// The size of the encoded data in bytes, **without** its selector.
    fn encoded_size(&self) -> usize {
        // This avoids unnecessary clones.
        if let Some(size) = <Self::Tuple<'_> as SolType>::ENCODED_SIZE {
            return size
        }

        <<Self as SolCall>::Tuple<'_> as SolType>::encoded_size(&self.to_rust())
    }

    /// ABI decode this call's arguments from the given slice, **without** its
    /// selector.
    #[inline]
    fn decode_raw(data: &[u8], validate: bool) -> Result<Self> {
        <Self::Tuple<'_> as SolType>::decode(data, validate).map(Self::from_rust)
    }

    /// ABI decode this call's arguments from the given slice, **with** the
    /// selector.
    #[inline]
    fn decode(data: &[u8], validate: bool) -> Result<Self> {
        let data = data
            .strip_prefix(&Self::SELECTOR)
            .ok_or_else(|| crate::Error::type_check_fail_sig(data, Self::SIGNATURE))?;
        Self::decode_raw(data, validate)
    }

    /// ABI encode the call to the given buffer **without** its selector.
    #[inline]
    fn encode_raw(&self, out: &mut Vec<u8>) {
        out.reserve(self.encoded_size());
        out.extend(crate::encode(&self.tokenize()));
    }

    /// ABI encode the call to the given buffer **with** its selector.
    #[inline]
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.encoded_size());
        out.extend(&Self::SELECTOR);
        self.encode_raw(&mut out);
        out
    }
}
