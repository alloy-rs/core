#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use alloc::vec::Vec;
use alloy_primitives::{Bytes, U256};

extern crate alloc;

/// The leaf key for account versions.
pub const VERSION_LEAF_KEY: U256 = U256::ZERO;
/// The leaf key for account balances.
pub const BALANCE_LEAF_KEY: U256 = U256::from_limbs([1, 0, 0, 0]);
/// The leaf key for account nonces.
pub const NONCE_LEAF_KEY: U256 = U256::from_limbs([2, 0, 0, 0]);
/// The leaf key for an account's code hash.
pub const CODE_KECCAK_LEAF_KEY: U256 = U256::from_limbs([3, 0, 0, 0]);
/// The leaf key for an account's code size.
pub const CODE_SIZE_LEAF_KEY: U256 = U256::from_limbs([4, 0, 0, 0]);

/// The offset for storage leafs in an account header.
pub const HEADER_STORAGE_OFFSET: U256 = U256::from_limbs([64, 0, 0, 0]);
/// The offset for code chunk leafs.
pub const CODE_OFFSET: U256 = U256::from_limbs([128, 0, 0, 0]);
/// The width of a verkle node.
pub const VERKLE_NODE_WIDTH: U256 = U256::from_limbs([256, 0, 0, 0]);
/// The offset for storage leafs.
pub const MAIN_STORAGE_OFFSET: U256 = U256::from_limbs([0, 0, 0, 2u64.pow(56)]);

/// Split a contigous chunk of bytecode into multiple, smaller chunks of 32 bytes each.
///
/// The first byte in each chunk is the number of leading bytes that are part of pushdata, i.e. data
/// for a preceding `PUSHn` operation.
///
/// See [EIP-6800](https://eips.ethereum.org/eips/eip-6800) for more information.
pub fn chunk_bytecode(_: Bytes) -> Vec<Bytes> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        BALANCE_LEAF_KEY, CODE_KECCAK_LEAF_KEY, CODE_OFFSET, CODE_SIZE_LEAF_KEY,
        HEADER_STORAGE_OFFSET, MAIN_STORAGE_OFFSET, NONCE_LEAF_KEY, VERKLE_NODE_WIDTH,
        VERSION_LEAF_KEY,
    };
    use alloy_primitives::U256;

    #[test]
    fn eip6800_invariants() {
        // It’s a required invariant that VERKLE_NODE_WIDTH > CODE_OFFSET > HEADER_STORAGE_OFFSET
        // and that HEADER_STORAGE_OFFSET is greater than the leaf keys. Additionally,
        // MAIN_STORAGE_OFFSET must be a power of VERKLE_NODE_WIDTH.
        //
        // See <https://eips.ethereum.org/eips/eip-6800>
        assert!(VERKLE_NODE_WIDTH > CODE_OFFSET && CODE_OFFSET > HEADER_STORAGE_OFFSET);
        assert!(
            HEADER_STORAGE_OFFSET > VERSION_LEAF_KEY
                && HEADER_STORAGE_OFFSET > BALANCE_LEAF_KEY
                && HEADER_STORAGE_OFFSET > NONCE_LEAF_KEY
                && HEADER_STORAGE_OFFSET > CODE_KECCAK_LEAF_KEY
                && HEADER_STORAGE_OFFSET > CODE_SIZE_LEAF_KEY
        );
        assert!(MAIN_STORAGE_OFFSET == VERKLE_NODE_WIDTH.pow(U256::from(31)));
    }
}
