//! Common Ethereum utilities.

use crate::B256;
use alloc::vec::Vec;
use cfg_if::cfg_if;
use core::{fmt, mem::MaybeUninit};

mod units;
pub use units::{
    format_ether, format_units, parse_ether, parse_units, ParseUnits, Unit, UnitsError,
};

cfg_if! {
    if #[cfg(all(feature = "asm-keccak", not(miri)))] {
        use keccak_asm::Digest as _;
    } else {
        use tiny_keccak::Hasher as _;
    }
}

#[doc(hidden)]
#[deprecated(since = "0.5.0", note = "use `Unit::ETHER.wei()` instead")]
pub const WEI_IN_ETHER: crate::U256 = Unit::ETHER.wei_const();

#[doc(hidden)]
#[deprecated(since = "0.5.0", note = "use `Unit` instead")]
pub type Units = Unit;

/// The prefix used for hashing messages according to EIP-191.
pub const EIP191_PREFIX: &str = "\x19Ethereum Signed Message:\n";

/// Hash a message according to [EIP-191] (version `0x01`).
///
/// The final message is a UTF-8 string, encoded as follows:
/// `"\x19Ethereum Signed Message:\n" + message.length + message`
///
/// This message is then hashed using [Keccak-256](keccak256).
///
/// [EIP-191]: https://eips.ethereum.org/EIPS/eip-191
pub fn eip191_hash_message<T: AsRef<[u8]>>(message: T) -> B256 {
    keccak256(eip191_message(message))
}

/// Constructs a message according to [EIP-191] (version `0x01`).
///
/// The final message is a UTF-8 string, encoded as follows:
/// `"\x19Ethereum Signed Message:\n" + message.length + message`
///
/// [EIP-191]: https://eips.ethereum.org/EIPS/eip-191
pub fn eip191_message<T: AsRef<[u8]>>(message: T) -> Vec<u8> {
    fn eip191_message(message: &[u8]) -> Vec<u8> {
        let len = message.len();
        let mut len_string_buffer = itoa::Buffer::new();
        let len_string = len_string_buffer.format(len);

        let mut eth_message = Vec::with_capacity(EIP191_PREFIX.len() + len_string.len() + len);
        eth_message.extend_from_slice(EIP191_PREFIX.as_bytes());
        eth_message.extend_from_slice(len_string.as_bytes());
        eth_message.extend_from_slice(message);
        eth_message
    }

    eip191_message(message.as_ref())
}

/// Simple interface to the [`Keccak-256`] hash function.
///
/// [`Keccak-256`]: https://en.wikipedia.org/wiki/SHA-3
pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> B256 {
    fn keccak256(bytes: &[u8]) -> B256 {
        let mut output = MaybeUninit::<B256>::uninit();

        cfg_if! {
            if #[cfg(all(feature = "native-keccak", not(feature = "tiny-keccak"), not(miri)))] {
                #[link(wasm_import_module = "vm_hooks")]
                extern "C" {
                    /// When targeting VMs with native keccak hooks, the `native-keccak` feature
                    /// can be enabled to import and use the host environment's implementation
                    /// of [`keccak256`] in place of [`tiny_keccak`]. This is overridden when
                    /// the `tiny-keccak` feature is enabled.
                    ///
                    /// # Safety
                    ///
                    /// The VM accepts the preimage by pointer and length, and writes the
                    /// 32-byte hash.
                    /// - `bytes` must point to an input buffer at least `len` long.
                    /// - `output` must point to a buffer that is at least 32-bytes long.
                    ///
                    /// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
                    /// [`tiny_keccak`]: https://docs.rs/tiny-keccak/latest/tiny_keccak/
                    fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8);
                }

                // SAFETY: The output is 32-bytes, and the input comes from a slice.
                unsafe { native_keccak256(bytes.as_ptr(), bytes.len(), output.as_mut_ptr().cast::<u8>()) };
            } else {
                let mut hasher = Keccak256::new();
                hasher.update(bytes);
                // SAFETY: Never reads from `output`.
                unsafe { hasher.finalize_into_raw(output.as_mut_ptr().cast()) };
            }
        }

        // SAFETY: Initialized above.
        unsafe { output.assume_init() }
    }

    keccak256(bytes.as_ref())
}

/// Simple [`Keccak-256`] hasher.
///
/// Note that the "native-keccak" feature is not supported for this struct, and will default to the
/// [`tiny_keccak`] implementation.
///
/// [`Keccak-256`]: https://en.wikipedia.org/wiki/SHA-3
#[derive(Clone)]
pub struct Keccak256 {
    #[cfg(all(feature = "asm-keccak", not(miri)))]
    hasher: keccak_asm::Keccak256,
    #[cfg(not(all(feature = "asm-keccak", not(miri))))]
    hasher: tiny_keccak::Keccak,
}

impl Default for Keccak256 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Keccak256 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Keccak256").finish_non_exhaustive()
    }
}

impl Keccak256 {
    /// Creates a new [`Keccak256`] hasher.
    #[inline]
    pub fn new() -> Self {
        cfg_if! {
            if #[cfg(all(feature = "asm-keccak", not(miri)))] {
                let hasher = keccak_asm::Keccak256::new();
            } else {
                let hasher = tiny_keccak::Keccak::v256();
            }
        }
        Self { hasher }
    }

    /// Absorbs additional input. Can be called multiple times.
    #[inline]
    pub fn update(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes.as_ref());
    }

    /// Pad and squeeze the state.
    #[inline]
    pub fn finalize(self) -> B256 {
        let mut output = MaybeUninit::<B256>::uninit();
        // SAFETY: The output is 32-bytes.
        unsafe { self.finalize_into_raw(output.as_mut_ptr().cast()) };
        // SAFETY: Initialized above.
        unsafe { output.assume_init() }
    }

    /// Pad and squeeze the state into `output`.
    ///
    /// # Panics
    ///
    /// Panics if `output` is not 32 bytes long.
    #[inline]
    #[track_caller]
    pub fn finalize_into(self, output: &mut [u8]) {
        self.finalize_into_array(output.try_into().unwrap())
    }

    /// Pad and squeeze the state into `output`.
    #[inline]
    pub fn finalize_into_array(self, output: &mut [u8; 32]) {
        cfg_if! {
            if #[cfg(all(feature = "asm-keccak", not(miri)))] {
                self.hasher.finalize_into(output.into());
            } else {
                self.hasher.finalize(output);
            }
        }
    }

    /// Pad and squeeze the state into `output`.
    ///
    /// # Safety
    ///
    /// `output` must point to a buffer that is at least 32-bytes long.
    #[inline]
    pub unsafe fn finalize_into_raw(self, output: *mut u8) {
        self.finalize_into_array(&mut *output.cast::<[u8; 32]>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test vector taken from:
    // https://web3js.readthedocs.io/en/v1.10.0/web3-eth-accounts.html#hashmessage
    #[test]
    fn test_hash_message() {
        let msg = "Hello World";
        let eip191_msg = eip191_message(msg);
        let hash = keccak256(&eip191_msg);
        assert_eq!(
            eip191_msg,
            [EIP191_PREFIX.as_bytes(), msg.len().to_string().as_bytes(), msg.as_bytes()].concat()
        );
        assert_eq!(hash, b256!("a1de988600a42c4b4ab089b619297c17d53cffae5d5120d82d8a92d0bb3b78f2"));
        assert_eq!(eip191_hash_message(msg), hash);
    }

    #[test]
    fn keccak256_hasher() {
        let expected = b256!("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad");
        assert_eq!(keccak256("hello world"), expected);

        let mut hasher = Keccak256::new();
        hasher.update(b"hello");
        hasher.update(b" world");

        assert_eq!(hasher.clone().finalize(), expected);

        let mut hash = [0u8; 32];
        hasher.clone().finalize_into(&mut hash);
        assert_eq!(hash, expected);

        let mut hash = [0u8; 32];
        hasher.clone().finalize_into_array(&mut hash);
        assert_eq!(hash, expected);

        let mut hash = [0u8; 32];
        unsafe { hasher.finalize_into_raw(hash.as_mut_ptr()) };
        assert_eq!(hash, expected);
    }
}
