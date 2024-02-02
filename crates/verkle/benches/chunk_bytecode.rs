use criterion::{criterion_group, criterion_main, Criterion};

fn chunking(_: &mut Criterion) {
    todo!()
}

criterion_group!(benches, chunking);
criterion_main!(benches);
