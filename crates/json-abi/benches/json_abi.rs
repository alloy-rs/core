use criterion::{criterion_group, criterion_main, Criterion};
use serde::{de::DeserializeOwned, Serialize};
use std::hint::black_box;

trait Group: Serialize + DeserializeOwned {
    const NAME: &'static str;
}

impl Group for alloy_json_abi::AbiJson {
    const NAME: &'static str = "alloy";
}
impl Group for ethabi::Contract {
    const NAME: &'static str = "ethabi";
}

fn bench<T: Group>(c: &mut Criterion) {
    let mut g = c.benchmark_group(T::NAME);

    macro_rules! benches {
        ($($name:literal => $path:literal),* $(,)?) => {$(
            let s = include_str!($path);
            g.bench_function(concat!($name, "/ser"), |b| {
                let abi = serde_json::from_str::<T>(s).unwrap();
                b.iter(|| serde_json::to_string(black_box(&abi)).unwrap());
            });
            g.bench_function(concat!($name, "/de"), |b| {
                b.iter(|| -> T { serde_json::from_str(black_box(s)).unwrap() });
            });
        )*};
    }

    benches! {
        "seaport" => "../tests/abi/Seaport.json",
        "console" => "../tests/abi/console.json",
    };
}

criterion_group!(
    benches,
    bench::<alloy_json_abi::AbiJson>,
    bench::<ethabi::Contract>
);
criterion_main!(benches);
