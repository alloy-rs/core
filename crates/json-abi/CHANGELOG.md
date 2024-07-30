# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.7](https://github.com/alloy-rs/core/releases/tag/v0.7.7) - 2024-07-08

### Documentation

- Add per-crate changelogs ([#669](https://github.com/alloy-rs/core/issues/669))

### Features

- [json-abi] Allow `serde_json::from_{value,reader}` ([#684](https://github.com/alloy-rs/core/issues/684))
- Add support for parsing visibility and state mutability ([#682](https://github.com/alloy-rs/core/issues/682))
- [primitives] Manually implement arbitrary for signature ([#663](https://github.com/alloy-rs/core/issues/663))

### Miscellaneous Tasks

- Use workspace.lints ([#676](https://github.com/alloy-rs/core/issues/676))
- Fix unnameable-types ([#675](https://github.com/alloy-rs/core/issues/675))

### Styling

- Sort derives ([#662](https://github.com/alloy-rs/core/issues/662))

## [0.7.5](https://github.com/alloy-rs/core/releases/tag/v0.7.5) - 2024-06-04

### Features

- Create new method on Param and EventParam ([#634](https://github.com/alloy-rs/core/issues/634))

### Miscellaneous Tasks

- [sol-macro] Add suggestion to remove name ([#647](https://github.com/alloy-rs/core/issues/647))

## [0.7.2](https://github.com/alloy-rs/core/releases/tag/v0.7.2) - 2024-05-02

### Documentation

- Unhide and mention `sol!` wrappers ([#615](https://github.com/alloy-rs/core/issues/615))

## [0.7.1](https://github.com/alloy-rs/core/releases/tag/v0.7.1) - 2024-04-23

### Features

- [json-abi] Support legacy JSON ABIs ([#596](https://github.com/alloy-rs/core/issues/596))

## [0.7.0](https://github.com/alloy-rs/core/releases/tag/v0.7.0) - 2024-03-30

### Bug Fixes

- [json-abi] Correct to_sol for arrays of contracts ([#586](https://github.com/alloy-rs/core/issues/586))
- Force clippy to stable ([#569](https://github.com/alloy-rs/core/issues/569))

### Features

- [json-abi] Add configuration for `JsonAbi::to_sol` ([#558](https://github.com/alloy-rs/core/issues/558))

## [0.6.4](https://github.com/alloy-rs/core/releases/tag/v0.6.4) - 2024-02-29

### Bug Fixes

- [dyn-abi] Correctly parse empty lists of bytes ([#548](https://github.com/alloy-rs/core/issues/548))

### Miscellaneous Tasks

- Remove unused imports ([#534](https://github.com/alloy-rs/core/issues/534))

## [0.6.3](https://github.com/alloy-rs/core/releases/tag/v0.6.3) - 2024-02-15

### Bug Fixes

- [json-abi] Accept nameless `Param`s ([#526](https://github.com/alloy-rs/core/issues/526))

## [0.6.1](https://github.com/alloy-rs/core/releases/tag/v0.6.1) - 2024-01-25

### Bug Fixes

- Deserialize missing state mutability as non payable ([#488](https://github.com/alloy-rs/core/issues/488))

### Features

- Add constructorCall to `sol!` ([#493](https://github.com/alloy-rs/core/issues/493))

### Miscellaneous Tasks

- [primitives] Pass B256 by reference in Signature methods ([#487](https://github.com/alloy-rs/core/issues/487))
- Improve unlinked bytecode deserde error ([#484](https://github.com/alloy-rs/core/issues/484))

### Testing

- Don't print constructors for Solc tests ([#501](https://github.com/alloy-rs/core/issues/501))

## [0.6.0](https://github.com/alloy-rs/core/releases/tag/v0.6.0) - 2024-01-10

### Features

- [json-abi] Add full_signature ([#480](https://github.com/alloy-rs/core/issues/480))

## [0.5.2](https://github.com/alloy-rs/core/releases/tag/v0.5.2) - 2023-12-01

### Testing

- Add some regression tests ([#443](https://github.com/alloy-rs/core/issues/443))

## [0.5.0](https://github.com/alloy-rs/core/releases/tag/v0.5.0) - 2023-11-23

### Bug Fixes

- Rust keyword conflict ([#405](https://github.com/alloy-rs/core/issues/405))
- [sol-type-parser] Normalize `u?int` to `u?int256` ([#397](https://github.com/alloy-rs/core/issues/397))
- [json-abi] `Param.ty` is not always a valid `TypeSpecifier` ([#386](https://github.com/alloy-rs/core/issues/386))
- [sol-macro] Bug fixes ([#372](https://github.com/alloy-rs/core/issues/372))

### Features

- [json-abi] Permit keyword prefixes in HR parser ([#420](https://github.com/alloy-rs/core/issues/420))
- [json-abi] Improve `JsonAbi::to_sol` ([#408](https://github.com/alloy-rs/core/issues/408))
- [dyn-abi] `DynSolType::coerce_str` ([#380](https://github.com/alloy-rs/core/issues/380))

### Miscellaneous Tasks

- Restructure tests ([#421](https://github.com/alloy-rs/core/issues/421))

### Styling

- Update rustfmt config ([#406](https://github.com/alloy-rs/core/issues/406))

### Testing

- Check version before running Solc ([#428](https://github.com/alloy-rs/core/issues/428))
- Add errors abi test ([#390](https://github.com/alloy-rs/core/issues/390))

## [0.4.1](https://github.com/alloy-rs/core/releases/tag/v0.4.1) - 2023-10-09

### Bug Fixes

- [json-abi] Fallback to tuple types for nested params in `to_sol` ([#354](https://github.com/alloy-rs/core/issues/354))
- [sol-macro] Dedup json abi items ([#346](https://github.com/alloy-rs/core/issues/346))
- Json-abi not using anonymous when converting to interface ([#342](https://github.com/alloy-rs/core/issues/342))

### Features

- [sol-macro] Add docs to generated contract modules ([#356](https://github.com/alloy-rs/core/issues/356))
- [json-abi] Deserialize more ContractObjects ([#348](https://github.com/alloy-rs/core/issues/348))
- Add parsing support for JSON items ([#329](https://github.com/alloy-rs/core/issues/329))
- Add logs, add log dynamic decoding ([#271](https://github.com/alloy-rs/core/issues/271))

### Other

- Run miri in ci ([#327](https://github.com/alloy-rs/core/issues/327))

### Testing

- Add regression test for [#351](https://github.com/alloy-rs/core/issues/351) ([#355](https://github.com/alloy-rs/core/issues/355))

## [0.4.0](https://github.com/alloy-rs/core/releases/tag/v0.4.0) - 2023-09-29

### Bug Fixes

- MSRV tests ([#246](https://github.com/alloy-rs/core/issues/246))

### Dependencies

- Fix MSRV CI and dev deps ([#267](https://github.com/alloy-rs/core/issues/267))

### Features

- [json-abi] Add `Function::signature_full` ([#289](https://github.com/alloy-rs/core/issues/289))
- [primitives] Add more methods to `Function` ([#290](https://github.com/alloy-rs/core/issues/290))

### Miscellaneous Tasks

- Sync crate level attributes ([#303](https://github.com/alloy-rs/core/issues/303))

### Other

- Pin anstyle to 1.65 compat ([#266](https://github.com/alloy-rs/core/issues/266))

### Styling

- Some clippy lints ([#251](https://github.com/alloy-rs/core/issues/251))

## [0.3.2](https://github.com/alloy-rs/core/releases/tag/v0.3.2) - 2023-08-23

### Bug Fixes

- [json-abi] Properly handle Param `type` field ([#233](https://github.com/alloy-rs/core/issues/233))

### Features

- Implement abi2sol ([#228](https://github.com/alloy-rs/core/issues/228))
- Add support for function input/output encoding/decoding ([#227](https://github.com/alloy-rs/core/issues/227))
- [sol-macro] Expand getter functions for public state variables ([#218](https://github.com/alloy-rs/core/issues/218))

### Miscellaneous Tasks

- [json-abi] Avoid unsafe, remove unused generics ([#229](https://github.com/alloy-rs/core/issues/229))

### Performance

- Optimize some stuff ([#231](https://github.com/alloy-rs/core/issues/231))

### Styling

- Port ethabi json tests ([#232](https://github.com/alloy-rs/core/issues/232))

## [0.3.1](https://github.com/alloy-rs/core/releases/tag/v0.3.1) - 2023-07-30

### Documentation

- [json-abi] Add README.md ([#209](https://github.com/alloy-rs/core/issues/209))

### Features

- Support `ethabi` Contract methods ([#195](https://github.com/alloy-rs/core/issues/195))

## [0.3.0](https://github.com/alloy-rs/core/releases/tag/v0.3.0) - 2023-07-26

### Bug Fixes

- [sol-types] Empty data decode ([#159](https://github.com/alloy-rs/core/issues/159))

### Features

- [sol-macro] `#[sol]` attributes and JSON ABI support ([#173](https://github.com/alloy-rs/core/issues/173))
- Solidity type parser ([#181](https://github.com/alloy-rs/core/issues/181))
- [json-abi] Add more impls ([#164](https://github.com/alloy-rs/core/issues/164))

### Miscellaneous Tasks

- Warn on all rustdoc lints ([#154](https://github.com/alloy-rs/core/issues/154))
- Add smaller image for favicon ([#142](https://github.com/alloy-rs/core/issues/142))

## [0.2.0](https://github.com/alloy-rs/core/releases/tag/v0.2.0) - 2023-06-23

### Bug Fixes

- Add `repr(C)` to json-abi items ([#100](https://github.com/alloy-rs/core/issues/100))

### Features

- Unify json-abi params impls ([#136](https://github.com/alloy-rs/core/issues/136))
- Json-abi event selector ([#104](https://github.com/alloy-rs/core/issues/104))
- Abi-json crate ([#78](https://github.com/alloy-rs/core/issues/78))

### Miscellaneous Tasks

- Add logo to all crates, add @gakonst to CODEOWNERS ([#138](https://github.com/alloy-rs/core/issues/138))

### Performance

- Improve rlp, update Address methods ([#118](https://github.com/alloy-rs/core/issues/118))

### Testing

- Add more json abi tests ([#89](https://github.com/alloy-rs/core/issues/89))

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
