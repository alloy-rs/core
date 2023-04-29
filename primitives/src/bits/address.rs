use derive_more::{AsRef, Deref};
use fixed_hash::{construct_fixed_hash, impl_fixed_hash_conversions};

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;
#[cfg(feature = "arbitrary")]
use proptest_derive::Arbitrary as PropTestArbitrary;

#[cfg(feature = "std")]
use std::borrow::Borrow;

#[cfg(not(feature = "std"))]
use alloc::{borrow::Borrow, format, string::String};

use crate::utils::keccak256;

use super::{hex, B160, B256};

/// Error type for address checksum validation
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AddressError {
    /// FromHexError
    FromHexError(hex::FromHexError),
    /// Invalid checksum for address. Expected chain id.
    ChecksumError {
        /// The chain ID the user expected the address to be bound to
        expected_chain_id: Option<u64>,
    },
}

impl From<hex::FromHexError> for AddressError {
    fn from(value: hex::FromHexError) -> Self {
        Self::FromHexError(value)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AddressError {}

impl core::fmt::Display for AddressError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AddressError::FromHexError(err) => write!(f, "AddressError: {}", err),
            AddressError::ChecksumError { expected_chain_id } => match expected_chain_id {
                Some(chain_id) => write!(
                    f,
                    "Invalid checksum for address on chain with id {}",
                    chain_id
                ),
                None => write!(f, "Invalid checksum for address"),
            },
        }
    }
}

construct_fixed_hash! {
    /// Ethereum address type
    #[cfg_attr(feature = "arbitrary", derive(Arbitrary, PropTestArbitrary))]
    #[derive(AsRef, Deref)]
    pub struct Address(20);
}

// manual impls because `impl_fixed_hash_conversions` macro requires one type to be smalelr
impl From<B160> for Address {
    fn from(value: B160) -> Self {
        value.as_fixed_bytes().into()
    }
}

impl From<Address> for B160 {
    fn from(value: Address) -> Self {
        value.0.into()
    }
}

impl_fixed_hash_conversions!(B256, Address);

impl Borrow<[u8; 20]> for Address {
    fn borrow(&self) -> &[u8; 20] {
        &self.0
    }
}

impl Address {
    /// Encodes an Ethereum address to its [EIP-55] checksum.
    ///
    /// You can optionally specify an [EIP-155 chain ID] to encode the address
    /// using [EIP-1191].
    ///
    /// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
    /// [EIP-155 chain ID]: https://eips.ethereum.org/EIPS/eip-155
    /// [EIP-1191]: https://eips.ethereum.org/EIPS/eip-1191
    pub fn to_checksum(&self, chain_id: Option<u64>) -> String {
        let prefixed_addr = match chain_id {
            Some(chain_id) => format!("{chain_id}0x{self:x}"),
            None => format!("{self:x}"),
        };

        let hash = keccak256(prefixed_addr);
        let mut hash_hex = [0u8; 64];
        hex::to_hex_raw(&mut hash_hex, hash.as_bytes(), false, false);

        let mut addr_hex = [0u8; 42];
        hex::to_hex_raw(&mut addr_hex, self.as_bytes(), false, false);

        addr_hex
            .iter_mut()
            .zip(hash.into_iter())
            .for_each(|(addr_byte, hash_byte)| {
                if hash_byte >= 56 {
                    *addr_byte = addr_byte.to_ascii_uppercase();
                }
            });

        unsafe { String::from_utf8_unchecked(addr_hex.to_vec()) }
    }

    /// Parse an Ethereum address, verifying its [EIP-55] checksum.
    ///
    /// You can optionally specify an [EIP-155 chain ID] to check the address
    /// using [EIP-1191].
    ///
    /// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
    /// [EIP-155 chain ID]: https://eips.ethereum.org/EIPS/eip-155
    /// [EIP-1191]: https://eips.ethereum.org/EIPS/eip-1191
    pub fn parse_checksummed<S: AsRef<str>>(
        s: S,
        chain_id: Option<u64>,
    ) -> Result<Self, AddressError> {
        // TODO
        let s = s.as_ref();
        let candidate: Self = s.parse().unwrap();
        if s != candidate.to_checksum(chain_id) {
            return Err(AddressError::ChecksumError {
                expected_chain_id: chain_id,
            });
        }
        Ok(candidate)
    }
}

#[cfg(test)]
mod test {
    use super::Address;

    #[test]
    fn it_parses() {
        let _a: Address = "0x0102030405060708090a0b0c0d0e0f1011121314"
            .parse()
            .unwrap();
    }
}
