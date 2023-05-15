use crate::bits::FixedBytes;

pub use tiny_keccak::{Hasher, Keccak};

/// Simple interface to the `keccak256` hash function.
pub fn keccak256(bytes: impl AsRef<[u8]>) -> FixedBytes<32> {
    fn internal(bytes: &[u8]) -> FixedBytes<32> {
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(bytes);
        hasher.finalize(&mut output);
        output.into()
    }

    internal(bytes.as_ref())
}
