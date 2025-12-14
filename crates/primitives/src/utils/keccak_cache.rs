//! A minimalistic one-way set associative cache for Keccak256 values.
//!
//! This cache has a fixed size to allow fast access and minimize per-call overhead.

use super::{
    hint::{likely, unlikely},
    keccak256_impl as keccak256,
};
use crate::{B256, KECCAK256_EMPTY};
use core::{
    cell::UnsafeCell,
    hash::{BuildHasher, Hasher},
    sync::atomic::{AtomicUsize, Ordering},
};

const ENABLE_STATS: bool = false || option_env!("KECCAK_CACHE_STATS").is_some();

/// Number of cache entries (must be a power of 2).
const COUNT: usize = 1 << 17; // ~131k entries

const INDEX_MASK: usize = COUNT - 1;
const HASH_MASK: usize = !INDEX_MASK;

const LOCKED_BIT: usize = 0x0000_8000;

/// Maximum input length that can be cached.
pub(super) const MAX_INPUT_LEN: usize = 128 - 32 - size_of::<usize>();

/// Global cache storage.
///
/// This is sort of an open-coded flat `HashMap<&[u8], Mutex<EntryData>>`.
static CACHE: [Entry; COUNT] = [const { Entry::new() }; COUNT];

pub(super) fn compute(input: &[u8]) -> B256 {
    if unlikely(input.is_empty() | (input.len() > MAX_INPUT_LEN)) {
        return if input.is_empty() {
            stats::hit(0);
            KECCAK256_EMPTY
        } else {
            stats::out_of_range(input.len());
            keccak256(input)
        };
    }

    let hash = hash_bytes(input);
    let entry = &CACHE[hash & INDEX_MASK];

    // Combine hash bits and length.
    // This acts as a cache key to quickly determine if the entry is valid in the next check.
    let combined = (hash & HASH_MASK) | input.len();

    if entry.try_lock(Some(combined)) {
        // SAFETY: We hold the lock, so we have exclusive access.
        let EntryData { value, keccak256: result } = unsafe { *entry.data.get() };

        entry.unlock(combined);

        if likely(value[..input.len()] == input[..]) {
            // Cache hit!
            stats::hit(input.len());
            return result;
        }
    }
    stats::miss(input.len());

    // Cache miss or contention - compute hash.
    let result = keccak256(input);

    // Try to update cache entry if not locked.
    if entry.try_lock(None) {
        // SAFETY: We hold the lock, so we have exclusive access.
        unsafe {
            let data = &mut *entry.data.get();
            data.value[..input.len()].copy_from_slice(input);
            data.keccak256 = result;
        }

        entry.unlock(combined);
    }

    result
}

/// A cache entry.
#[repr(C, align(128))]
struct Entry {
    combined: AtomicUsize,
    data: UnsafeCell<EntryData>,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct EntryData {
    value: [u8; MAX_INPUT_LEN],
    keccak256: B256,
}

impl Entry {
    #[inline]
    const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    #[inline]
    fn try_lock(&self, expected: Option<usize>) -> bool {
        let state = self.combined.load(Ordering::Relaxed);
        if let Some(expected) = expected {
            if state != expected {
                return false;
            }
        } else if state & LOCKED_BIT != 0 {
            return false;
        }
        self.combined
            .compare_exchange(state, state | LOCKED_BIT, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    #[inline]
    fn unlock(&self, combined: usize) {
        self.combined.store(combined, Ordering::Release);
    }
}

// SAFETY: `Entry` is a specialized `Mutex<EntryData>` that never blocks.
unsafe impl Send for Entry {}
unsafe impl Sync for Entry {}

#[inline(always)]
fn hash_bytes(input: &[u8]) -> usize {
    // Use `Hasher::write` instead of `Hash::hash` to avoid hashing the length since it's already
    // considered in the `foldhash` algorithm.
    let mut hasher = foldhash::fast::FixedState::with_seed(0).build_hasher();
    hasher.write(input);
    let hash = hasher.finish();
    if cfg!(target_pointer_width = "32") {
        ((hash >> 32) as usize) ^ (hash as usize)
    } else {
        hash as usize
    }
}

// NOT PUBLIC API.
pub(super) mod stats {
    use super::*;
    use std::{collections::HashMap, sync::Mutex};

    static STATS: KeccakCacheStats = KeccakCacheStats {
        hits: [const { AtomicUsize::new(0) }; MAX_INPUT_LEN + 1],
        misses: [const { AtomicUsize::new(0) }; MAX_INPUT_LEN + 1],
        out_of_range: Mutex::new(HashMap::with_hasher(foldhash::fast::FixedState::with_seed(0))),
    };

    struct KeccakCacheStats {
        hits: [AtomicUsize; MAX_INPUT_LEN + 1],
        misses: [AtomicUsize; MAX_INPUT_LEN + 1],
        out_of_range: Mutex<HashMap<usize, usize, foldhash::fast::FixedState>>,
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

    #[inline]
    pub(super) fn out_of_range(len: usize) {
        if !ENABLE_STATS {
            return;
        }
        *STATS.out_of_range.lock().unwrap().entry(len).or_insert(0) += 1;
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

        out
    }
}
