use crate::{bits::FixedBytes, Address};
use core::borrow::Borrow;

pub use tiny_keccak::{Hasher, Keccak};

/// Simple interface to the [`keccak256`] hash function.
///
/// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> FixedBytes<32> {
    fn keccak256(bytes: &[u8]) -> FixedBytes<32> {
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(bytes);
        hasher.finalize(&mut output);
        output.into()
    }

    keccak256(bytes.as_ref())
}

/// Computes the `create` address for the given address and nonce.
///
/// The address for an Ethereum contract is deterministically computed from the
/// address of its creator (sender) and how many transactions the creator has
/// sent (nonce). The sender and nonce are RLP encoded and then hashed with
/// [`keccak256`].
#[cfg(feature = "rlp")]
pub fn create_address<T: Borrow<[u8; 20]>>(sender: T, nonce: u64) -> Address {
    fn create_address(sender: &[u8; 20], nonce: u64) -> Address {
        use ethers_rlp::Encodable;

        let mut out = alloc::vec::Vec::with_capacity(64);
        let buf = &mut out as &mut dyn bytes::BufMut;
        sender.encode(buf);
        crate::U256::from(nonce).encode(buf);
        let hash = keccak256(&out);
        Address::from_word(hash)
    }

    create_address(sender.borrow(), nonce)
}

/// Returns the `CREATE2` address of a smart contract as specified in
/// [EIP1014](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1014.md):
///
/// `keccak256(0xff ++ address ++ salt ++ keccak256(init_code))[12:]`
pub fn create2_address_from_code<A, S, C>(address: A, salt: S, init_code: C) -> Address
where
    A: Borrow<[u8; 20]>,
    S: Borrow<[u8; 32]>,
    C: AsRef<[u8]>,
{
    create2_address(address, salt, keccak256(init_code.as_ref()).0)
}

/// Returns the `CREATE2` address of a smart contract as specified in
/// [EIP1014](https://eips.ethereum.org/EIPS/eip-1014),
/// taking the pre-computed hash of the init code as input:
///
/// `keccak256(0xff ++ address ++ salt ++ init_code_hash)[12:]`
pub fn create2_address<A, S, H>(address: A, salt: S, init_code_hash: H) -> Address
where
    // not `AsRef` because `[u8; N]` does not implement `AsRef<[u8; N]>`
    A: Borrow<[u8; 20]>,
    S: Borrow<[u8; 32]>,
    H: Borrow<[u8; 32]>,
{
    fn create2_address(address: &[u8; 20], salt: &[u8; 32], init_code_hash: &[u8; 32]) -> Address {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Uint RLP"]
    #[cfg(feature = "rlp")]
    fn test_create_address() {
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
            let address = create_address(from, nonce as u64);
            assert_eq!(address, expected.parse::<Address>().unwrap());
        }
    }

    // Test vectors from https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1014.md#examples
    #[test]
    fn test_create2_address() {
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

            assert_eq!(expected, create2_address(from, salt, init_code_hash));
            assert_eq!(expected, create2_address_from_code(from, salt, init_code));
        }
    }
}
