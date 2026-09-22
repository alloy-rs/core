use std::{env, fs, path::Path};

use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::{SolType, sol_data};

type EncoderShape = (
    sol_data::Bytes,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Array<sol_data::Array<sol_data::Bytes>>,
    sol_data::FixedArray<sol_data::Bytes, 2>,
);

type MixedShape = (
    sol_data::Uint<256>,
    sol_data::Bytes,
    sol_data::Address,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Bool,
    sol_data::FixedArray<sol_data::Bytes, 2>,
);

type NestedArrayShape = (
    sol_data::Array<sol_data::FixedArray<sol_data::Bytes, 2>>,
    sol_data::Array<(sol_data::Bytes, sol_data::String)>,
);

type EmptyFixedThenBytes = (sol_data::FixedArray<sol_data::Bytes, 0>, sol_data::Bytes);

fn word(value: usize) -> [u8; 32] {
    let mut word = [0; 32];
    word[24..].copy_from_slice(&(value as u64).to_be_bytes());
    word
}

fn write_seed(corpus: &Path, name: &str, words: &[usize]) {
    let mut bytes = Vec::with_capacity(words.len() * 32);
    for &value in words {
        bytes.extend(word(value));
    }
    fs::write(corpus.join(name), bytes).expect("write ABI fuzz seed");
}

fn write_encoded_seed<T: SolType>(corpus: &Path, name: &str, value: &T::RustType) {
    fs::write(corpus.join(name), T::abi_encode(value)).expect("write encoded ABI fuzz seed");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory");
    let corpus = Path::new(&manifest_dir).join("corpus/strict_abi_roundtrip");
    fs::create_dir_all(&corpus).expect("create ABI fuzz corpus");

    // ABI encoding of `EncoderShape` with every dynamic value empty. This
    // reaches nested dynamic arrays and a fixed array of dynamic bytes.
    write_seed(&corpus, "encoder_shape_empty", &[32, 128, 160, 192, 224, 0, 0, 0, 64, 96, 0, 0]);
    // A compiler-generated nested layout with non-empty tails. This is
    // materialized in the corpus before libFuzzer starts rather than generated
    // in the fuzz target at runtime.
    let byte = Bytes::from_static(&[0]);
    let nested = (byte.clone(), vec![byte.clone()], vec![vec![byte.clone()]], [byte.clone(), byte]);
    write_encoded_seed::<EncoderShape>(&corpus, "encoder_shape_nested", &nested);

    // Static words interleaved with dynamic data exercise the head cursor
    // accounting used by the protocol structures.
    let mixed = (
        U256::ZERO,
        Bytes::from_static(&[1, 2, 3]),
        Address::ZERO,
        vec![Bytes::from_static(&[4])],
        false,
        [Bytes::from_static(&[5]), Bytes::from_static(&[6, 7])],
    );
    write_encoded_seed::<MixedShape>(&corpus, "mixed_static_dynamic", &mixed);

    // A dynamic array whose elements each have dynamic heads, plus a dynamic
    // array of dynamic tuples. This forces strict cursor hand-offs through
    // several nested decoder children.
    let nested_arrays = (
        vec![
            [Bytes::from_static(&[8]), Bytes::from_static(&[9, 10])],
            [Bytes::new(), Bytes::from_static(&[11])],
        ],
        vec![(Bytes::from_static(&[12]), String::from("tempo")), (Bytes::new(), String::new())],
    );
    write_encoded_seed::<NestedArrayShape>(&corpus, "nested_array_dynamic_heads", &nested_arrays);

    // `bytes[0]` has a dynamic ABI type but an empty tail. Its following
    // dynamic value must begin at the same tail offset, which is an important
    // boundary case for strict monotonic offsets.
    let empty_fixed_then_bytes = ([Bytes::new(); 0], Bytes::from_static(&[13]));
    write_encoded_seed::<EmptyFixedThenBytes>(
        &corpus,
        "empty_fixed_dynamic_tail",
        &empty_fixed_then_bytes,
    );

    // `advanceTempo(bytes,QueuedDeposit[],DecryptionData[],EnabledToken[])`
    // with every dynamic argument empty.
    write_seed(&corpus, "advance_tempo_empty", &[128, 160, 192, 224, 0, 0, 0, 0]);
    write_seed(&corpus, "advance_tempo_header", &[128, 192, 224, 256, 1, 0, 0, 0]);

    // `fixedBytes(bytes[2])` with both byte strings empty.
    write_seed(&corpus, "fixed_bytes_empty", &[32, 64, 96, 0, 0]);
    write_seed(&corpus, "fixed_bytes_nonempty", &[32, 64, 128, 1, 0, 1, 0]);

    // Canonical zero values for the scalar coverage tuple.
    write_seed(&corpus, "primitive_zero", &[0; 8]);

    // Reuse the strict corpus byte-for-byte for the configuration target.
    let config_corpus = Path::new(&manifest_dir).join("corpus/abi_decoder_configs");
    fs::create_dir_all(&config_corpus).expect("create ABI config fuzz corpus");
    for entry in fs::read_dir(&corpus).expect("read ABI fuzz corpus") {
        let entry = entry.expect("read ABI fuzz corpus entry");
        if !entry.file_type().expect("read ABI fuzz corpus entry type").is_file() {
            continue;
        }
        fs::copy(entry.path(), config_corpus.join(entry.file_name()))
            .expect("alias ABI config fuzz seed");
    }
}
