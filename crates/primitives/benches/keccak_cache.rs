//! Benchmark for keccak256 cache performance.
//!
//! Run with: cargo bench -p alloy-primitives --features keccak-cache,rand --bench keccak_cache

#![allow(unknown_lints, clippy::incompatible_msrv, missing_docs)]

use alloy_primitives::{Address, B256, keccak256, utils::clear_keccak_cache};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const NUM_UNIQUE_ADDRESSES: usize = 5000;
const NUM_UNIQUE_KEYS: usize = 5000;

fn keccak_cache_benchmarks(c: &mut Criterion) {
    // Generate unique addresses and storage keys
    let addresses: Vec<Address> = (0..NUM_UNIQUE_ADDRESSES).map(|_| Address::random()).collect();
    let storage_keys: Vec<B256> = (0..NUM_UNIQUE_KEYS).map(|_| B256::random()).collect();

    let mut g = c.benchmark_group("keccak_cache");

    // Benchmark: Cold cache - 20-byte addresses (all misses)
    g.bench_function(BenchmarkId::new("cold", "20-byte"), |b| {
        b.iter_batched(
            || {
                clear_keccak_cache();
            },
            |()| {
                for addr in &addresses {
                    black_box(keccak256(addr.as_slice()));
                }
            },
            criterion::BatchSize::PerIteration,
        )
    });

    // Benchmark: Warm cache - 20-byte addresses (all hits)
    // First populate the cache
    clear_keccak_cache();
    for addr in &addresses {
        let _ = keccak256(addr.as_slice());
    }
    g.bench_function(BenchmarkId::new("warm", "20-byte"), |b| {
        b.iter(|| {
            for addr in &addresses {
                black_box(keccak256(addr.as_slice()));
            }
        })
    });

    // Benchmark: Cold cache - 32-byte storage keys (all misses)
    g.bench_function(BenchmarkId::new("cold", "32-byte"), |b| {
        b.iter_batched(
            || {
                clear_keccak_cache();
            },
            |()| {
                for key in &storage_keys {
                    black_box(keccak256(key.as_slice()));
                }
            },
            criterion::BatchSize::PerIteration,
        )
    });

    // Benchmark: Warm cache - 32-byte storage keys (all hits)
    // First populate the cache
    clear_keccak_cache();
    for key in &storage_keys {
        let _ = keccak256(key.as_slice());
    }
    g.bench_function(BenchmarkId::new("warm", "32-byte"), |b| {
        b.iter(|| {
            for key in &storage_keys {
                black_box(keccak256(key.as_slice()));
            }
        })
    });

    // Benchmark: Realistic workload with mixed access patterns
    g.bench_function("realistic_workload", |b| {
        b.iter_batched(
            || {
                clear_keccak_cache();
            },
            |()| {
                // Simulate realistic access: repeated access to hot addresses/keys
                for i in 0..10000 {
                    if i % 2 == 0 {
                        // Access addresses with bias toward "hot" ones
                        let idx = (i % 100).min(addresses.len() - 1);
                        black_box(keccak256(addresses[idx].as_slice()));
                    } else {
                        // Access storage keys
                        let idx = (i % 200).min(storage_keys.len() - 1);
                        black_box(keccak256(storage_keys[idx].as_slice()));
                    }
                }
            },
            criterion::BatchSize::PerIteration,
        )
    });

    g.finish();
}

criterion_group!(benches, keccak_cache_benchmarks);
criterion_main!(benches);
