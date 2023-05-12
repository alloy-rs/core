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
    /// The underlying tuple type which represents the error's members.
    /// If the error has no arguments, this will be the unit type `()`
    type Tuple: SolDataType<TokenType = Self::Token>;

    /// The corresponding Token type
    type Token: TokenSeq;

    /// The error ABI signature
    const SIGNATURE: &'static str;

    /// The error selector: `keccak256(SIGNATURE)[0..4]`
    const SELECTOR: [u8; 4];

    /// Convert to the tuple type used for ABI encoding/decoding
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType;

    /// Convert from the tuple type used for ABI encoding/decoding
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized;

    /// The size of the encoded data in bytes, selector excluded
    fn data_size(&self) -> usize;

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
            return Err(crate::Error::type_check_fail_sig(data, Self::SIGNATURE));
        }
        let data = data
            .strip_prefix(&Self::SELECTOR)
            .ok_or_else(|| crate::Error::type_check_fail_sig(&data[..4], Self::SIGNATURE))?;
        Self::decode_raw(data, validate)
    }

    /// Encode the error contents to the provided buffer WITHOUT the error selector
    fn encode_raw(&self, out: &mut Vec<u8>) {
        out.extend(<Self::Tuple as SolType>::encode(self.to_rust()));
    }

    /// Encode an error to an ABI-encoded byte vector
    fn encode_with_selector(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.data_size());
        out.extend(&Self::SELECTOR);
        self.encode_raw(&mut out);
        out
    }
}

/// Represents a standard Solidity revert. These are thrown by
/// `require(condition, reason)` statements in Solidity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Revert {
    /// The reason string, provided by the Solidity contract
    pub reason: String,
}

impl AsRef<str> for Revert {
    fn as_ref(&self) -> &str {
        &self.reason
    }
}

impl Borrow<str> for Revert {
    fn borrow(&self) -> &str {
        &self.reason
    }
}

impl Display for Revert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Revert: {}", self.reason)
    }
}

impl From<Revert> for String {
    fn from(value: Revert) -> Self {
        value.reason
    }
}

impl From<String> for Revert {
    fn from(reason: String) -> Self {
        Self { reason }
    }
}

impl SolError for Revert {
    type Token = (PackedSeqToken,);
    type Tuple = (crate::sol_data::String,);

    const SIGNATURE: &'static str = "Error(string)";
    const SELECTOR: [u8; 4] = [0x08, 0xc3, 0x79, 0xa0];

    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType {
        (self.reason.clone(),)
    }

    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized,
    {
        Self { reason: tuple.0 }
    }

    fn data_size(&self) -> usize {
        let body_words = (self.reason.len() + 31) / 32;
        (2 + body_words) * 32
    }
}

/// Represents a Solidity Panic. These are thrown by
/// `assert(condition, reason)` and by Solidity internal checks.
///
/// [Solidity Panic](https://docs.soliditylang.org/en/v0.8.6/control-structures.html#panic-via-assert-and-error-via-require)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Panic {
    /// The panic code.
    ///
    /// [Solidity Panic Codes](https://docs.soliditylang.org/en/v0.8.6/control-structures.html#panic-via-assert-and-error-via-require)
    pub error_code: U256,
}

impl AsRef<U256> for Panic {
    fn as_ref(&self) -> &U256 {
        &self.error_code
    }
}

impl Borrow<U256> for Panic {
    fn borrow(&self) -> &U256 {
        &self.error_code
    }
}

impl Display for Panic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Panic: {}", self.error_code)
    }
}

impl From<u64> for Panic {
    fn from(value: u64) -> Self {
        Self {
            error_code: U256::from(value),
        }
    }
}

impl From<Panic> for U256 {
    fn from(value: Panic) -> Self {
        value.error_code
    }
}

impl From<U256> for Panic {
    fn from(error_code: U256) -> Self {
        Self { error_code }
    }
}

impl SolError for Panic {
    type Token = (WordToken,);
    type Tuple = (crate::sol_data::Uint<256>,);

    const SIGNATURE: &'static str = "Panic(uint256)";
    const SELECTOR: [u8; 4] = [0x4e, 0x48, 0x7b, 0x71];

    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType {
        (self.error_code,)
    }

    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self
    where
        Self: Sized,
    {
        Self {
            error_code: tuple.0,
        }
    }

    fn data_size(&self) -> usize {
        32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ethers_primitives::keccak256;

    #[test]
    fn test_revert_encoding() {
        let revert = Revert::from("test".to_string());
        let encoded = revert.encode_with_selector();
        let decoded = Revert::decode(&encoded, true).unwrap();
        assert_eq!(encoded.len(), revert.data_size() + 4);
        assert_eq!(encoded.len(), 100);
        assert_eq!(revert, decoded);
    }

    #[test]
    fn test_panic_encoding() {
        let panic = Panic {
            error_code: U256::from(0),
        };
        let encoded = panic.encode_with_selector();
        let decoded = Panic::decode(&encoded, true).unwrap();

        assert_eq!(encoded.len(), panic.data_size() + 4);
        assert_eq!(encoded.len(), 36);
        assert_eq!(panic, decoded);
    }

    #[test]
    fn test_selectors() {
        assert_eq!(
            Revert::SELECTOR,
            &keccak256(b"Error(string)")[..4],
            "Revert selector is incorrect"
        );
        assert_eq!(
            Panic::SELECTOR,
            &keccak256(b"Panic(uint256)")[..4],
            "Panic selector is incorrect"
        );
    }
}
