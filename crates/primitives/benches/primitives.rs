#![allow(unused)]

use alloy_primitives::Address;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn primitives(c: &mut Criterion) {
    let mut g = c.benchmark_group("primitives");
    // g.bench_function("address/checksum", |b| {
    //     let address = Address::random();
    //     let out = &mut [0u8; 42];
    //     b.iter(|| {
    //         let x = address.to_checksum_raw(black_box(out), None);
    //         black_box(x);
    //     })
    // });
    g.bench_function("keccak256/32", |b| {
        let mut out = alloy_primitives::B256::random();
        b.iter(|| {
            for _ in 0..10 {
                out = alloy_primitives::keccak256(&out);
            }
            black_box(&out);
        });
    });
    g.finish();
}

criterion_group!(benches, primitives);
criterion_main!(benches);
