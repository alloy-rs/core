use derive_more::{AsRef, Deref};
use fixed_hash::{construct_fixed_hash, impl_fixed_hash_conversions};

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;
#[cfg(feature = "arbitrary")]
use proptest_derive::Arbitrary as PropTestArbitrary;

#[cfg(feature = "std")]
use std::borrow::Borrow;

#[cfg(not(feature = "std"))]
use alloc::borrow::Borrow;

use crate::B256;

use super::B160;

construct_fixed_hash! {
    /// Ethereum address type
    #[cfg_attr(feature = "arbitrary", derive(Arbitrary, PropTestArbitrary))]
    #[derive(AsRef, Deref)]
    pub struct Address(20);
}

// manual impls because `impl_fixed_hash_conversions` macro requires one type to be smalelr
impl From<B160> for Address {
    fn from(value: B160) -> Self {
        value.as_fixed_bytes().into()
    }
}

impl From<Address> for B160 {
    fn from(value: Address) -> Self {
        value.0.into()
    }
}

impl_fixed_hash_conversions!(B256, Address);

impl Borrow<[u8; 20]> for Address {
    fn borrow(&self) -> &[u8; 20] {
        &self.0
    }
}
