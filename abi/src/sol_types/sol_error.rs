use core::fmt::Display;

use ethers_primitives::U256;

use crate::{
    no_std_prelude::*,
    token::{PackedSeqToken, TokenSeq, WordToken},
    SolDataType, SolType,
};

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

    /// The size (in bytes) of this data when encoded, including the selector
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

/// Represents a standard Solidity revert. These are thrown by
/// `require(condition, reason)` statements in Solidity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Revert(pub String);

impl AsRef<str> for Revert {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Revert {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Display for Revert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Revert: {}", self.0)
    }
}

impl From<Revert> for String {
    fn from(value: Revert) -> Self {
        value.0
    }
}

impl From<String> for Revert {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Revert {
    /// Get the reason string for the revert
    pub fn reason(&self) -> &str {
        &self.0
    }
}

impl SolError for Revert {
    type Token = (PackedSeqToken,);

    type Tuple = (crate::sol_data::String,);

    // Selector for `"Error(string)"`
    const SELECTOR: [u8; 4] = [0x08, 0xc3, 0x79, 0xa0];

    const NAME: &'static str = "Error";

    const FIELDS: &'static [&'static str] = &["reason"];

    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType {
        (self.0.clone(),)
    }

    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized,
    {
        Self(tuple.0)
    }

    fn encoded_size(&self) -> usize {
        64 + (self.0.len() + 31) / 32
    }
}

/// Represents a Solidity Panic. These are thrown by
/// `assert(condition, reason)` and by Solidity internal checks.
///
/// [Solidity Panic](https://docs.soliditylang.org/en/v0.8.6/control-structures.html#panic-via-assert-and-error-via-require)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Panic(pub U256);

impl AsRef<U256> for Panic {
    fn as_ref(&self) -> &U256 {
        &self.0
    }
}

impl Borrow<U256> for Panic {
    fn borrow(&self) -> &U256 {
        &self.0
    }
}

impl Display for Panic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Panic: {}", self.0)
    }
}

impl From<Panic> for U256 {
    fn from(value: Panic) -> Self {
        value.0
    }
}

impl From<U256> for Panic {
    fn from(s: U256) -> Self {
        Self(s)
    }
}

impl Panic {
    /// Get the reason code for the panic
    pub const fn code(&self) -> U256 {
        self.0
    }
}

impl SolError for Panic {
    type Token = (WordToken,);

    type Tuple = (crate::sol_data::Uint<256>,);

    const SELECTOR: [u8; 4] = [0x4e, 0x48, 0x7b, 0x71];

    const NAME: &'static str = "Panic";

    const FIELDS: &'static [&'static str] = &["code"];

    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType {
        (self.0,)
    }

    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized,
    {
        Self(tuple.0)
    }

    fn encoded_size(&self) -> usize {
        36
    }
}
