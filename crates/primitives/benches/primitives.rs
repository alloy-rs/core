#![allow(unknown_lints, clippy::incompatible_msrv, missing_docs)]

use alloy_primitives::{Address, B256, FixedBytes, keccak256};
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

/// Unconditional chunked comparison (no `memcmp` fallback for large `N`),
/// mirroring `FixedBytes::cmp` below its cutoff. Benchmarked against
/// [`SliceOrd`] across sizes to locate the cutoff.
#[derive(Clone, Copy, PartialEq, Eq)]
struct ChunkedOrd<const N: usize>([u8; N]);

impl<const N: usize> PartialOrd for ChunkedOrd<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for ChunkedOrd<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering;
        let (mut lhs, mut rhs) = (self.0.as_slice(), other.0.as_slice());
        while let (Some((a, lhs_rest)), Some((b, rhs_rest))) =
            (lhs.split_first_chunk(), rhs.split_first_chunk())
        {
            match u64::from_be_bytes(*a).cmp(&u64::from_be_bytes(*b)) {
                Ordering::Equal => (lhs, rhs) = (lhs_rest, rhs_rest),
                unequal => return unequal,
            }
        }
        if let (Some((a, lhs_rest)), Some((b, rhs_rest))) =
            (lhs.split_first_chunk(), rhs.split_first_chunk())
        {
            match u32::from_be_bytes(*a).cmp(&u32::from_be_bytes(*b)) {
                Ordering::Equal => (lhs, rhs) = (lhs_rest, rhs_rest),
                unequal => return unequal,
            }
        }
        if let (Some((a, lhs_rest)), Some((b, rhs_rest))) =
            (lhs.split_first_chunk(), rhs.split_first_chunk())
        {
            match u16::from_be_bytes(*a).cmp(&u16::from_be_bytes(*b)) {
                Ordering::Equal => (lhs, rhs) = (lhs_rest, rhs_rest),
                unequal => return unequal,
            }
        }
        if let (Some(a), Some(b)) = (lhs.first(), rhs.first()) {
            match a.cmp(b) {
                Ordering::Equal => {}
                unequal => return unequal,
            }
        }
        Ordering::Equal
    }
}

/// Benchmarks chunked vs slice (`memcmp`) comparison for a single size `N`,
/// over random pairs (typical map-search comparison: the first chunk decides)
/// and equal pairs (worst case: the full length is compared, where `memcmp`'s
/// vectorization pays off and a cutoff is expected).
fn bench_ord_crossover<const N: usize>(
    g: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
) {
    let random: Vec<(FixedBytes<N>, FixedBytes<N>)> =
        (0..1024).map(|_| (FixedBytes::random(), FixedBytes::random())).collect();
    let equal: Vec<(FixedBytes<N>, FixedBytes<N>)> =
        (0..1024).map(|_| FixedBytes::random()).map(|x| (x, x)).collect();

    for (input, pairs) in [("random", &random), ("equal", &equal)] {
        g.bench_function(format!("{N}/{input}/chunked"), |b| {
            let pairs: Vec<_> =
                pairs.iter().map(|(x, y)| (ChunkedOrd(x.0), ChunkedOrd(y.0))).collect();
            b.iter(|| {
                for (x, y) in &pairs {
                    black_box(x.cmp(y));
                }
            })
        });
        g.bench_function(format!("{N}/{input}/memcmp"), |b| {
            let pairs: Vec<_> = pairs.iter().map(|(x, y)| (SliceOrd(x.0), SliceOrd(y.0))).collect();
            b.iter(|| {
                for (x, y) in &pairs {
                    black_box(x.cmp(y));
                }
            })
        });
    }
}

fn fixed_bytes_ord(c: &mut Criterion) {
    let mut g = c.benchmark_group("fixed_bytes_ord");

    bench_ord_crossover::<8>(&mut g);
    bench_ord_crossover::<20>(&mut g);
    bench_ord_crossover::<32>(&mut g);
    bench_ord_crossover::<64>(&mut g);
    bench_ord_crossover::<128>(&mut g);
    bench_ord_crossover::<256>(&mut g);

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
