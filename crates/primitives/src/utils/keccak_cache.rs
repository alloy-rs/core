//! A minimalistic one-way set associative cache for Keccak256 values.
//!
//! This cache has a fixed size to allow fast access and minimize per-call overhead.
//!
//! With the `keccak-cache-stats` feature, hit/miss/insert/collision counts are
//! tracked in the global [`KECCAK_CACHE_STATS`] static.

use super::{hint::unlikely, keccak256_impl as keccak256};
use crate::{B256, KECCAK256_EMPTY};
use std::{ptr, sync::OnceLock};

#[cfg(feature = "keccak-cache-stats")]
pub use stats::{KECCAK_CACHE_STATS, KeccakCacheStats};

/// Maximum input length that can be cached.
pub(super) const MAX_INPUT_LEN: usize =
    128 - size_of::<B256>() - size_of::<u8>() - size_of::<usize>();

const DEFAULT_COUNT: usize = 1 << 17; // ~131k entries * 128 bytes = 16MiB
static CACHE: OnceLock<Cache> = OnceLock::new();

type Cache = fixed_cache::Cache<Key, B256, BuildHasher, CacheConfig>;

struct CacheConfig {}
impl fixed_cache::CacheConfig for CacheConfig {
    const STATS: bool = cfg!(feature = "keccak-cache-stats");
    const EPOCHS: bool = false;
    // SAFETY: `Key` and `B256` are plain bytes without padding, and `Key` zeroes its unused bytes.
    const LOCK_FREE_READS: fixed_cache::LockFreeReads =
        unsafe { fixed_cache::LockFreeReads::enabled() };
}

/// Initializes the process-global keccak cache with `entries` buckets.
///
/// Returns `true` if this call initialized the cache, and `false` if the cache was already
/// initialized by an earlier call or by the first cached hash computation. If this is never called,
/// the cache is initialized lazily with the default size on first use.
///
/// # Panics
///
/// Panics if `entries` is not a power of two or is less than 4.
#[must_use]
pub fn init_keccak_cache(entries: usize) -> bool {
    init_cache(&CACHE, entries)
}

pub(super) fn compute(input: &[u8], imp: impl FnOnce(&[u8]) -> B256) -> B256 {
    if unlikely(input.is_empty() | (input.len() > MAX_INPUT_LEN)) {
        return if input.is_empty() { KECCAK256_EMPTY } else { keccak256(input) };
    }

    let cache = CACHE.get_or_init(default_cache);
    cache.get_or_insert_with_ref(&Input(input), |input| imp(input.0), |input| Key::new(input.0))
}

fn init_cache(cache: &OnceLock<Cache>, entries: usize) -> bool {
    if cache.get().is_some() {
        return false;
    }
    cache.set(new_cache(entries)).is_ok()
}

fn default_cache() -> Cache {
    new_cache(DEFAULT_COUNT)
}

fn new_cache(entries: usize) -> Cache {
    let cache = fixed_cache::Cache::new(entries, BuildHasher::new());
    #[cfg(feature = "keccak-cache-stats")]
    let cache = cache.with_stats(Some(fixed_cache::Stats::new(&stats::KECCAK_CACHE_STATS)));
    cache
}

type BuildHasher = std::hash::BuildHasherDefault<Hasher>;
#[derive(Default)]
struct Hasher(u64);

impl std::hash::Hasher for Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // This is tricky because our most common inputs are medium length: 16..=88
        // `foldhash` and `rapidhash` have a fast-path for ..16 bytes and outline the rest,
        // but really we want the opposite, or at least the 16.. path to be inlined.

        // SAFETY: `bytes.len()` is checked to be within the bounds of `MAX_INPUT_LEN` by caller.
        unsafe { core::hint::assert_unchecked(bytes.len() <= MAX_INPUT_LEN) };
        if bytes.len() <= 16 {
            super::hint::cold_path();
        }
        self.0 = rapidhash::v3::rapidhash_v3_micro_inline::<false, false>(
            bytes,
            const { &rapidhash::v3::RapidSecrets::seed(0) },
        );
    }

    // We can just skip hashing the length prefix entirely since we know it's always
    // `<=MAX_INPUT_LEN`, and the hash is good enough.

    // `write_length_prefix` calls `write_usize` by default.
    #[inline]
    fn write_usize(&mut self, i: usize) {
        debug_assert!(i <= MAX_INPUT_LEN, "{i} > {MAX_INPUT_LEN}")
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn write_length_prefix(&mut self, len: usize) {
        debug_assert!(len <= MAX_INPUT_LEN, "{len} > {MAX_INPUT_LEN}")
    }
}

/// A cached input.
///
/// `len` comes last so that `data` lines up with the words that the cache copies atomically.
#[derive(Clone, Copy)]
#[repr(C)]
struct Key {
    data: [u8; MAX_INPUT_LEN],
    len: u8,
}

impl PartialEq for Key {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl Eq for Key {}

impl std::hash::Hash for Key {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.get());
    }
}

impl Key {
    #[inline]
    const fn new(input: &[u8]) -> Self {
        // Lock-free reads and `word` load the whole key, so the unused bytes must be initialized.
        let mut data = [0; MAX_INPUT_LEN];
        unsafe { ptr::copy_nonoverlapping(input.as_ptr(), data.as_mut_ptr(), input.len()) };
        Self { data, len: input.len() as u8 }
    }

    /// Returns the `i`th little-endian word of the key's bytes, including `len` in the last one.
    #[inline(always)]
    fn word(&self, i: usize) -> u64 {
        debug_assert!(i < size_of::<Self>() / 8);
        // SAFETY: `Key` is 88 initialized bytes, and `i` is in bounds.
        unsafe { u64::from_le(ptr::from_ref(self).cast::<u64>().add(i).read_unaligned()) }
    }

    #[inline]
    const fn get(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len as usize) }
    }
}

/// A lookup input that compares against [`Key`]s word by word instead of calling `bcmp`.
struct Input<'a>(&'a [u8]);

impl std::hash::Hash for Input<'_> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Must match `Key`'s `Hash` implementation.
        state.write(self.0);
    }
}

impl equivalent::Equivalent<Key> for Input<'_> {
    #[inline]
    fn equivalent(&self, key: &Key) -> bool {
        let input = self.0;
        let len = input.len();
        if key.len as usize != len {
            return false;
        }
        // Compare one word at a time: the cached key was just copied out with 8-byte stores, and
        // wider loads from it would stall on store-to-load forwarding.
        let full = len / 8;
        for i in 0..full {
            // SAFETY: `i * 8 + 8 <= len`.
            if key.word(i) != unsafe { read_u64(input, i * 8) } {
                return false;
            }
        }
        let rem = len % 8;
        rem == 0 || key.word(full) & ((1 << (rem * 8)) - 1) == load_tail(input)
    }
}

/// Loads the last `input.len() % 8` bytes of `input` as a little-endian integer, without reading
/// out of bounds.
#[inline(always)]
fn load_tail(input: &[u8]) -> u64 {
    let len = input.len();
    debug_assert!(len % 8 != 0);
    // SAFETY: All reads are in bounds of `input`, which is not empty.
    unsafe {
        if len >= 8 {
            read_u64(input, len - 8) >> ((8 - len % 8) * 8)
        } else if len >= 4 {
            // The two reads overlap, which is fine since they put the same bytes in the same place.
            let lo = read_u32(input, 0) as u64;
            let hi = read_u32(input, len - 4) as u64;
            lo | (hi << ((len - 4) * 8))
        } else {
            let a = *input.get_unchecked(0) as u64;
            let b = *input.get_unchecked(len / 2) as u64;
            let c = *input.get_unchecked(len - 1) as u64;
            a | (b << (len / 2 * 8)) | (c << ((len - 1) * 8))
        }
    }
}

/// # Safety
///
/// `i + 8 <= s.len()`.
#[inline(always)]
const unsafe fn read_u64(s: &[u8], i: usize) -> u64 {
    u64::from_le(unsafe { s.as_ptr().add(i).cast::<u64>().read_unaligned() })
}

/// # Safety
///
/// `i + 4 <= s.len()`.
#[inline(always)]
const unsafe fn read_u32(s: &[u8], i: usize) -> u32 {
    u32::from_le(unsafe { s.as_ptr().add(i).cast::<u32>().read_unaligned() })
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
                // Race: another thread inserted the same key concurrently. Same input
                // always produces the same hash, so this is a no-op redundant write.
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
        assert_eq!(size_of::<Key>(), MAX_INPUT_LEN + 1);
        assert_eq!(size_of::<fixed_cache::Bucket<(Key, B256)>>(), 128);
    }

    #[test]
    fn input_equivalent() {
        use equivalent::Equivalent;

        let bytes: Vec<u8> = (0..=MAX_INPUT_LEN as u8).map(|i| i.wrapping_mul(37) ^ 0xa5).collect();
        for len in 1..=MAX_INPUT_LEN {
            let input = &bytes[..len];
            let key = Key::new(input);
            assert!(Input(input).equivalent(&key), "len {len}");
            assert!(!Input(&bytes[..len - 1]).equivalent(&key), "len {len}");
            assert!(!Input(&bytes[..len + 1]).equivalent(&key), "len {len}");
            for i in 0..len {
                let mut other = input.to_vec();
                other[i] ^= 1;
                assert!(!Input(&other).equivalent(&key), "len {len}, byte {i}");
            }
        }
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
