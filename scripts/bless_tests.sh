#!/usr/bin/env bash

set -eo pipefail

export RUSTFLAGS="-Zthreads=1"
export TRYBUILD=overwrite
cargo +nightly test -p alloy-sol-types --test compiletest
cargo +nightly test -p alloy-sol-types --test compiletest --features json
