use super::{super::TrieMask, HashBuilderValue};
use serde::{Deserialize, Serialize};

/// The hash builder state for storing in the database.
/// Check the `reth-trie` crate for more info on hash builder.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(
    any(test, feature = "arbitrary"),
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct HashBuilderState {
    /// The current key.
    pub key: Vec<u8>,
    /// The builder stack.
    pub stack: Vec<Vec<u8>>,
    /// The current node value.
    pub value: HashBuilderValue,

    /// Group masks.
    pub groups: Vec<TrieMask>,
    /// Tree masks.
    pub tree_masks: Vec<TrieMask>,
    /// Hash masks.
    pub hash_masks: Vec<TrieMask>,

    /// Flag indicating if the current node is stored in the database.
    pub stored_in_database: bool,
}
