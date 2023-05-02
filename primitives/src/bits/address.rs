#[cfg(feature = "std")]
use std::borrow::Borrow;

#[cfg(not(feature = "std"))]
use alloc::{
    borrow::Borrow,
    format,
    string::{String, ToString},
};

use crate::{utils::keccak256, wrap_fixed_bytes, FixedBytes};

/// Error type for address checksum validation
#[derive(Debug, Copy, Clone)]
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

wrap_fixed_bytes!(
    /// An Ethereum address, 20 bytes in length.
    Address<20>
);

impl Borrow<[u8; 20]> for Address {
    fn borrow(&self) -> &[u8; 20] {
        self.0.borrow()
    }
}

impl From<Address> for [u8; 20] {
    fn from(addr: Address) -> Self {
        addr.0.into()
    }
}

impl From<Address> for FixedBytes<32> {
    fn from(addr: Address) -> Self {
        let mut buf: FixedBytes<32> = Default::default();
        buf[12..].copy_from_slice(addr.as_bytes());
        buf
    }
}

impl Address {
    /// Encodes an Ethereum address to its [EIP-55] checksum.
    ///
    /// You can optionally specify an [EIP-155 chain ID] to encode the address
    /// using [EIP-1191].
    ///
    /// # Panics
    ///
    /// If `addr_buf` is shorter than 42 bytes.
    ///
    /// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
    /// [EIP-155 chain ID]: https://eips.ethereum.org/EIPS/eip-155
    /// [EIP-1191]: https://eips.ethereum.org/EIPS/eip-1191
    pub fn to_checksum_raw<'a>(&'_ self, addr_buf: &'a mut [u8], chain_id: Option<u64>) -> &'a str {
        debug_assert!(
            addr_buf.len() >= 42,
            "addr_buf must be at least 42 bytes long"
        );
        addr_buf[0] = b'0';
        addr_buf[1] = b'x';
        hex::encode_to_slice(self.as_bytes(), &mut addr_buf[2..]).unwrap();

        let prefixed_addr = match chain_id {
            Some(chain_id) => format!("{chain_id}0x{self:x}"),
            None => format!("{self:x}"),
        };
        let hash = keccak256(prefixed_addr);
        let mut hash_hex = [0u8; 64];

        hex::encode_to_slice(hash.as_bytes(), &mut hash_hex).unwrap();

        addr_buf[2..]
            .iter_mut()
            .zip(hash_hex.into_iter())
            .for_each(|(addr_byte, hash_byte)| {
                if hash_byte >= 56 {
                    addr_byte.make_ascii_uppercase();
                }
            });

        // SAFETY: all characters come either from to_hex_raw, which produces
        // only valid UTF8. therefore valid UTF8
        unsafe { core::str::from_utf8_unchecked(&addr_buf[..42]) }
    }

    /// Encodes an Ethereum address to its [EIP-55] checksum.
    ///
    /// You can optionally specify an [EIP-155 chain ID] to encode the address
    /// using [EIP-1191].
    ///
    /// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
    /// [EIP-155 chain ID]: https://eips.ethereum.org/EIPS/eip-155
    /// [EIP-1191]: https://eips.ethereum.org/EIPS/eip-1191
    pub fn to_checksum(&self, chain_id: Option<u64>) -> String {
        let mut buf = [0u8; 42];
        self.to_checksum_raw(&mut buf, chain_id).to_string()
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

    #[test]
    fn eip_55() {
        let addr_list = vec![
            // mainnet
            (
                None,
                "27b1fdb04752bbc536007a920d24acb045561c26",
                "0x27b1fdb04752bbc536007a920d24acb045561c26",
            ),
            (
                None,
                "3599689e6292b81b2d85451025146515070129bb",
                "0x3599689E6292b81B2d85451025146515070129Bb",
            ),
            (
                None,
                "42712d45473476b98452f434e72461577d686318",
                "0x42712D45473476b98452f434e72461577D686318",
            ),
            (
                None,
                "52908400098527886e0f7030069857d2e4169ee7",
                "0x52908400098527886E0F7030069857D2E4169EE7",
            ),
            (
                None,
                "5aaeb6053f3e94c9b9a09f33669435e7ef1beaed",
                "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
            ),
            (
                None,
                "6549f4939460de12611948b3f82b88c3c8975323",
                "0x6549f4939460DE12611948b3f82b88C3C8975323",
            ),
            (
                None,
                "66f9664f97f2b50f62d13ea064982f936de76657",
                "0x66f9664f97F2b50F62D13eA064982f936dE76657",
            ),
            (
                None,
                "88021160c5c792225e4e5452585947470010289d",
                "0x88021160C5C792225E4E5452585947470010289D",
            ),
        ];

        for (chain_id, addr, checksummed_addr) in addr_list {
            let parsed = addr.parse::<Address>().unwrap();
            assert_eq!(parsed.to_checksum(chain_id), String::from(checksummed_addr));
            assert!(Address::parse_checksummed(checksummed_addr, chain_id).is_ok());
            if addr != checksummed_addr {
                assert!(Address::parse_checksummed(addr, chain_id).is_err());
            }
        }
    }

    #[test]
    fn eip_1191() {
        let addr_list = vec![
            // rsk mainnet
            (
                Some(30),
                "27b1fdb04752bbc536007a920d24acb045561c26",
                "0x27b1FdB04752BBc536007A920D24ACB045561c26",
            ),
            (
                Some(30),
                "3599689e6292b81b2d85451025146515070129bb",
                "0x3599689E6292B81B2D85451025146515070129Bb",
            ),
            (
                Some(30),
                "42712d45473476b98452f434e72461577d686318",
                "0x42712D45473476B98452f434E72461577d686318",
            ),
            (
                Some(30),
                "52908400098527886e0f7030069857d2e4169ee7",
                "0x52908400098527886E0F7030069857D2E4169ee7",
            ),
            (
                Some(30),
                "5aaeb6053f3e94c9b9a09f33669435e7ef1beaed",
                "0x5aaEB6053f3e94c9b9a09f33669435E7ef1bEAeD",
            ),
            (
                Some(30),
                "6549f4939460de12611948b3f82b88c3c8975323",
                "0x6549F4939460DE12611948B3F82B88C3C8975323",
            ),
            (
                Some(30),
                "66f9664f97f2b50f62d13ea064982f936de76657",
                "0x66F9664f97f2B50F62d13EA064982F936de76657",
            ),
        ];

        for (chain_id, addr, checksummed_addr) in addr_list {
            let parsed = addr.parse::<Address>().unwrap();
            assert_eq!(parsed.to_checksum(chain_id), String::from(checksummed_addr));
            assert!(Address::parse_checksummed(checksummed_addr, chain_id).is_ok());
            if addr != checksummed_addr {
                assert!(Address::parse_checksummed(addr, chain_id).is_err());
            }
        }
    }
}
