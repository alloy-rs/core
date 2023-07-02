# Alloy

Core libraries at the root of the Rust Ethereum ecosystem.

Alloy is a rewrite of ethers-rs from the ground up, with exciting new features,
high performance & excellent docs.
Ethers-rs will continue to be maintained until we have feature-parity in Alloy.
No action is needed from devs.

[![Build Status][actions-badge]][actions-url]
[![Telegram chat][telegram-badge]][telegram-url]

[actions-badge]: https://img.shields.io/github/actions/workflow/status/alloy-rs/core/ci.yml?branch=main&style=for-the-badge
[actions-url]: https://github.com/alloy-rs/core/actions?query=branch%3Amain
[telegram-badge]: https://img.shields.io/endpoint?color=neon&style=for-the-badge&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Fethers_rs
[telegram-url]: https://t.me/ethers_rs

## Overview

This repository contains the following crates:

- [`alloy-primitives`] - Primitive integer and byte types
- [`alloy-rlp`] - Implementation of [Ethereum RLP serialization][rlp]
- [`alloy-rlp-derive`] - Derive macros for `alloy-rlp`
- [`alloy-dyn-abi`] - Run-time ABI and [EIP-712] implementations
- [`alloy-sol-types`] - Compile-time ABI and [EIP-712] implementations
- [`alloy-json-abi`] - [JSON-ABI] implementation
- [`alloy-sol-macro`] - The `sol!` procedural macro
- [`syn-solidity`] - [`syn`]-powered Solidity parser, used by `alloy-sol-macro`

[`alloy-primitives`]: ./crates/primitives
[`alloy-rlp`]: ./crates/rlp
[`alloy-rlp-derive`]: ./crates/rlp-derive
[`alloy-dyn-abi`]: ./crates/dyn-abi
[`alloy-sol-types`]: ./crates/sol-types
[`alloy-json-abi`]: ./crates/json-abi
[`alloy-sol-macro`]: ./crates/sol-macro
[`syn-solidity`]: ./crates/syn-solidity

[rlp]: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp
[EIP-712]: https://eips.ethereum.org/EIPS/eip-712
[`syn`]: https://github.com/dtolnay/syn

## Supported Rust Versions

<!--
When updating this, also update:
- .clippy.toml
- Cargo.toml
- .github/workflows/ci.yml
-->

Alloy will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.65.0.

Note that the MSRV is not increased automatically, and only as part of a minor
release.

## Contributing

Thanks for your help improving the project! We are so happy to have you! We have
[a contributing guide](./CONTRIBUTING.md) to help you get involved in the
Alloy project.

Pull requests will not be merged unless CI passes, so please ensure that your
contribution follows the linting rules and passes clippy.

## WASM support

We provide full support for all the `wasm*-*` targets. If a crate does not
build on a WASM target, please [open an issue].

When building for the `wasm32-unknown-unknown` target and the `"getrandom"`
feature is enabled, compilation for the `getrandom` crate will fail. This is
expected: see [their documentation][getrandom] for more details.

To fix this, either disable the `"getrandom"` feature on `alloy-core` or add
`getrandom` to your dependencies with the `"js"` feature enabled:

```toml
getrandom = { version = "0.2", features = ["js"] }
```

There is currently no plan to provide an official JS/TS-accessible library
interface, as we believe [viem] or [ethers.js] serve that need very well.

[open an issue]: https://github.com/alloy-rs/core/issues/new/choose
[getrandom]: https://docs.rs/getrandom/#webassembly-support
[viem]: https://viem.sh
[ethers.js]: https://docs.ethers.io/v6/

## Note on `no_std`

We intend these crates to support `no_std` with `alloc`, and have written them
with that in mind. However, a key dependency, `ruint`, does not yet support
`no_std`. We strive to maintain `no_std` + `alloc` compatibility, and intend to
contribute upstream PRs to achieve it in ruint.

## Credits

None of these crates would have been possible without the great work done in:

- [`ethers.js`](https://github.com/ethers-io/ethers.js/)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
- [`ruint`](https://github.com/recmo/uint)
- [`ethabi`](https://github.com/rust-ethereum/ethabi)
- [`ethcontract-rs`](https://github.com/gnosis/ethcontract-rs/)
- [`guac_rs`](https://github.com/althea-net/guac_rs/)
