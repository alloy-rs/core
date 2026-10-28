//! Bloom type.
//!
//! Adapted from <https://github.com/paritytech/parity-common/blob/2fb72eea96b6de4a085144ce239feb49da0cd39e/ethbloom/src/lib.rs>

use crate::{Address, B256, Log, LogData, keccak256};

/// Number of bits to set per input in Ethereum bloom filter.
pub const BLOOM_BITS_PER_ITEM: usize = 3;
/// Size of the bloom filter in bytes.
pub const BLOOM_SIZE_BYTES: usize = 256;
/// Size of the bloom filter in bits
pub const BLOOM_SIZE_BITS: usize = BLOOM_SIZE_BYTES * 8;

// BLOOM_SIZE_BYTES must be a power of 2
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(BLOOM_SIZE_BYTES.is_power_of_two());

/// Input to the [`Bloom::accrue`] method.
#[derive(Clone, Copy, Debug)]
pub enum BloomInput<'a> {
    /// Raw input to be hashed.
    Raw(&'a [u8]),
    /// Already hashed input.
    Hash(B256),
}

impl BloomInput<'_> {
    /// Consume the input, converting it to the hash.
    #[inline]
    pub fn into_hash(self) -> B256 {
        match self {
            BloomInput::Raw(raw) => keccak256(raw),
            BloomInput::Hash(hash) => hash,
        }
    }
}

impl From<BloomInput<'_>> for Bloom {
    #[inline]
    fn from(input: BloomInput<'_>) -> Self {
        let mut bloom = Self::ZERO;
        bloom.accrue(input);
        bloom
    }
}

wrap_fixed_bytes!(
    /// Ethereum 256 byte bloom filter.
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
    #[cfg_attr(feature = "rkyv", rkyv(derive(Copy, Clone, Hash, PartialEq, Eq)))]
    pub struct Bloom<256>;
);

impl<'a> FromIterator<&'a (Address, LogData)> for Bloom {
    fn from_iter<T: IntoIterator<Item = &'a (Address, LogData)>>(iter: T) -> Self {
        let mut bloom = Self::ZERO;
        bloom.extend(iter);
        bloom
    }
}

impl<'a> Extend<&'a (Address, LogData)> for Bloom {
    fn extend<T: IntoIterator<Item = &'a (Address, LogData)>>(&mut self, iter: T) {
        for (address, log_data) in iter {
            self.accrue_raw_log(*address, log_data.topics())
        }
    }
}

impl<'a> FromIterator<&'a Log> for Bloom {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a Log>>(logs: T) -> Self {
        let mut bloom = Self::ZERO;
        bloom.extend(logs);
        bloom
    }
}

impl<'a> Extend<&'a Log> for Bloom {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a Log>>(&mut self, logs: T) {
        for log in logs {
            self.accrue_log(log)
        }
    }
}

impl<'a, 'b> FromIterator<&'a BloomInput<'b>> for Bloom {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a BloomInput<'b>>>(inputs: T) -> Self {
        let mut bloom = Self::ZERO;
        bloom.extend(inputs);
        bloom
    }
}

impl<'a, 'b> Extend<&'a BloomInput<'b>> for Bloom {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a BloomInput<'b>>>(&mut self, inputs: T) {
        for input in inputs {
            self.accrue(*input);
        }
    }
}

impl Bloom {
    /// Returns a reference to the underlying data.
    #[inline]
    pub const fn data(&self) -> &[u8; BLOOM_SIZE_BYTES] {
        &self.0.0
    }

    /// Returns a mutable reference to the underlying data.
    #[inline]
    pub const fn data_mut(&mut self) -> &mut [u8; BLOOM_SIZE_BYTES] {
        &mut self.0.0
    }

    /// Returns true if this bloom filter is a possible superset of the other
    /// bloom filter, admitting false positives.
    ///
    /// Note: This method may return false positives. This is inherent to the
    /// bloom filter data structure.
    #[inline]
    pub fn contains_input(&self, input: BloomInput<'_>) -> bool {
        let hash = input.into_hash();
        self.contains_m3_2048_hashed(&hash)
    }

    /// Compile-time version of [`contains`](Self::contains).
    ///
    /// Note: This method may return false positives. This is inherent to the
    /// bloom filter data structure.
    pub const fn const_contains(self, other: Self) -> bool {
        self.0.const_covers(other.0)
    }

    /// Returns true if this bloom filter is a possible superset of the other
    /// bloom filter, admitting false positives.
    ///
    /// Note: This method may return false positives. This is inherent to the
    /// bloom filter data structure.
    pub fn contains(&self, other: &Self) -> bool {
        self.0.covers(&other.0)
    }

    /// Accrues the input into the bloom filter.
    pub fn accrue(&mut self, input: BloomInput<'_>) {
        let hash = input.into_hash();
        self.m3_2048_hashed(&hash);
    }

    /// Accrues the input into the bloom filter.
    pub fn accrue_bloom(&mut self, bloom: &Self) {
        *self |= *bloom;
    }

    /// Specialised Bloom filter that sets three bits out of 2048, given an
    /// arbitrary byte sequence.
    ///
    /// See Section 4.3.1 "Transaction Receipt" of the
    /// [Ethereum Yellow Paper][ref] (page 6).
    ///
    /// [ref]: https://ethereum.github.io/yellowpaper/paper.pdf
    pub fn m3_2048(&mut self, bytes: &[u8]) {
        self.m3_2048_hashed(&keccak256(bytes));
    }

    /// [`m3_2048`](Self::m3_2048) but with a pre-hashed input.
    pub fn m3_2048_hashed(&mut self, hash: &B256) {
        for i in [0, 2, 4] {
            let bit = (hash[i + 1] as usize + ((hash[i] as usize) << 8)) & 0x7FF;
            self[BLOOM_SIZE_BYTES - 1 - bit / 8] |= 1 << (bit % 8);
        }
    }

    /// Returns true if this bloom filter contains the bits set by [`m3_2048`](Self::m3_2048).
    fn contains_m3_2048(&self, bytes: &[u8]) -> bool {
        self.contains_m3_2048_hashed(&keccak256(bytes))
    }

    /// [`contains_m3_2048`](Self::contains_m3_2048) but with a pre-hashed input.
    fn contains_m3_2048_hashed(&self, hash: &B256) -> bool {
        for i in [0, 2, 4] {
            // Each adjacent hash byte pair forms one big-endian 11-bit index.
            // Bloom bytes are stored high-to-low, while bits within a byte are low-to-high.
            let bit = (hash[i + 1] as usize + ((hash[i] as usize) << 8)) & 0x7FF;
            if self[BLOOM_SIZE_BYTES - 1 - bit / 8] & (1 << (bit % 8)) == 0 {
                return false;
            }
        }
        true
    }

    /// Ingests a raw log into the bloom filter.
    pub fn accrue_raw_log(&mut self, address: Address, topics: &[B256]) {
        self.m3_2048(address.as_slice());
        for topic in topics.iter() {
            self.m3_2048(topic.as_slice());
        }
    }

    /// Ingests a log into the bloom filter.
    pub fn accrue_log(&mut self, log: &Log) {
        self.accrue_raw_log(log.address, log.topics())
    }

    /// Ingests multiple logs into the bloom filter.
    pub fn accrue_logs(&mut self, logs: &[Log]) {
        for log in logs {
            self.accrue_log(log)
        }
    }

    /// True if the bloom filter contains a log with given address and topics.
    ///
    /// Note: This method may return false positives. This is inherent to the
    /// bloom filter data structure.
    pub fn contains_raw_log(&self, address: Address, topics: &[B256]) -> bool {
        self.contains_m3_2048(address.as_slice())
            && topics.iter().all(|topic| self.contains_m3_2048(topic.as_slice()))
    }

    /// True if the bloom filter contains a log with given address and topics.
    ///
    /// Note: This method may return false positives. This is inherent to the
    /// bloom filter data structure.
    pub fn contains_log(&self, log: &Log) -> bool {
        self.contains_raw_log(log.address, log.topics())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex;

    #[test]
    fn works() {
        let bloom = bloom!(
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
        );
        let address = hex!("ef2d6d194084c2de36e0dabfce45d046b37d1106");
        let topic = hex!("02c69be41d0b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc");
        let address_hash = keccak256(address);
        let topic_hash = keccak256(topic);

        let mut my_bloom = Bloom::default();
        assert!(!my_bloom.contains_input(BloomInput::Raw(&address)));
        assert!(!my_bloom.contains_input(BloomInput::Raw(&topic)));
        assert!(!my_bloom.contains_input(BloomInput::Hash(address_hash)));
        assert!(!my_bloom.contains_input(BloomInput::Hash(topic_hash)));

        my_bloom.accrue(BloomInput::Raw(&address));
        assert!(my_bloom.contains_input(BloomInput::Raw(&address)));
        assert!(!my_bloom.contains_input(BloomInput::Raw(&topic)));
        assert!(my_bloom.contains_input(BloomInput::Hash(address_hash)));
        assert!(!my_bloom.contains_input(BloomInput::Hash(topic_hash)));

        my_bloom.accrue(BloomInput::Raw(&topic));
        assert!(my_bloom.contains_input(BloomInput::Raw(&address)));
        assert!(my_bloom.contains_input(BloomInput::Raw(&topic)));
        assert!(my_bloom.contains_input(BloomInput::Hash(address_hash)));
        assert!(my_bloom.contains_input(BloomInput::Hash(topic_hash)));

        assert_eq!(my_bloom, bloom);
    }

    #[test]
    #[cfg(feature = "arbitrary")]
    #[cfg_attr(miri, ignore = "proptest is too slow under miri")]
    fn contains_raw_log_matches_bloom_contains() {
        use proptest::{arbitrary::any, collection::vec};

        fn contains_via_bloom(bloom: &Bloom, address: Address, topics: &[B256]) -> bool {
            let mut log_bloom = Bloom::default();
            log_bloom.accrue_raw_log(address, topics);
            bloom.contains(&log_bloom)
        }

        proptest::proptest!(|(bloom: Bloom, address: Address, topics in vec(any::<B256>(), 0..8))| {
            proptest::prop_assert_eq!(
                bloom.contains_raw_log(address, &topics),
                contains_via_bloom(&bloom, address, &topics)
            );
        });
    }
}
