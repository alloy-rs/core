use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

/// The current value of the hash builder.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arbitrary",
    derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub enum HashBuilderValue {
    /// Value of the leaf node.
    Hash(B256),
    /// Hash of adjacent nodes.
    Bytes(Vec<u8>),
}

impl std::fmt::Debug for HashBuilderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bytes(bytes) => write!(f, "Bytes({:?})", hex::encode(bytes)),
            Self::Hash(hash) => write!(f, "Hash({:?})", hash),
        }
    }
}

impl From<Vec<u8>> for HashBuilderValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<&[u8]> for HashBuilderValue {
    fn from(value: &[u8]) -> Self {
        Self::Bytes(value.to_vec())
    }
}

impl From<B256> for HashBuilderValue {
    fn from(value: B256) -> Self {
        Self::Hash(value)
    }
}

impl Default for HashBuilderValue {
    fn default() -> Self {
        Self::Bytes(vec![])
    }
}
