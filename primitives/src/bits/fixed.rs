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

construct_fixed_hash! {
    /// 256 bits fixed hash
    #[cfg_attr(feature = "arbitrary", derive(Arbitrary, PropTestArbitrary))]
    #[derive(AsRef, Deref)]
    pub struct B256(32);
}

construct_fixed_hash! {
    /// 160 bits fixed hash
    #[cfg_attr(feature = "arbitrary", derive(Arbitrary, PropTestArbitrary))]
    #[derive(AsRef, Deref)]
    pub struct B160(20);
}

construct_fixed_hash! {
    /// 512 bits fixed hash
    #[cfg_attr(feature = "arbitrary", derive(Arbitrary, PropTestArbitrary))]
    #[derive(AsRef, Deref)]
    pub struct B512(64);
}

impl From<u64> for B160 {
    fn from(fr: u64) -> Self {
        let x_bytes = fr.to_be_bytes();
        let mut b = B160::default();
        b[12..].copy_from_slice(&x_bytes);
        b
    }
}

impl From<ruint::aliases::U256> for B256 {
    fn from(fr: ruint::aliases::U256) -> Self {
        B256(fr.to_be_bytes())
    }
}

impl From<B256> for ruint::aliases::U256 {
    fn from(fr: B256) -> Self {
        ruint::aliases::U256::from_be_bytes(fr.0)
    }
}

impl_fixed_hash_conversions!(B256, B160);
impl_fixed_hash_conversions!(B512, B160);
impl_fixed_hash_conversions!(B512, B256);

impl Borrow<[u8; 32]> for B256 {
    fn borrow(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Borrow<[u8; 20]> for B160 {
    fn borrow(&self) -> &[u8; 20] {
        &self.0
    }
}

impl Borrow<[u8; 64]> for B512 {
    fn borrow(&self) -> &[u8; 64] {
        &self.0
    }
}

#[cfg(all(test, feature = "arbitrary"))]
mod tests {
    use super::*;

    #[test]
    fn arbitrary() {
        proptest::proptest!(|(_v1: B160, _v2: B256)| {});
    }
}
