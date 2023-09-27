use crate::{
    abi::{TokenSeq, TokenType},
    Encodable, Result, SolType, Word,
};
use alloc::vec::Vec;

/// Solidity call (a tuple with a selector).
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity function
/// definition.
pub trait SolCall: Sized {
    /// The underlying tuple type which represents this type's arguments.
    ///
    /// If this type has no arguments, this will be the unit type `()`.
    type Arguments<'a>: SolType<TokenType<'a> = Self::Token<'a>>;

    /// The arguments' corresponding [TokenSeq] type.
    type Token<'a>: TokenSeq<'a>;

    /// The function's return struct.
    type Return;

    /// The underlying tuple type which represents this type's return values.
    ///
    /// If this type has no return values, this will be the unit type `()`.
    type ReturnTuple<'a>: SolType<TokenType<'a> = Self::ReturnToken<'a>>;

    /// The returns' corresponding [TokenSeq] type.
    type ReturnToken<'a>: TokenSeq<'a>;

    /// The function's ABI signature.
    const SIGNATURE: &'static str;

    /// The function selector: `keccak256(SIGNATURE)[0..4]`
    const SELECTOR: [u8; 4];

    /// Convert from the tuple type used for ABI encoding and decoding.
    fn new(tuple: <Self::Arguments<'_> as SolType>::RustType) -> Self;

    /// Tokenize the call's arguments.
    fn tokenize(&self) -> Self::Token<'_>;

    /// The size of the encoded data in bytes, **without** its selector.
    #[inline]
    fn abi_encoded_size(&self) -> usize {
        if let Some(size) = <Self::Arguments<'_> as SolType>::ENCODED_SIZE {
            return size
        }

        self.tokenize().total_words() * Word::len_bytes()
    }

    /// ABI decode this call's arguments from the given slice, **without** its
    /// selector.
    #[inline]
    fn abi_decode_raw(data: &[u8], validate: bool) -> Result<Self> {
        <Self::Arguments<'_> as SolType>::abi_decode_sequence(data, validate).map(Self::new)
    }

    /// ABI decode this call's arguments from the given slice, **with** the
    /// selector.
    #[inline]
    fn abi_decode(data: &[u8], validate: bool) -> Result<Self> {
        let data = data
            .strip_prefix(&Self::SELECTOR)
            .ok_or_else(|| crate::Error::type_check_fail_sig(data, Self::SIGNATURE))?;
        Self::abi_decode_raw(data, validate)
    }

    /// ABI encode the call to the given buffer **without** its selector.
    #[inline]
    fn abi_encode_raw(&self, out: &mut Vec<u8>) {
        out.reserve(self.abi_encoded_size());
        out.extend(crate::abi::encode_sequence(&self.tokenize()));
    }

    /// ABI encode the call to the given buffer **with** its selector.
    #[inline]
    fn abi_encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.abi_encoded_size());
        out.extend(&Self::SELECTOR);
        self.abi_encode_raw(&mut out);
        out
    }

    /// ABI decode this call's return values from the given slice.
    fn abi_decode_returns(data: &[u8], validate: bool) -> Result<Self::Return>;

    /// ABI encode the call's return values.
    #[inline]
    fn abi_encode_returns<'a, E>(e: &'a E) -> Vec<u8>
    where
        E: Encodable<Self::ReturnTuple<'a>>,
    {
        crate::abi::encode_sequence(&e.to_tokens())
    }
}
