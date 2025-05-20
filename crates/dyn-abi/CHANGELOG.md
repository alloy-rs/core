# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.2](https://github.com/alloy-rs/core/releases/tag/v1.1.2) - 2025-05-20

### Dependencies

- Enhance eip712 string parser to canonicalize inputs ([#950](https://github.com/alloy-rs/core/issues/950))

## [1.1.1](https://github.com/alloy-rs/core/releases/tag/v1.1.1) - 2025-05-19

### Miscellaneous Tasks

- Release 1.1.1
- Re-use alloy_primitives::hex ([#952](https://github.com/alloy-rs/core/issues/952))

## [1.1.0](https://github.com/alloy-rs/core/releases/tag/v1.1.0) - 2025-04-30

### Documentation

- Improve eip712 docs and discoverability ([#940](https://github.com/alloy-rs/core/issues/940))

### Miscellaneous Tasks

- Release 1.1.0

## [1.0.0](https://github.com/alloy-rs/core/releases/tag/v1.0.0) - 2025-04-03

### Dependencies

- [deps] Bump getrandom to 0.3, rand to 0.9 ([#869](https://github.com/alloy-rs/core/issues/869))

### Features

- 1.0-rc.1
- [sol-types] Rm `validate: bool`  ([#863](https://github.com/alloy-rs/core/issues/863))

### Miscellaneous Tasks

- Release 1.0.0
- Release 1.0.0-rc.1
- Release 0.8.25

### Other

- Merge branch 'main' into v1.0-rc

### Testing

- Missing import
- [dyn-abi] Remove dev-dependency on self

## [0.8.24](https://github.com/alloy-rs/core/releases/tag/v0.8.24) - 2025-03-21

### Miscellaneous Tasks

- Release 0.8.24

## [0.8.23](https://github.com/alloy-rs/core/releases/tag/v0.8.23) - 2025-03-13

### Miscellaneous Tasks

- Release 0.8.23
- Clippy ([#894](https://github.com/alloy-rs/core/issues/894))

## [0.8.22](https://github.com/alloy-rs/core/releases/tag/v0.8.22) - 2025-02-27

### Features

- [primitives] Improve rand implementations, use `thread_rng` when available ([#870](https://github.com/alloy-rs/core/issues/870))

### Miscellaneous Tasks

- Release 0.8.22

## [0.8.21](https://github.com/alloy-rs/core/releases/tag/v0.8.21) - 2025-02-10

### Miscellaneous Tasks

- Release 0.8.21

## [0.8.20](https://github.com/alloy-rs/core/releases/tag/v0.8.20) - 2025-02-02

### Dependencies

- [deps] Bump winnow 0.7 ([#862](https://github.com/alloy-rs/core/issues/862))

### Miscellaneous Tasks

- Release 0.8.20
- Clippy ([#858](https://github.com/alloy-rs/core/issues/858))

## [0.8.19](https://github.com/alloy-rs/core/releases/tag/v0.8.19) - 2025-01-15

### Documentation

- Enable some useful rustdoc features on docs.rs ([#850](https://github.com/alloy-rs/core/issues/850))

### Miscellaneous Tasks

- Release 0.8.19

## [0.8.18](https://github.com/alloy-rs/core/releases/tag/v0.8.18) - 2025-01-04

### Miscellaneous Tasks

- Release 0.8.18

## [0.8.17](https://github.com/alloy-rs/core/releases/tag/v0.8.17) - 2025-01-04

### Bug Fixes

- Coerce pow overflow ([#838](https://github.com/alloy-rs/core/issues/838))

### Features

- Support 0x in hex! and similar macros ([#841](https://github.com/alloy-rs/core/issues/841))

### Miscellaneous Tasks

- Release 0.8.17

## [0.8.16](https://github.com/alloy-rs/core/releases/tag/v0.8.16) - 2025-01-01

### Features

- [dyn-abi] Support parse scientific number ([#835](https://github.com/alloy-rs/core/issues/835))

### Miscellaneous Tasks

- Release 0.8.16
- Clippy ([#834](https://github.com/alloy-rs/core/issues/834))

## [0.8.15](https://github.com/alloy-rs/core/releases/tag/v0.8.15) - 2024-12-09

### Miscellaneous Tasks

- Release 0.8.15

## [0.8.14](https://github.com/alloy-rs/core/releases/tag/v0.8.14) - 2024-11-28

### Dependencies

- Bump MSRV to 1.81 ([#790](https://github.com/alloy-rs/core/issues/790))

### Features

- Switch all std::error to core::error ([#815](https://github.com/alloy-rs/core/issues/815))

### Miscellaneous Tasks

- Release 0.8.14

## [0.8.13](https://github.com/alloy-rs/core/releases/tag/v0.8.13) - 2024-11-26

### Features

- Expose `returns` field for `DynSolCall` type ([#809](https://github.com/alloy-rs/core/issues/809))

### Miscellaneous Tasks

- Release 0.8.13 ([#813](https://github.com/alloy-rs/core/issues/813))
- Release 0.8.12 ([#806](https://github.com/alloy-rs/core/issues/806))
- Release 0.8.11 ([#803](https://github.com/alloy-rs/core/issues/803))

## [0.8.10](https://github.com/alloy-rs/core/releases/tag/v0.8.10) - 2024-10-28

### Bug Fixes

- Revert MSRV changes ([#789](https://github.com/alloy-rs/core/issues/789))

### Dependencies

- Bump MSRV to 1.81 & use `core::error::Error` in place of `std` ([#780](https://github.com/alloy-rs/core/issues/780))

### Miscellaneous Tasks

- Release 0.8.10

## [0.8.9](https://github.com/alloy-rs/core/releases/tag/v0.8.9) - 2024-10-21

### Miscellaneous Tasks

- Release 0.8.9

## [0.8.8](https://github.com/alloy-rs/core/releases/tag/v0.8.8) - 2024-10-14

### Miscellaneous Tasks

- Release 0.8.8

## [0.8.7](https://github.com/alloy-rs/core/releases/tag/v0.8.7) - 2024-10-08

### Miscellaneous Tasks

- Release 0.8.7

### Other

- Revert "Add custom serialization for Address" ([#765](https://github.com/alloy-rs/core/issues/765))

## [0.8.6](https://github.com/alloy-rs/core/releases/tag/v0.8.6) - 2024-10-06

### Bug Fixes

- Fix lint `alloy-dyn-abi` ([#758](https://github.com/alloy-rs/core/issues/758))

### Miscellaneous Tasks

- Release 0.8.6
- Remove a stabilized impl_core function

## [0.8.5](https://github.com/alloy-rs/core/releases/tag/v0.8.5) - 2024-09-25

### Miscellaneous Tasks

- Release 0.8.5

## [0.8.4](https://github.com/alloy-rs/core/releases/tag/v0.8.4) - 2024-09-25

### Features

- [primitives] Implement `map` module ([#743](https://github.com/alloy-rs/core/issues/743))

### Miscellaneous Tasks

- Release 0.8.4

### Other

- Add custom serialization for Address ([#742](https://github.com/alloy-rs/core/issues/742))

### Testing

- Add another dyn-abi test

## [0.8.3](https://github.com/alloy-rs/core/releases/tag/v0.8.3) - 2024-09-10

### Miscellaneous Tasks

- Release 0.8.3

## [0.8.2](https://github.com/alloy-rs/core/releases/tag/v0.8.2) - 2024-09-06

### Bug Fixes

- `no_std` and workflow ([#727](https://github.com/alloy-rs/core/issues/727))

### Miscellaneous Tasks

- Release 0.8.2

## [0.8.1](https://github.com/alloy-rs/core/releases/tag/v0.8.1) - 2024-09-06

### Miscellaneous Tasks

- Release 0.8.1

## [0.8.0](https://github.com/alloy-rs/core/releases/tag/v0.8.0) - 2024-08-21

### Bug Fixes

- Parsing stack overflow ([#703](https://github.com/alloy-rs/core/issues/703))

### Features

- [sol-macro] Support namespaces ([#694](https://github.com/alloy-rs/core/issues/694))

### Miscellaneous Tasks

- Release 0.8.0

### Other

- Implement specific bit types for integers ([#677](https://github.com/alloy-rs/core/issues/677))

## [0.7.7](https://github.com/alloy-rs/core/releases/tag/v0.7.7) - 2024-07-08

### Bug Fixes

- Small fixes for `DynSolValue` strategies ([#683](https://github.com/alloy-rs/core/issues/683))
- Fixed bytes dyn abi packed encoding ([#671](https://github.com/alloy-rs/core/issues/671))
- [sol-macro] Json feature ([#629](https://github.com/alloy-rs/core/issues/629))
- [json-abi] Correct to_sol for arrays of contracts ([#586](https://github.com/alloy-rs/core/issues/586))
- [dyn-abi] Correctly parse uints in `coerce_str` ([#577](https://github.com/alloy-rs/core/issues/577))
- Force clippy to stable ([#569](https://github.com/alloy-rs/core/issues/569))
- [dyn-abi] Correctly parse empty lists of bytes ([#548](https://github.com/alloy-rs/core/issues/548))
- [dyn-abi] Enable `DynSolType.coerce_json` to convert array of numbers to bytes ([#541](https://github.com/alloy-rs/core/issues/541))
- [json-abi] Accept nameless `Param`s ([#526](https://github.com/alloy-rs/core/issues/526))
- [dyn-abi] Abi-encode-packed always pads arrays ([#519](https://github.com/alloy-rs/core/issues/519))
- Properly test ABI packed encoding ([#517](https://github.com/alloy-rs/core/issues/517))
- Ingest domain when instantiating TypedData ([#453](https://github.com/alloy-rs/core/issues/453))
- Don't decode ZSTs ([#454](https://github.com/alloy-rs/core/issues/454))
- [dyn-abi] Fixed arrays coerce_str ([#442](https://github.com/alloy-rs/core/issues/442))
- [dyn-abi] Correctly parse strings in `coerce_str` ([#410](https://github.com/alloy-rs/core/issues/410))
- [dyn-abi] Handle empty hex strings ([#400](https://github.com/alloy-rs/core/issues/400))
- [json-abi] `Param.ty` is not always a valid `TypeSpecifier` ([#386](https://github.com/alloy-rs/core/issues/386))
- [dyn-abi] Generate Int, Uint, FixedBytes adjusted to their size ([#384](https://github.com/alloy-rs/core/issues/384))
- Serde rename resolver to types ([#335](https://github.com/alloy-rs/core/issues/335))
- MSRV tests ([#246](https://github.com/alloy-rs/core/issues/246))
- [sol-macro] Encode UDVTs as their underlying type in EIP-712 ([#220](https://github.com/alloy-rs/core/issues/220))
- Remove unwrap in decode_populate ([#172](https://github.com/alloy-rs/core/issues/172))
- Doc in dyn-abi ([#155](https://github.com/alloy-rs/core/issues/155))
- Make detokenize infallible ([#86](https://github.com/alloy-rs/core/issues/86))

### Dependencies

- [deps] Update winnow to 0.6 ([#533](https://github.com/alloy-rs/core/issues/533))
- Recursion mitigations ([#495](https://github.com/alloy-rs/core/issues/495))
- Fix MSRV CI and dev deps ([#267](https://github.com/alloy-rs/core/issues/267))

### Documentation

- Add per-crate changelogs ([#669](https://github.com/alloy-rs/core/issues/669))
- Unhide and mention `sol!` wrappers ([#615](https://github.com/alloy-rs/core/issues/615))
- Remove stray list element ([#500](https://github.com/alloy-rs/core/issues/500))
- Fixes ([#498](https://github.com/alloy-rs/core/issues/498))
- Improve `ResolveSolType` documentation ([#296](https://github.com/alloy-rs/core/issues/296))
- [rlp] Move example to README.md ([#177](https://github.com/alloy-rs/core/issues/177))

### Features

- DynSolCall ([#632](https://github.com/alloy-rs/core/issues/632))
- IntoLogData ([#666](https://github.com/alloy-rs/core/issues/666))
- Add `abi_packed_encoded_size` ([#672](https://github.com/alloy-rs/core/issues/672))
- [dyn-abi] Derive `Eq` for `TypedData` ([#623](https://github.com/alloy-rs/core/issues/623))
- Add `TxKind` ([#542](https://github.com/alloy-rs/core/issues/542))
- Make some allocations fallible in ABI decoding ([#513](https://github.com/alloy-rs/core/issues/513))
- [dyn-abi] Improve hex error messages ([#474](https://github.com/alloy-rs/core/issues/474))
- [primitives] Update Bytes formatting, add UpperHex ([#446](https://github.com/alloy-rs/core/issues/446))
- [json-abi] Permit keyword prefixes in HR parser ([#420](https://github.com/alloy-rs/core/issues/420))
- Added Hash to DynSolType and StructProp ([#411](https://github.com/alloy-rs/core/issues/411))
- [dyn-abi] `DynSolType::coerce_str` ([#380](https://github.com/alloy-rs/core/issues/380))
- [sol-types] Introduce `SolValue`, make `Encodable` an impl detail ([#333](https://github.com/alloy-rs/core/issues/333))
- Add parsing support for JSON items ([#329](https://github.com/alloy-rs/core/issues/329))
- Add logs, add log dynamic decoding ([#271](https://github.com/alloy-rs/core/issues/271))
- [primitives] Add more methods to `Function` ([#290](https://github.com/alloy-rs/core/issues/290))
- Use `FixedBytes` for `sol_data::FixedBytes` ([#276](https://github.com/alloy-rs/core/issues/276))
- [dyn-abi] Implement more ext traits for json-abi ([#243](https://github.com/alloy-rs/core/issues/243))
- Add support for function input/output encoding/decoding ([#227](https://github.com/alloy-rs/core/issues/227))
- [dyn-abi] Add match functions to value and doc aliases ([#234](https://github.com/alloy-rs/core/issues/234))
- Function type ([#224](https://github.com/alloy-rs/core/issues/224))
- [dyn-abi] Allow `T: Into<Cow<str>>` in `eip712_domain!` ([#222](https://github.com/alloy-rs/core/issues/222))
- [dyb-abi] Impl ResolveSolType for Rc ([#189](https://github.com/alloy-rs/core/issues/189))
- [sol-macro] `#[sol]` attributes and JSON ABI support ([#173](https://github.com/alloy-rs/core/issues/173))
- Solidity type parser ([#181](https://github.com/alloy-rs/core/issues/181))
- [dyn-abi] Add arbitrary impls and proptests ([#175](https://github.com/alloy-rs/core/issues/175))
- [dyn-abi] Cfg CustomStruct for eip712, rm CustomValue ([#178](https://github.com/alloy-rs/core/issues/178))
- [dyn-abi] Clean up and improve performance ([#174](https://github.com/alloy-rs/core/issues/174))
- DynSolType::decode_params ([#166](https://github.com/alloy-rs/core/issues/166))
- `SolEnum` and `SolInterface` ([#153](https://github.com/alloy-rs/core/issues/153))
- Unify json-abi params impls ([#136](https://github.com/alloy-rs/core/issues/136))
- Add `Encodable` trait ([#121](https://github.com/alloy-rs/core/issues/121))
- More FixedBytes impls ([#111](https://github.com/alloy-rs/core/issues/111))
- Abi benchmarks ([#57](https://github.com/alloy-rs/core/issues/57))
- Primitive utils and improvements ([#52](https://github.com/alloy-rs/core/issues/52))

### Miscellaneous Tasks

- Release 0.7.7
- Use workspace.lints ([#676](https://github.com/alloy-rs/core/issues/676))
- Fix unnameable-types ([#675](https://github.com/alloy-rs/core/issues/675))
- Fix dyn abi
- Fix tests ([#624](https://github.com/alloy-rs/core/issues/624))
- Remove dead code ([#571](https://github.com/alloy-rs/core/issues/571))
- Allow unknown lints ([#543](https://github.com/alloy-rs/core/issues/543))
- Remove unused imports ([#534](https://github.com/alloy-rs/core/issues/534))
- Fix winnow deprecation warnings ([#507](https://github.com/alloy-rs/core/issues/507))
- Clippy uninlined_format_args, use_self ([#475](https://github.com/alloy-rs/core/issues/475))
- Rename `TokenType` GAT and trait to `Token` ([#417](https://github.com/alloy-rs/core/issues/417))
- Remove dead code ([#416](https://github.com/alloy-rs/core/issues/416))
- Clean up ABI, EIP-712, docs ([#373](https://github.com/alloy-rs/core/issues/373))
- Enable ruint std feature ([#326](https://github.com/alloy-rs/core/issues/326))
- [dyn-abi] Make `resolve` module private ([#324](https://github.com/alloy-rs/core/issues/324))
- Prefix ABI encode and decode functions with `abi_` ([#311](https://github.com/alloy-rs/core/issues/311))
- Sync crate level attributes ([#303](https://github.com/alloy-rs/core/issues/303))
- Assert_eq! on Ok instead of unwrapping where possible ([#297](https://github.com/alloy-rs/core/issues/297))
- Use `hex!` macro from `primitives` re-export ([#299](https://github.com/alloy-rs/core/issues/299))
- Rename coding functions ([#274](https://github.com/alloy-rs/core/issues/274))
- Replace `ruint2` with `ruint` ([#192](https://github.com/alloy-rs/core/issues/192))
- [dyn-abi] Gate eip712 behind a feature ([#176](https://github.com/alloy-rs/core/issues/176))
- Warn on all rustdoc lints ([#154](https://github.com/alloy-rs/core/issues/154))
- Add smaller image for favicon ([#142](https://github.com/alloy-rs/core/issues/142))
- Add logo to all crates, add @gakonst to CODEOWNERS ([#138](https://github.com/alloy-rs/core/issues/138))
- Typos ([#132](https://github.com/alloy-rs/core/issues/132))
- Typo fix ([#131](https://github.com/alloy-rs/core/issues/131))
- Rename to Alloy ([#69](https://github.com/alloy-rs/core/issues/69))
- Enable `feature(doc_cfg, doc_auto_cfg)` ([#67](https://github.com/alloy-rs/core/issues/67))
- Rename crates ([#45](https://github.com/alloy-rs/core/issues/45))
- Pre-release mega cleanup ([#35](https://github.com/alloy-rs/core/issues/35))
- Use crates.io uint, move crates to `crates/*` ([#31](https://github.com/alloy-rs/core/issues/31))

### Other

- Prestwich/dyn sol error ([#551](https://github.com/alloy-rs/core/issues/551))
- Run miri in ci ([#327](https://github.com/alloy-rs/core/issues/327))
- Pin anstyle to 1.65 compat ([#266](https://github.com/alloy-rs/core/issues/266))
- Significant dyn-abi fixes :) ([#168](https://github.com/alloy-rs/core/issues/168))
- Prestwich/crate readmes ([#41](https://github.com/alloy-rs/core/issues/41))

### Performance

- [sol-macro] Decode bytecode hex strings ourselves ([#562](https://github.com/alloy-rs/core/issues/562))
- Optimize identifier parsing ([#295](https://github.com/alloy-rs/core/issues/295))
- Optimize some stuff ([#231](https://github.com/alloy-rs/core/issues/231))
- Improve rlp, update Address methods ([#118](https://github.com/alloy-rs/core/issues/118))

### Refactor

- Change identical resolve traits to Specifier<T> ([#550](https://github.com/alloy-rs/core/issues/550))
- Log implementation ([#465](https://github.com/alloy-rs/core/issues/465))
- Rewrite type parser with `winnow` ([#292](https://github.com/alloy-rs/core/issues/292))
- Refactoring `dyn-abi` to performance parity with ethabi ([#144](https://github.com/alloy-rs/core/issues/144))
- Lifetimes for token types ([#120](https://github.com/alloy-rs/core/issues/120))
- Implement `SolType` for `{Ui,I}nt<N>` and `FixedBytes<N>` with const-generics ([#92](https://github.com/alloy-rs/core/issues/92))

### Styling

- Format some imports
- Sort derives ([#662](https://github.com/alloy-rs/core/issues/662))
- Update rustfmt config ([#406](https://github.com/alloy-rs/core/issues/406))
- Format code snippets in docs ([#313](https://github.com/alloy-rs/core/issues/313))
- Some clippy lints ([#251](https://github.com/alloy-rs/core/issues/251))
- Add rustfmt.toml ([#42](https://github.com/alloy-rs/core/issues/42))

### Testing

- Add another ABI encode test ([#547](https://github.com/alloy-rs/core/issues/547))
- Add some more coerce error message tests ([#535](https://github.com/alloy-rs/core/issues/535))

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
