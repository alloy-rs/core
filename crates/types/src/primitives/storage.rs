use ethers_primitives::{B256, U256};
use serde::{Deserialize, Serialize};

/// Account storage entry.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct StorageEntry {
    /// Storage key.
    pub key: B256,
    /// Value on storage key.
    pub value: U256,
}

impl From<(B256, U256)> for StorageEntry {
    fn from((key, value): (B256, U256)) -> Self {
        StorageEntry { key, value }
    }
}
