#![allow(unknown_lints, clippy::incompatible_msrv, missing_docs)]

use alloy_primitives::{I256, U256};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const N: usize = 256;

/// Deterministic xorshift64* generator.
struct Rng(u64);

impl Rng {
    const fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Random value with `limbs` non-zero 64-bit limbs and a random sign.
    fn int(&mut self, limbs: usize) -> I256 {
        let mut l = [0u64; 4];
        for x in &mut l[..limbs] {
            *x = self.next();
        }
        // Clear the top bit of the last used limb so the magnitude is below `2^(64 * limbs - 1)`.
        l[limbs - 1] >>= 1;
        let abs = I256::from_raw(U256::from_limbs(l));
        if self.next() & 1 == 0 { abs } else { -abs }
    }

    fn non_zero(&mut self, limbs: usize) -> I256 {
        loop {
            let x = self.int(limbs);
            if !x.is_zero() {
                return x;
            }
        }
    }
}

/// `(lhs limbs, rhs limbs)` shapes: small (64-bit), mixed, and full-width operands.
const SHAPES: [(&str, usize, usize); 3] = [("small", 1, 1), ("mixed", 4, 1), ("full", 4, 4)];

fn pairs(limbs: (usize, usize)) -> Vec<(I256, I256)> {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    (0..N).map(|_| (rng.int(limbs.0), rng.non_zero(limbs.1))).collect()
}

fn bench_binary(c: &mut Criterion, name: &str, f: impl Fn(I256, I256) -> I256) {
    let mut g = c.benchmark_group(name);
    for (shape, l, r) in SHAPES {
        let pairs = pairs((l, r));
        g.bench_function(shape, |b| {
            b.iter(|| {
                for &(x, y) in &pairs {
                    black_box(f(black_box(x), black_box(y)));
                }
            })
        });
    }
    g.finish();
}

fn signed(c: &mut Criterion) {
    bench_binary(c, "I256/mul", |a, b| a * b);
    bench_binary(c, "I256/rem", |a, b| a % b);
    bench_binary(c, "I256/div_euclid", |a, b| a.wrapping_div_euclid(b));
    bench_binary(c, "I256/rem_euclid", |a, b| a.wrapping_rem_euclid(b));
}

criterion_group!(benches, signed);
criterion_main!(benches);
