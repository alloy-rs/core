# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.7](https://github.com/alloy-rs/core/releases/tag/v0.7.7) - 2024-07-08

### Documentation

- Add per-crate changelogs ([#669](https://github.com/alloy-rs/core/issues/669))

### Miscellaneous Tasks

- Use workspace.lints ([#676](https://github.com/alloy-rs/core/issues/676))
- Fix unnameable-types ([#675](https://github.com/alloy-rs/core/issues/675))

## [0.7.6](https://github.com/alloy-rs/core/releases/tag/v0.7.6) - 2024-06-10

### Features

- [sol-macro] Add return value names to simple getters ([#648](https://github.com/alloy-rs/core/issues/648))

## [0.7.5](https://github.com/alloy-rs/core/releases/tag/v0.7.5) - 2024-06-04

### Documentation

- Update some READMEs ([#641](https://github.com/alloy-rs/core/issues/641))

## [0.7.0](https://github.com/alloy-rs/core/releases/tag/v0.7.0) - 2024-03-30

### Documentation

- Do not accept grammar prs ([#575](https://github.com/alloy-rs/core/issues/575))

## [0.6.4](https://github.com/alloy-rs/core/releases/tag/v0.6.4) - 2024-02-29

### Miscellaneous Tasks

- Allow unknown lints ([#543](https://github.com/alloy-rs/core/issues/543))
- Remove unused imports ([#534](https://github.com/alloy-rs/core/issues/534))

## [0.6.3](https://github.com/alloy-rs/core/releases/tag/v0.6.3) - 2024-02-15

### Features

- [sol-macro] Expand state variable getters in contracts ([#514](https://github.com/alloy-rs/core/issues/514))

## [0.6.0](https://github.com/alloy-rs/core/releases/tag/v0.6.0) - 2024-01-10

### Miscellaneous Tasks

- Clippy uninlined_format_args, use_self ([#475](https://github.com/alloy-rs/core/issues/475))

## [0.5.0](https://github.com/alloy-rs/core/releases/tag/v0.5.0) - 2023-11-23

### Bug Fixes

- [sol-types] Remove `SolType::ENCODED_SIZE` default ([#418](https://github.com/alloy-rs/core/issues/418))
- [syn-solidity] Raw keyword identifiers ([#415](https://github.com/alloy-rs/core/issues/415))
- Rust keyword conflict ([#405](https://github.com/alloy-rs/core/issues/405))
- [syn-solidity] Allow some duplicate attributes ([#399](https://github.com/alloy-rs/core/issues/399))
- [json-abi] `Param.ty` is not always a valid `TypeSpecifier` ([#386](https://github.com/alloy-rs/core/issues/386))
- [sol-macro] Bug fixes ([#372](https://github.com/alloy-rs/core/issues/372))
- [syn-solidity] Struct fields formatting ([#364](https://github.com/alloy-rs/core/issues/364))

### Features

- [sol-macro] Add `json-abi` item generation ([#422](https://github.com/alloy-rs/core/issues/422))
- [json-abi] Improve `JsonAbi::to_sol` ([#408](https://github.com/alloy-rs/core/issues/408))

### Miscellaneous Tasks

- Restructure tests ([#421](https://github.com/alloy-rs/core/issues/421))
- Remove dead code ([#416](https://github.com/alloy-rs/core/issues/416))

### Styling

- Update rustfmt config ([#406](https://github.com/alloy-rs/core/issues/406))

### Testing

- Check version before running Solc ([#428](https://github.com/alloy-rs/core/issues/428))

## [0.4.1](https://github.com/alloy-rs/core/releases/tag/v0.4.1) - 2023-10-09

### Bug Fixes

- [sol-macro] Correct `TypeArray::is_abi_dynamic` ([#353](https://github.com/alloy-rs/core/issues/353))
- [sol-macro] Pass attributes to all generated items ([#340](https://github.com/alloy-rs/core/issues/340))

### Features

- [sol-macro] Improve error messages ([#345](https://github.com/alloy-rs/core/issues/345))
- [sol-types] Introduce `SolValue`, make `Encodable` an impl detail ([#333](https://github.com/alloy-rs/core/issues/333))
- [syn-solidity] Add even more Display impls ([#339](https://github.com/alloy-rs/core/issues/339))
- [syn-solidity] Add some more Display impls ([#337](https://github.com/alloy-rs/core/issues/337))

### Miscellaneous Tasks

- Fix typos ([#325](https://github.com/alloy-rs/core/issues/325))

### Other

- Run miri in ci ([#327](https://github.com/alloy-rs/core/issues/327))

## [0.4.0](https://github.com/alloy-rs/core/releases/tag/v0.4.0) - 2023-09-29

### Bug Fixes

- [syn-solidity] Test
- [syn-solidity] Parse modifiers without parens ([#284](https://github.com/alloy-rs/core/issues/284))
- [syn-solidity] Imports ([#252](https://github.com/alloy-rs/core/issues/252))

### Documentation

- Document dollar sign in idents ([#288](https://github.com/alloy-rs/core/issues/288))

### Features

- [sol-macro] Add support for overloaded events ([#318](https://github.com/alloy-rs/core/issues/318))
- [syn-solidity] Added visitor hooks for all statements and expressions ([#314](https://github.com/alloy-rs/core/issues/314))
- [syn-solidity] Add more `Spanned` impls ([#301](https://github.com/alloy-rs/core/issues/301))
- Unsupported message for $idents ([#293](https://github.com/alloy-rs/core/issues/293))
- [sol-macro] Expand getter functions' return types ([#262](https://github.com/alloy-rs/core/issues/262))
- Add attributes to enum variants ([#264](https://github.com/alloy-rs/core/issues/264))
- [syn-solidity] Improve variable getters generation ([#260](https://github.com/alloy-rs/core/issues/260))
- [sol-macro] Add opt-in attributes for extra methods and derives ([#250](https://github.com/alloy-rs/core/issues/250))

### Miscellaneous Tasks

- Touch up [#314](https://github.com/alloy-rs/core/issues/314) ([#315](https://github.com/alloy-rs/core/issues/315))
- Sync crate level attributes ([#303](https://github.com/alloy-rs/core/issues/303))

### Styling

- Support yul ast  ([#268](https://github.com/alloy-rs/core/issues/268))
- Some clippy lints ([#251](https://github.com/alloy-rs/core/issues/251))

### Testing

- [syn-solidity] Improve contract tests ([#316](https://github.com/alloy-rs/core/issues/316))

## [0.3.2](https://github.com/alloy-rs/core/releases/tag/v0.3.2) - 2023-08-23

### Bug Fixes

- [sol-macro] Encode UDVTs as their underlying type in EIP-712 ([#220](https://github.com/alloy-rs/core/issues/220))
- [sol-macro] Don't panic when encountering functions without names ([#217](https://github.com/alloy-rs/core/issues/217))

### Features

- [syn-solidity] Add statements and expressions ([#199](https://github.com/alloy-rs/core/issues/199))
- Function type ([#224](https://github.com/alloy-rs/core/issues/224))
- [sol-macro] Expand getter functions for public state variables ([#218](https://github.com/alloy-rs/core/issues/218))

## [0.3.0](https://github.com/alloy-rs/core/releases/tag/v0.3.0) - 2023-07-26

### Features

- [sol-macro] `#[sol]` attributes and JSON ABI support ([#173](https://github.com/alloy-rs/core/issues/173))
- [json-abi] Add more impls ([#164](https://github.com/alloy-rs/core/issues/164))
- `SolEnum` and `SolInterface` ([#153](https://github.com/alloy-rs/core/issues/153))

### Miscellaneous Tasks

- Clippy ([#196](https://github.com/alloy-rs/core/issues/196))
- Warn on all rustdoc lints ([#154](https://github.com/alloy-rs/core/issues/154))
- Clean ups ([#150](https://github.com/alloy-rs/core/issues/150))
- Add smaller image for favicon ([#142](https://github.com/alloy-rs/core/issues/142))

## [0.2.0](https://github.com/alloy-rs/core/releases/tag/v0.2.0) - 2023-06-23

### Bug Fixes

- Extra-traits in syn-solidity ([#65](https://github.com/alloy-rs/core/issues/65))

### Features

- Finish high-level Solidity parser ([#119](https://github.com/alloy-rs/core/issues/119))
- Compute encoded size statically where possible ([#105](https://github.com/alloy-rs/core/issues/105))
- Solidity events support ([#83](https://github.com/alloy-rs/core/issues/83))
- `sol!` contracts ([#77](https://github.com/alloy-rs/core/issues/77))
- Syn-solidity visitors ([#68](https://github.com/alloy-rs/core/issues/68))
- Move Solidity syn AST to `syn-solidity` ([#63](https://github.com/alloy-rs/core/issues/63))

### Miscellaneous Tasks

- Add logo to all crates, add @gakonst to CODEOWNERS ([#138](https://github.com/alloy-rs/core/issues/138))
- Typos ([#132](https://github.com/alloy-rs/core/issues/132))
- Rename to Alloy ([#69](https://github.com/alloy-rs/core/issues/69))
- Enable `feature(doc_cfg, doc_auto_cfg)` ([#67](https://github.com/alloy-rs/core/issues/67))
- Remove syn "full" feature ([#66](https://github.com/alloy-rs/core/issues/66))

### Performance

- Improve rlp, update Address methods ([#118](https://github.com/alloy-rs/core/issues/118))

### Refactor

- Sol-macro expansion ([#113](https://github.com/alloy-rs/core/issues/113))

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
