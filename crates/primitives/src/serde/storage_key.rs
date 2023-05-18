use crate::{B256, U256};
use serde::{Deserialize, Serialize, Serializer};

/// A storage key type that can be serialized to and from a hex string up to 40
/// characters. Used for `eth_getStorageAt` and `eth_getProof` RPCs.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(from = "U256")] // TODO: Uint serde
pub struct JsonStorageKey(pub B256);

impl From<U256> for JsonStorageKey {
    fn from(value: U256) -> Self {
        JsonStorageKey(B256::from(value.to_be_bytes()))
    }
}

impl Serialize for JsonStorageKey {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        // serialize byte by byte
        //
        // this is mainly so we can return an output that hive testing expects, because
        // the `eth_getProof` implementation in geth simply mirrors the input
        //
        // see the use of `hexKey` in the `eth_getProof` response:
        // <https://github.com/ethereum/go-ethereum/blob/00a73fbcce3250b87fc4160f3deddc44390848f4/internal/ethapi/api.go#L658-L690>
        let mut b = hex::Buffer::<32>::new();
        s.serialize_str(b.format(&self.0 .0))
    }
}
