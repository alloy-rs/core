use alloy_primitives::{Address, Bytes, B256};
use alloy_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

/// Ethereum Log
#[derive(
    Clone, Debug, Default, PartialEq, Eq, RlpDecodable, RlpEncodable, Deserialize, Serialize,
)]
pub struct Log {
    /// Contract that emitted this log.
    pub address: Address,
    /// Topics of the log. The number of logs depend on what `LOG` opcode is
    /// used.
    pub topics: Vec<B256>,
    /// Arbitrary length data.
    pub data: Bytes,
}
