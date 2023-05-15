mod address;
pub use address::{Address, AddressError};

mod fixed;
pub use fixed::FixedBytes;

mod macros;

// code stolen from: https://docs.rs/impl-serde/0.4.0/impl_serde/
#[cfg(feature = "serde")]
mod serialize;

#[cfg(feature = "rlp")]
mod rlp;

/// 32-byte fixed array type.
pub type B256 = FixedBytes<32>;

impl From<crate::U256> for B256 {
    #[inline]
    fn from(value: crate::U256) -> Self {
        Self::from(value.to_be_bytes())
    }
}

impl From<B256> for crate::U256 {
    #[inline]
    fn from(value: B256) -> Self {
        Self::from_be_bytes(value.0)
    }
}
