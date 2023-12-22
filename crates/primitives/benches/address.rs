use alloy_primitives::Address;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn address(c: &mut Criterion) {
    let mut g = c.benchmark_group("address");
    g.bench_function("checksum", |b| {
        let address = Address::random();
        let out = &mut [0u8; 42];
        b.iter(|| {
            let x = address.to_checksum_raw(black_box(out), None);
            black_box(x);
        })
    });
}

criterion_group!(benches, address);
criterion_main!(benches);
