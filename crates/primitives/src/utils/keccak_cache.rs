//! A minimalistic one-way set associative cache for Keccak256 values.
//!
//! This cache has a fixed size to allow fast access and minimize per-call overhead.
//!
//! Keys are stored as a 128-bit fingerprint (two independent rapidhashes of the input),
//! not the input bytes. False-positive probability is ~2^-128.
//!
//! With the `keccak-cache-stats` feature, hit/miss/insert/collision counts are
//! tracked in the global [`KECCAK_CACHE_STATS`] static.

use super::{hint::unlikely, keccak256_impl as keccak256};
use crate::{B256, KECCAK256_EMPTY};
use rapidhash::v3::{RapidSecrets, rapidhash_v3_micro_inline};
use std::{
    hash::{Hash, Hasher},
    sync::OnceLock,
};

#[cfg(feature = "keccak-cache-stats")]
pub use stats::{KECCAK_CACHE_STATS, KeccakCacheStats};

/// Maximum input length that can be cached.
pub(super) const MAX_INPUT_LEN: usize =
    128 - size_of::<B256>() - size_of::<u8>() - size_of::<usize>();

const COUNT: usize = 1 << 17;
static CACHE: OnceLock<fixed_cache::Cache<Key, B256, BuildHasher, CacheConfig>> = OnceLock::new();

struct CacheConfig {}
impl fixed_cache::CacheConfig for CacheConfig {
    const STATS: bool = cfg!(feature = "keccak-cache-stats");
    const EPOCHS: bool = false;
}

const SECRETS_LO: RapidSecrets = RapidSecrets::seed(0);
const SECRETS_HI: RapidSecrets = RapidSecrets::seed(0x9E37_79B9_7F4A_7C15);

pub(super) fn compute(input: &[u8], imp: impl FnOnce(&[u8]) -> B256) -> B256 {
    if unlikely(input.is_empty() | (input.len() > MAX_INPUT_LEN)) {
        return if input.is_empty() { KECCAK256_EMPTY } else { keccak256(input) };
    }

    let key = Key::from_input(input);
    let cache = CACHE.get_or_init(|| {
        let cache = fixed_cache::Cache::new(COUNT, BuildHasher::new());
        #[cfg(feature = "keccak-cache-stats")]
        let cache = cache.with_stats(Some(fixed_cache::Stats::new(&stats::KECCAK_CACHE_STATS)));
        cache
    });
    cache.get_or_insert_with(key, |_| imp(input))
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Key {
    lo: u64,
    hi: u64,
}

impl Key {
    #[inline]
    fn from_input(input: &[u8]) -> Self {
        unsafe { core::hint::assert_unchecked(input.len() <= MAX_INPUT_LEN) };
        let lo = rapidhash_v3_micro_inline::<false, false>(input, &SECRETS_LO);
        let hi = rapidhash_v3_micro_inline::<false, false>(input, &SECRETS_HI);
        Self { lo, hi }
    }
}

impl Hash for Key {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.lo);
    }
}

type BuildHasher = std::hash::BuildHasherDefault<PassthroughHasher>;

#[derive(Default)]
struct PassthroughHasher(u64);

impl Hasher for PassthroughHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
    #[inline]
    fn write(&mut self, _bytes: &[u8]) {
        debug_assert!(false, "PassthroughHasher::write called");
    }
}

#[cfg(feature = "keccak-cache-stats")]
mod stats {
    use super::Key;
    use crate::B256;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Counters for the global keccak cache.
    ///
    /// Accessed via the [`KECCAK_CACHE_STATS`] static. All counters use relaxed atomics.
    ///
    /// Available only with the `keccak-cache-stats` feature.
    #[derive(Debug, Default)]
    pub struct KeccakCacheStats {
        hits: AtomicU64,
        misses: AtomicU64,
        inserts: AtomicU64,
        collisions: AtomicU64,
    }

    impl KeccakCacheStats {
        const fn new() -> Self {
            Self {
                hits: AtomicU64::new(0),
                misses: AtomicU64::new(0),
                inserts: AtomicU64::new(0),
                collisions: AtomicU64::new(0),
            }
        }

        /// Returns the number of cache hits.
        #[inline]
        pub fn hits(&self) -> u64 {
            self.hits.load(Ordering::Relaxed)
        }

        /// Returns the number of cache misses.
        #[inline]
        pub fn misses(&self) -> u64 {
            self.misses.load(Ordering::Relaxed)
        }

        /// Returns the number of inserted entries.
        ///
        /// Includes inserts that evicted a different key on hash collision (see
        /// [`collisions`](Self::collisions)).
        #[inline]
        pub fn inserts(&self) -> u64 {
            self.inserts.load(Ordering::Relaxed)
        }

        /// Returns the number of collisions (a different key was evicted on insert).
        #[inline]
        pub fn collisions(&self) -> u64 {
            self.collisions.load(Ordering::Relaxed)
        }

        /// Resets all counters to zero.
        pub fn reset(&self) {
            self.hits.store(0, Ordering::Relaxed);
            self.misses.store(0, Ordering::Relaxed);
            self.inserts.store(0, Ordering::Relaxed);
            self.collisions.store(0, Ordering::Relaxed);
        }
    }

    impl fixed_cache::StatsHandler<Key, B256> for &'static KeccakCacheStats {
        #[inline]
        fn on_hit(&self, _key: &Key, _value: &B256) {
            self.hits.fetch_add(1, Ordering::Relaxed);
        }

        #[inline]
        fn on_miss(&self, _key: fixed_cache::AnyRef<'_>) {
            self.misses.fetch_add(1, Ordering::Relaxed);
        }

        #[inline]
        fn on_insert(&self, key: &Key, _value: &B256, evicted: Option<(&Key, &B256)>) {
            match evicted {
                Some((old, _)) if old == key => {}
                Some(_) => {
                    self.inserts.fetch_add(1, Ordering::Relaxed);
                    self.collisions.fetch_add(1, Ordering::Relaxed);
                }
                None => {
                    self.inserts.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    /// Global counters for the keccak cache.
    ///
    /// Available only with the `keccak-cache-stats` feature.
    pub static KECCAK_CACHE_STATS: KeccakCacheStats = KeccakCacheStats::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes() {
        assert_eq!(size_of::<Key>(), 16);
    }

    #[test]
    fn caching() {
        let mut count: usize = 0;
        let mut compute = |input| {
            compute(input, |x| {
                count += 1;
                keccak256(x)
            })
        };

        let input = b"Hello World!";
        let input2 = b"Hello World! 2";

        let a = compute(input);
        let b = compute(input);
        let c = compute(input);
        assert_eq!(a, b);
        assert_eq!(a, c);

        let d = compute(input2);
        let e = compute(input2);
        assert_ne!(a, d);
        assert_eq!(d, e);

        assert_eq!(count, 2);
    }
}
