use crate::{utils::keccak256, wrap_fixed_bytes, FixedBytes};
use alloc::{
    borrow::Borrow,
    string::{String, ToString},
};
use core::{fmt, str};

/// Error type for address checksum validation.
#[derive(Debug, Copy, Clone)]
pub enum AddressError {
    /// Error while decoding hex.
    Hex(hex::FromHexError),

    /// Invalid ERC-55 checksum.
    InvalidChecksum,
}

impl From<hex::FromHexError> for AddressError {
    fn from(value: hex::FromHexError) -> Self {
        Self::Hex(value)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AddressError {}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(err) => err.fmt(f),
            Self::InvalidChecksum => f.write_str("Bad address checksum"),
        }
    }
}

wrap_fixed_bytes!(
    /// An Ethereum address, 20 bytes in length.
    Address<20>
);

impl Borrow<[u8; 20]> for Address {
    #[inline]
    fn borrow(&self) -> &[u8; 20] {
        &self.0
    }
}

impl From<Address> for FixedBytes<32> {
    #[inline]
    fn from(addr: Address) -> Self {
        addr.into_word()
    }
}

impl Address {
    /// Creates an Ethereum address from an EVM word's upper 20 bytes.
    #[inline]
    pub fn from_word(hash: FixedBytes<32>) -> Self {
        Self(FixedBytes(hash[12..].try_into().unwrap()))
    }

    /// Left-pads the address to 32 bytes (EVM word size).
    #[inline]
    pub fn into_word(self) -> FixedBytes<32> {
        let mut buf = [0; 32];
        buf[12..].copy_from_slice(self.as_bytes());
        FixedBytes(buf)
    }

    /// Parse an Ethereum address, verifying its [EIP-55] checksum.
    ///
    /// You can optionally specify an [EIP-155 chain ID] to check the address
    /// using [EIP-1191].
    ///
    /// # Errors
    ///
    /// If the provided string does not match the expected checksum.
    ///
    /// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
    /// [EIP-155 chain ID]: https://eips.ethereum.org/EIPS/eip-155
    /// [EIP-1191]: https://eips.ethereum.org/EIPS/eip-1191
    pub fn parse_checksummed<S: AsRef<str>>(
        s: S,
        chain_id: Option<u64>,
    ) -> Result<Self, AddressError> {
        fn inner(s: &str, chain_id: Option<u64>) -> Result<Address, AddressError> {
            if !s.starts_with("0x") {
                return Err(AddressError::Hex(hex::FromHexError::InvalidStringLength))
            }

            let address: Address = s.parse()?;
            let buf = &mut [0; 42];
            let ss = address.to_checksum_raw(buf, chain_id);
            if s == ss {
                Ok(address)
            } else {
                Err(AddressError::InvalidChecksum)
            }
        }

        inner(s.as_ref(), chain_id)
    }

    /// Encodes an Ethereum address to its [EIP-55] checksum.
    ///
    /// You can optionally specify an [EIP-155 chain ID] to encode the address
    /// using [EIP-1191].
    ///
    /// # Panics
    ///
    /// If `addr_buf` is not 42 bytes long.
    ///
    /// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
    /// [EIP-155 chain ID]: https://eips.ethereum.org/EIPS/eip-155
    /// [EIP-1191]: https://eips.ethereum.org/EIPS/eip-1191
    pub fn to_checksum_raw<'a>(&self, addr_buf: &'a mut [u8], chain_id: Option<u64>) -> &'a str {
        assert_eq!(addr_buf.len(), 42, "addr_buf must be 42 bytes long");
        addr_buf[0] = b'0';
        addr_buf[1] = b'x';
        hex::encode_to_slice(self.as_bytes(), &mut addr_buf[2..]).unwrap();

        let mut storage;
        let to_hash = match chain_id {
            Some(chain_id) => {
                // A decimal `u64` string is at most 20 bytes long: round up 20 + 42 to 64.
                storage = [0u8; 64];

                // Format the `chain_id` into a stack-allocated buffer using `itoa`
                let mut temp = itoa::Buffer::new();
                let prefix_str = temp.format(chain_id);
                let prefix_len = prefix_str.len();
                debug_assert!(prefix_len <= 20);
                let len = prefix_len + 42;

                // SAFETY: prefix_len <= 20; len <= 62; storage.len() == 64
                unsafe {
                    storage
                        .get_unchecked_mut(..prefix_len)
                        .copy_from_slice(prefix_str.as_bytes());
                    storage
                        .get_unchecked_mut(prefix_len..len)
                        .copy_from_slice(addr_buf);
                }
                &storage[..len]
            }
            None => &addr_buf[2..],
        };
        let hash = keccak256(to_hash);
        let mut hash_hex = [0u8; 64];
        hex::encode_to_slice(hash.as_bytes(), &mut hash_hex).unwrap();

        // generates significantly less code than zipping the two arrays, or
        // `.into_iter()`
        for (i, x) in hash_hex.iter().enumerate().take(40) {
            if *x >= b'8' {
                // SAFETY: `addr_buf` is 42 bytes long, `2..42` is always in range
                unsafe { addr_buf.get_unchecked_mut(i + 2).make_ascii_uppercase() };
            }
        }

        // SAFETY: All bytes in the buffer are valid UTF-8
        unsafe { str::from_utf8_unchecked(addr_buf) }
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

    /// Computes the `create` address for the given address and nonce.
    ///
    /// The address for an Ethereum contract is deterministically computed from
    /// the address of its creator (sender) and how many transactions the
    /// creator has sent (nonce). The sender and nonce are RLP encoded and
    /// then hashed with [`keccak256`].
    #[cfg(feature = "rlp")]
    pub fn create<T: Borrow<[u8; 20]>>(sender: T, nonce: u64) -> Address {
        fn create(sender: &[u8; 20], nonce: u64) -> Address {
            use ethers_rlp::Encodable;

            let mut out = alloc::vec::Vec::with_capacity(64);
            let buf = &mut out as &mut dyn bytes::BufMut;
            sender.encode(buf);
            let _ = nonce;
            #[cfg(TODO_UINT_RLP)]
            crate::U256::from(nonce).encode(buf);
            let hash = keccak256(&out);
            Address::from_word(hash)
        }

        create(sender.borrow(), nonce)
    }

    /// Returns the `CREATE2` address of a smart contract as specified in
    /// [EIP1014](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1014.md):
    ///
    /// `keccak256(0xff ++ address ++ salt ++ keccak256(init_code))[12:]`
    pub fn create2_from_code<A, S, C>(address: A, salt: S, init_code: C) -> Address
    where
        A: Borrow<[u8; 20]>,
        S: Borrow<[u8; 32]>,
        C: AsRef<[u8]>,
    {
        Self::create2(address, salt, keccak256(init_code.as_ref()).0)
    }

    /// Returns the `CREATE2` address of a smart contract as specified in
    /// [EIP1014](https://eips.ethereum.org/EIPS/eip-1014),
    /// taking the pre-computed hash of the init code as input:
    ///
    /// `keccak256(0xff ++ address ++ salt ++ init_code_hash)[12:]`
    pub fn create2<A, S, H>(address: A, salt: S, init_code_hash: H) -> Address
    where
        // not `AsRef` because `[u8; N]` does not implement `AsRef<[u8; N]>`
        A: Borrow<[u8; 20]>,
        S: Borrow<[u8; 32]>,
        H: Borrow<[u8; 32]>,
    {
        fn create2_address(
            address: &[u8; 20],
            salt: &[u8; 32],
            init_code_hash: &[u8; 32],
        ) -> Address {
            let mut bytes = [0; 85];
            bytes[0] = 0xff;
            bytes[1..21].copy_from_slice(address);
            bytes[21..53].copy_from_slice(salt);
            bytes[53..85].copy_from_slice(init_code_hash);
            let hash = keccak256(bytes);
            Address::from_word(hash)
        }

        create2_address(address.borrow(), salt.borrow(), init_code_hash.borrow())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            "0x0102030405060708090a0b0c0d0e0f1011121314"
                .parse::<Address>()
                .unwrap()
                .to_fixed_bytes(),
            hex_literal::hex!("0102030405060708090a0b0c0d0e0f1011121314")
        );
    }

    #[test]
    fn eip_55() {
        let addresses = [
            // All caps
            "0x52908400098527886E0F7030069857D2E4169EE7",
            "0x8617E340B3D01FA5F11F306F4090FD50E238070D",
            // All Lower
            "0xde709f2102306220921060314715629080e2fb77",
            "0x27b1fdb04752bbc536007a920d24acb045561c26",
            // Normal
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
            "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
            "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
            "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
        ];
        for addr in addresses {
            let parsed1: Address = addr.parse().unwrap();
            let parsed2 = Address::parse_checksummed(addr, None).unwrap();
            assert_eq!(parsed1, parsed2);
            assert_eq!(parsed2.to_checksum(None), addr);
        }
    }

    #[test]
    fn eip_1191() {
        let eth_mainnet = [
            "0x27b1fdb04752bbc536007a920d24acb045561c26",
            "0x3599689E6292b81B2d85451025146515070129Bb",
            "0x42712D45473476b98452f434e72461577D686318",
            "0x52908400098527886E0F7030069857D2E4169EE7",
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
            "0x6549f4939460DE12611948b3f82b88C3C8975323",
            "0x66f9664f97F2b50F62D13eA064982f936dE76657",
            "0x8617E340B3D01FA5F11F306F4090FD50E238070D",
            "0x88021160C5C792225E4E5452585947470010289D",
            "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
            "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
            "0xde709f2102306220921060314715629080e2fb77",
            "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
        ];
        let rsk_mainnet = [
            "0x27b1FdB04752BBc536007A920D24ACB045561c26",
            "0x3599689E6292B81B2D85451025146515070129Bb",
            "0x42712D45473476B98452f434E72461577d686318",
            "0x52908400098527886E0F7030069857D2E4169ee7",
            "0x5aaEB6053f3e94c9b9a09f33669435E7ef1bEAeD",
            "0x6549F4939460DE12611948B3F82B88C3C8975323",
            "0x66F9664f97f2B50F62d13EA064982F936de76657",
            "0x8617E340b3D01Fa5f11f306f4090fd50E238070D",
            "0x88021160c5C792225E4E5452585947470010289d",
            "0xD1220A0Cf47c7B9BE7a2e6ba89F429762E7B9adB",
            "0xDBF03B407c01E7CD3cBea99509D93F8Dddc8C6FB",
            "0xDe709F2102306220921060314715629080e2FB77",
            "0xFb6916095cA1Df60bb79ce92cE3EA74c37c5d359",
        ];
        let rsk_testnet = [
            "0x27B1FdB04752BbC536007a920D24acB045561C26",
            "0x3599689e6292b81b2D85451025146515070129Bb",
            "0x42712D45473476B98452F434E72461577D686318",
            "0x52908400098527886E0F7030069857D2e4169EE7",
            "0x5aAeb6053F3e94c9b9A09F33669435E7EF1BEaEd",
            "0x6549f4939460dE12611948b3f82b88C3c8975323",
            "0x66f9664F97F2b50f62d13eA064982F936DE76657",
            "0x8617e340b3D01fa5F11f306F4090Fd50e238070d",
            "0x88021160c5C792225E4E5452585947470010289d",
            "0xd1220a0CF47c7B9Be7A2E6Ba89f429762E7b9adB",
            "0xdbF03B407C01E7cd3cbEa99509D93f8dDDc8C6fB",
            "0xDE709F2102306220921060314715629080e2Fb77",
            "0xFb6916095CA1dF60bb79CE92ce3Ea74C37c5D359",
        ];
        for (addresses, chain_id) in [(eth_mainnet, 1), (rsk_mainnet, 30), (rsk_testnet, 31)] {
            // EIP-1191 test cases treat mainnet as "not adopted"
            let id = if chain_id == 1 { None } else { Some(chain_id) };
            for addr in addresses {
                let parsed1: Address = addr.parse().unwrap();
                let parsed2 = Address::parse_checksummed(addr, id).unwrap();
                assert_eq!(parsed1, parsed2);
                assert_eq!(parsed2.to_checksum(id), addr);
            }
        }
    }

    #[test]
    #[ignore = "Uint RLP"]
    #[cfg(feature = "rlp")]
    fn create() {
        // http://ethereum.stackexchange.com/questions/760/how-is-the-address-of-an-ethereum-contract-computed
        let from = "6ac7ea33f8831ea9dcc53393aaa88b25a785dbf0"
            .parse::<Address>()
            .unwrap();
        for (nonce, expected) in [
            "cd234a471b72ba2f1ccf0a70fcaba648a5eecd8d",
            "343c43a37d37dff08ae8c4a11544c718abb4fcf8",
            "f778b86fa74e846c4f0a1fbd1335fe81c00a0c91",
            "fffd933a0bc612844eaf0c6fe3e5b8e9b6c1d19c",
        ]
        .iter()
        .enumerate()
        {
            let address = Address::create(from, nonce as u64);
            assert_eq!(address, expected.parse::<Address>().unwrap());
        }
    }

    // Test vectors from https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1014.md#examples
    #[test]
    fn eip_1014_create2() {
        for (from, salt, init_code, expected) in &[
            (
                "0000000000000000000000000000000000000000",
                "0000000000000000000000000000000000000000000000000000000000000000",
                "00",
                "4D1A2e2bB4F88F0250f26Ffff098B0b30B26BF38",
            ),
            (
                "deadbeef00000000000000000000000000000000",
                "0000000000000000000000000000000000000000000000000000000000000000",
                "00",
                "B928f69Bb1D91Cd65274e3c79d8986362984fDA3",
            ),
            (
                "deadbeef00000000000000000000000000000000",
                "000000000000000000000000feed000000000000000000000000000000000000",
                "00",
                "D04116cDd17beBE565EB2422F2497E06cC1C9833",
            ),
            (
                "0000000000000000000000000000000000000000",
                "0000000000000000000000000000000000000000000000000000000000000000",
                "deadbeef",
                "70f2b2914A2a4b783FaEFb75f459A580616Fcb5e",
            ),
            (
                "00000000000000000000000000000000deadbeef",
                "00000000000000000000000000000000000000000000000000000000cafebabe",
                "deadbeef",
                "60f3f640a8508fC6a86d45DF051962668E1e8AC7",
            ),
            (
                "00000000000000000000000000000000deadbeef",
                "00000000000000000000000000000000000000000000000000000000cafebabe",
                "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                "1d8bfDC5D46DC4f61D6b6115972536eBE6A8854C",
            ),
            (
                "0000000000000000000000000000000000000000",
                "0000000000000000000000000000000000000000000000000000000000000000",
                "",
                "E33C0C7F7df4809055C3ebA6c09CFe4BaF1BD9e0",
            ),
        ] {
            let from = from.parse::<Address>().unwrap();

            let salt = hex::decode(salt).unwrap();
            let salt: [u8; 32] = salt.try_into().unwrap();

            let init_code = hex::decode(init_code).unwrap();
            let init_code_hash = keccak256(&init_code);

            let expected = expected.parse::<Address>().unwrap();

            assert_eq!(expected, Address::create2(from, salt, init_code_hash));
            assert_eq!(expected, Address::create2_from_code(from, salt, init_code));
        }
    }
}
