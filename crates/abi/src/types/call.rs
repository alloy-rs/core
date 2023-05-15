use crate::{no_std_prelude::*, token::TokenSeq, Result, SolType};

/// Solidity Call (a tuple with a selector)
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity function definition.
pub trait SolCall: Sized {
    /// The underlying tuple type which represents this type's members.
    ///
    /// If this type has no arguments, this will be the unit type `()`.
    type Tuple: SolType<TokenType = Self::Token>;

    /// The corresponding [TokenSeq] type.
    type Token: TokenSeq;

    /// The function's ABI signature.
    const SIGNATURE: &'static str;

    /// The function selector: `keccak256(SIGNATURE)[0..4]`
    const SELECTOR: [u8; 4];

    /// Converts to the tuple type used for ABI encoding and decoding.
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding and decoding.
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self;

    /// The size of the encoded data in bytes, **without** its selector.
    fn data_size(&self) -> usize;

    /// ABI decode this call's arguments from the given slice, **without** its selector.
    #[inline]
    fn decode_raw(data: &[u8], validate: bool) -> Result<Self> {
        let tuple = <Self::Tuple as SolType>::decode(data, validate)?;
        Ok(Self::from_rust(tuple))
    }

    /// ABI decode this call's arguments from the given slice, **with** the selector.
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
        out.extend(<Self::Tuple as SolType>::encode(self.to_rust()));
    }

    /// ABI encode the call to the given buffer **with** its selector.
    #[inline]
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.data_size());
        out.extend(&Self::SELECTOR);
        self.encode_raw(&mut out);
        out
    }
}
