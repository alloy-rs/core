# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.7](https://github.com/alloy-rs/core/releases/tag/v0.7.7) - 2024-07-08

### Bug Fixes

- [primitives] Include in aliases export to prevent having to import from `aliases::{..}` ([#655](https://github.com/alloy-rs/core/issues/655))

### Documentation

- [primitives] Fix rustdoc for Signature ([#680](https://github.com/alloy-rs/core/issues/680))
- Add per-crate changelogs ([#669](https://github.com/alloy-rs/core/issues/669))

### Features

- IntoLogData ([#666](https://github.com/alloy-rs/core/issues/666))
- Add `abi_packed_encoded_size` ([#672](https://github.com/alloy-rs/core/issues/672))
- [primitives] Manually implement arbitrary for signature ([#663](https://github.com/alloy-rs/core/issues/663))

### Miscellaneous Tasks

- Use workspace.lints ([#676](https://github.com/alloy-rs/core/issues/676))

### Styling

- Format some imports
- Sort derives ([#662](https://github.com/alloy-rs/core/issues/662))

## [0.7.6](https://github.com/alloy-rs/core/releases/tag/v0.7.6) - 2024-06-10

### Features

- [primitives] Add additional common aliases ([#654](https://github.com/alloy-rs/core/issues/654))
- [primitives] Derive `Arbitrary` for Signature ([#652](https://github.com/alloy-rs/core/issues/652))
- [primitives] Implement `ops::Not` for fixed bytes ([#650](https://github.com/alloy-rs/core/issues/650))

### Miscellaneous Tasks

- [docs] Add doc aliases for `Tx` prefixed names ([#649](https://github.com/alloy-rs/core/issues/649))

## [0.7.5](https://github.com/alloy-rs/core/releases/tag/v0.7.5) - 2024-06-04

### Bug Fixes

- [primitives] Signed formatting ([#643](https://github.com/alloy-rs/core/issues/643))
- Fix Log serde for non self describing protocols ([#639](https://github.com/alloy-rs/core/issues/639))
- Handle 0 for inverting eip155 parity. ([#633](https://github.com/alloy-rs/core/issues/633))

### Features

- [primitives] Implement TryInto for ParseUnits ([#646](https://github.com/alloy-rs/core/issues/646))

## [0.7.1](https://github.com/alloy-rs/core/releases/tag/v0.7.1) - 2024-04-23

### Features

- Add arbitrary for TxKind ([#604](https://github.com/alloy-rs/core/issues/604))

### Miscellaneous Tasks

- FixedBytes instead of array

## [0.7.0](https://github.com/alloy-rs/core/releases/tag/v0.7.0) - 2024-03-30

### Bug Fixes

- Force clippy to stable ([#569](https://github.com/alloy-rs/core/issues/569))
- [primitives] Re-implement RLP for `Log<LogData>` ([#573](https://github.com/alloy-rs/core/issues/573))

### Documentation

- Do not accept grammar prs ([#575](https://github.com/alloy-rs/core/issues/575))

### Features

- Rlp encoding for logs with generic event data ([#553](https://github.com/alloy-rs/core/issues/553))
- Add LogData::split ([#559](https://github.com/alloy-rs/core/issues/559))

### Miscellaneous Tasks

- No-default-features k256 ([#576](https://github.com/alloy-rs/core/issues/576))

### Other

- Small helpers for alloy serde PR ([#582](https://github.com/alloy-rs/core/issues/582))

### Styling

- Make `Bytes` map to `Bytes` in `SolType` ([#545](https://github.com/alloy-rs/core/issues/545))

## [0.6.4](https://github.com/alloy-rs/core/releases/tag/v0.6.4) - 2024-02-29

### Bug Fixes

- [dyn-abi] Correctly parse empty lists of bytes ([#548](https://github.com/alloy-rs/core/issues/548))

### Documentation

- [primitives] Add a bytes! macro example ([#539](https://github.com/alloy-rs/core/issues/539))

### Features

- Add `TxKind` ([#542](https://github.com/alloy-rs/core/issues/542))
- [core] Re-export `uint!` ([#537](https://github.com/alloy-rs/core/issues/537))
- Derive Allocative on FixedBytes ([#531](https://github.com/alloy-rs/core/issues/531))

### Miscellaneous Tasks

- [primitives] Improve `from_slice` functions ([#546](https://github.com/alloy-rs/core/issues/546))
- Remove unused imports ([#534](https://github.com/alloy-rs/core/issues/534))

## [0.6.3](https://github.com/alloy-rs/core/releases/tag/v0.6.3) - 2024-02-15

### Bug Fixes

- [json-abi] Accept nameless `Param`s ([#526](https://github.com/alloy-rs/core/issues/526))
- Signature bincode serialization ([#509](https://github.com/alloy-rs/core/issues/509))

### Features

- [primitives] Add some more implementations to Bytes ([#528](https://github.com/alloy-rs/core/issues/528))
- Add `alloy-core` prelude crate ([#521](https://github.com/alloy-rs/core/issues/521))
- Make some allocations fallible in ABI decoding ([#513](https://github.com/alloy-rs/core/issues/513))

### Testing

- Remove unused test ([#504](https://github.com/alloy-rs/core/issues/504))

## [0.6.2](https://github.com/alloy-rs/core/releases/tag/v0.6.2) - 2024-01-25

### Bug Fixes

- [`signature`] Construct Signature bytes using v+27 when we do not have an EIP155 `v` ([#503](https://github.com/alloy-rs/core/issues/503))

## [0.6.1](https://github.com/alloy-rs/core/releases/tag/v0.6.1) - 2024-01-25

### Features

- [`primitives`] Add `y_parity_byte_non_eip155` to `Parity` ([#499](https://github.com/alloy-rs/core/issues/499))
- [primitives] Add `Address::from_private_key` ([#483](https://github.com/alloy-rs/core/issues/483))

### Miscellaneous Tasks

- [primitives] Pass B256 by reference in Signature methods ([#487](https://github.com/alloy-rs/core/issues/487))

### Testing

- Parity roundtripping ([#497](https://github.com/alloy-rs/core/issues/497))

## [0.6.0](https://github.com/alloy-rs/core/releases/tag/v0.6.0) - 2024-01-10

### Bug Fixes

- [primitives] Also apply EIP-155 to Parity::Parity ([#476](https://github.com/alloy-rs/core/issues/476))
- Clean the sealed ([#468](https://github.com/alloy-rs/core/issues/468))

### Dependencies

- [deps] Relax k256 requirement ([#481](https://github.com/alloy-rs/core/issues/481))

### Documentation

- Update docs on parity ([#477](https://github.com/alloy-rs/core/issues/477))

### Features

- [primitives] Add Signature type and utils ([#459](https://github.com/alloy-rs/core/issues/459))
- [primitives] Add a buffer type for address checksums ([#472](https://github.com/alloy-rs/core/issues/472))
- [primitives] Add Keccak256 hasher struct ([#469](https://github.com/alloy-rs/core/issues/469))

### Miscellaneous Tasks

- Clippy uninlined_format_args, use_self ([#475](https://github.com/alloy-rs/core/issues/475))

### Refactor

- Log implementation ([#465](https://github.com/alloy-rs/core/issues/465))

## [0.5.4](https://github.com/alloy-rs/core/releases/tag/v0.5.4) - 2023-12-27

### Features

- Sealed ([#467](https://github.com/alloy-rs/core/issues/467))
- [primitives] Re-export ::bytes ([#462](https://github.com/alloy-rs/core/issues/462))
- [primitives] Support parsing numbers in Unit::from_str ([#461](https://github.com/alloy-rs/core/issues/461))
- Enable postgres ruint feature ([#460](https://github.com/alloy-rs/core/issues/460))

### Miscellaneous Tasks

- Clean up address checksum implementation ([#464](https://github.com/alloy-rs/core/issues/464))

### Performance

- Add optional support for keccak-asm ([#466](https://github.com/alloy-rs/core/issues/466))

### Styling

- Add ToSql and FromSql to Signed and FixedBytes ([#447](https://github.com/alloy-rs/core/issues/447))

## [0.5.3](https://github.com/alloy-rs/core/releases/tag/v0.5.3) - 2023-12-16

### Bug Fixes

- [primitives] Return correct fixed length in ssz::Encode ([#451](https://github.com/alloy-rs/core/issues/451))

### Features

- Address from pubkey ([#455](https://github.com/alloy-rs/core/issues/455))
- [primitives] Update Bytes formatting, add UpperHex ([#446](https://github.com/alloy-rs/core/issues/446))

## [0.5.0](https://github.com/alloy-rs/core/releases/tag/v0.5.0) - 2023-11-23

### Bug Fixes

- Avoid symlinks ([#396](https://github.com/alloy-rs/core/issues/396))
- [primitives] Signed cleanup ([#395](https://github.com/alloy-rs/core/issues/395))

### Features

- [primitives] Left and right padding conversions ([#424](https://github.com/alloy-rs/core/issues/424))
- [primitives] Improve utils ([#432](https://github.com/alloy-rs/core/issues/432))
- [sol-macro] `SolEventInterface`: `SolInterface` for contract events enum ([#426](https://github.com/alloy-rs/core/issues/426))
- Enable ruint ssz when primitives ssz ([#419](https://github.com/alloy-rs/core/issues/419))
- [dyn-abi] `DynSolType::coerce_str` ([#380](https://github.com/alloy-rs/core/issues/380))

### Miscellaneous Tasks

- Clean up ABI, EIP-712, docs ([#373](https://github.com/alloy-rs/core/issues/373))

### Other

- SSZ implementation for alloy primitives ([#407](https://github.com/alloy-rs/core/issues/407))
- Enable rand feature for re-exported ruint crate ([#385](https://github.com/alloy-rs/core/issues/385))

### Styling

- Update rustfmt config ([#406](https://github.com/alloy-rs/core/issues/406))

## [0.4.2](https://github.com/alloy-rs/core/releases/tag/v0.4.2) - 2023-10-09

### Bug Fixes

- [primitives] Set serde derive feature ([#359](https://github.com/alloy-rs/core/issues/359))

## [0.4.1](https://github.com/alloy-rs/core/releases/tag/v0.4.1) - 2023-10-09

### Features

- Add parsing support for JSON items ([#329](https://github.com/alloy-rs/core/issues/329))
- Add logs, add log dynamic decoding ([#271](https://github.com/alloy-rs/core/issues/271))

### Miscellaneous Tasks

- Enable ruint std feature ([#326](https://github.com/alloy-rs/core/issues/326))

### Other

- Run miri in ci ([#327](https://github.com/alloy-rs/core/issues/327))

## [0.4.0](https://github.com/alloy-rs/core/releases/tag/v0.4.0) - 2023-09-29

### Bug Fixes

- Add super import on generated modules ([#307](https://github.com/alloy-rs/core/issues/307))
- Hex compatibility ([#244](https://github.com/alloy-rs/core/issues/244))

### Documentation

- Improve `ResolveSolType` documentation ([#296](https://github.com/alloy-rs/core/issues/296))
- Add note regarding ruint::uint macro ([#265](https://github.com/alloy-rs/core/issues/265))
- Update fixed bytes docs ([#255](https://github.com/alloy-rs/core/issues/255))

### Features

- [json-abi] Add `Function::signature_full` ([#289](https://github.com/alloy-rs/core/issues/289))
- [primitives] Add more methods to `Function` ([#290](https://github.com/alloy-rs/core/issues/290))
- Add more `FixedBytes` to int conversion impls ([#281](https://github.com/alloy-rs/core/issues/281))
- Add support for `rand` ([#282](https://github.com/alloy-rs/core/issues/282))
- Impl `bytes::Buf` for our own `Bytes` ([#279](https://github.com/alloy-rs/core/issues/279))
- Add more `Bytes` conversion impls ([#280](https://github.com/alloy-rs/core/issues/280))
- [primitives] Improve Bytes ([#269](https://github.com/alloy-rs/core/issues/269))
- [primitives] Allow empty input in hex macros ([#245](https://github.com/alloy-rs/core/issues/245))

### Miscellaneous Tasks

- Sync crate level attributes ([#303](https://github.com/alloy-rs/core/issues/303))
- Use `hex!` macro from `primitives` re-export ([#299](https://github.com/alloy-rs/core/issues/299))
- Re-export ::bytes ([#278](https://github.com/alloy-rs/core/issues/278))

### Other

- Hash_message ([#304](https://github.com/alloy-rs/core/issues/304))
- Typo ([#249](https://github.com/alloy-rs/core/issues/249))

### Performance

- Use `slice::Iter` where possible ([#256](https://github.com/alloy-rs/core/issues/256))

### Styling

- Some clippy lints ([#251](https://github.com/alloy-rs/core/issues/251))

## [0.3.2](https://github.com/alloy-rs/core/releases/tag/v0.3.2) - 2023-08-23

### Bug Fixes

- Fix bincode serialization ([#223](https://github.com/alloy-rs/core/issues/223))

### Features

- [primitives] More `FixedBytes<N>` <-> `[u8; N]` conversions ([#239](https://github.com/alloy-rs/core/issues/239))
- Function type ([#224](https://github.com/alloy-rs/core/issues/224))

### Miscellaneous Tasks

- [primitives] Discourage use of `B160` ([#235](https://github.com/alloy-rs/core/issues/235))
- Clippy ([#225](https://github.com/alloy-rs/core/issues/225))

### Performance

- Optimize some stuff ([#231](https://github.com/alloy-rs/core/issues/231))

## [0.3.0](https://github.com/alloy-rs/core/releases/tag/v0.3.0) - 2023-07-26

### Bug Fixes

- [alloy-primitives] Fix broken documentation link ([#152](https://github.com/alloy-rs/core/issues/152))

### Features

- Bytes handles numeric arrays and bytearrays in deser ([#202](https://github.com/alloy-rs/core/issues/202))
- Native keccak feature flag ([#185](https://github.com/alloy-rs/core/issues/185))
- [rlp] Improve implementations ([#182](https://github.com/alloy-rs/core/issues/182))
- [dyn-abi] Add arbitrary impls and proptests ([#175](https://github.com/alloy-rs/core/issues/175))
- [dyn-abi] Clean up and improve performance ([#174](https://github.com/alloy-rs/core/issues/174))
- [json-abi] Add more impls ([#164](https://github.com/alloy-rs/core/issues/164))
- [primitives] Add some impls ([#162](https://github.com/alloy-rs/core/issues/162))
- `SolEnum` and `SolInterface` ([#153](https://github.com/alloy-rs/core/issues/153))
- [primitives] Fixed bytes macros ([#156](https://github.com/alloy-rs/core/issues/156))

### Miscellaneous Tasks

- Wrap Bytes methods which return Self ([#206](https://github.com/alloy-rs/core/issues/206))
- Warn on all rustdoc lints ([#154](https://github.com/alloy-rs/core/issues/154))
- Add smaller image for favicon ([#142](https://github.com/alloy-rs/core/issues/142))

### Other

- Kuly14/cleanup ([#151](https://github.com/alloy-rs/core/issues/151))

## [0.2.0](https://github.com/alloy-rs/core/releases/tag/v0.2.0) - 2023-06-23

### Bug Fixes

- (u)int tokenization ([#123](https://github.com/alloy-rs/core/issues/123))
- Rlp impls ([#56](https://github.com/alloy-rs/core/issues/56))
- Hex breaking change ([#50](https://github.com/alloy-rs/core/issues/50))

### Dependencies

- Bump ruint to have alloy-rlp

### Features

- More FixedBytes impls ([#111](https://github.com/alloy-rs/core/issues/111))
- Primitive utils and improvements ([#52](https://github.com/alloy-rs/core/issues/52))

### Miscellaneous Tasks

- Add logo to all crates, add @gakonst to CODEOWNERS ([#138](https://github.com/alloy-rs/core/issues/138))
- Clean up features ([#116](https://github.com/alloy-rs/core/issues/116))
- Feature-gate `getrandom`, document in README.md ([#71](https://github.com/alloy-rs/core/issues/71))
- Rename to Alloy ([#69](https://github.com/alloy-rs/core/issues/69))
- Enable `feature(doc_cfg, doc_auto_cfg)` ([#67](https://github.com/alloy-rs/core/issues/67))
- Pre-release mega cleanup ([#35](https://github.com/alloy-rs/core/issues/35))
- Use crates.io uint, move crates to `crates/*` ([#31](https://github.com/alloy-rs/core/issues/31))

### Other

- Fix dep job, add feature-checks job ([#64](https://github.com/alloy-rs/core/issues/64))
- Prestwich/crate readmes ([#41](https://github.com/alloy-rs/core/issues/41))

### Performance

- Improve rlp, update Address methods ([#118](https://github.com/alloy-rs/core/issues/118))

### Refactor

- Implement `SolType` for `{Ui,I}nt<N>` and `FixedBytes<N>` with const-generics ([#92](https://github.com/alloy-rs/core/issues/92))

### Styling

- Add rustfmt.toml ([#42](https://github.com/alloy-rs/core/issues/42))

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
