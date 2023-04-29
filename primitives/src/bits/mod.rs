mod address;
mod fixed;

// code stolen from: https://docs.rs/impl-serde/0.4.0/impl_serde/
#[cfg(feature = "serde")]
mod serialize;

#[cfg(feature = "rlp")]
mod rlp;

pub use address::Address;
pub use fixed::{B160, B256, B512};
