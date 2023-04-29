mod address;
mod fixed;

pub(self) mod hex;
pub(self) use hex::{from_hex_raw, to_hex_raw};

// code stolen from: https://docs.rs/impl-serde/0.4.0/impl_serde/
#[cfg(feature = "serde")]
mod serialize;

#[cfg(feature = "rlp")]
mod rlp;

pub use address::{Address, AddressError};
pub use fixed::{B160, B256, B512};
