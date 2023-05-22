use super::{BranchNodeCompact, StoredNibblesSubKey};
use serde::{Deserialize, Serialize};

/// Account storage trie node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct StorageTrieEntry {
    /// The nibbles of the intermediate node
    pub nibbles: StoredNibblesSubKey,
    /// Encoded node.
    pub node: BranchNodeCompact,
}
