use ethers_primitives::{Address, Bytes, B256, U256};
use serde::{Deserialize, Serialize};

/// Ethereum Log emitted by a transaction
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    /// Address
    pub address: Address,
    /// All topics of the log
    pub topics: Vec<B256>,
    /// Additional data fields of the log
    pub data: Bytes,
    /// Hash of the block the transaction that emitted this log was mined in
    pub block_hash: Option<B256>,
    /// Number of the block the transaction that emitted this log was mined in
    pub block_number: Option<U256>,
    /// Transaction Hash
    pub transaction_hash: Option<B256>,
    /// Index of the Transaction in the block
    pub transaction_index: Option<U256>,
    /// Log Index in Block
    pub log_index: Option<U256>,
    /// Geth Compatibility Field: whether this log was removed
    #[serde(default)]
    pub removed: bool,
}

impl Log {
    /// Creates a new rpc Log from a primitive log type from DB
    pub fn from_primitive(log: crate::primitives::Log) -> Self {
        Self {
            address: log.address,
            topics: log.topics,
            data: log.data,
            block_hash: None,
            block_number: None,
            transaction_hash: None,
            transaction_index: None,
            log_index: None,
            removed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_log() {
        let log = Log {
            address: Address::with_first_byte(1),
            topics: vec![B256::with_first_byte(2)],
            data: Bytes::from(vec![0x12, 0x34]),
            block_hash: Some(B256::with_first_byte(3)),
            block_number: Some(U256::from(4)),
            transaction_hash: Some(B256::with_first_byte(5)),
            transaction_index: Some(U256::from(6)),
            log_index: Some(U256::from(7)),
            removed: false,
        };
        let serialized = serde_json::to_string(&log).unwrap();
        assert_eq!(
            serialized,
            r#"{"address":"0x0000000000000000000000000000000000000001","topics":["0x0000000000000000000000000000000000000000000000000000000000000002"],"data":"0x1234","blockHash":"0x0000000000000000000000000000000000000000000000000000000000000003","blockNumber":"0x4","transactionHash":"0x0000000000000000000000000000000000000000000000000000000000000005","transactionIndex":"0x6","logIndex":"0x7","removed":false}"#
        );

        let deserialized: Log = serde_json::from_str(&serialized).unwrap();
        assert_eq!(log, deserialized);
    }
}
