use ethers_primitives::B256;

/// Additional fields in the context of a block that contains this transaction.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[cfg_attr(
    any(test, feature = "arbitrary"),
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct TransactionMeta {
    /// Hash of the transaction.
    pub tx_hash: B256,
    /// Index of the transaction in the block
    pub index: u64,
    /// Hash of the block.
    pub block_hash: B256,
    /// Number of the block.
    pub block_number: u64,
    /// Base fee of the block.
    pub base_fee: Option<u64>,
}
