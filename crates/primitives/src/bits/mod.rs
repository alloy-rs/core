#[macro_use]
mod macros;

mod address;
pub use address::{Address, AddressError};

mod bloom;
pub use bloom::{Bloom, BloomInput, BLOOM_BITS_PER_ITEM, BLOOM_SIZE_BITS, BLOOM_SIZE_BYTES};

mod fixed;
pub use fixed::FixedBytes;

#[cfg(feature = "rlp")]
mod rlp;

#[cfg(feature = "serde")]
mod serde;

mod impl_core;
