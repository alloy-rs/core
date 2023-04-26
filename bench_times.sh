#!/usr/bin/env bash

set -e

feats=(
    "--no-default-features"
    ""
    "--all-features"
)

for feat in "${feats[@]}"; do
    cargo clean
    cargo test --workspace --timings "$feat" >&1
    cp -rf target/cargo-timings ./
done
