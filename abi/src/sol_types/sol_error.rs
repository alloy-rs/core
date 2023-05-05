use crate::{no_std_prelude::*, token::TokenSeq, SolDataType, SolType};

/// Solidity Error (a tuple with a selector)
///
/// ### Implementer's Guide
///
///  We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`crate::sol`] proc macro to parse a solidity error definition.
pub trait SolError: Sized {
    /// The corresponding Token type
    type Token: TokenSeq;
    /// The underlying tuple type which represents the error's members.
    /// If the error is empty, this will be the unit type `()`
    type Tuple: SolDataType<TokenType = Self::Token>;

    /// The error selector
    const SELECTOR: [u8; 4];

    /// The error name
    const NAME: &'static str;

    /// The error fields
    const FIELDS: &'static [&'static str];

    /// Convert to the tuple type used for ABI encoding/decoding
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding/decoding
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized;

    /// The size (in bytes) of this data when encoded
    fn encoded_size(&self) -> usize;

    /// Decode an error contents from an ABI-encoded byte slice WITHOUT its
    /// selector
    fn decode_raw(data: &[u8], validate: bool) -> crate::AbiResult<Self> {
        let tuple = <Self::Tuple as SolType>::decode(data, validate)?;
        Ok(Self::from_rust(tuple))
    }

    /// Decode an error from an ABI-encoded byte slice with its selector
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

    /// Encode the error contents to the provided buffer WITHOUT the error selector
    fn encode_raw(&self, out: &mut Vec<u8>) {
        out.extend(<Self::Tuple as SolType>::encode(self.to_rust()));
    }

    /// Encode an error to an ABI-encoded byte vector
    fn encode_with_selector(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.encoded_size());
        out.extend(&Self::SELECTOR);
        self.encode_raw(&mut out);
        out
    }
}
