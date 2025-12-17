//! A minimalistic one-way set associative cache, generic over key-value types.
//!
//! This cache has a fixed size to allow fast access and minimize per-call overhead.

use core::{
    cell::UnsafeCell,
    hash::{BuildHasher, Hash},
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};

const LOCKED_BIT: usize = 0x0000_8000;

type DefaultBuildHasher = std::hash::BuildHasherDefault<rapidhash::fast::RapidHasher<'static>>;

/// A concurrent set-associative cache, generic over key-value types.
///
/// The cache uses a fixed number of entries and resolves collisions by eviction.
/// It is designed for fast access with minimal per-call overhead.
///
/// # Type Parameters
///
/// - `K`: The key type, must implement `Hash + Eq`.
/// - `V`: The value type, must implement `Clone`.
/// - `S`: The hash builder type, must implement `BuildHasher`.
pub struct Cache<K, V, S = DefaultBuildHasher> {
    entries: *const [Entry<(K, V)>],
    build_hasher: S,
}

impl<K, V, S> core::fmt::Debug for Cache<K, V, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Cache").finish_non_exhaustive()
    }
}

// SAFETY: `Cache` is safe to share across threads because `Entry` uses atomic operations.
unsafe impl<K: Send, V: Send, S: Send> Send for Cache<K, V, S> {}
unsafe impl<K: Send, V: Send, S: Sync> Sync for Cache<K, V, S> {}

impl<K, V, S> Cache<K, V, S> {
    /// Creates a new cache with the specified entries and hasher.
    ///
    /// # Panics
    ///
    /// Panics if `entries.len()` is not a power of two.
    pub const fn with_entries(entries: &'static [Entry<(K, V)>], build_hasher: S) -> Self {
        assert!(entries.len().is_power_of_two());
        Self { entries, build_hasher }
    }

    #[inline]
    const fn index_mask(&self) -> usize {
        self.len() - 1
    }

    #[inline]
    const fn tag_mask(&self) -> usize {
        !self.index_mask()
    }

    /// Returns the hash builder used by this cache.
    pub const fn hasher(&self) -> &S {
        &self.build_hasher
    }

    /// Returns the number of entries in this cache.
    pub const fn len(&self) -> usize {
        (&raw const *self.entries).len()
    }

    /// Returns `true` if the cache has no entries.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V, S> Cache<K, V, S>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher,
{
    /// Gets a value from the cache, or inserts one computed by `f` if not present.
    ///
    /// If the key is found in the cache, returns a clone of the cached value.
    /// Otherwise, calls `f` to compute the value, attempts to insert it, and returns it.
    #[inline]
    pub fn get_or_insert_with<F>(&self, key: &K, f: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        let hash = self.hash_key(key);
        let index_mask = self.index_mask();
        // SAFETY: index is masked to be within bounds.
        let entry = unsafe {
            let entries = &*self.entries;
            entries.get_unchecked(hash & index_mask)
        };

        // Combine hash bits (tag) for quick validation.
        let tag = hash & self.tag_mask();

        // Try to read from cache.
        if entry.try_lock(Some(tag)) {
            // SAFETY: We hold the lock, so we have exclusive access.
            let (cached_key, cached_value) = unsafe { (*entry.data.get()).assume_init_ref() };
            if cached_key == key {
                let cached_value = cached_value.clone();
                entry.unlock(tag);
                return cached_value;
            }
            entry.unlock(tag);
            // Hash collision: same tag but different key.
        }

        // Cache miss or contention - compute value.
        let value = f(key);

        // Try to update cache entry if not locked.
        if entry.try_lock(None) {
            // SAFETY: We hold the lock, so we have exclusive access.
            unsafe {
                let data = (*entry.data.get()).assume_init_mut();
                data.0.clone_from(key);
                data.1.clone_from(&value);
            }
            entry.unlock(tag);
        }

        value
    }

    #[inline]
    fn hash_key(&self, key: &K) -> usize {
        let hash = self.build_hasher.hash_one(key);

        if cfg!(target_pointer_width = "32") {
            ((hash >> 32) as usize) ^ (hash as usize)
        } else {
            hash as usize
        }
    }
}

/// A cache entry.
#[repr(C, align(128))]
pub struct Entry<T> {
    tag: AtomicUsize,
    data: UnsafeCell<MaybeUninit<T>>,
}

impl<T> core::fmt::Debug for Entry<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Entry").finish_non_exhaustive()
    }
}

impl<T> Entry<T> {
    /// Creates a new zeroed cache entry.
    pub const fn new() -> Self {
        Self { tag: AtomicUsize::new(0), data: UnsafeCell::new(MaybeUninit::zeroed()) }
    }

    #[inline]
    fn try_lock(&self, expected: Option<usize>) -> bool {
        let state = self.tag.load(Ordering::Relaxed);
        if let Some(expected) = expected {
            if state != expected {
                return false;
            }
        } else if state & LOCKED_BIT != 0 {
            return false;
        }
        self.tag
            .compare_exchange(state, state | LOCKED_BIT, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    #[inline]
    fn unlock(&self, tag: usize) {
        self.tag.store(tag, Ordering::Release);
    }
}

impl<T> Default for Entry<T> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: `Entry` is a specialized `Mutex<T>` that never blocks.
unsafe impl<T: Send> Send for Entry<T> {}
unsafe impl<T: Send> Sync for Entry<T> {}

/// Declares a static cache with the given name, key type, value type, and size.
///
/// The size must be a power of two.
///
/// # Example
///
/// ```ignore
/// use alloy_primitives::utils::{Cache, define_cache};
///
/// define_cache!(MY_CACHE, u64, String, 1024);
///
/// let value = MY_CACHE.get_or_insert_with(&42, |k| k.to_string());
/// ```
#[macro_export]
macro_rules! define_cache {
    ($name:ident, $K:ty, $V:ty, $size:expr) => {
        static $name: $crate::utils::Cache<$K, $V> = {
            static ENTRIES: [$crate::utils::cache::Entry<($K, $V)>; $size] =
                [const { $crate::utils::cache::Entry::new() }; $size];
            $crate::utils::Cache::with_entries(&ENTRIES, std::hash::BuildHasherDefault::new())
        };
    };
}

#[cfg(test)]
mod tests {
    define_cache!(TEST_CACHE_U64, u64, u64, 1024);
    define_cache!(TEST_CACHE_STRING, String, usize, 1024);

    #[test]
    fn test_basic_get_or_insert() {
        let mut computed = false;
        let value = TEST_CACHE_U64.get_or_insert_with(&42, |&k| {
            computed = true;
            k * 2
        });
        assert!(computed);
        assert_eq!(value, 84);

        computed = false;
        let value = TEST_CACHE_U64.get_or_insert_with(&42, |&k| {
            computed = true;
            k * 2
        });
        assert!(!computed);
        assert_eq!(value, 84);
    }

    #[test]
    fn test_different_keys() {
        let v1 = TEST_CACHE_STRING.get_or_insert_with(&"hello".to_string(), |s| s.len());
        let v2 = TEST_CACHE_STRING.get_or_insert_with(&"world!".to_string(), |s| s.len());

        assert_eq!(v1, 5);
        assert_eq!(v2, 6);
    }
}
