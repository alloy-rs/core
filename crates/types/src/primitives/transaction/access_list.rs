use alloy_primitives::{Address, B256, U256};
use alloy_rlp::{RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper};
use serde::{Deserialize, Serialize};

/// A list of addresses and storage keys that the transaction plans to access.
/// Accesses outside the list are possible, but become more expensive.
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, Default, RlpDecodable, RlpEncodable, Deserialize, Serialize,
)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "arbitrary",
    derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct AccessListItem {
    /// Account addresses that would be loaded at the start of execution
    pub address: Address,
    /// Keys of storage that would be loaded at the start of execution
    pub storage_keys: Vec<B256>,
}

/// AccessList as defined in EIP-2930
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    RlpDecodableWrapper,
    RlpEncodableWrapper,
    Deserialize,
    Serialize,
)]
#[cfg_attr(
    feature = "arbitrary",
    derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct AccessList(pub Vec<AccessListItem>);

impl AccessList {
    /// Converts the list into a vec, expected by revm
    pub fn flattened(self) -> Vec<(Address, Vec<U256>)> {
        self.0
            .into_iter()
            .map(|item| {
                (
                    item.address,
                    item.storage_keys
                        .into_iter()
                        .map(|slot| U256::from_be_bytes(slot.0))
                        .collect(),
                )
            })
            .collect()
    }
}

/// Access list with gas used appended.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "arbitrary",
    derive(derive_arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct AccessListWithGasUsed {
    /// List with accounts accessed during transaction.
    pub access_list: AccessList,
    /// Estimated gas used with access list.
    pub gas_used: U256,
}
