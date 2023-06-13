//! Bloom type.
//!
//! Adapted from <https://github.com/paritytech/parity-common/blob/2fb72eea96b6de4a085144ce239feb49da0cd39e/ethbloom/src/lib.rs>

use crate::{keccak256, wrap_fixed_bytes, FixedBytes};
use core::borrow::Borrow;

/// Number of bits to set per input in Ethereum bloom filter.
pub const BLOOM_BITS_PER_ITEM: usize = 3;
/// Size of the bloom filter in bytes.
pub const BLOOM_SIZE_BYTES: usize = 256;
/// Size of the bloom filter in bits
pub const BLOOM_SIZE_BITS: usize = BLOOM_SIZE_BYTES * 8;

/// Size of the keccak256 hash in bytes, used in accrue
const ITEM_HASH_LEN: usize = 32;
/// Mask, used in accrue
const MASK: usize = BLOOM_SIZE_BITS - 1;
/// Number of bytes per item, used in accrue
const ITEM_BYTES: usize = (log2(BLOOM_SIZE_BITS) + 7) / 8;

// BLOOM_SIZE_BYTES must be a power of 2
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(BLOOM_SIZE_BYTES.is_power_of_two());
// Assertion for accrue. This is preserved from parity code, but I do not
// understand its purpose.
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(BLOOM_BITS_PER_ITEM * ITEM_BYTES <= ITEM_HASH_LEN);

/// Input to the [`Bloom::accrue`] method.
#[derive(Debug, Clone, Copy)]
pub enum BloomInput<'a> {
    /// Raw input to be hashed.
    Raw(&'a [u8]),
    /// Already hashed input.
    Hash(FixedBytes<ITEM_HASH_LEN>),
}

impl BloomInput<'_> {
    /// Consume the input, converting it to the hash.
    pub fn into_hash(self) -> FixedBytes<ITEM_HASH_LEN> {
        match self {
            BloomInput::Raw(raw) => keccak256(raw),
            BloomInput::Hash(hash) => hash,
        }
    }
}

impl From<BloomInput<'_>> for Bloom {
    fn from(input: BloomInput<'_>) -> Bloom {
        let mut bloom = Bloom::default();
        bloom.accrue(input);
        bloom
    }
}

wrap_fixed_bytes!(
    /// Ethereum 256 byte bloom filter.
    pub struct Bloom<256>;
);

impl Bloom {
    /// Returns a reference to the underlying data.
    #[inline]
    pub const fn data(&self) -> &[u8; BLOOM_SIZE_BYTES] {
        &self.0 .0
    }

    /// Returns a mutable reference to the underlying data.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [u8; BLOOM_SIZE_BYTES] {
        &mut self.0 .0
    }

    /// Returns whether the bloom filter contains the given input (allowing for
    /// false positives)
    pub fn contains_input(&self, input: BloomInput<'_>) -> bool {
        let bloom: Bloom = input.into();
        self.contains(bloom)
    }

    /// True if this bloom filter is a possible superset of the other bloom
    /// filter, admitting false positives.
    pub const fn const_contains(self, other: Self) -> bool {
        // (self & other) == other
        other.0.const_eq(&self.0.bit_and(other.0))
    }

    /// Returns whether the bloom filter is a superset of the given bloom
    /// filter (allowing for false positives)
    pub fn contains<B: Borrow<Self>>(&self, other: B) -> bool {
        self.const_contains(*(other.borrow()))
    }

    /// Accrues the input into the bloom filter.
    pub fn accrue(&mut self, input: BloomInput<'_>) {
        let hash = input.into_hash();

        let mut ptr = 0;

        for _ in 0..3 {
            let mut index = 0_usize;
            for _ in 0..ITEM_BYTES {
                index = (index << 8) | hash[ptr] as usize;
                ptr += 1;
            }
            index &= MASK;
            self.0[BLOOM_SIZE_BYTES - 1 - index / 8] |= 1 << (index % 8);
        }
    }

    /// Accrues the input into the bloom filter.
    pub fn accrue_bloom<B: Borrow<Bloom>>(&mut self, bloom: B) {
        let other = bloom.borrow();
        *self |= *other;
    }

    /// See Section 4.3.1 "Transaction Receipt" of the Ethereum Yellow Paper.
    pub fn m3_2048(&mut self, x: &[u8]) {
        let hash = keccak256(x);
        let h: &[u8; 32] = hash.as_ref();
        for i in [0, 2, 4] {
            let bit = (h[i + 1] as usize + ((h[i] as usize) << 8)) & 0x7FF;
            self.0[BLOOM_SIZE_BYTES - 1 - bit / 8] |= 1 << (bit % 8);
        }
    }
}

#[inline]
const fn log2(x: usize) -> usize {
    if x <= 1 {
        return 0
    }

    (usize::BITS - x.leading_zeros()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn works() {
        let bloom: Bloom = hex!(
            "00000000000000000000000000000000
             00000000100000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000002020000000000000000000000
             00000000000000000000000800000000
             10000000000000000000000000000000
             00000000000000000000001000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000
             00000000000000000000000000000000"
        )
        .into();
        let address = hex!("ef2d6d194084c2de36e0dabfce45d046b37d1106");
        let topic = hex!("02c69be41d0b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc");

        let mut my_bloom = Bloom::default();
        assert!(!my_bloom.contains_input(BloomInput::Raw(&address)));
        assert!(!my_bloom.contains_input(BloomInput::Raw(&topic)));

        my_bloom.accrue(BloomInput::Raw(&address));
        assert!(my_bloom.contains_input(BloomInput::Raw(&address)));
        assert!(!my_bloom.contains_input(BloomInput::Raw(&topic)));

        my_bloom.accrue(BloomInput::Raw(&topic));
        assert!(my_bloom.contains_input(BloomInput::Raw(&address)));
        assert!(my_bloom.contains_input(BloomInput::Raw(&topic)));

        assert_eq!(my_bloom, bloom);
    }
}
