# Alloy

Core libraries at the root of the Rust Ethereum ecosystem.

Alloy is a rewrite of [`ethers-rs`] from the ground up, with exciting new
features, high performance, and excellent docs.

[`ethers-rs`] will continue to be maintained until we have achieved
feature-parity in Alloy. No action is currently needed from devs.

[`ethers-rs`]: https://github.com/gakonst/ethers-rs

[![Build Status][actions-badge]][actions-url]
[![Telegram chat][telegram-badge]][telegram-url]

[actions-badge]: https://img.shields.io/github/actions/workflow/status/alloy-rs/core/ci.yml?branch=main&style=for-the-badge
[actions-url]: https://github.com/alloy-rs/core/actions?query=branch%3Amain
[telegram-badge]: https://img.shields.io/endpoint?color=neon&style=for-the-badge&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Fethers_rs
[telegram-url]: https://t.me/ethers_rs

## Overview

This repository contains the following crates:

- [`alloy-core`]: Meta-crate for the entire project
- [`alloy-primitives`] - Primitive integer and byte types
- [`alloy-sol-types`] - Compile-time [ABI] and [EIP-712] implementations
- [`alloy-sol-macro`] - The [`sol!`] procedural macro
- [`alloy-dyn-abi`] - Run-time [ABI] and [EIP-712] implementations
- [`alloy-json-abi`] - Full Ethereum [JSON-ABI] implementation
- [`alloy-sol-type-parser`] - A simple parser for Solidity type strings
- [`syn-solidity`] - [`syn`]-powered Solidity parser

[`alloy-core`]: https://github.com/alloy-rs/core/tree/main/crates/core
[`alloy-primitives`]: https://github.com/alloy-rs/core/tree/main/crates/primitives
[`alloy-sol-types`]: https://github.com/alloy-rs/core/tree/main/crates/sol-types
[`alloy-sol-macro`]: https://github.com/alloy-rs/core/tree/main/crates/sol-macro
[`alloy-dyn-abi`]: https://github.com/alloy-rs/core/tree/main/crates/dyn-abi
[`alloy-json-abi`]: https://github.com/alloy-rs/core/tree/main/crates/json-abi
[`alloy-sol-type-parser`]: https://github.com/alloy-rs/core/tree/main/crates/sol-type-parser
[`syn-solidity`]: https://github.com/alloy-rs/core/tree/main/crates/syn-solidity
[JSON-ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html#json
[ABI]: https://docs.soliditylang.org/en/latest/abi-spec.html
[EIP-712]: https://eips.ethereum.org/EIPS/eip-712
[`sol!`]: https://docs.rs/alloy-sol-macro/latest/alloy_sol_macro/macro.sol.html
[`syn`]: https://github.com/dtolnay/syn

## Supported Rust Versions

<!--
When updating this, also update:
- clippy.toml
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

All crates in this workspace should support `no_std` environments, with the
`alloc` crate. If you find a crate that does not support `no_std`, please
[open an issue].

[open an issue]: https://github.com/alloy-rs/core/issues/new/choose

## Credits

None of these crates would have been possible without the great work done in:

- [`ethers.js`](https://github.com/ethers-io/ethers.js/)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
- [`ruint`](https://github.com/recmo/uint)
- [`ethabi`](https://github.com/rust-ethereum/ethabi)
- [`ethcontract-rs`](https://github.com/gnosis/ethcontract-rs/)
- [`guac_rs`](https://github.com/althea-net/guac_rs/)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
