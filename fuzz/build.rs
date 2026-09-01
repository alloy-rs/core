use std::{env, fs, path::Path};

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

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory");
    let corpus = Path::new(&manifest_dir).join("corpus/strict_abi_roundtrip");
    fs::create_dir_all(&corpus).expect("create ABI fuzz corpus");

    // ABI encoding of `EncoderShape` with every dynamic value empty. This
    // reaches nested dynamic arrays and a fixed array of dynamic bytes.
    write_seed(&corpus, "encoder_shape_empty", &[32, 128, 160, 192, 224, 0, 0, 0, 64, 96, 0, 0]);

    // `advanceTempo(bytes,QueuedDeposit[],DecryptionData[],EnabledToken[])`
    // with every dynamic argument empty.
    write_seed(&corpus, "advance_tempo_empty", &[128, 160, 192, 224, 0, 0, 0, 0]);

    // `fixedBytes(bytes[2])` with both byte strings empty.
    write_seed(&corpus, "fixed_bytes_empty", &[32, 64, 96, 0, 0]);

    // Canonical zero values for the scalar coverage tuple.
    write_seed(&corpus, "primitive_zero", &[0; 8]);
}
