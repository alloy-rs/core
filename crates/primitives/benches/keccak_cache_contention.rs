//! Run with `LLC_BENCH_CPUS=0,1,2,3,4,5 cargo bench -p alloy-primitives --features
//! keccak-cache-global --bench keccak_cache_contention`, then again with
//! `LLC_BENCH_CPUS=0,6,12,18,1,7` (one L3 vs spread on an EPYC 9254).
//!
//! Every thread hashes 64-byte mapping-slot preimages through the global keccak cache. `hot`
//! shares 256 keys between all threads and `wide` 65536. Thread `i` is pinned to the `i`-th
//! CPU of `LLC_BENCH_CPUS` on Linux; without it, 4 unpinned threads run.

#![allow(unknown_lints, clippy::incompatible_msrv, missing_docs)]

use alloy_primitives::keccak256;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::{
    hint::black_box,
    sync::Barrier,
    time::{Duration, Instant},
};

fn keccak_cache_contention(c: &mut Criterion) {
    let cpus = cpus();
    let mut g = c.benchmark_group("keccak_cache_contention");
    g.throughput(Throughput::Elements(cpus.len() as u64));
    for (scenario, n) in [("hot", 256), ("wide", 1 << 16)] {
        let keys = keys(n);
        // Fill the cache so every thread measures steady-state hits.
        for key in &keys {
            black_box(keccak256(key));
        }
        g.bench_function(scenario, |b| {
            b.iter_custom(|iters| {
                run(&cpus, |t| {
                    let mut rng = XorShift(0x9E37_79B9 ^ ((t as u64) << 32) ^ iters | 1);
                    for _ in 0..iters {
                        black_box(keccak256(keys[rng.next() as usize % keys.len()]));
                    }
                })
            })
        });
    }
    g.finish();
}

/// Reads the CPUs to pin threads to from `LLC_BENCH_CPUS`; `None` per thread runs it unpinned.
fn cpus() -> Vec<Option<usize>> {
    match std::env::var("LLC_BENCH_CPUS") {
        Ok(list) => {
            list.split(',').map(|cpu| Some(cpu.trim().parse().expect("bad CPU id"))).collect()
        }
        Err(_) => vec![None; 4],
    }
}

/// Runs `f(thread)` on one thread per CPU, started together, and returns the slowest thread's
/// time.
fn run(cpus: &[Option<usize>], f: impl Fn(usize) + Sync) -> Duration {
    let barrier = Barrier::new(cpus.len());
    std::thread::scope(|scope| {
        let handles: Vec<_> = cpus
            .iter()
            .enumerate()
            .map(|(t, &cpu)| {
                let (barrier, f) = (&barrier, &f);
                scope.spawn(move || {
                    if let Some(cpu) = cpu {
                        pin(cpu);
                    }
                    barrier.wait();
                    let start = Instant::now();
                    f(t);
                    start.elapsed()
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).max().unwrap()
    })
}

#[cfg(target_os = "linux")]
fn pin(cpu: usize) {
    // SAFETY: an all-zero `cpu_set_t` is empty, and pid 0 is the calling thread.
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_SET(cpu, &mut set);
        let rc = libc::sched_setaffinity(0, size_of::<libc::cpu_set_t>(), &set);
        assert_eq!(rc, 0, "sched_setaffinity({cpu}) failed");
    }
}

#[cfg(not(target_os = "linux"))]
fn pin(_cpu: usize) {}

/// `abi.encode(address holder, uint256 slot)` preimages, as Solidity mapping slots hash them.
fn keys(n: usize) -> Vec<[u8; 64]> {
    let mut rng = XorShift(0x1234_5678 ^ n as u64);
    (0..n)
        .map(|i| {
            let mut key = [0u8; 64];
            for b in &mut key[12..32] {
                *b = rng.next() as u8;
            }
            key[56..].copy_from_slice(&(i as u64 % 7).to_be_bytes());
            key
        })
        .collect()
}

struct XorShift(u64);

impl XorShift {
    const fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

criterion_group!(benches, keccak_cache_contention);
criterion_main!(benches);
