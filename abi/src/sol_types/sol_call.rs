use crate::{no_std_prelude::*, token::TokenSeq, SolDataType, SolType};

/// Solidity Call (a tuple with a selector)
///
/// ### Implementer's Guide
///
///  We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`crate::sol`] proc macro to parse a solidity function definition.
pub trait SolCall: Sized {
    /// The corresponding Token type
    type Token: TokenSeq;
    /// The underlying tuple type which represents the calls's arguments.
    /// If the function call is empty, this will be the unit type `()`
    type Tuple: SolDataType<TokenType = Self::Token>;

    /// The function selector
    const SELECTOR: [u8; 4];

    /// The function name
    const NAME: &'static str;

    /// The error arguments
    const ARGS: &'static [&'static str];

    /// Convert to the tuple type used for ABI encoding/decoding
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding/decoding
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized;

    /// The size (in bytes) of this data when encoded, NOT including the
    /// selector
    fn data_size(&self) -> usize;

    /// Decode function args from an ABI-encoded byte slice WITHOUT its
    /// selector
    fn decode_raw(data: &[u8], validate: bool) -> crate::AbiResult<Self> {
        let tuple = <Self::Tuple as SolType>::decode(data, validate)?;
        Ok(Self::from_rust(tuple))
    }

    /// Decode function args from an ABI-encoded byte slice with its selector
    fn decode(data: &[u8], validate: bool) -> crate::AbiResult<Self>
    where
        Self: Sized,
    {
        if data.len() < 4 {
            return Err(crate::Error::type_check_fail(hex::encode(data), Self::NAME));
        }
        let data = data
            .strip_prefix(&Self::SELECTOR)
            .ok_or_else(|| crate::Error::type_check_fail(hex::encode(&data[..4]), Self::NAME))?;
        Self::decode_raw(data, validate)
    }

    /// Encode the function call to the provided buffer WITHOUT the selector
    fn encode_raw(&self, out: &mut Vec<u8>) {
        out.extend(<Self::Tuple as SolType>::encode(self.to_rust()));
    }

    /// Encode the call to an ABI-encoded byte vector
    fn encode_with_selector(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.data_size());
        out.extend(&Self::SELECTOR);
        self.encode_raw(&mut out);
        out
    }
}
