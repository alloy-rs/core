<h1 align="center"> ethers-rs core </h1>

This repository holds the core crates at the root of the the ethers-rs
ecosystem. These types and libraries will be used by revm, reth, ethers, and
foundry.

## What's in the box?

This repository contains the following crates:

- **ethers-rlp** - A Rust implementation of the Ethereum RLP encoding.
- **ethers-rlp-derive** - A derive macro for RLP encoding.
- **ethers-primitives** - Signed & unsigned integers, and fixed-sized
  bytearrays.
- **ethers-sol-types** - Rust representation of Solidity types with ABI and
  EIP-712 encoding.
- **ethers-sol-macro** - A macro for generating Rust representations of Solidity
  types by parsing Solidity snippets.
- **ethers-dyn-abi** - ABI encoding and decoding of types that are not known at
  runtime.
- **ethers-serde-helpers** - Serde helpers for the ethers crates (coming soon).

## Developer Information

No special actions needed. Clone the repo and make sure rust is installed

### Build commands

- `$ cargo clippy`
- `$ cargo clippy --no-default-features`
- `$ cargo clippy --all-features`

### Testing commands

- `$ cargo test`
- `$ cargo test --no-default-features`
- `$ cargo test --all-features`

### MSRV

We do not guarantee a specific MSRV. We like to keep up-to-date and take
advantage of new features.

### Features

- `ethers-rlp/std` - `std` support for `ethers-rlp` Enabled by default.
- `ethers-primitives/rlp` - `rlp` support for `ethers-primitives` Enabled by
  default.
- `ethers-primitives/serde` - `serde` support for `ethers-primitives` Enabled by
  default.
- `ethers-sol-types/eip712-serde` - `serde` support for the `Eip712Domain`
  struct.

## Contributing

Thanks for your help improving the project! We are so happy to have you! We have
[a contributing guide](./CONTRIBUTING.md) to help you get involved in the
ethers-rs project.

If you open a Pull Request, do not forget to add your changes in the
[CHANGELOG](./CHANGELOG.md).

Pull requests will not be merged unless CI passes, so please ensure that your
contribution follows the linting rules and passes clippy. :)

## Note on WASM and FFI bindings

You should be able to build a wasm app that uses ethers-rs. If ethers fails to
compile in WASM, please
[open an issue](https://github.com/ethers-rs/core/issues/new). There is
currently no plan to provide an official JS/TS-accessible library interface. We
believe [viem](https://viem.sh) or [ethers.js](https://docs.ethers.io/v6/)
serves that need very well.

Similarly, you should be able to build FFI bindings to ethers-rs. If ethers
fails to compile in c lib formats, please
[open an issue](https://github.com/ethers-rs/core/issues/new).
There is currently no plan to provide official FFI bindings, and as ethers-rs is
not yet stable 1.0.0, its interface may change significantly between versions.

## Credits

These librarires would not have been possible without the great work done in:

- [`ethers.js`](https://github.com/ethers-io/ethers.js/)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
- [`ethcontract-rs`](https://github.com/gnosis/ethcontract-rs/)
- [`guac_rs`](https://github.com/althea-net/guac_rs/)
- [`ruint`](https://github.com/recmo/uint)

A lot of the code was inspired and adapted from them, to a unified and
opinionated interface, built with async/await and std futures from the ground
up.
