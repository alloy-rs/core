//! Ethereum protocol-related constants.

use alloy_primitives::{b256, FixedBytes, B256, U256};
use core::time::Duration;

pub use alloy_primitives::{BLOOM_BITS_PER_ITEM, BLOOM_SIZE_BITS, BLOOM_SIZE_BYTES};

/// The first four bytes of the call data for a function call specifies the
/// function to be called.
pub const SELECTOR_LEN: usize = 4;

/// Maximum extra data size in a block after genesis
pub const MAXIMUM_EXTRA_DATA_SIZE: usize = 32;

/// An EPOCH is a series of 32 slots.
pub const EPOCH_SLOTS: u64 = 32;

/// The duration of a slot in seconds.
///
/// This is the time period of 12 seconds in which a randomly chosen validator
/// has time to propose a block.
pub const SLOT_DURATION: Duration = Duration::from_secs(12);

/// An EPOCH is a series of 32 slots (~6.4min).
pub const EPOCH_DURATION: Duration = Duration::from_secs(12 * EPOCH_SLOTS);

/// The default block nonce in the beacon consensus
pub const BEACON_NONCE: u64 = 0u64;

/// The default Ethereum block gas limit.
///
/// TODO: This should be a chain spec parameter.
/// See <https://github.com/paradigmxyz/reth/issues/3233>.
pub const ETHEREUM_BLOCK_GAS_LIMIT: u64 = 30_000_000;

/// The minimal value the basefee can decrease to.
///
/// The `BASE_FEE_MAX_CHANGE_DENOMINATOR` <https://eips.ethereum.org/EIPS/eip-1559> is `8`, or 12.5%.
/// Once the base fee has dropped to `7` WEI it cannot decrease further because
/// 12.5% of 7 is less than 1.
pub const MIN_PROTOCOL_BASE_FEE: u128 = 7;

/// Same as [MIN_PROTOCOL_BASE_FEE] but as a U256.
pub const MIN_PROTOCOL_BASE_FEE_U256: U256 = U256::from_limbs([7u64, 0, 0, 0]);

/// Initial base fee as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_INITIAL_BASE_FEE: u64 = 1_000_000_000;

/// Base fee max change denominator as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR: u64 = 8;

/// Elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_ELASTICITY_MULTIPLIER: u64 = 2;

/// Multiplier for converting gwei to wei.
pub const GWEI_TO_WEI: u64 = 1_000_000_000;

/// Multiplier for converting finney (milliether) to wei.
pub const FINNEY_TO_WEI: u128 = (GWEI_TO_WEI as u128) * 1_000_000;

/// Multiplier for converting ether to wei.
pub const ETH_TO_WEI: u128 = FINNEY_TO_WEI * 1000;

/// Multiplier for converting mgas to gas.
pub const MGAS_TO_GAS: u64 = 1_000_000u64;

/// The Ethereum mainnet genesis hash.
pub const MAINNET_GENESIS: B256 =
    b256!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3");

/// Goerli genesis hash.
pub const GOERLI_GENESIS: B256 =
    b256!("bf7e331f7f7c1dd2e05159666b3bf8bc7a8a3a9eb1d518969eab529dd9b88c1a");

/// Sepolia genesis hash.
pub const SEPOLIA_GENESIS: B256 =
    b256!("25a5cc106eea7138acab33231d7160d69cb777ee0c2c553fcddf5138993e6dd9");

/// Keccak256 over empty array.
pub const KECCAK_EMPTY: B256 =
    b256!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");

/// Hash of an empty RLP set: `keccak256(rlp([]))`
const EMPTY_SET_HASH: B256 =
    b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");

/// Hash of an empty RLP list: `keccak256("\xc0")`
pub const EMPTY_LIST_HASH: B256 =
    b256!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347");

/// Root hash of an empty trie.
pub const EMPTY_ROOT: B256 = EMPTY_SET_HASH;

/// Ommer root of empty list.
pub const EMPTY_OMMER_ROOT: B256 = EMPTY_LIST_HASH;

/// Transactions root of empty receipts set.
pub const EMPTY_RECEIPTS: B256 = EMPTY_SET_HASH;

/// Transactions root of empty transactions set.
pub const EMPTY_TRANSACTIONS: B256 = EMPTY_SET_HASH;

/// Withdrawals root of empty withdrawals set.
pub const EMPTY_WITHDRAWALS: B256 = EMPTY_SET_HASH;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_protocol_sanity() {
        assert_eq!(
            MIN_PROTOCOL_BASE_FEE_U256.to::<u128>(),
            MIN_PROTOCOL_BASE_FEE
        );
    }
}
