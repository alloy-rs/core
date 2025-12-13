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
    hash::BuildHasher,
    sync::atomic::{AtomicU32, Ordering},
};

/// Number of cache entries (must be a power of 2).
const COUNT: usize = 1 << 17; // ~131k entries
const INDEX_MASK: u32 = (COUNT - 1) as u32;
const HASH_MASK: u32 = !INDEX_MASK;

const LOCKED_BIT: u32 = 0x0000_8000;

/// Maximum input length that can be cached.
pub(super) const MAX_INPUT_LEN: usize = 60;

/// A cache entry.
#[repr(C, align(32))]
struct Entry {
    combined: AtomicU32,
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
    fn try_lock(&self, expected: Option<u32>) -> bool {
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
    fn unlock(&self, combined: u32) {
        self.combined.store(combined, Ordering::Release);
    }
}

// SAFETY: `Entry` is a specialized `Mutex<EntryData>` that never blocks.
unsafe impl Send for Entry {}
unsafe impl Sync for Entry {}

/// Global cache storage.
///
/// This is sort of an open-coded flat `HashMap<&[u8], Mutex<EntryData>>`.
static CACHE: [Entry; COUNT] = [const { Entry::new() }; COUNT];

pub(super) fn compute(input: &[u8]) -> B256 {
    if unlikely(input.is_empty() | (input.len() > MAX_INPUT_LEN)) {
        return if input.is_empty() { KECCAK256_EMPTY } else { keccak256(input) };
    }

    let hash64 = crate::map::DefaultHashBuilder::default().hash_one(input);
    let hash = ((hash64 >> 32) as u32) ^ (hash64 as u32);
    let index = (hash & INDEX_MASK) as usize;
    let entry = &CACHE[index];

    // Combine hash bits and length.
    // This acts as a cache key to quickly determine if the entry is valid in the next check.
    let combined = (hash & HASH_MASK) | input.len() as u32;

    if entry.try_lock(Some(combined)) {
        // SAFETY: We hold the lock, so we have exclusive access.
        let EntryData { value, keccak256: result } = unsafe { *entry.data.get() };

        entry.unlock(combined);

        if likely(value[..input.len()] == input[..]) {
            // Cache hit!
            return result;
        }
    }

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
