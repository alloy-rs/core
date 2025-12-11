//! Benchmark for keccak256 cache performance.
//!
//! Run with: cargo bench -p alloy-primitives --features keccak-cache --bench keccak_cache

use alloy_primitives::{keccak256, B256};
use std::time::{Duration, Instant};

const NUM_UNIQUE_ADDRESSES: usize = 5000;
const NUM_UNIQUE_KEYS: usize = 5000;
const NUM_ITERATIONS: usize = 100_000;

fn generate_random_bytes<const N: usize>(seed: u64) -> [u8; N] {
    let mut result = [0u8; N];
    let mut state = seed;
    for byte in &mut result {
        // Simple xorshift64 PRNG
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        *byte = state as u8;
    }
    result
}

fn main() {
    println!("Generating test data...");

    // Generate unique addresses (20 bytes) and storage keys (32 bytes)
    let addresses: Vec<[u8; 20]> = (0..NUM_UNIQUE_ADDRESSES as u64)
        .map(|i| generate_random_bytes::<20>(i * 31337))
        .collect();

    let storage_keys: Vec<[u8; 32]> = (0..NUM_UNIQUE_KEYS as u64)
        .map(|i| generate_random_bytes::<32>(i * 31337 + 1000000))
        .collect();

    println!("Generated {} unique addresses and {} unique storage keys\n",
             addresses.len(), storage_keys.len());

    // Clear cache and stats
    #[cfg(feature = "keccak-cache")]
    {
        alloy_primitives::utils::clear_keccak_cache();
    }

    // =========================================================================
    // Benchmark 1: Cold cache (all misses) - 20 byte addresses
    // =========================================================================
    println!("=== Benchmark 1: Cold Cache - 20-byte Addresses ===");
    #[cfg(feature = "keccak-cache")]
    {
        alloy_primitives::utils::clear_keccak_cache();
    }

    let start = Instant::now();
    for addr in &addresses {
        let _ = keccak256(addr);
    }
    let cold_20_duration = start.elapsed();
    let cold_20_per_op = cold_20_duration.as_nanos() / addresses.len() as u128;
    println!("  {} ops in {:?}", addresses.len(), cold_20_duration);
    println!("  Average: {} ns/op (cache misses)\n", cold_20_per_op);

    // =========================================================================
    // Benchmark 2: Warm cache (all hits) - 20 byte addresses
    // =========================================================================
    println!("=== Benchmark 2: Warm Cache - 20-byte Addresses ===");
    let start = Instant::now();
    for addr in &addresses {
        let _ = keccak256(addr);
    }
    let warm_20_duration = start.elapsed();
    let warm_20_per_op = warm_20_duration.as_nanos() / addresses.len() as u128;
    println!("  {} ops in {:?}", addresses.len(), warm_20_duration);
    println!("  Average: {} ns/op (cache hits)\n", warm_20_per_op);

    // =========================================================================
    // Benchmark 3: Cold cache (all misses) - 32 byte keys
    // =========================================================================
    println!("=== Benchmark 3: Cold Cache - 32-byte Storage Keys ===");
    #[cfg(feature = "keccak-cache")]
    {
        alloy_primitives::utils::clear_keccak_cache();
    }

    let start = Instant::now();
    for key in &storage_keys {
        let _ = keccak256(key);
    }
    let cold_32_duration = start.elapsed();
    let cold_32_per_op = cold_32_duration.as_nanos() / storage_keys.len() as u128;
    println!("  {} ops in {:?}", storage_keys.len(), cold_32_duration);
    println!("  Average: {} ns/op (cache misses)\n", cold_32_per_op);

    // =========================================================================
    // Benchmark 4: Warm cache (all hits) - 32 byte keys
    // =========================================================================
    println!("=== Benchmark 4: Warm Cache - 32-byte Storage Keys ===");
    let start = Instant::now();
    for key in &storage_keys {
        let _ = keccak256(key);
    }
    let warm_32_duration = start.elapsed();
    let warm_32_per_op = warm_32_duration.as_nanos() / storage_keys.len() as u128;
    println!("  {} ops in {:?}", storage_keys.len(), warm_32_duration);
    println!("  Average: {} ns/op (cache hits)\n", warm_32_per_op);

    // =========================================================================
    // Benchmark 5: Realistic workload - repeated access pattern
    // =========================================================================
    println!("=== Benchmark 5: Realistic Workload (repeated access) ===");
    #[cfg(feature = "keccak-cache")]
    {
        alloy_primitives::utils::clear_keccak_cache();
    }

    // Simulate realistic access: some addresses/keys accessed many times
    let mut workload: Vec<&[u8]> = Vec::with_capacity(NUM_ITERATIONS);
    let mut state = 12345u64;
    for _ in 0..NUM_ITERATIONS {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;

        if state % 2 == 0 {
            // Pick an address (with bias toward earlier addresses = hot addresses)
            let idx = ((state >> 8) as usize % 100).min(addresses.len() - 1);
            workload.push(&addresses[idx]);
        } else {
            // Pick a storage key
            let idx = ((state >> 8) as usize % 200).min(storage_keys.len() - 1);
            workload.push(&storage_keys[idx]);
        }
    }

    let start = Instant::now();
    for input in &workload {
        let _ = keccak256(*input);
    }
    let workload_duration = start.elapsed();
    let workload_per_op = workload_duration.as_nanos() / NUM_ITERATIONS as u128;
    println!("  {} ops in {:?}", NUM_ITERATIONS, workload_duration);
    println!("  Average: {} ns/op\n", workload_per_op);

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary ===");
    println!("  20-byte addresses:");
    println!("    Cache miss: {} ns", cold_20_per_op);
    println!("    Cache hit:  {} ns", warm_20_per_op);
    if cold_20_per_op > 0 {
        println!("    Speedup:    {:.2}x", cold_20_per_op as f64 / warm_20_per_op.max(1) as f64);
    }
    println!();
    println!("  32-byte storage keys:");
    println!("    Cache miss: {} ns", cold_32_per_op);
    println!("    Cache hit:  {} ns", warm_32_per_op);
    if cold_32_per_op > 0 {
        println!("    Speedup:    {:.2}x", cold_32_per_op as f64 / warm_32_per_op.max(1) as f64);
    }
    println!();

    #[cfg(feature = "keccak-cache")]
    {
        use alloy_primitives::utils::{keccak_cache_len, keccak_cache_len_20, keccak_cache_len_32, keccak_cache_stats};
        let stats = keccak_cache_stats();
        println!("  Cache stats:");
        println!("    Hits (20-byte):   {}", stats.hits_20());
        println!("    Hits (32-byte):   {}", stats.hits_32());
        println!("    Hits (total):     {}", stats.hits());
        println!("    Misses (20-byte): {}", stats.misses_20());
        println!("    Misses (32-byte): {}", stats.misses_32());
        println!("    Misses (total):   {}", stats.misses());
        println!("    Bypassed:         {}", stats.bypassed());
        println!("    Hit rate:         {:.2}%", stats.hit_rate());
        println!("    Entries (20-byte): {}", keccak_cache_len_20());
        println!("    Entries (32-byte): {}", keccak_cache_len_32());
        println!("    Entries (total):   {}", keccak_cache_len());
    }
}
