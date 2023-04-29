pub use tiny_keccak::{Hasher, Keccak};

use super::B256;

/// Simple interface to keccak256 hash function
pub fn keccak256(bytes: impl AsRef<[u8]>) -> B256 {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output.into()
}
