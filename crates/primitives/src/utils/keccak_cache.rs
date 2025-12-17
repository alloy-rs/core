//! A minimalistic one-way set associative cache for Keccak256 values.
//!
//! This cache has a fixed size to allow fast access and minimize per-call overhead.

use super::{hint::unlikely, keccak256_impl as keccak256};
use crate::{B256, KECCAK256_EMPTY};
use core::sync::atomic::{AtomicUsize, Ordering};

const ENABLE_STATS: bool = false || option_env!("KECCAK_CACHE_STATS").is_some();

/// Maximum input length that can be cached.
pub(super) const MAX_INPUT_LEN: usize = 128 - 32 - size_of::<usize>() - 1;

const COUNT: usize = 1 << 17; // ~131k entries
static CACHE: fixed_cache::Cache<Key, B256, BuildHasher> =
    fixed_cache::static_cache!(Key, B256, COUNT, BuildHasher::new());

pub(super) fn compute(input: &[u8]) -> B256 {
    if unlikely(input.is_empty() | (input.len() > MAX_INPUT_LEN)) {
        return if input.is_empty() {
            // stats::hit(0);
            KECCAK256_EMPTY
        } else {
            // stats::out_of_range(input.len());
            keccak256(input)
        };
    }

    CACHE.get_or_insert_with_ref(input, keccak256, |input| {
        let mut data = [0u8; MAX_INPUT_LEN];
        data[..input.len()].copy_from_slice(input);
        Key { len: input.len() as u8, data }
    })
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
}

#[derive(Clone, Copy)]
struct Key {
    len: u8,
    data: [u8; MAX_INPUT_LEN],
}

impl PartialEq for Key {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.get() == other.get()
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
    fn get(&self) -> &[u8] {
        unsafe { self.data.get_unchecked(..self.len as usize) }
    }
}

// NOT PUBLIC API.
pub(super) mod stats {
    use super::*;
    use std::{collections::HashMap, sync::Mutex};

    type BuildHasher = std::hash::BuildHasherDefault<rapidhash::fast::RapidHasher<'static>>;

    static STATS: KeccakCacheStats = KeccakCacheStats {
        hits: [const { AtomicUsize::new(0) }; MAX_INPUT_LEN + 1],
        misses: [const { AtomicUsize::new(0) }; MAX_INPUT_LEN + 1],
        out_of_range: Mutex::new(HashMap::with_hasher(BuildHasher::new())),
        collisions: Mutex::new(Vec::new()),
    };

    struct KeccakCacheStats {
        hits: [AtomicUsize; MAX_INPUT_LEN + 1],
        misses: [AtomicUsize; MAX_INPUT_LEN + 1],
        out_of_range: Mutex<HashMap<usize, usize, BuildHasher>>,
        collisions: Mutex<Vec<(String, String)>>,
    }

    #[inline]
    pub(super) fn hit(len: usize) {
        if !ENABLE_STATS {
            return;
        }
        STATS.hits[len].fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub(super) fn miss(len: usize) {
        if !ENABLE_STATS {
            return;
        }
        STATS.misses[len].fetch_add(1, Ordering::Relaxed);
    }

    #[inline(never)]
    pub(super) fn out_of_range(len: usize) {
        if !ENABLE_STATS {
            return;
        }
        *STATS.out_of_range.lock().unwrap().entry(len).or_insert(0) += 1;
    }

    #[inline(never)]
    pub(super) fn collision(input: &[u8], cached: &[u8]) {
        if !ENABLE_STATS {
            return;
        }
        let input_hex = crate::hex::encode(input);
        let cached_hex = crate::hex::encode(cached);
        STATS.collisions.lock().unwrap().push((input_hex, cached_hex));
    }

    #[doc(hidden)]
    pub fn format() -> String {
        use core::fmt::Write;
        let mut out = String::new();

        if !ENABLE_STATS {
            out.push_str("keccak cache stats: DISABLED");
            return out;
        }

        let mut total_hits = 0usize;
        let mut total_misses = 0usize;
        let mut entries: Vec<(usize, usize, usize)> = Vec::new();
        for len in 0..=MAX_INPUT_LEN {
            let hits = STATS.hits[len].load(Ordering::Relaxed);
            let misses = STATS.misses[len].load(Ordering::Relaxed);
            if hits > 0 || misses > 0 {
                entries.push((len, hits, misses));
                total_hits += hits;
                total_misses += misses;
            }
        }
        for (&len, &misses) in STATS.out_of_range.lock().unwrap().iter() {
            entries.push((len, 0, misses));
            total_misses += misses;
        }
        entries.sort_by_key(|(len, _, _)| *len);

        writeln!(out, "keccak cache stats by length:").unwrap();
        writeln!(out, "{:>6} {:>12} {:>12} {:>8}", "len", "hits", "misses", "hit%").unwrap();
        for (len, hits, misses) in entries {
            let total = hits + misses;
            let hit_rate = (hits as f64 / total as f64) * 100.0;
            writeln!(out, "{len:>6} {hits:>12} {misses:>12} {hit_rate:>7.1}%").unwrap();
        }
        let total = total_hits + total_misses;
        if total > 0 {
            let hit_rate = (total_hits as f64 / total as f64) * 100.0;
            writeln!(
                out,
                "{:>6} {:>12} {:>12} {:>7.1}%",
                "all", total_hits, total_misses, hit_rate
            )
            .unwrap();
        }

        let collisions = STATS.collisions.lock().unwrap();
        if !collisions.is_empty() {
            writeln!(out, "\nhash collisions ({}):", collisions.len()).unwrap();
            for (input, cached) in collisions.iter() {
                writeln!(out, "  input:  0x{input}").unwrap();
                writeln!(out, "  cached: 0x{cached}").unwrap();
                writeln!(out).unwrap();
            }
        }

        out
    }
}
