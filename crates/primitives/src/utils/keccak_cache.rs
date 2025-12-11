//! LRU cache for keccak256 hash computations.
//!
//! This module provides a transparent caching layer for `keccak256` operations.
//! When the `keccak-cache` feature is enabled, repeated hashes of the same input
//! are served from cache, avoiding redundant computation.
//!
//! The cache is particularly effective for:
//! - Account address hashing (20 bytes) during trie operations
//! - Storage key hashing (32 bytes) during state access
//!
//! Variable-length inputs bypass the cache to avoid polluting it with
//! one-time hashes (RLP encodings, block headers, etc.).
//!
//! ## Cache Architecture
//!
//! Two separate LRU caches are maintained to ensure correctness:
//! - A cache for 20-byte inputs (addresses) using `B160` keys
//! - A cache for 32-byte inputs (storage keys) using `B256` keys
//!
//! This design eliminates any possibility of key collisions between different
//! input sizes.
//!
//! ## Configuration
//!
//! Cache size can be configured via the `ALLOY_KECCAK_CACHE_SIZE` environment
//! variable. The value specifies the total number of entries, split evenly
//! between the two caches (50% for addresses, 50% for storage keys).
//!
//! Default: 100,000 total entries (~8-10MB memory)

use crate::{Address, B256};
use core::sync::atomic::{AtomicU64, Ordering};
use parking_lot::Mutex;
use schnellru::{ByLength, LruMap};
use std::sync::OnceLock;

/// Default cache size (total number of entries across both caches).
///
/// Each 20-byte entry is 52 bytes (20-byte key + 32-byte value).
/// Each 32-byte entry is 64 bytes (32-byte key + 32-byte value).
/// With 50k entries each, total memory is approximately 5.8MB plus LRU overhead.
///
/// Override via `ALLOY_KECCAK_CACHE_SIZE` environment variable.
pub const DEFAULT_KECCAK_CACHE_SIZE: u32 = 100_000;

/// Environment variable name for configuring cache size.
pub const KECCAK_CACHE_SIZE_ENV: &str = "ALLOY_KECCAK_CACHE_SIZE";

/// Statistics for the keccak256 cache.
///
/// These statistics are always collected (no additional overhead beyond atomic increments).
#[derive(Debug, Default)]
pub struct KeccakCacheStats {
    /// Number of cache hits (20-byte inputs).
    hits_20: AtomicU64,
    /// Number of cache hits (32-byte inputs).
    hits_32: AtomicU64,
    /// Number of cache misses (20-byte inputs).
    misses_20: AtomicU64,
    /// Number of cache misses (32-byte inputs).
    misses_32: AtomicU64,
    /// Number of operations that bypassed the cache (variable-length inputs).
    bypassed: AtomicU64,
}

impl KeccakCacheStats {
    /// Returns the total number of cache hits.
    #[inline]
    pub fn hits(&self) -> u64 {
        self.hits_20.load(Ordering::Relaxed) + self.hits_32.load(Ordering::Relaxed)
    }

    /// Returns the number of cache hits for 20-byte (address) inputs.
    #[inline]
    pub fn hits_20(&self) -> u64 {
        self.hits_20.load(Ordering::Relaxed)
    }

    /// Returns the number of cache hits for 32-byte (storage key) inputs.
    #[inline]
    pub fn hits_32(&self) -> u64 {
        self.hits_32.load(Ordering::Relaxed)
    }

    /// Returns the total number of cache misses.
    #[inline]
    pub fn misses(&self) -> u64 {
        self.misses_20.load(Ordering::Relaxed) + self.misses_32.load(Ordering::Relaxed)
    }

    /// Returns the number of cache misses for 20-byte (address) inputs.
    #[inline]
    pub fn misses_20(&self) -> u64 {
        self.misses_20.load(Ordering::Relaxed)
    }

    /// Returns the number of cache misses for 32-byte (storage key) inputs.
    #[inline]
    pub fn misses_32(&self) -> u64 {
        self.misses_32.load(Ordering::Relaxed)
    }

    /// Returns the number of operations that bypassed the cache.
    #[inline]
    pub fn bypassed(&self) -> u64 {
        self.bypassed.load(Ordering::Relaxed)
    }

    /// Returns the total number of cached operations (hits + misses).
    #[inline]
    pub fn total_cached(&self) -> u64 {
        self.hits() + self.misses()
    }

    /// Returns the cache hit rate as a percentage (0.0 - 100.0).
    ///
    /// Only considers cacheable operations (excludes bypassed).
    #[inline]
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits() as f64;
        let total = self.total_cached() as f64;
        if total == 0.0 { 0.0 } else { (hits / total) * 100.0 }
    }

    /// Resets all statistics to zero.
    ///
    /// Note: This is called automatically by [`clear_keccak_cache`].
    pub fn reset(&self) {
        self.hits_20.store(0, Ordering::Relaxed);
        self.hits_32.store(0, Ordering::Relaxed);
        self.misses_20.store(0, Ordering::Relaxed);
        self.misses_32.store(0, Ordering::Relaxed);
        self.bypassed.store(0, Ordering::Relaxed);
    }

    #[inline]
    fn record_hit_20(&self) {
        self.hits_20.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_hit_32(&self) {
        self.hits_32.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_miss_20(&self) {
        self.misses_20.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_miss_32(&self) {
        self.misses_32.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_bypass(&self) {
        self.bypassed.fetch_add(1, Ordering::Relaxed);
    }
}

/// Internal cache structure with separate caches for 20-byte and 32-byte inputs.
///
/// Using separate caches eliminates any possibility of key collisions between
/// different input sizes, ensuring correctness.
struct KeccakCache {
    /// Cache for 20-byte inputs (addresses).
    cache_20: Mutex<LruMap<Address, B256, ByLength>>,
    /// Cache for 32-byte inputs (storage keys).
    cache_32: Mutex<LruMap<B256, B256, ByLength>>,
    /// Statistics.
    stats: KeccakCacheStats,
}

impl KeccakCache {
    fn new(total_size: u32) -> Self {
        // Split cache size evenly between 20-byte and 32-byte caches
        let size_per_cache = total_size / 2;
        Self {
            cache_20: Mutex::new(LruMap::new(ByLength::new(size_per_cache.max(1)))),
            cache_32: Mutex::new(LruMap::new(ByLength::new(size_per_cache.max(1)))),
            stats: KeccakCacheStats::default(),
        }
    }

    /// Returns the total number of entries across both caches.
    fn len(&self) -> usize {
        self.cache_20.lock().len() + self.cache_32.lock().len()
    }

    /// Returns the number of entries in the 20-byte (address) cache.
    fn len_20(&self) -> usize {
        self.cache_20.lock().len()
    }

    /// Returns the number of entries in the 32-byte (storage key) cache.
    fn len_32(&self) -> usize {
        self.cache_32.lock().len()
    }

    /// Clears all entries from both caches and resets statistics.
    fn clear(&self) {
        self.cache_20.lock().clear();
        self.cache_32.lock().clear();
        self.stats.reset();
    }
}

/// Global cache instance.
static GLOBAL_CACHE: OnceLock<KeccakCache> = OnceLock::new();

/// Returns the cache size from environment variable or default.
fn cache_size_from_env() -> u32 {
    std::env::var(KECCAK_CACHE_SIZE_ENV)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_KECCAK_CACHE_SIZE)
}

/// Returns a reference to the global cache.
fn global_cache() -> &'static KeccakCache {
    GLOBAL_CACHE.get_or_init(|| KeccakCache::new(cache_size_from_env()))
}

/// Computes keccak256 with caching for fixed-size inputs.
///
/// This is the internal implementation called by `keccak256` when the
/// `keccak-cache` feature is enabled.
///
/// Caching behavior:
/// - 20-byte inputs: cached in the address cache
/// - 32-byte inputs: cached in the storage key cache
/// - Other sizes: bypass cache entirely
#[inline]
pub(crate) fn keccak256_cached(input: &[u8], compute: impl FnOnce(&[u8]) -> B256) -> B256 {
    let cache = global_cache();

    match input.len() {
        20 => {
            let key = Address::from_slice(input);
            let mut map = cache.cache_20.lock();

            // Use get() to update LRU recency on hit
            if let Some(result) = map.get(&key) {
                cache.stats.record_hit_20();
                return *result;
            }

            // Cache miss - compute and insert while holding lock
            // This prevents duplicate computation under contention
            cache.stats.record_miss_20();
            let result = compute(input);
            map.insert(key, result);
            result
        }
        32 => {
            let key = B256::from_slice(input);
            let mut map = cache.cache_32.lock();

            // Use get() to update LRU recency on hit
            if let Some(result) = map.get(&key) {
                cache.stats.record_hit_32();
                return *result;
            }

            // Cache miss - compute and insert while holding lock
            // This prevents duplicate computation under contention
            cache.stats.record_miss_32();
            let result = compute(input);
            map.insert(key, result);
            result
        }
        _ => {
            // Variable-length inputs bypass the cache
            cache.stats.record_bypass();
            compute(input)
        }
    }
}

/// Returns a reference to the global keccak256 cache statistics.
///
/// Use this to monitor cache effectiveness.
///
/// # Example
///
/// ```ignore
/// use alloy_primitives::utils::keccak_cache_stats;
///
/// let stats = keccak_cache_stats();
/// println!("Cache hit rate: {:.2}%", stats.hit_rate());
/// println!("Hits: {}, Misses: {}, Bypassed: {}",
///     stats.hits(), stats.misses(), stats.bypassed());
/// println!("  20-byte: {} hits, {} misses", stats.hits_20(), stats.misses_20());
/// println!("  32-byte: {} hits, {} misses", stats.hits_32(), stats.misses_32());
/// ```
pub fn keccak_cache_stats() -> &'static KeccakCacheStats {
    &global_cache().stats
}

/// Returns the total number of entries in the keccak256 cache.
///
/// This is the sum of entries in both the 20-byte (address) and 32-byte
/// (storage key) caches.
pub fn keccak_cache_len() -> usize {
    global_cache().len()
}

/// Returns the number of entries in the 20-byte (address) cache.
pub fn keccak_cache_len_20() -> usize {
    global_cache().len_20()
}

/// Returns the number of entries in the 32-byte (storage key) cache.
pub fn keccak_cache_len_32() -> usize {
    global_cache().len_32()
}

/// Clears the keccak256 cache and resets statistics.
///
/// This removes all cached entries from both the 20-byte and 32-byte caches
/// and resets all statistics counters to zero.
///
/// This is primarily useful for testing or when you want to free memory.
pub fn clear_keccak_cache() {
    global_cache().clear();
}

/// Resets the cache statistics without clearing the cache entries.
///
/// This allows you to start fresh statistics collection while preserving
/// the cached values.
pub fn reset_keccak_cache_stats() {
    global_cache().stats.reset();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::keccak256_impl as compute_keccak;

    #[test]
    fn test_cache_20_byte_input() {
        // Use unique input unlikely to collide with other tests
        let address = [0x11u8; 20];
        let expected = compute_keccak(&address);

        // First call produces correct result
        let result1 = keccak256_cached(&address, compute_keccak);
        assert_eq!(result1, expected);

        // Second call also produces correct result (from cache)
        let result2 = keccak256_cached(&address, compute_keccak);
        assert_eq!(result2, expected);
    }

    #[test]
    fn test_cache_32_byte_input() {
        let storage_key = B256::repeat_byte(0x22);
        let expected = compute_keccak(storage_key.as_slice());

        // First call produces correct result
        let result1 = keccak256_cached(storage_key.as_slice(), compute_keccak);
        assert_eq!(result1, expected);

        // Second call also produces correct result (from cache)
        let result2 = keccak256_cached(storage_key.as_slice(), compute_keccak);
        assert_eq!(result2, expected);
    }

    #[test]
    fn test_cache_bypass_variable_length() {
        // Variable-length input should bypass cache but still return correct result
        let input = b"unique bypass test string 12345";
        let expected = compute_keccak(input);

        let result = keccak256_cached(input, compute_keccak);
        assert_eq!(result, expected);

        // Calling again still works (bypasses again)
        let result2 = keccak256_cached(input, compute_keccak);
        assert_eq!(result2, expected);
    }

    #[test]
    fn test_cache_correctness() {
        // Test that cached values are correct for various inputs
        let inputs: Vec<&[u8]> = vec![
            &[0x33u8; 20],                   // 20-byte address
            &[0x44u8; 32],                   // 32-byte key
            b"another variable length test", // bypassed
            &[0x55u8; 20],                   // another address
        ];

        for input in inputs {
            let expected = compute_keccak(input);
            let cached = keccak256_cached(input, compute_keccak);
            assert_eq!(cached, expected, "Mismatch for input of length {}", input.len());

            // Call again to test cache hit path (or bypass path)
            let cached2 = keccak256_cached(input, compute_keccak);
            assert_eq!(cached2, expected);
        }
    }

    #[test]
    fn test_stats_increment() {
        // Test that stats increment correctly using a fresh stats instance
        let stats = KeccakCacheStats::default();

        assert_eq!(stats.hits(), 0);
        assert_eq!(stats.misses(), 0);
        assert_eq!(stats.bypassed(), 0);

        stats.record_hit_20();
        assert_eq!(stats.hits(), 1);
        assert_eq!(stats.hits_20(), 1);
        assert_eq!(stats.hits_32(), 0);

        stats.record_hit_32();
        assert_eq!(stats.hits(), 2);
        assert_eq!(stats.hits_32(), 1);

        stats.record_miss_20();
        assert_eq!(stats.misses(), 1);
        assert_eq!(stats.misses_20(), 1);

        stats.record_miss_32();
        assert_eq!(stats.misses(), 2);
        assert_eq!(stats.misses_32(), 1);

        stats.record_bypass();
        assert_eq!(stats.bypassed(), 1);
    }

    #[test]
    fn test_hit_rate_calculation() {
        // Test the hit rate calculation logic with a fresh stats instance
        let stats = KeccakCacheStats::default();

        // No operations = 0% hit rate
        assert_eq!(stats.hit_rate(), 0.0);

        // 1 miss = 0%
        stats.record_miss_20();
        assert_eq!(stats.hit_rate(), 0.0);

        // 1 hit, 1 miss = 50%
        stats.record_hit_20();
        assert_eq!(stats.hit_rate(), 50.0);

        // 2 hits, 1 miss = 66.67%
        stats.record_hit_32();
        let rate = stats.hit_rate();
        assert!((rate - 66.666666).abs() < 0.001);

        // Bypassed operations don't affect hit rate
        stats.record_bypass();
        assert!((stats.hit_rate() - 66.666666).abs() < 0.001);
    }

    #[test]
    fn test_stats_reset() {
        let stats = KeccakCacheStats::default();

        stats.record_hit_20();
        stats.record_hit_32();
        stats.record_miss_20();
        stats.record_miss_32();
        stats.record_bypass();

        assert!(stats.hits() > 0);
        assert!(stats.misses() > 0);
        assert!(stats.bypassed() > 0);

        stats.reset();

        assert_eq!(stats.hits(), 0);
        assert_eq!(stats.misses(), 0);
        assert_eq!(stats.bypassed(), 0);
        assert_eq!(stats.hits_20(), 0);
        assert_eq!(stats.hits_32(), 0);
        assert_eq!(stats.misses_20(), 0);
        assert_eq!(stats.misses_32(), 0);
    }

    #[test]
    fn test_total_cached() {
        let stats = KeccakCacheStats::default();

        assert_eq!(stats.total_cached(), 0);

        stats.record_hit_20();
        stats.record_miss_20();
        stats.record_miss_32();
        stats.record_bypass(); // bypass doesn't count

        assert_eq!(stats.total_cached(), 3);
        assert_eq!(stats.hits(), 1);
        assert_eq!(stats.misses(), 2);
    }

    #[test]
    fn test_separate_caches_no_collision() {
        // This test ensures that 20-byte and 32-byte inputs use separate caches
        // and cannot possibly collide.

        // 20-byte address (all zeros)
        let address_20 = [0u8; 20];

        // 32-byte key that starts with the same 20 zero bytes
        let key_32 = [0u8; 32];

        // These should produce DIFFERENT hashes
        let hash_20 = keccak256_cached(&address_20, compute_keccak);
        let hash_32 = keccak256_cached(&key_32, compute_keccak);

        // Verify they're actually different
        assert_ne!(
            hash_20, hash_32,
            "20-byte and 32-byte inputs with same prefix must hash differently"
        );

        // Verify the hashes are correct
        assert_eq!(hash_20, compute_keccak(&address_20));
        assert_eq!(hash_32, compute_keccak(&key_32));

        // Now test cache hits return correct values
        let hash_20_again = keccak256_cached(&address_20, compute_keccak);
        let hash_32_again = keccak256_cached(&key_32, compute_keccak);

        assert_eq!(hash_20, hash_20_again);
        assert_eq!(hash_32, hash_32_again);
        assert_ne!(hash_20_again, hash_32_again);
    }

    #[test]
    fn test_cache_isolation() {
        // Verify that the two caches are truly separate
        clear_keccak_cache();

        // Add some addresses
        for i in 0..10u8 {
            let addr = [i; 20];
            keccak256_cached(&addr, compute_keccak);
        }

        // Add some storage keys
        for i in 0..10u8 {
            let key = [i; 32];
            keccak256_cached(&key, compute_keccak);
        }

        // Check that entries are in separate caches
        assert!(keccak_cache_len_20() >= 10);
        assert!(keccak_cache_len_32() >= 10);
    }

    #[test]
    fn test_concurrent_access() {
        use std::{sync::Arc, thread};

        clear_keccak_cache();

        let num_threads = 4;
        let iterations = 1000;

        // Create a set of addresses and keys to hash
        let addresses: Arc<Vec<[u8; 20]>> = Arc::new((0..100u8).map(|i| [i; 20]).collect());
        let keys: Arc<Vec<[u8; 32]>> = Arc::new((0..100u8).map(|i| [i; 32]).collect());

        let handles: Vec<_> = (0..num_threads)
            .map(|t| {
                let addresses = Arc::clone(&addresses);
                let keys = Arc::clone(&keys);
                thread::spawn(move || {
                    for i in 0..iterations {
                        let addr = &addresses[(i + t * 7) % addresses.len()];
                        let key = &keys[(i + t * 11) % keys.len()];

                        // Hash and verify correctness
                        let hash_addr = keccak256_cached(addr, compute_keccak);
                        let hash_key = keccak256_cached(key, compute_keccak);

                        assert_eq!(hash_addr, compute_keccak(addr));
                        assert_eq!(hash_key, compute_keccak(key));
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify stats are reasonable
        let stats = keccak_cache_stats();
        assert!(stats.hits() > 0, "Should have some cache hits");
        assert!(stats.misses() > 0, "Should have some cache misses");
    }
}
