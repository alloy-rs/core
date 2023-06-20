mod address;
pub use address::{Address, AddressError};

mod bloom;
pub use bloom::{Bloom, BloomInput, BLOOM_BITS_PER_ITEM, BLOOM_SIZE_BITS, BLOOM_SIZE_BYTES};

mod fixed;
pub use fixed::FixedBytes;

mod macros;

#[cfg(feature = "rlp")]
mod rlp;

#[cfg(feature = "serde")]
mod serde;

mod impl_core;

/// 8-byte fixed array type.
pub type B64 = FixedBytes<8>;

/// 16-byte fixed array type.
pub type B128 = FixedBytes<16>;

/// 32-byte fixed array type.
pub type B256 = FixedBytes<32>;

/// 64-byte fixed array type.
pub type B512 = FixedBytes<64>;

impl From<crate::U256> for B256 {
    #[inline]
    fn from(value: crate::U256) -> Self {
        Self(value.to_be_bytes())
    }
}

impl From<B256> for crate::U256 {
    #[inline]
    fn from(value: B256) -> Self {
        Self::from_be_bytes(value.0)
    }
}
