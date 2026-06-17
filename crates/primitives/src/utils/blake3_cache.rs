//! A minimalistic one-way set associative cache for BLAKE3-256 values.
//!
//! This cache has a fixed size to allow fast access and minimize per-call overhead.

use super::{blake3_256_impl as blake3_256, hint::unlikely};
use crate::{B256, BLAKE3_EMPTY};
use std::{mem::MaybeUninit, sync::OnceLock};

/// Maximum input length that can be cached.
pub(super) const MAX_INPUT_LEN: usize =
    128 - size_of::<B256>() - size_of::<u8>() - size_of::<usize>();

const DEFAULT_COUNT: usize = 1 << 17; // ~131k entries * 128 bytes = 16MiB
static CACHE: OnceLock<Cache> = OnceLock::new();

type Cache = fixed_cache::Cache<Key, B256, BuildHasher, CacheConfig>;

struct CacheConfig {}
impl fixed_cache::CacheConfig for CacheConfig {
    const STATS: bool = false;
    const EPOCHS: bool = false;
}

/// Initializes the process-global BLAKE3 cache with `entries` buckets.
///
/// Returns `true` if this call initialized the cache, and `false` if the cache was already
/// initialized by an earlier call or by the first cached hash computation. If this is never called,
/// the cache is initialized lazily with the default size on first use.
///
/// # Panics
///
/// Panics if `entries` is not a power of two or is less than 4.
#[must_use]
pub fn init_blake3_cache(entries: usize) -> bool {
    init_cache(&CACHE, entries)
}

pub(super) fn compute(input: &[u8], imp: impl FnOnce(&[u8]) -> B256) -> B256 {
    if unlikely(input.is_empty() | (input.len() > MAX_INPUT_LEN)) {
        return if input.is_empty() { BLAKE3_EMPTY } else { blake3_256(input) };
    }

    let cache = CACHE.get_or_init(default_cache);
    cache.get_or_insert_with_ref(input, imp, |input| {
        let mut data = [MaybeUninit::uninit(); MAX_INPUT_LEN];
        unsafe {
            std::ptr::copy_nonoverlapping(input.as_ptr(), data.as_mut_ptr().cast(), input.len())
        };
        Key { len: input.len() as u8, data }
    })
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
    fixed_cache::Cache::new(entries, BuildHasher::new())
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

#[derive(Clone, Copy)]
struct Key {
    len: u8,
    data: [MaybeUninit<u8>; MAX_INPUT_LEN],
}

impl PartialEq for Key {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl Eq for Key {}

impl std::borrow::Borrow<[u8]> for Key {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.get()
    }
}

impl std::hash::Hash for Key {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.get());
    }
}

impl Key {
    #[inline]
    const fn get(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast(), self.len as usize) }
    }
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
    fn caching() {
        let mut count: usize = 0;
        let mut compute = |input| {
            compute(input, |x| {
                count += 1;
                blake3_256(x)
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

    #[test]
    fn bypasses_over_max_input() {
        let mut count: usize = 0;
        let input = vec![0xBB; MAX_INPUT_LEN + 1];
        let hash = compute(&input, |_| {
            count += 1;
            B256::ZERO
        });

        assert_eq!(hash, blake3_256(&input));
        assert_eq!(count, 0);
    }
}
