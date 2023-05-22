use super::BranchNodeCompact;

/// Walker sub node for storing intermediate state root calculation state in the
/// database. See [crate::MerkleCheckpoint].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StoredSubNode {
    /// The key of the current node.
    pub key: Vec<u8>,
    /// The index of the next child to visit.
    pub nibble: Option<u8>,
    /// The node itself.
    pub node: Option<BranchNodeCompact>,
}
