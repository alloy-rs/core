//! Bloom type.
//!
//! Adapted from <https://github.com/paritytech/parity-common/blob/2fb72eea96b6de4a085144ce239feb49da0cd39e/ethbloom/src/lib.rs>

use crate::{keccak256, FixedBytes};
use alloc::borrow::Cow;
use core::mem;

/// Length of bloom filter used for Ethereum.
pub const BLOOM_BITS: u32 = 3;
/// Size of the bloom filter in bytes.
pub const BLOOM_SIZE: usize = 256;

/// A 256-byte Ethereum bloom filter.
pub type Bloom = FixedBytes<256>;

/// Input to the [`Bloom::accrue`] method.
#[derive(Debug)]
pub enum BloomInput<'a> {
    /// Raw input to be hashed.
    Raw(&'a [u8]),
    /// Already hashed input.
    Hash(&'a [u8; 32]),
}

impl PartialEq<BloomRef<'_>> for Bloom {
    fn eq(&self, other: &BloomRef<'_>) -> bool {
        self.0 == *other.0
    }
}

impl From<BloomInput<'_>> for Bloom {
    fn from(input: BloomInput<'_>) -> Bloom {
        let mut bloom = Bloom::default();
        bloom.accrue(input);
        bloom
    }
}

impl Bloom {
    /// Returns the underlying data.
    #[inline]
    pub const fn data(&self) -> &[u8; BLOOM_SIZE] {
        &self.0
    }

    /// Returns whether the bloom filter contains the given input.
    pub fn contains_input(&self, input: BloomInput<'_>) -> bool {
        let bloom: Bloom = input.into();
        self.contains_bloom(&bloom)
    }

    /// Returns whether the bloom filter contains the given input.
    pub fn contains_bloom<'a, B: Into<BloomRef<'a>>>(&self, bloom: B) -> bool {
        self.contains_bloom_ref(bloom.into())
    }

    fn contains_bloom_ref(&self, bloom: BloomRef<'_>) -> bool {
        let self_ref: BloomRef<'_> = self.into();
        self_ref.contains_bloom(bloom)
    }

    /// Accrues the input into the bloom filter.
    pub fn accrue(&mut self, input: BloomInput<'_>) {
        let p = BLOOM_BITS;

        let m = self.0.len();
        let bloom_bits = m * 8;
        let mask = bloom_bits - 1;
        let bloom_bytes = (log2(bloom_bits) + 7) / 8;

        let hash = match input {
            BloomInput::Raw(raw) => Cow::Owned(keccak256(raw).0),
            BloomInput::Hash(hash) => Cow::Borrowed(hash),
        };

        // must be a power of 2
        assert_eq!(m & (m - 1), 0);
        // out of range
        assert!(p * bloom_bytes <= hash.len() as u32);

        let mut ptr = 0;

        for _ in 0..3 {
            let mut index = 0_usize;
            for _ in 0..bloom_bytes {
                index = (index << 8) | hash[ptr] as usize;
                ptr += 1;
            }
            index &= mask;
            self.0[m - 1 - index / 8] |= 1 << (index % 8);
        }
    }

    /// Accrues the input into the bloom filter.
    pub fn accrue_bloom<'a, B: Into<BloomRef<'a>>>(&mut self, bloom: B) {
        let bloom_ref: BloomRef<'_> = bloom.into();
        for i in 0..BLOOM_SIZE {
            self.0[i] |= bloom_ref.0[i];
        }
    }

    /// See Section 4.3.1 "Transaction Receipt" of the Ethereum Yellow Paper.
    pub fn m3_2048(&mut self, x: &[u8]) {
        let hash = keccak256(x);
        let h: &[u8; 32] = hash.as_ref();
        for i in [0, 2, 4] {
            let bit = (h[i + 1] as usize + ((h[i] as usize) << 8)) & 0x7FF;
            self.0[BLOOM_SIZE - 1 - bit / 8] |= 1 << (bit % 8);
        }
    }
}

/// A reference to a bloom filter. Can be
#[derive(Clone, Copy, Debug)]
pub struct BloomRef<'a>(pub &'a [u8; BLOOM_SIZE]);

impl<'a> BloomRef<'a> {
    /// Returns whether the bloom filter contains the given input.
    pub fn contains_bloom<'b, B: Into<BloomRef<'b>>>(self, bloom: B) -> bool {
        let bloom_ref: BloomRef<'_> = bloom.into();
        self.0.iter().zip(bloom_ref.0).all(|(&a, &b)| (a & b) == b)
    }

    /// Returns the underlying data.
    #[inline]
    pub const fn data(self) -> &'a [u8; BLOOM_SIZE] {
        self.0
    }

    /// Returns `true` if bloom only consists only of `0`.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }
}

impl<'a> From<&'a [u8; BLOOM_SIZE]> for BloomRef<'a> {
    fn from(data: &'a [u8; BLOOM_SIZE]) -> Self {
        Self(data)
    }
}

impl<'a> From<&'a Bloom> for BloomRef<'a> {
    fn from(bloom: &'a Bloom) -> Self {
        Self(&bloom.0)
    }
}

#[inline]
const fn log2(x: usize) -> u32 {
    if x <= 1 {
        return 0
    }

    let n = x.leading_zeros();
    mem::size_of::<usize>() as u32 * 8 - n
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
