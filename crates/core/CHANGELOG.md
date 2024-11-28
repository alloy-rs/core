# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.13](https://github.com/alloy-rs/core/releases/tag/v0.8.13) - 2024-11-26

### Miscellaneous Tasks

- Release 0.8.13 ([#813](https://github.com/alloy-rs/core/issues/813))

## [0.8.12](https://github.com/alloy-rs/core/releases/tag/v0.8.12) - 2024-11-12

### Miscellaneous Tasks

- Release 0.8.12 ([#806](https://github.com/alloy-rs/core/issues/806))

## [0.8.11](https://github.com/alloy-rs/core/releases/tag/v0.8.11) - 2024-11-05

### Miscellaneous Tasks

- Release 0.8.11 ([#803](https://github.com/alloy-rs/core/issues/803))

## [0.8.10](https://github.com/alloy-rs/core/releases/tag/v0.8.10) - 2024-10-28

### Miscellaneous Tasks

- Release 0.8.10

## [0.8.9](https://github.com/alloy-rs/core/releases/tag/v0.8.9) - 2024-10-21

### Bug Fixes

- Re-enable foldhash by default, but exclude it from zkvm ([#777](https://github.com/alloy-rs/core/issues/777))

### Miscellaneous Tasks

- Release 0.8.9

## [0.8.8](https://github.com/alloy-rs/core/releases/tag/v0.8.8) - 2024-10-14

### Miscellaneous Tasks

- Release 0.8.8

## [0.8.7](https://github.com/alloy-rs/core/releases/tag/v0.8.7) - 2024-10-08

### Miscellaneous Tasks

- Release 0.8.7

## [0.8.6](https://github.com/alloy-rs/core/releases/tag/v0.8.6) - 2024-10-06

### Miscellaneous Tasks

- Release 0.8.6

## [0.8.5](https://github.com/alloy-rs/core/releases/tag/v0.8.5) - 2024-09-25

### Miscellaneous Tasks

- Release 0.8.5

## [0.8.4](https://github.com/alloy-rs/core/releases/tag/v0.8.4) - 2024-09-25

### Features

- [primitives] Implement `map` module ([#743](https://github.com/alloy-rs/core/issues/743))
- Support Keccak with sha3 ([#737](https://github.com/alloy-rs/core/issues/737))

### Miscellaneous Tasks

- Release 0.8.4
- Remove unused unstable-doc feature

### Testing

- Add another dyn-abi test

## [0.8.3](https://github.com/alloy-rs/core/releases/tag/v0.8.3) - 2024-09-10

### Miscellaneous Tasks

- Release 0.8.3

## [0.8.2](https://github.com/alloy-rs/core/releases/tag/v0.8.2) - 2024-09-06

### Miscellaneous Tasks

- Release 0.8.2

## [0.8.1](https://github.com/alloy-rs/core/releases/tag/v0.8.1) - 2024-09-06

### Miscellaneous Tasks

- Release 0.8.1

### Refactor

- Remove `Signature` generic ([#719](https://github.com/alloy-rs/core/issues/719))

## [0.8.0](https://github.com/alloy-rs/core/releases/tag/v0.8.0) - 2024-08-21

### Miscellaneous Tasks

- Release 0.8.0

### Styling

- Remove `ethereum_ssz` dependency ([#701](https://github.com/alloy-rs/core/issues/701))

## [0.7.7](https://github.com/alloy-rs/core/releases/tag/v0.7.7) - 2024-07-08

### Documentation

- [primitives] Fix rustdoc for Signature ([#680](https://github.com/alloy-rs/core/issues/680))
- Add per-crate changelogs ([#669](https://github.com/alloy-rs/core/issues/669))

### Miscellaneous Tasks

- Release 0.7.7
- Use workspace.lints ([#676](https://github.com/alloy-rs/core/issues/676))

## [0.7.4](https://github.com/alloy-rs/core/releases/tag/v0.7.4) - 2024-05-14

### Bug Fixes

- [sol-macro] Json feature ([#629](https://github.com/alloy-rs/core/issues/629))

## [0.7.3](https://github.com/alloy-rs/core/releases/tag/v0.7.3) - 2024-05-14

### Documentation

- Update alloy-core homepage link

### Refactor

- Move `expand` from `sol-macro` to its own crate ([#626](https://github.com/alloy-rs/core/issues/626))

## [0.7.2](https://github.com/alloy-rs/core/releases/tag/v0.7.2) - 2024-05-02

### Documentation

- Unhide and mention `sol!` wrappers ([#615](https://github.com/alloy-rs/core/issues/615))

## [0.6.4](https://github.com/alloy-rs/core/releases/tag/v0.6.4) - 2024-02-29

### Features

- [core] Re-export `uint!` ([#537](https://github.com/alloy-rs/core/issues/537))

### Miscellaneous Tasks

- [core] Add comments to `cfg(doc)` ([#538](https://github.com/alloy-rs/core/issues/538))
- Remove unused imports ([#534](https://github.com/alloy-rs/core/issues/534))

## [0.6.3](https://github.com/alloy-rs/core/releases/tag/v0.6.3) - 2024-02-15

### Documentation

- Update alloy_core::sol reference to real sol ([#529](https://github.com/alloy-rs/core/issues/529))

### Features

- [sol-macro] Provide a way to override import paths for dependencies ([#527](https://github.com/alloy-rs/core/issues/527))
- Add `alloy-core` prelude crate ([#521](https://github.com/alloy-rs/core/issues/521))

[`dyn-abi`]: https://crates.io/crates/alloy-dyn-abi
[dyn-abi]: https://crates.io/crates/alloy-dyn-abi
[`json-abi`]: https://crates.io/crates/alloy-json-abi
[json-abi]: https://crates.io/crates/alloy-json-abi
[`primitives`]: https://crates.io/crates/alloy-primitives
[primitives]: https://crates.io/crates/alloy-primitives
[`sol-macro`]: https://crates.io/crates/alloy-sol-macro
[sol-macro]: https://crates.io/crates/alloy-sol-macro
[`sol-type-parser`]: https://crates.io/crates/alloy-sol-type-parser
[sol-type-parser]: https://crates.io/crates/alloy-sol-type-parser
[`sol-types`]: https://crates.io/crates/alloy-sol-types
[sol-types]: https://crates.io/crates/alloy-sol-types
[`syn-solidity`]: https://crates.io/crates/syn-solidity
[syn-solidity]: https://crates.io/crates/syn-solidity

<!-- generated by git-cliff -->
