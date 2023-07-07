use crate::trie::{hash_builder::HashBuilderState, StoredSubNode};
use alloy_primitives::{Address, BlockNumber, TxNumber, B256};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Saves the progress of Merkle stage.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MerkleCheckpoint {
    /// The target block number.
    pub target_block: BlockNumber,
    /// The last hashed account key processed.
    pub last_account_key: B256,
    /// The last walker key processed.
    pub last_walker_key: Vec<u8>,
    /// Previously recorded walker stack.
    pub walker_stack: Vec<StoredSubNode>,
    /// The hash builder state.
    pub state: HashBuilderState,
}

impl MerkleCheckpoint {
    /// Creates a new Merkle checkpoint.
    pub fn new(
        target_block: BlockNumber,
        last_account_key: B256,
        last_walker_key: Vec<u8>,
        walker_stack: Vec<StoredSubNode>,
        state: HashBuilderState,
    ) -> Self {
        Self {
            target_block,
            last_account_key,
            last_walker_key,
            walker_stack,
            state,
        }
    }
}

/// Saves the progress of AccountHashing
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountHashingCheckpoint {
    /// The next account to start hashing from
    pub address: Option<Address>,
    /// Start transition id
    pub from: u64,
    /// Last transition id
    pub to: u64,
}

/// Saves the progress of StorageHashing
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StorageHashingCheckpoint {
    /// The next account to start hashing from
    pub address: Option<Address>,
    /// The next storage slot to start hashing from
    pub storage: Option<B256>,
    /// Start transition id
    pub from: u64,
    /// Last transition id
    pub to: u64,
}

/// Saves the progress of a stage.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct StageCheckpoint {
    /// The maximum block processed by the stage.
    pub block_number: BlockNumber,
    /// Stage-specific checkpoint. None if stage uses only block-based
    /// checkpoints.
    pub stage_checkpoint: Option<StageUnitCheckpoint>,
}

impl StageCheckpoint {
    /// Creates a new [`StageCheckpoint`] with only `block_number` set.
    pub fn new(block_number: BlockNumber) -> Self {
        Self {
            block_number,
            ..Default::default()
        }
    }

    /// Returns the account hashing stage checkpoint, if any.
    pub fn account_hashing_stage_checkpoint(&self) -> Option<AccountHashingCheckpoint> {
        match self.stage_checkpoint {
            Some(StageUnitCheckpoint::Account(checkpoint)) => Some(checkpoint),
            _ => None,
        }
    }

    /// Returns the storage hashing stage checkpoint, if any.
    pub fn storage_hashing_stage_checkpoint(&self) -> Option<StorageHashingCheckpoint> {
        match self.stage_checkpoint {
            Some(StageUnitCheckpoint::Storage(checkpoint)) => Some(checkpoint),
            _ => None,
        }
    }

    /// Sets the stage checkpoint to account hashing.
    pub fn with_account_hashing_stage_checkpoint(
        mut self,
        checkpoint: AccountHashingCheckpoint,
    ) -> Self {
        self.stage_checkpoint = Some(StageUnitCheckpoint::Account(checkpoint));
        self
    }

    /// Sets the stage checkpoint to storage hashing.
    pub fn with_storage_hashing_stage_checkpoint(
        mut self,
        checkpoint: StorageHashingCheckpoint,
    ) -> Self {
        self.stage_checkpoint = Some(StageUnitCheckpoint::Storage(checkpoint));
        self
    }
}

// TODO(alexey): ideally, we'd want to display block number + stage-specific
// metric (if available)  in places like logs or traces
impl Display for StageCheckpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.block_number, f)
    }
}

// TODO(alexey): add a merkle checkpoint. Currently it's hard because
// [`MerkleCheckpoint`]  is not a Copy type.
/// Stage-specific checkpoint metrics.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum StageUnitCheckpoint {
    /// Saves the progress of transaction-indexed stages.
    Transaction(TxNumber),
    /// Saves the progress of AccountHashing stage.
    Account(AccountHashingCheckpoint),
    /// Saves the progress of StorageHashing stage.
    Storage(StorageHashingCheckpoint),
}
