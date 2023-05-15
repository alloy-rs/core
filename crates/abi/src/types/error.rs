use crate::{
    no_std_prelude::*,
    token::{PackedSeqToken, TokenSeq, WordToken},
    Result, SolType,
};
use core::fmt::Display;
use ethers_primitives::U256;

/// Solidity Error (a tuple with a selector)
///
/// ### Implementer's Guide
///
/// We do not recommend implementing this trait directly. Instead, we recommend
/// using the [`sol`][crate::sol] proc macro to parse a Solidity error definition.
pub trait SolError: Sized {
    /// The underlying tuple type which represents the error's members.
    ///
    /// If the error has no arguments, this will be the unit type `()`
    type Tuple: SolType<TokenType = Self::Token>;

    /// The corresponding [`TokenSeq`] type.
    type Token: TokenSeq;

    /// The error's ABI signature.
    const SIGNATURE: &'static str;

    /// The error selector: `keccak256(SIGNATURE)[0..4]`
    const SELECTOR: [u8; 4];

    /// Convert to the tuple type used for ABI encoding and decoding.
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

    /// ABI decode this error's arguments from the given slice, **with** the selector.
    #[inline]
    fn decode(data: &[u8], validate: bool) -> Result<Self> {
        let data = data
            .strip_prefix(&Self::SELECTOR)
            .ok_or_else(|| crate::Error::type_check_fail_sig(data, Self::SIGNATURE))?;
        Self::decode_raw(data, validate)
    }

    /// ABI encode the error to the given buffer **without** its selector.
    #[inline]
    fn encode_raw(&self, out: &mut Vec<u8>) {
        out.extend(<Self::Tuple as SolType>::encode(self.to_rust()));
    }

    /// ABI encode the error to the given buffer **with** its selector.
    #[inline]
    fn encode(&self) -> Vec<u8> {
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
    /// The reason string, provided by the Solidity contract.
    pub reason: String,
}

impl AsRef<str> for Revert {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.reason
    }
}

impl Borrow<str> for Revert {
    #[inline]
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
    #[inline]
    fn from(value: Revert) -> Self {
        value.reason
    }
}

impl From<String> for Revert {
    #[inline]
    fn from(reason: String) -> Self {
        Self { reason }
    }
}

impl SolError for Revert {
    type Token = (PackedSeqToken,);
    type Tuple = (crate::sol_data::String,);

    const SIGNATURE: &'static str = "Error(string)";
    const SELECTOR: [u8; 4] = [0x08, 0xc3, 0x79, 0xa0];

    #[inline]
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType {
        (self.reason.clone(),)
    }

    #[inline]
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self {
        Self { reason: tuple.0 }
    }

    #[inline]
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
    #[inline]
    fn as_ref(&self) -> &U256 {
        &self.error_code
    }
}

impl Borrow<U256> for Panic {
    #[inline]
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
    #[inline]
    fn from(value: u64) -> Self {
        Self {
            error_code: U256::from(value),
        }
    }
}

impl From<Panic> for U256 {
    #[inline]
    fn from(value: Panic) -> Self {
        value.error_code
    }
}

impl From<U256> for Panic {
    #[inline]
    fn from(error_code: U256) -> Self {
        Self { error_code }
    }
}

impl SolError for Panic {
    type Token = (WordToken,);
    type Tuple = (crate::sol_data::Uint<256>,);

    const SIGNATURE: &'static str = "Panic(uint256)";
    const SELECTOR: [u8; 4] = [0x4e, 0x48, 0x7b, 0x71];

    #[inline]
    fn to_rust(&self) -> <Self::Tuple as SolType>::RustType {
        (self.error_code,)
    }

    #[inline]
    fn from_rust(tuple: <Self::Tuple as SolType>::RustType) -> Self {
        Self {
            error_code: tuple.0,
        }
    }

    #[inline]
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
        let encoded = revert.encode();
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
        let encoded = panic.encode();
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
