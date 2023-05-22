use crate::constants::KECCAK_EMPTY;
use ethers_primitives::{B256, U256, U64};

/// An Ethereum account.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Account {
    /// Account nonce.
    pub nonce: U64,
    /// Account balance.
    pub balance: U256,
    /// Hash of the account's bytecode.
    pub bytecode_hash: Option<B256>,
}

impl Account {
    /// Whether the account has bytecode.
    pub fn has_bytecode(&self) -> bool {
        self.bytecode_hash.is_some()
    }

    /// After SpuriousDragon empty account is defined as account with nonce == 0
    /// && balance == 0 && bytecode = None.
    pub fn is_empty(&self) -> bool {
        let is_bytecode_empty = match self.bytecode_hash {
            None => true,
            Some(hash) => hash == KECCAK_EMPTY,
        };

        self.nonce == U64::ZERO && self.balance == U256::ZERO && is_bytecode_empty
    }

    /// Returns an account bytecode's hash.
    /// In case of no bytecode, returns [`KECCAK_EMPTY`].
    pub fn get_bytecode_hash(&self) -> B256 {
        match self.bytecode_hash {
            Some(hash) => hash,
            None => KECCAK_EMPTY,
        }
    }
}
