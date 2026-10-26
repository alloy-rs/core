#!/usr/bin/env bash
set -eo pipefail

target=riscv32imac-unknown-none-elf
crates=(
    alloy-core
    alloy-core-sol-test
    alloy-dyn-abi
    alloy-json-abi
    alloy-primitives
    # alloy-sol-macro
    # alloy-sol-macro-expander
    # alloy-sol-macro-input
    alloy-sol-type-parser
    alloy-sol-types
    # syn-solidity
)

cmd=(cargo +stable hack check --no-default-features --target "$target")
for crate in "${crates[@]}"; do
    cmd+=(-p "$crate")
done

echo "Running: ${cmd[*]}"
"${cmd[@]}"
