use crate::bits::FixedBytes;

pub use tiny_keccak::{Hasher, Keccak};

/// Simple interface to the [`keccak256`] hash function.
///
/// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> FixedBytes<32> {
    fn keccak256(bytes: &[u8]) -> FixedBytes<32> {
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(bytes);
        hasher.finalize(&mut output);
        output.into()
    }

    keccak256(bytes.as_ref())
}
