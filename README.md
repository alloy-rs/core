# Ethers Core

This repository will hold the core crates at the root of the the Ethers-rs
ecosystem. These types and libraries will be used by revm, reth, ethers, and
foundry.

# Some TODOS

- double-check license info
- set up cargo deny
- set up CI
- Set up branch protection
- fix uint CI
  - maybe: integrate uint CI with local CI?
- fix wasm in uint
- unify workspace deps with `.workspace = true`
- meta crate?

# Current build command

cargo clippy --no-default-features
cargo clippy
cargo clippy --target wasm32-unknown-unknown
cargo clippy --features eip712-serde
