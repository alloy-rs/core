#![allow(unknown_lints, clippy::incompatible_msrv, missing_docs)]

use alloy_primitives::{Address, B256, keccak256};
use criterion::{Criterion, criterion_group, criterion_main};
use std::{collections::BTreeMap, hint::black_box};

fn primitives(c: &mut Criterion) {
    let mut g = c.benchmark_group("primitives");
    g.bench_function("address/checksum", |b| {
        let address = Address::random();
        let out = &mut [0u8; 42];
        b.iter(|| {
            let x = address.to_checksum_raw(black_box(out), None);
            black_box(x);
        })
    });
    g.bench_function("keccak256/32", |b| {
        let mut out = B256::random();
        b.iter(|| {
            out = keccak256(out.as_slice());
            black_box(&out);
        });
    });
    g.finish();
}

/// Baseline for the `Ord` benchmarks: byte-slice comparison, which is what the
/// derived `Ord` implementation produced (lowers to a `memcmp` libcall).
#[derive(Clone, Copy, PartialEq, Eq)]
struct SliceOrd<const N: usize>([u8; N]);

impl<const N: usize> PartialOrd for SliceOrd<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for SliceOrd<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.as_slice().cmp(other.0.as_slice())
    }
}

fn fixed_bytes_ord(c: &mut Criterion) {
    let mut g = c.benchmark_group("fixed_bytes_ord");

    // Raw comparison throughput over random pairs (~50% of real map-search
    // comparisons decide on the first chunk; random pairs model that).
    let address_pairs: Vec<(Address, Address)> =
        (0..1024).map(|_| (Address::random(), Address::random())).collect();
    let address_slice_pairs: Vec<(SliceOrd<20>, SliceOrd<20>)> =
        address_pairs.iter().map(|(x, y)| (SliceOrd(x.0.0), SliceOrd(y.0.0))).collect();
    g.bench_function("address/cmp/chunked", |b| {
        b.iter(|| {
            for (x, y) in &address_pairs {
                black_box(x.cmp(y));
            }
        })
    });
    g.bench_function("address/cmp/slice_baseline", |b| {
        b.iter(|| {
            for (x, y) in &address_slice_pairs {
                black_box(x.cmp(y));
            }
        })
    });

    let b256_pairs: Vec<(B256, B256)> =
        (0..1024).map(|_| (B256::random(), B256::random())).collect();
    let b256_slice_pairs: Vec<(SliceOrd<32>, SliceOrd<32>)> =
        b256_pairs.iter().map(|(x, y)| (SliceOrd(x.0), SliceOrd(y.0))).collect();
    g.bench_function("b256/cmp/chunked", |b| {
        b.iter(|| {
            for (x, y) in &b256_pairs {
                black_box(x.cmp(y));
            }
        })
    });
    g.bench_function("b256/cmp/slice_baseline", |b| {
        b.iter(|| {
            for (x, y) in &b256_slice_pairs {
                black_box(x.cmp(y));
            }
        })
    });

    // Real-world shape: BTreeMap keyed by Address, hit lookups.
    let keys: Vec<Address> = (0..100_000).map(|_| Address::random()).collect();
    let map: BTreeMap<Address, u64> =
        keys.iter().enumerate().map(|(i, k)| (*k, i as u64)).collect();
    let slice_map: BTreeMap<SliceOrd<20>, u64> =
        keys.iter().enumerate().map(|(i, k)| (SliceOrd(k.0.0), i as u64)).collect();
    g.bench_function("address/btreemap_get/chunked", |b| {
        let mut i = 0;
        b.iter(|| {
            i = (i + 1) % keys.len();
            black_box(map.get(black_box(&keys[i])));
        })
    });
    g.bench_function("address/btreemap_get/slice_baseline", |b| {
        let mut i = 0;
        b.iter(|| {
            i = (i + 1) % keys.len();
            black_box(slice_map.get(black_box(&SliceOrd(keys[i].0.0))));
        })
    });

    g.finish();
}

criterion_group!(benches, primitives, fixed_bytes_ord);
criterion_main!(benches);
