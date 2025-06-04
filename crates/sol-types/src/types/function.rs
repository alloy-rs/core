use crate::{
    Result, SolType, Word,
    abi::{Token, TokenSeq},
    private::SolTypeValue,
};
use alloc::vec::Vec;

/// A Solidity function call.
///
/// # Implementer's Guide
///
/// It should not be necessary to implement this trait manually. Instead, use
/// the [`sol!`](crate::sol!) procedural macro to parse Solidity syntax into
/// types that implement this trait.
pub trait SolCall: Sized {
    /// The underlying tuple type which represents this type's arguments.
    ///
    /// If this type has no arguments, this will be the unit type `()`.
    type Parameters<'a>: SolType<Token<'a> = Self::Token<'a>>;

    /// The arguments' corresponding [TokenSeq] type.
    type Token<'a>: TokenSeq<'a>;

    /// The function's return struct.
    type Return;

    /// The underlying tuple type which represents this type's return values.
    ///
    /// If this type has no return values, this will be the unit type `()`.
    type ReturnTuple<'a>: SolType<Token<'a> = Self::ReturnToken<'a>>;

    /// The returns' corresponding [TokenSeq] type.
    type ReturnToken<'a>: TokenSeq<'a>;

    /// The function's ABI signature.
    const SIGNATURE: &'static str;

    /// The function selector: `keccak256(SIGNATURE)[0..4]`
    const SELECTOR: [u8; 4];

    /// Convert from the tuple type used for ABI encoding and decoding.
    fn new(tuple: <Self::Parameters<'_> as SolType>::RustType) -> Self;

    /// Tokenize the call's arguments.
    fn tokenize(&self) -> Self::Token<'_>;

    /// Tokenize the call's return values.
    fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_>;

    /// The size of the encoded data in bytes, **without** its selector.
    #[inline]
    fn abi_encoded_size(&self) -> usize {
        if let Some(size) = <Self::Parameters<'_> as SolType>::ENCODED_SIZE {
            return size;
        }

        // `total_words` includes the first dynamic offset which we ignore.
        let offset = <<Self::Parameters<'_> as SolType>::Token<'_> as Token>::DYNAMIC as usize * 32;
        (self.tokenize().total_words() * Word::len_bytes()).saturating_sub(offset)
    }

    /// ABI decode this call's arguments from the given slice, **without** its
    /// selector.
    #[inline]
    fn abi_decode_raw(data: &[u8]) -> Result<Self> {
        <Self::Parameters<'_> as SolType>::abi_decode_sequence(data).map(Self::new)
    }

    /// ABI decode this call's arguments from the given slice, **without** its
    /// selector, with validation.
    ///
    /// This is the same as [`abi_decode_raw`](Self::abi_decode_raw), but performs
    /// validation checks on the decoded parameters tuple.
    #[inline]
    fn abi_decode_raw_validate(data: &[u8]) -> Result<Self> {
        <Self::Parameters<'_> as SolType>::abi_decode_sequence_validate(data).map(Self::new)
    }

    /// ABI decode this call's arguments from the given slice, **with** the
    /// selector.
    #[inline]
    fn abi_decode(data: &[u8]) -> Result<Self> {
        let data = data
            .strip_prefix(&Self::SELECTOR)
            .ok_or_else(|| crate::Error::type_check_fail_sig(data, Self::SIGNATURE))?;
        Self::abi_decode_raw(data)
    }

    /// ABI decode this call's arguments from the given slice, **with** the
    /// selector, with validation.
    ///
    /// This is the same as [`abi_decode`](Self::abi_decode), but performs
    /// validation checks on the decoded parameters tuple.
    #[inline]
    fn abi_decode_validate(data: &[u8]) -> Result<Self> {
        let data = data
            .strip_prefix(&Self::SELECTOR)
            .ok_or_else(|| crate::Error::type_check_fail_sig(data, Self::SIGNATURE))?;
        Self::abi_decode_raw_validate(data)
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
    fn abi_decode_returns(data: &[u8]) -> Result<Self::Return>;

    /// ABI decode this call's return values from the given slice, with validation.
    ///
    /// This is the same as [`abi_decode_returns`](Self::abi_decode_returns), but performs
    /// validation checks on the decoded return tuple.
    fn abi_decode_returns_validate(data: &[u8]) -> Result<Self::Return>;

    /// ABI encode the call's return value.
    #[inline]
    fn abi_encode_returns(ret: &Self::Return) -> Vec<u8> {
        crate::abi::encode_sequence(&Self::tokenize_returns(ret))
    }

    /// ABI encode the call's return values.
    #[inline]
    fn abi_encode_returns_tuple<'a, E>(e: &'a E) -> Vec<u8>
    where
        E: SolTypeValue<Self::ReturnTuple<'a>>,
    {
        crate::abi::encode_sequence(&e.stv_to_tokens())
    }
}

/// A Solidity constructor.
pub trait SolConstructor: Sized {
    /// The underlying tuple type which represents this type's arguments.
    ///
    /// If this type has no arguments, this will be the unit type `()`.
    type Parameters<'a>: SolType<Token<'a> = Self::Token<'a>>;

    /// The arguments' corresponding [TokenSeq] type.
    type Token<'a>: TokenSeq<'a>;

    /// Convert from the tuple type used for ABI encoding and decoding.
    fn new(tuple: <Self::Parameters<'_> as SolType>::RustType) -> Self;

    /// Tokenize the call's arguments.
    fn tokenize(&self) -> Self::Token<'_>;

    /// The size of the encoded data in bytes.
    #[inline]
    fn abi_encoded_size(&self) -> usize {
        if let Some(size) = <Self::Parameters<'_> as SolType>::ENCODED_SIZE {
            return size;
        }

        // `total_words` includes the first dynamic offset which we ignore.
        let offset = <<Self::Parameters<'_> as SolType>::Token<'_> as Token>::DYNAMIC as usize * 32;
        (self.tokenize().total_words() * Word::len_bytes()).saturating_sub(offset)
    }

    /// ABI encode the call to the given buffer.
    #[inline]
    fn abi_encode(&self) -> Vec<u8> {
        crate::abi::encode_sequence(&self.tokenize())
    }
}
