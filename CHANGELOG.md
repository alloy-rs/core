# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.7](https://github.com/alloy-rs/core/releases/tag/v1.5.7) - 2026-02-18

### Bug Fixes

- [sol-macro] Prevent direct String usage in expanded code ([#1076](https://github.com/alloy-rs/core/issues/1076))

## [1.5.6](https://github.com/alloy-rs/core/releases/tag/v1.5.6) - 2026-02-12

### Dependencies

- [deps] Bump fixed-cache ([#1073](https://github.com/alloy-rs/core/issues/1073))

### Miscellaneous Tasks

- Release 1.5.6
- Fix changelog

## [1.5.5](https://github.com/alloy-rs/core/releases/tag/v1.5.5) - 2026-02-11

### Dependencies

- [deps] Bump breakings ([#1069](https://github.com/alloy-rs/core/issues/1069))

### Features

- [primitives] Add *flatten* extensions ([#1070](https://github.com/alloy-rs/core/issues/1070))

### Miscellaneous Tasks

- Release 1.5.5
- Enable alloy-rlp MSRV features ([#1068](https://github.com/alloy-rs/core/issues/1068))

### Testing

- Speed up Miri tests ([#1072](https://github.com/alloy-rs/core/issues/1072))
- Use checked methods in Signed tests for cranelift compat ([#1071](https://github.com/alloy-rs/core/issues/1071))

## [1.5.4](https://github.com/alloy-rs/core/releases/tag/v1.5.4) - 2026-01-28

### Miscellaneous Tasks

- Release 1.5.4
- [sol-macro] Use sha3 ([#1064](https://github.com/alloy-rs/core/issues/1064))

### Other

- Fix rkyv miri breakage ([#1066](https://github.com/alloy-rs/core/issues/1066))

### Performance

- [primitives] Use keccak_asm::Keccak256::digest ([#1067](https://github.com/alloy-rs/core/issues/1067))
-  perf(primitives): remove unnecessary keccak cache length hash computation ([#1065](https://github.com/alloy-rs/core/issues/1065))

## [1.5.3](https://github.com/alloy-rs/core/releases/tag/v1.5.3) - 2026-01-27

### Dependencies

- [deps] Run cargo shear ([#1055](https://github.com/alloy-rs/core/issues/1055))

### Features

- [primitives] FixedBytes: schemars::JsonSchema ([#1059](https://github.com/alloy-rs/core/issues/1059))

### Miscellaneous Tasks

- Release 1.5.3
- Allow bincode advisory in deny.toml ([#1060](https://github.com/alloy-rs/core/issues/1060))
- [primitives] Switch default keccak to sha3 ([#1057](https://github.com/alloy-rs/core/issues/1057))

### Other

- Update to tempoxyz ([#1062](https://github.com/alloy-rs/core/issues/1062))

### Performance

- [primitives] Always enable 'sha3/asm' feature ([#1058](https://github.com/alloy-rs/core/issues/1058))

## [1.5.2](https://github.com/alloy-rs/core/releases/tag/v1.5.2) - 2025-12-22

### Miscellaneous Tasks

- Release 1.5.2

### Performance

- [primitives] Always use FxHash for Fb* ([#1054](https://github.com/alloy-rs/core/issues/1054))

## [1.5.1](https://github.com/alloy-rs/core/releases/tag/v1.5.1) - 2025-12-18

### Features

- Extract cache to a separate crate ([#1053](https://github.com/alloy-rs/core/issues/1053))
- [primitives] Add U256Map ([#1052](https://github.com/alloy-rs/core/issues/1052))

### Miscellaneous Tasks

- Release 1.5.1

## [1.5.0](https://github.com/alloy-rs/core/releases/tag/v1.5.0) - 2025-12-16

### Bug Fixes

- [primitives] Cache keccaks up to 88 bytes ([#1049](https://github.com/alloy-rs/core/issues/1049))

### Documentation

- Document allow(unexpected_cfgs) for wrap_fixed_bytes ([#1043](https://github.com/alloy-rs/core/issues/1043))

### Features

- Add rapidhash to available hashers ([#1051](https://github.com/alloy-rs/core/issues/1051))
- Add keccak256_uncached ([#1050](https://github.com/alloy-rs/core/issues/1050))
- [primitives] Add UintTryTo trait for Signed type ([#1029](https://github.com/alloy-rs/core/issues/1029))
- [primitives] Add keccak256_cached ([#1046](https://github.com/alloy-rs/core/issues/1046))
- [primitives] Add `Signature::as_rsy` method ([#1041](https://github.com/alloy-rs/core/issues/1041))
- Add Bloom::accrue_logs method ([#1039](https://github.com/alloy-rs/core/issues/1039))

### Miscellaneous Tasks

- Release 1.5.0
- Rm all deprecations ([#1048](https://github.com/alloy-rs/core/issues/1048))
- [doc] Complete alloy-dyn-abi readme ([#1044](https://github.com/alloy-rs/core/issues/1044))
- Clippy ([#1037](https://github.com/alloy-rs/core/issues/1037))

### Testing

- [primitives] Use correct keccak fn

## [1.4.1](https://github.com/alloy-rs/core/releases/tag/v1.4.1) - 2025-10-14

### Features

- Gate 60 tuple impls behind 'more-tuple-impls' feature flag ([#1027](https://github.com/alloy-rs/core/issues/1027))
- [sol-macro] Add transient storage keyword support ([#1026](https://github.com/alloy-rs/core/issues/1026))
- Add Sqlx Traits for `Bytes` Type  ([#1020](https://github.com/alloy-rs/core/issues/1020))
- [primitives] Add Borsh support for `TxKind` ([#1022](https://github.com/alloy-rs/core/issues/1022))

### Miscellaneous Tasks

- Release 1.4.1
- Remove some inlines ([#1028](https://github.com/alloy-rs/core/issues/1028))
- Fix docs, typos ([#1023](https://github.com/alloy-rs/core/issues/1023))
- Remove feature(doc_auto_cfg) ([#1019](https://github.com/alloy-rs/core/issues/1019))

### Other

- Merge commit from fork

### Refactor

- [dyn-abi] Clean up Resolver ([#1030](https://github.com/alloy-rs/core/issues/1030))

## [1.4.0](https://github.com/alloy-rs/core/releases/tag/v1.4.0) - 2025-09-26

### Bug Fixes

- [sol-macro] Internal SC derives ([#1017](https://github.com/alloy-rs/core/issues/1017))
- [sol-macro-expander] Propagate `all_derives` and `extra_derives` to periphery SC structs ([#1011](https://github.com/alloy-rs/core/issues/1011))
- [sol-macro] Remove #[automatically_derived] from non-trait impls ([#1012](https://github.com/alloy-rs/core/issues/1012))
- [sol-types] Fix `encode_topic_bytes` for byte slices whose length is a non-zero multiple of 32 ([#1000](https://github.com/alloy-rs/core/issues/1000))

### Dependencies

- [deps] Bumpies ([#1014](https://github.com/alloy-rs/core/issues/1014))

### Documentation

- [primitives] Inline doc for uint! macro ([#1007](https://github.com/alloy-rs/core/issues/1007))

### Features

- Rkyv support ([#990](https://github.com/alloy-rs/core/issues/990))
- Add Sqlx Traits for `Signed` Type ([#1008](https://github.com/alloy-rs/core/issues/1008))
- [sol-macro] Inherit attributes from contract ([#1004](https://github.com/alloy-rs/core/issues/1004))
- [primitives] Bump map deps, wrap `DefaultHashBuilder` ([#1001](https://github.com/alloy-rs/core/issues/1001))
- [sol-macro-expander] Add `Clone` trait to enum contracts containers ([#1003](https://github.com/alloy-rs/core/issues/1003))
- [primitives] Extend implementation of diesel's ToSql to Sqlite for FixedBytes and Address ([#977](https://github.com/alloy-rs/core/issues/977))
- [sol-macro-expander] Add `name_by_selector` method for enum variant retrieval ([#995](https://github.com/alloy-rs/core/issues/995))
- [primitives] Add borsh support ([#993](https://github.com/alloy-rs/core/issues/993))

### Miscellaneous Tasks

- Release 1.4.0
- Tweak postgres.rs ([#1018](https://github.com/alloy-rs/core/issues/1018))
- [sol-types] Sync panic reasons from geth ([#1015](https://github.com/alloy-rs/core/issues/1015))
- Typo rollup ([#997](https://github.com/alloy-rs/core/issues/997))

### Performance

- [sol-macro] Improve abi expansion ([#1005](https://github.com/alloy-rs/core/issues/1005))

## [1.3.1](https://github.com/alloy-rs/core/releases/tag/v1.3.1) - 2025-08-17

### Bug Fixes

- [primitives] Re-export correct `Entry` types ([#989](https://github.com/alloy-rs/core/issues/989))
- [rpc] Check reserved function names ([#987](https://github.com/alloy-rs/core/issues/987))

### Miscellaneous Tasks

- Release 1.3.1
- Add typos ([#991](https://github.com/alloy-rs/core/issues/991))

### Other

- Implement conversion from `Word` for `DynSolValue` ([#983](https://github.com/alloy-rs/core/issues/983))

## [1.3.0](https://github.com/alloy-rs/core/releases/tag/v1.3.0) - 2025-07-22

### Bug Fixes

- [sol-types] Overflow in abi decoder ([#982](https://github.com/alloy-rs/core/issues/982))

### Documentation

- Add EIP-712 usage example to README ([#975](https://github.com/alloy-rs/core/issues/975))

### Features

- [primitives] Serialize `Signed` with a compact binary representation ([#953](https://github.com/alloy-rs/core/issues/953))
- Add native sqlx support for Address (MySQL/Postgres/SQLite) with feature gating and tests ([#970](https://github.com/alloy-rs/core/issues/970))
- Add `is_dynamic` method to `DynSolType` ([#974](https://github.com/alloy-rs/core/issues/974))

### Miscellaneous Tasks

- Release 1.3.0
- Fix warning in generated code ([#976](https://github.com/alloy-rs/core/issues/976))
- [meta] Update .gitignore
- Add helper to find function by selector ([#971](https://github.com/alloy-rs/core/issues/971))

## [1.2.1](https://github.com/alloy-rs/core/releases/tag/v1.2.1) - 2025-06-20

### Bug Fixes

- Colon 712 identifiers ([#963](https://github.com/alloy-rs/core/issues/963))

### Dependencies

- [meta] Add edition 2024 bump to .git-blame-ignore-revs

### Miscellaneous Tasks

- Release 1.2.1
- Re-enable clippy::missing-const-for-fn ([#961](https://github.com/alloy-rs/core/issues/961))

## [1.2.0](https://github.com/alloy-rs/core/releases/tag/v1.2.0) - 2025-06-04

### Dependencies

- Bump to edition 2024 ([#960](https://github.com/alloy-rs/core/issues/960))
- Bump MSRV to 1.85 ([#959](https://github.com/alloy-rs/core/issues/959))

### Miscellaneous Tasks

- Release 1.2.0

## [1.1.3](https://github.com/alloy-rs/core/releases/tag/v1.1.3) - 2025-06-04

### Bug Fixes

- Eip712 string interface prefix ([#954](https://github.com/alloy-rs/core/issues/954))

### Features

- Added `decode_log_validate` method ([#957](https://github.com/alloy-rs/core/issues/957))

### Miscellaneous Tasks

- Release 1.1.3

## [1.1.2](https://github.com/alloy-rs/core/releases/tag/v1.1.2) - 2025-05-20

### Dependencies

- Enhance eip712 string parser to canonicalize inputs ([#950](https://github.com/alloy-rs/core/issues/950))

### Miscellaneous Tasks

- Release 1.1.2

## [1.1.1](https://github.com/alloy-rs/core/releases/tag/v1.1.1) - 2025-05-19

### Features

- Added standalone format_units_with ([#947](https://github.com/alloy-rs/core/issues/947))

### Miscellaneous Tasks

- Release 1.1.1
- Re-use alloy_primitives::hex ([#952](https://github.com/alloy-rs/core/issues/952))

### Other

- Remove unnecessary bound on `sol_data::Array` ([#951](https://github.com/alloy-rs/core/issues/951))

## [1.1.0](https://github.com/alloy-rs/core/releases/tag/v1.1.0) - 2025-04-30

### Bug Fixes

- Array size evaluation with modulo operator ([#930](https://github.com/alloy-rs/core/issues/930))

### Documentation

- Improve eip712 docs and discoverability ([#940](https://github.com/alloy-rs/core/issues/940))
- Update getrandom js feature

### Features

- [primitives] Add BitX<&Self> for wrap_fixed_bytes ([#945](https://github.com/alloy-rs/core/issues/945))
- [primitives] Add BitX<&Self> for FixedBytes ([#943](https://github.com/alloy-rs/core/issues/943))
- [`sol!`] Ignore unlinked bytecode ([#935](https://github.com/alloy-rs/core/issues/935))
- Add validated variants to ABI decoding methods ([#928](https://github.com/alloy-rs/core/issues/928))
- [primitives] Add `Address::create_eof` ([#932](https://github.com/alloy-rs/core/issues/932))
- Added format_units_with ([#936](https://github.com/alloy-rs/core/issues/936))
- Add `KECCAK256_EMPTY` from `revm::primitives` ([#931](https://github.com/alloy-rs/core/issues/931))
- Convert between `Signed` of different length ([#923](https://github.com/alloy-rs/core/issues/923))

### Miscellaneous Tasks

- Release 1.1.0

## [1.0.0](https://github.com/alloy-rs/core/releases/tag/v1.0.0) - 2025-04-03

### Features

- [primitives] Supporting diesel @ 2.2 ([#915](https://github.com/alloy-rs/core/issues/915))
- 1.0-rc.1
- Bump ruint, adjust rand feature

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

### Features

- [sol-macro] Improve call return encoding ([#909](https://github.com/alloy-rs/core/issues/909))

## [0.8.23](https://github.com/alloy-rs/core/releases/tag/v0.8.23) - 2025-03-13

### Bug Fixes

- [`sol-expander`] Rename from/into + impl From ([#905](https://github.com/alloy-rs/core/issues/905))
- [`sol!`] Pass correct call_struct to call_builder in expansion ([#901](https://github.com/alloy-rs/core/issues/901))
- [sol-macro] Rm fake transport from contract expansion ([#865](https://github.com/alloy-rs/core/issues/865))

### Dependencies

- [deps] Bump getrandom to 0.3, rand to 0.9 ([#869](https://github.com/alloy-rs/core/issues/869))

### Features

- [primitives] Remove `From<String> for Bytes` ([#907](https://github.com/alloy-rs/core/issues/907))
- [`sol!`] Gen unit/tuple structs for errors, calls, events with 0/1 param ([#883](https://github.com/alloy-rs/core/issues/883))
- [sol-macro] Function calls should directly yield result  ([#855](https://github.com/alloy-rs/core/issues/855))
- [sol-types] Rm `validate: bool`  ([#863](https://github.com/alloy-rs/core/issues/863))

### Miscellaneous Tasks

- Remove deprecated `Signature` ([#899](https://github.com/alloy-rs/core/issues/899))

### Other

- Merge branch 'main' into v1.0-rc

## [1.0.0](https://github.com/alloy-rs/core/releases/tag/v1.0.0) - 2025-04-03

### Bug Fixes

- [primitives] Remove undefined behavior in FixedBytes ([#919](https://github.com/alloy-rs/core/issues/919))
- Do not rely on bytes dependency in `wrap_fixed_bytes!` ([#918](https://github.com/alloy-rs/core/issues/918))

### Features

- Add inner mut ([#921](https://github.com/alloy-rs/core/issues/921))

### Miscellaneous Tasks

- Add hash_ref function to sealed.rs ([#920](https://github.com/alloy-rs/core/issues/920))

## [0.8.24](https://github.com/alloy-rs/core/releases/tag/v0.8.24) - 2025-03-21

### Features

- [`json-abi`] Config to generate types in interface ([#911](https://github.com/alloy-rs/core/issues/911))
- [sol-macro] Add `#![sol(extra_derives(...)]` ([#910](https://github.com/alloy-rs/core/issues/910))

### Miscellaneous Tasks

- Release 0.8.24
- [meta] Update .gitignore
- Restore clippy allow
- [meta] Update CODEOWNERS

### Other

- Make PrimitiveSignature::new a const fn ([#913](https://github.com/alloy-rs/core/issues/913))

### Testing

- Add a test for namespaced types

## [0.8.23](https://github.com/alloy-rs/core/releases/tag/v0.8.23) - 2025-03-13

### Bug Fixes

- [`sol-expander`] Map `self` to `this` in codegen ([#903](https://github.com/alloy-rs/core/issues/903))

### Features

- [sol-macro] Allow standard library macros for string literals ([#898](https://github.com/alloy-rs/core/issues/898))
- [`primitives`] Impl Display for PrimitiveSig ([#892](https://github.com/alloy-rs/core/issues/892))

### Miscellaneous Tasks

- Release 0.8.23
- Clippy ([#894](https://github.com/alloy-rs/core/issues/894))
- [primitives] Make TxKind::into_to const ([#890](https://github.com/alloy-rs/core/issues/890))

### Testing

- Move 'self' keyword test ([#906](https://github.com/alloy-rs/core/issues/906))

## [0.8.22](https://github.com/alloy-rs/core/releases/tag/v0.8.22) - 2025-02-27

### Dependencies

- [deps] Bump derive_more to 2 ([#871](https://github.com/alloy-rs/core/issues/871))

### Documentation

- [primitives] Report some Bytes methods may panic ([#877](https://github.com/alloy-rs/core/issues/877))
- [primitives] `random` functions are cryptographically secure ([#872](https://github.com/alloy-rs/core/issues/872))

### Features

- [primitives] Add some more utility methods to PrimitiveSignature ([#888](https://github.com/alloy-rs/core/issues/888))
- Erc2098 signature representation ([#874](https://github.com/alloy-rs/core/issues/874))
- Add TxKind::into_to ([#875](https://github.com/alloy-rs/core/issues/875))
- [primitives] Improve rand implementations, use `thread_rng` when available ([#870](https://github.com/alloy-rs/core/issues/870))

### Miscellaneous Tasks

- Release 0.8.22
- Simplify uninit_array usage ([#889](https://github.com/alloy-rs/core/issues/889))

## [0.8.21](https://github.com/alloy-rs/core/releases/tag/v0.8.21) - 2025-02-10

### Bug Fixes

- [sol-macro] Call proc_macro_error handler manually ([#866](https://github.com/alloy-rs/core/issues/866))

### Features

- Add helpers for revertreason ([#867](https://github.com/alloy-rs/core/issues/867))
- [`sol-macro-expander`] Increase resolve limit to 128 ([#864](https://github.com/alloy-rs/core/issues/864))

### Miscellaneous Tasks

- Release 0.8.21

## [0.8.20](https://github.com/alloy-rs/core/releases/tag/v0.8.20) - 2025-02-02

### Dependencies

- [deps] Bump winnow 0.7 ([#862](https://github.com/alloy-rs/core/issues/862))

### Documentation

- Add 0x to alloy-primitives readme example ([#861](https://github.com/alloy-rs/core/issues/861))

### Features

- Add Sealed::as_sealed_ref ([#859](https://github.com/alloy-rs/core/issues/859))
- Add Sealed::cloned ([#860](https://github.com/alloy-rs/core/issues/860))

### Miscellaneous Tasks

- Release 0.8.20
- Clippy ([#858](https://github.com/alloy-rs/core/issues/858))

## [0.8.19](https://github.com/alloy-rs/core/releases/tag/v0.8.19) - 2025-01-15

### Documentation

- Enable some useful rustdoc features on docs.rs ([#850](https://github.com/alloy-rs/core/issues/850))
- Hide hex_literal export ([#849](https://github.com/alloy-rs/core/issues/849))

### Features

- [json-abi] Add Param.name() accessor ([#856](https://github.com/alloy-rs/core/issues/856))
- [sol-types] Improve ABI decoding error messages ([#851](https://github.com/alloy-rs/core/issues/851))

### Miscellaneous Tasks

- Release 0.8.19

## [0.8.18](https://github.com/alloy-rs/core/releases/tag/v0.8.18) - 2025-01-04

### Bug Fixes

- [primitives] Hex macro re-export ([#848](https://github.com/alloy-rs/core/issues/848))

### Miscellaneous Tasks

- Release 0.8.18

## [0.8.17](https://github.com/alloy-rs/core/releases/tag/v0.8.17) - 2025-01-04

### Bug Fixes

- Coerce pow overflow ([#838](https://github.com/alloy-rs/core/issues/838))

### Documentation

- Typos ([#847](https://github.com/alloy-rs/core/issues/847))
- [sol-macro] Document visibility and state mutability ([#846](https://github.com/alloy-rs/core/issues/846))

### Features

- [sol-macro] Translate contract types to address ([#842](https://github.com/alloy-rs/core/issues/842))
- Support 0x in hex! and similar macros ([#841](https://github.com/alloy-rs/core/issues/841))
- [sol-macro] Evaluate array sizes ([#840](https://github.com/alloy-rs/core/issues/840))
- [primitives] Re-export foldhash ([#839](https://github.com/alloy-rs/core/issues/839))
- Re-export rayon traits implementations ([#836](https://github.com/alloy-rs/core/issues/836))

### Miscellaneous Tasks

- Release 0.8.17

### Testing

- [sol-macro] Add a test for missing_docs ([#845](https://github.com/alloy-rs/core/issues/845))
- Re-enable miri on foldhash ([#844](https://github.com/alloy-rs/core/issues/844))
- [sol-macro] Add a test for namespaced types ([#843](https://github.com/alloy-rs/core/issues/843))

## [0.8.16](https://github.com/alloy-rs/core/releases/tag/v0.8.16) - 2025-01-01

### Bug Fixes

- Re-enable foldhash on zkvm ([#833](https://github.com/alloy-rs/core/issues/833))
- Allow non-boolean v values for PrimitiveSignature ([#832](https://github.com/alloy-rs/core/issues/832))
- [syn-solidity] Correctly parse invalid bytes* etc as custom ([#830](https://github.com/alloy-rs/core/issues/830))

### Features

- [dyn-abi] Support parse scientific number ([#835](https://github.com/alloy-rs/core/issues/835))
- Re-export `rayon` feature ([#827](https://github.com/alloy-rs/core/issues/827))

### Miscellaneous Tasks

- Release 0.8.16
- Clippy ([#834](https://github.com/alloy-rs/core/issues/834))
- Add clone_inner ([#825](https://github.com/alloy-rs/core/issues/825))
- Shorten map type alias names ([#824](https://github.com/alloy-rs/core/issues/824))
- [primitives] Remove rustc-hash workaround ([#822](https://github.com/alloy-rs/core/issues/822))

### Other

- Move deny to ci ([#821](https://github.com/alloy-rs/core/issues/821))

## [0.8.15](https://github.com/alloy-rs/core/releases/tag/v0.8.15) - 2024-12-09

### Miscellaneous Tasks

- Release 0.8.15
- Mark `Signature` as deprecated ([#819](https://github.com/alloy-rs/core/issues/819))
- AsRef for Log ([#820](https://github.com/alloy-rs/core/issues/820))
- Update release.toml ([#817](https://github.com/alloy-rs/core/issues/817))

### Other

- Remove unsafe code from macro expansions ([#818](https://github.com/alloy-rs/core/issues/818))

## [0.8.14](https://github.com/alloy-rs/core/releases/tag/v0.8.14) - 2024-11-28

### Dependencies

- Bump MSRV to 1.81 ([#790](https://github.com/alloy-rs/core/issues/790))

### Features

- Switch all std::error to core::error ([#815](https://github.com/alloy-rs/core/issues/815))

### Miscellaneous Tasks

- Release 0.8.14

## [0.8.13](https://github.com/alloy-rs/core/releases/tag/v0.8.13) - 2024-11-26

### Bug Fixes

- [sol-macro] Expand all getter return types ([#812](https://github.com/alloy-rs/core/issues/812))

### Dependencies

- Remove cron schedule for deps.yml ([#808](https://github.com/alloy-rs/core/issues/808))

### Features

- Expose `returns` field for `DynSolCall` type ([#809](https://github.com/alloy-rs/core/issues/809))

### Miscellaneous Tasks

- Release 0.8.13 ([#813](https://github.com/alloy-rs/core/issues/813))

### Other

- Make Signature::new a const fn ([#810](https://github.com/alloy-rs/core/issues/810))

## [0.8.12](https://github.com/alloy-rs/core/releases/tag/v0.8.12) - 2024-11-12

### Bug Fixes

- `Sealed::hash` serde ([#805](https://github.com/alloy-rs/core/issues/805))

### Features

- Add `AsRef` impl and `hash` method to `Sealed` ([#804](https://github.com/alloy-rs/core/issues/804))

### Miscellaneous Tasks

- Release 0.8.12 ([#806](https://github.com/alloy-rs/core/issues/806))

## [0.8.11](https://github.com/alloy-rs/core/releases/tag/v0.8.11) - 2024-11-05

### Bug Fixes

- [serde] Add alias `v` for `yParity` ([#801](https://github.com/alloy-rs/core/issues/801))

### Documentation

- Update ethers-rs README note ([#798](https://github.com/alloy-rs/core/issues/798))

### Features

- [json-abi] Add `AbiItem::json_type` ([#797](https://github.com/alloy-rs/core/issues/797))
- Add has_eip155_value convenience function to signature ([#791](https://github.com/alloy-rs/core/issues/791))

### Miscellaneous Tasks

- Release 0.8.11 ([#803](https://github.com/alloy-rs/core/issues/803))
- [json-abi] Clean up utils ([#794](https://github.com/alloy-rs/core/issues/794))
- [meta] Update SECURITY.md ([#793](https://github.com/alloy-rs/core/issues/793))

### Other

- Revert "chore: replace Signature with PrimitiveSignature" ([#800](https://github.com/alloy-rs/core/issues/800))
- Add success job ([#795](https://github.com/alloy-rs/core/issues/795))

### Performance

- Improve normalize_v ([#792](https://github.com/alloy-rs/core/issues/792))

### Styling

- Replace Signature with PrimitiveSignature ([#796](https://github.com/alloy-rs/core/issues/796))

## [0.8.10](https://github.com/alloy-rs/core/releases/tag/v0.8.10) - 2024-10-28

### Bug Fixes

- Revert MSRV changes ([#789](https://github.com/alloy-rs/core/issues/789))

### Dependencies

- Bump MSRV to 1.81 & use `core::error::Error` in place of `std` ([#780](https://github.com/alloy-rs/core/issues/780))

### Documentation

- Fix param type in example comment ([#784](https://github.com/alloy-rs/core/issues/784))

### Miscellaneous Tasks

- Release 0.8.10
- Address MSRV TODOs for 1.81 ([#781](https://github.com/alloy-rs/core/issues/781))

### Other

- Implement `DerefMut` for `Log<T>` ([#786](https://github.com/alloy-rs/core/issues/786))

### Refactor

- Use simple boolean for parity in signature ([#776](https://github.com/alloy-rs/core/issues/776))

## [0.8.9](https://github.com/alloy-rs/core/releases/tag/v0.8.9) - 2024-10-21

### Bug Fixes

- Re-enable foldhash by default, but exclude it from zkvm ([#777](https://github.com/alloy-rs/core/issues/777))

### Features

- Expand Seal api ([#773](https://github.com/alloy-rs/core/issues/773))

### Miscellaneous Tasks

- Release 0.8.9

## [0.8.8](https://github.com/alloy-rs/core/releases/tag/v0.8.8) - 2024-10-14

### Bug Fixes

- Properly account for sign in pg to/from sql implementation for signed ([#772](https://github.com/alloy-rs/core/issues/772))
- Don't enable foldhash by default ([#771](https://github.com/alloy-rs/core/issues/771))
- [alloy-sol-macro] Allow clippy::pub_underscore_fields on `sol!` output ([#770](https://github.com/alloy-rs/core/issues/770))

### Features

- Add logs_bloom ([#768](https://github.com/alloy-rs/core/issues/768))

### Miscellaneous Tasks

- Release 0.8.8

## [0.8.7](https://github.com/alloy-rs/core/releases/tag/v0.8.7) - 2024-10-08

### Miscellaneous Tasks

- Release 0.8.7

### Other

- Revert "Add custom serialization for Address" ([#765](https://github.com/alloy-rs/core/issues/765))

## [0.8.6](https://github.com/alloy-rs/core/releases/tag/v0.8.6) - 2024-10-06

### Bug Fixes

- Fix lint `alloy-primitives` ([#756](https://github.com/alloy-rs/core/issues/756))
- Fix lint `alloy-json-abi` ([#757](https://github.com/alloy-rs/core/issues/757))
- Fix lint `alloy-dyn-abi` ([#758](https://github.com/alloy-rs/core/issues/758))
- Fix lint alloy-sol-types ([#761](https://github.com/alloy-rs/core/issues/761))
- Fix lint `alloy-sol-macro-expander` ([#760](https://github.com/alloy-rs/core/issues/760))

### Dependencies

- [deps] Bump hashbrown to 0.15 ([#753](https://github.com/alloy-rs/core/issues/753))

### Features

- Add `Default` for `Sealed<T>` ([#755](https://github.com/alloy-rs/core/issues/755))
- [primitives] Add and use foldhash as default hasher ([#763](https://github.com/alloy-rs/core/issues/763))

### Miscellaneous Tasks

- Release 0.8.6
- [meta] Update CODEOWNERS
- Remove a stabilized impl_core function

### Other

- Derive `Arbitrary` for `Sealed<T>` ([#762](https://github.com/alloy-rs/core/issues/762))
- Derive `Deref` for `Sealed<T>` ([#759](https://github.com/alloy-rs/core/issues/759))
- Add conversion `TxKind` -> `Option<Address>` ([#750](https://github.com/alloy-rs/core/issues/750))

## [0.8.5](https://github.com/alloy-rs/core/releases/tag/v0.8.5) - 2024-09-25

### Bug Fixes

- [primitives] Make sure DefaultHashBuilder implements Clone ([#748](https://github.com/alloy-rs/core/issues/748))

### Miscellaneous Tasks

- Release 0.8.5
- [primitives] Remove Fx* aliases ([#749](https://github.com/alloy-rs/core/issues/749))

## [0.8.4](https://github.com/alloy-rs/core/releases/tag/v0.8.4) - 2024-09-25

### Bug Fixes

- [json-abi] Normalize $ to _ in identifiers in to_sol ([#747](https://github.com/alloy-rs/core/issues/747))
- [json-abi] Correct to-sol for UDVT arrays in structs ([#745](https://github.com/alloy-rs/core/issues/745))
- [sol-types] Check signature in SolEvent if non-anonymous ([#741](https://github.com/alloy-rs/core/issues/741))

### Features

- [primitives] Implement `map` module ([#743](https://github.com/alloy-rs/core/issues/743))
- Support Keccak with sha3 ([#737](https://github.com/alloy-rs/core/issues/737))

### Miscellaneous Tasks

- Release 0.8.4
- Remove unused unstable-doc feature

### Other

- Add custom serialization for Address ([#742](https://github.com/alloy-rs/core/issues/742))

### Testing

- Allow missing_docs in tests
- Add another dyn-abi test

## [0.8.3](https://github.com/alloy-rs/core/releases/tag/v0.8.3) - 2024-09-10

### Bug Fixes

- [sol-macro] Correctly determine whether event parameters are hashes ([#735](https://github.com/alloy-rs/core/issues/735))
- [sol-macro] Namespaced custom type resolution ([#731](https://github.com/alloy-rs/core/issues/731))
- Parse selector hashes in `sol` macro ([#730](https://github.com/alloy-rs/core/issues/730))

### Features

- Prepare reth Signature migration to alloy ([#732](https://github.com/alloy-rs/core/issues/732))

### Miscellaneous Tasks

- Release 0.8.3

## [0.8.2](https://github.com/alloy-rs/core/releases/tag/v0.8.2) - 2024-09-06

### Bug Fixes

- `no_std` and workflow ([#727](https://github.com/alloy-rs/core/issues/727))

### Documentation

- [primitives] Document features in `wrap_fixed_bytes`-generated types ([#726](https://github.com/alloy-rs/core/issues/726))

### Miscellaneous Tasks

- Release 0.8.2

## [0.8.1](https://github.com/alloy-rs/core/releases/tag/v0.8.1) - 2024-09-06

### Bug Fixes

- [sol-type-parser] Winnow std error ([#720](https://github.com/alloy-rs/core/issues/720))
- Use quantity for v value ([#715](https://github.com/alloy-rs/core/issues/715))

### Dependencies

- Bump MSRV to 1.79 ([#712](https://github.com/alloy-rs/core/issues/712))
- Revert "chore(deps): bump derive_more to 1.0"
- [deps] Bump derive_more to 1.0

### Miscellaneous Tasks

- Release 0.8.1
- Clippy
- Use proc-macro-error2 ([#723](https://github.com/alloy-rs/core/issues/723))

### Performance

- [primitives] Improve checksum algorithm ([#713](https://github.com/alloy-rs/core/issues/713))

### Refactor

- Remove `Signature` generic ([#719](https://github.com/alloy-rs/core/issues/719))

### Testing

- [sol] Add a test for custom paths

## [0.8.0](https://github.com/alloy-rs/core/releases/tag/v0.8.0) - 2024-08-21

### Bug Fixes

- Parsing stack overflow ([#703](https://github.com/alloy-rs/core/issues/703))

### Dependencies

- [deps] Bump proptest-derive ([#708](https://github.com/alloy-rs/core/issues/708))

### Documentation

- Typo

### Features

- Derive ser deser on `Sealed` ([#710](https://github.com/alloy-rs/core/issues/710))
- [sol-macro] Support namespaces ([#694](https://github.com/alloy-rs/core/issues/694))
- Derive `Hash` for `Sealed` ([#707](https://github.com/alloy-rs/core/issues/707))
- [sol-types] Implement traits for longer tuples ([#699](https://github.com/alloy-rs/core/issues/699))

### Miscellaneous Tasks

- Release 0.8.0
- [primitives] Re-use ruint mask function ([#698](https://github.com/alloy-rs/core/issues/698))
- Derive hash for parity ([#686](https://github.com/alloy-rs/core/issues/686))
- Add some TODO comments

### Other

- Implement specific bit types for integers ([#677](https://github.com/alloy-rs/core/issues/677))
- Add testcase for overflowing_from_sign_and_abs ([#696](https://github.com/alloy-rs/core/issues/696))

### Styling

- Remove `ethereum_ssz` dependency ([#701](https://github.com/alloy-rs/core/issues/701))

## [0.7.7](https://github.com/alloy-rs/core/releases/tag/v0.7.7) - 2024-07-08

### Bug Fixes

- Small fixes for `DynSolValue` strategies ([#683](https://github.com/alloy-rs/core/issues/683))
- Fixed bytes dyn abi packed encoding ([#671](https://github.com/alloy-rs/core/issues/671))
- [primitives] Include in aliases export to prevent having to import from `aliases::{..}` ([#655](https://github.com/alloy-rs/core/issues/655))

### Documentation

- [primitives] Fix rustdoc for Signature ([#680](https://github.com/alloy-rs/core/issues/680))
- [sol-types] Update README.md using crate docs ([#679](https://github.com/alloy-rs/core/issues/679))
- Add per-crate changelogs ([#669](https://github.com/alloy-rs/core/issues/669))
- Update MSRV policy ([#665](https://github.com/alloy-rs/core/issues/665))

### Features

- [json-abi] Allow `serde_json::from_{value,reader}` ([#684](https://github.com/alloy-rs/core/issues/684))
- Add support for parsing visibility and state mutability ([#682](https://github.com/alloy-rs/core/issues/682))
- DynSolCall ([#632](https://github.com/alloy-rs/core/issues/632))
- IntoLogData ([#666](https://github.com/alloy-rs/core/issues/666))
- Add `abi_packed_encoded_size` ([#672](https://github.com/alloy-rs/core/issues/672))
- [primitives] Manually implement arbitrary for signature ([#663](https://github.com/alloy-rs/core/issues/663))

### Miscellaneous Tasks

- Release 0.7.7
- Use workspace.lints ([#676](https://github.com/alloy-rs/core/issues/676))
- Fix unnameable-types ([#675](https://github.com/alloy-rs/core/issues/675))
- [sol-macro] Allow clippy all when emitting contract bytecode ([#674](https://github.com/alloy-rs/core/issues/674))
- Add book/examples to readme
- [sol-types] Exit early if Abigen input is invalid
- Swap sol macro doctests symlink ([#657](https://github.com/alloy-rs/core/issues/657))

### Styling

- Format some imports
- Format GHA workflow
- Sort derives ([#662](https://github.com/alloy-rs/core/issues/662))

## [0.7.6](https://github.com/alloy-rs/core/releases/tag/v0.7.6) - 2024-06-10

### Features

- [primitives] Add additional common aliases ([#654](https://github.com/alloy-rs/core/issues/654))
- [primitives] Derive `Arbitrary` for Signature ([#652](https://github.com/alloy-rs/core/issues/652))
- [primitives] Implement `ops::Not` for fixed bytes ([#650](https://github.com/alloy-rs/core/issues/650))
- [sol-macro] Add return value names to simple getters ([#648](https://github.com/alloy-rs/core/issues/648))

### Miscellaneous Tasks

- Release 0.7.6
- [docs] Add doc aliases for `Tx` prefixed names ([#649](https://github.com/alloy-rs/core/issues/649))
- Update changelog.sh
- Fix CHANGELOG parsers for uppercase

## [0.7.5](https://github.com/alloy-rs/core/releases/tag/v0.7.5) - 2024-06-04

### Bug Fixes

- [sol-macro] Allow deriving `Default` on contracts ([#645](https://github.com/alloy-rs/core/issues/645))
- [sol-macro] Overridden event signatures ([#642](https://github.com/alloy-rs/core/issues/642))
- [primitives] Signed formatting ([#643](https://github.com/alloy-rs/core/issues/643))
- Fix Log serde for non self describing protocols ([#639](https://github.com/alloy-rs/core/issues/639))
- Handle 0 for inverting eip155 parity. ([#633](https://github.com/alloy-rs/core/issues/633))

### Documentation

- Update some READMEs ([#641](https://github.com/alloy-rs/core/issues/641))

### Features

- [primitives] Implement TryInto for ParseUnits ([#646](https://github.com/alloy-rs/core/issues/646))
- [sol-macro] Allow overridden custom errors ([#644](https://github.com/alloy-rs/core/issues/644))
- Create new method on Param and EventParam ([#634](https://github.com/alloy-rs/core/issues/634))

### Miscellaneous Tasks

- Release 0.7.5
- [sol-macro] Add suggestion to remove name ([#647](https://github.com/alloy-rs/core/issues/647))
- Temporarily disable tests that OOM Miri ([#637](https://github.com/alloy-rs/core/issues/637))

## [0.7.4](https://github.com/alloy-rs/core/releases/tag/v0.7.4) - 2024-05-14

### Bug Fixes

- [sol-macro] Json feature ([#629](https://github.com/alloy-rs/core/issues/629))

### Miscellaneous Tasks

- Release 0.7.4
- Release 0.7.3
- Fix dyn abi
- Release 0.7.3

## [0.7.3](https://github.com/alloy-rs/core/releases/tag/v0.7.3) - 2024-05-14

### Documentation

- Update alloy-core homepage link

### Features

- [dyn-abi] Derive `Eq` for `TypedData` ([#623](https://github.com/alloy-rs/core/issues/623))
- [sol-macro] Allow missing docs for event fields ([#619](https://github.com/alloy-rs/core/issues/619))

### Miscellaneous Tasks

- Release 0.7.3
- Fix tests ([#624](https://github.com/alloy-rs/core/issues/624))
- Unused cfgs

### Refactor

- Move `expand` from `sol-macro` to its own crate ([#626](https://github.com/alloy-rs/core/issues/626))

## [0.7.2](https://github.com/alloy-rs/core/releases/tag/v0.7.2) - 2024-05-02

### Documentation

- Unhide and mention `sol!` wrappers ([#615](https://github.com/alloy-rs/core/issues/615))

### Miscellaneous Tasks

- Release 0.7.2
- [general] Add basic CI workflow for Windows ([#613](https://github.com/alloy-rs/core/issues/613))

### Other

- Add derive[Clone] to SolEvent creation ([#616](https://github.com/alloy-rs/core/issues/616))

## [0.7.1](https://github.com/alloy-rs/core/releases/tag/v0.7.1) - 2024-04-23

### Bug Fixes

- Use deploy in sol expansion ([#606](https://github.com/alloy-rs/core/issues/606))

### Documentation

- Update README crate links to use URLs ([#603](https://github.com/alloy-rs/core/issues/603))
- [sol-macro] Add some more disclaimers ([#595](https://github.com/alloy-rs/core/issues/595))

### Features

- Add arbitrary for TxKind ([#604](https://github.com/alloy-rs/core/issues/604))
- [json-abi] Support legacy JSON ABIs ([#596](https://github.com/alloy-rs/core/issues/596))

### Miscellaneous Tasks

- Release 0.7.1
- FixedBytes instead of array
- Add a automatically_derived ([#597](https://github.com/alloy-rs/core/issues/597))
- Update tests and clippy

## [0.7.0](https://github.com/alloy-rs/core/releases/tag/v0.7.0) - 2024-03-30

### Bug Fixes

- [json-abi] Correct to_sol for arrays of contracts ([#586](https://github.com/alloy-rs/core/issues/586))
- [sol-macro] Don't double attributes in JSON input ([#583](https://github.com/alloy-rs/core/issues/583))
- [dyn-abi] Correctly parse uints in `coerce_str` ([#577](https://github.com/alloy-rs/core/issues/577))
- Force clippy to stable ([#569](https://github.com/alloy-rs/core/issues/569))
- [primitives] Re-implement RLP for `Log<LogData>` ([#573](https://github.com/alloy-rs/core/issues/573))
- [sol-macro] Rpc event filter function name ([#572](https://github.com/alloy-rs/core/issues/572))
- [sol-macro] Enumerate before filtering when expanding events ([#561](https://github.com/alloy-rs/core/issues/561))

### Documentation

- Do not accept grammar prs ([#575](https://github.com/alloy-rs/core/issues/575))
- [sol-macro] Add a note about sol(rpc) in Contracts paragraph ([#556](https://github.com/alloy-rs/core/issues/556))

### Features

- Rlp encoding for logs with generic event data ([#553](https://github.com/alloy-rs/core/issues/553))
- [sol-macro] Add event filters to contracts ([#563](https://github.com/alloy-rs/core/issues/563))
- [json-abi] Add configuration for `JsonAbi::to_sol` ([#558](https://github.com/alloy-rs/core/issues/558))
- Add LogData::split ([#559](https://github.com/alloy-rs/core/issues/559))
- Add network generic to sol-macro ([#557](https://github.com/alloy-rs/core/issues/557))

### Miscellaneous Tasks

- Release 0.7.0
- No-default-features k256 ([#576](https://github.com/alloy-rs/core/issues/576))
- Remove dead code ([#571](https://github.com/alloy-rs/core/issues/571))

### Other

- Small helpers for alloy serde PR ([#582](https://github.com/alloy-rs/core/issues/582))
- Use latest stable
- Prestwich/dyn sol error ([#551](https://github.com/alloy-rs/core/issues/551))

### Performance

- [sol-macro] Decode bytecode hex strings ourselves ([#562](https://github.com/alloy-rs/core/issues/562))

### Refactor

- Break SolInput to its own crate ([#578](https://github.com/alloy-rs/core/issues/578))
- Change identical resolve traits to Specifier<T> ([#550](https://github.com/alloy-rs/core/issues/550))

### Styling

- Rearranged type param order so that the Network param is the last ([#587](https://github.com/alloy-rs/core/issues/587))
- Make `Bytes` map to `Bytes` in `SolType` ([#545](https://github.com/alloy-rs/core/issues/545))

## [0.6.4](https://github.com/alloy-rs/core/releases/tag/v0.6.4) - 2024-02-29

### Bug Fixes

- [dyn-abi] Correctly parse empty lists of bytes ([#548](https://github.com/alloy-rs/core/issues/548))
- [dyn-abi] Enable `DynSolType.coerce_json` to convert array of numbers to bytes ([#541](https://github.com/alloy-rs/core/issues/541))

### Dependencies

- [deps] Update winnow to 0.6 ([#533](https://github.com/alloy-rs/core/issues/533))

### Documentation

- [primitives] Add a bytes! macro example ([#539](https://github.com/alloy-rs/core/issues/539))
- Fix relative paths in README files ([#532](https://github.com/alloy-rs/core/issues/532))

### Features

- Add `TxKind` ([#542](https://github.com/alloy-rs/core/issues/542))
- [core] Re-export `uint!` ([#537](https://github.com/alloy-rs/core/issues/537))
- Derive Allocative on FixedBytes ([#531](https://github.com/alloy-rs/core/issues/531))

### Miscellaneous Tasks

- Release 0.6.4
- [primitives] Improve `from_slice` functions ([#546](https://github.com/alloy-rs/core/issues/546))
- Allow unknown lints ([#543](https://github.com/alloy-rs/core/issues/543))
- [core] Add comments to `cfg(doc)` ([#538](https://github.com/alloy-rs/core/issues/538))
- Remove unused imports ([#534](https://github.com/alloy-rs/core/issues/534))

### Other

- Add concurrency ([#540](https://github.com/alloy-rs/core/issues/540))

### Testing

- Add another ABI encode test ([#547](https://github.com/alloy-rs/core/issues/547))
- Add some more coerce error message tests ([#535](https://github.com/alloy-rs/core/issues/535))
- Bless tests ([#530](https://github.com/alloy-rs/core/issues/530))

## [0.6.3](https://github.com/alloy-rs/core/releases/tag/v0.6.3) - 2024-02-15

### Bug Fixes

- [json-abi] Accept nameless `Param`s ([#526](https://github.com/alloy-rs/core/issues/526))
- [dyn-abi] Abi-encode-packed always pads arrays ([#519](https://github.com/alloy-rs/core/issues/519))
- Properly test ABI packed encoding ([#517](https://github.com/alloy-rs/core/issues/517))
- Signature bincode serialization ([#509](https://github.com/alloy-rs/core/issues/509))
- Don't validate when decoding revert reason ([#511](https://github.com/alloy-rs/core/issues/511))

### Dependencies

- [deps] Update some dependencies ([#522](https://github.com/alloy-rs/core/issues/522))
- [deps] Bump winnow ([#518](https://github.com/alloy-rs/core/issues/518))
- Recursion mitigations ([#495](https://github.com/alloy-rs/core/issues/495))

### Documentation

- Update alloy_core::sol reference to real sol ([#529](https://github.com/alloy-rs/core/issues/529))
- Mention `alloy-core` meta crate in README.md overview ([#523](https://github.com/alloy-rs/core/issues/523))

### Features

- [primitives] Add some more implementations to Bytes ([#528](https://github.com/alloy-rs/core/issues/528))
- [sol-macro] Provide a way to override import paths for dependencies ([#527](https://github.com/alloy-rs/core/issues/527))
- Add `alloy-core` prelude crate ([#521](https://github.com/alloy-rs/core/issues/521))
- [sol-types] Constify type name formatting ([#520](https://github.com/alloy-rs/core/issues/520))
- [sol-macro] Add `#[sol(rpc)]` to generate type-safe provider contract calls ([#510](https://github.com/alloy-rs/core/issues/510))
- [sol-macro] Expand state variable getters in contracts ([#514](https://github.com/alloy-rs/core/issues/514))
- Make some allocations fallible in ABI decoding ([#513](https://github.com/alloy-rs/core/issues/513))

### Miscellaneous Tasks

- Release 0.6.3
- Fix winnow deprecation warnings ([#507](https://github.com/alloy-rs/core/issues/507))
- [sol-macro] Tweak inline attributes in generated code ([#505](https://github.com/alloy-rs/core/issues/505))

### Other

- Update actions/checkout to v4 ([#512](https://github.com/alloy-rs/core/issues/512))

### Performance

- [sol-macro] Use a lookup table when generating `SolInterface::abi_decode_raw` ([#508](https://github.com/alloy-rs/core/issues/508))
- [sol-macro] Use `binary_search` in `SolInterface::valid_selector` ([#506](https://github.com/alloy-rs/core/issues/506))

### Testing

- Bless tests ([#524](https://github.com/alloy-rs/core/issues/524))
- Remove unused test ([#504](https://github.com/alloy-rs/core/issues/504))

## [0.6.2](https://github.com/alloy-rs/core/releases/tag/v0.6.2) - 2024-01-25

### Bug Fixes

- [`signature`] Construct Signature bytes using v+27 when we do not have an EIP155 `v` ([#503](https://github.com/alloy-rs/core/issues/503))

### Miscellaneous Tasks

- Release 0.6.2

## [0.6.1](https://github.com/alloy-rs/core/releases/tag/v0.6.1) - 2024-01-25

### Bug Fixes

- Deserialize missing state mutability as non payable ([#488](https://github.com/alloy-rs/core/issues/488))

### Documentation

- Remove stray list element ([#500](https://github.com/alloy-rs/core/issues/500))
- Fixes ([#498](https://github.com/alloy-rs/core/issues/498))

### Features

- [`primitives`] Add `y_parity_byte_non_eip155` to `Parity` ([#499](https://github.com/alloy-rs/core/issues/499))
- Add constructorCall to `sol!` ([#493](https://github.com/alloy-rs/core/issues/493))
- [primitives] Add `Address::from_private_key` ([#483](https://github.com/alloy-rs/core/issues/483))

### Miscellaneous Tasks

- Release 0.6.1
- Add SECURITY.md ([#494](https://github.com/alloy-rs/core/issues/494))
- [primitives] Pass B256 by reference in Signature methods ([#487](https://github.com/alloy-rs/core/issues/487))
- Include path in error ([#486](https://github.com/alloy-rs/core/issues/486))
- Improve unlinked bytecode deserde error ([#484](https://github.com/alloy-rs/core/issues/484))

### Testing

- Don't print constructors for Solc tests ([#501](https://github.com/alloy-rs/core/issues/501))
- Parity roundtripping ([#497](https://github.com/alloy-rs/core/issues/497))

## [0.6.0](https://github.com/alloy-rs/core/releases/tag/v0.6.0) - 2024-01-10

### Bug Fixes

- [primitives] Also apply EIP-155 to Parity::Parity ([#476](https://github.com/alloy-rs/core/issues/476))
- Clean the sealed ([#468](https://github.com/alloy-rs/core/issues/468))

### Dependencies

- [deps] Relax k256 requirement ([#481](https://github.com/alloy-rs/core/issues/481))
- [deps] Bump const-hex requirement ([#479](https://github.com/alloy-rs/core/issues/479))

### Documentation

- Update docs on parity ([#477](https://github.com/alloy-rs/core/issues/477))

### Features

- [json-abi] Add full_signature ([#480](https://github.com/alloy-rs/core/issues/480))
- [primitives] Add Signature type and utils ([#459](https://github.com/alloy-rs/core/issues/459))
- [primitives] Add a buffer type for address checksums ([#472](https://github.com/alloy-rs/core/issues/472))
- [dyn-abi] Improve hex error messages ([#474](https://github.com/alloy-rs/core/issues/474))
- [sol-type-parser] Improve error message for bad array size ([#470](https://github.com/alloy-rs/core/issues/470))
- [primitives] Add Keccak256 hasher struct ([#469](https://github.com/alloy-rs/core/issues/469))

### Miscellaneous Tasks

- Release 0.6.0
- Bless tests ([#478](https://github.com/alloy-rs/core/issues/478))
- Clippy uninlined_format_args, use_self ([#475](https://github.com/alloy-rs/core/issues/475))
- Touch up UDVT expansion ([#473](https://github.com/alloy-rs/core/issues/473))
- Move define_udt! decl macro to sol! proc macro ([#471](https://github.com/alloy-rs/core/issues/471))
- Release 0.5.4

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
- Clippy ([#463](https://github.com/alloy-rs/core/issues/463))
- [sol-types] Make PanicKind non_exhaustive ([#458](https://github.com/alloy-rs/core/issues/458))

### Performance

- Add optional support for keccak-asm ([#466](https://github.com/alloy-rs/core/issues/466))

### Styling

- Add ToSql and FromSql to Signed and FixedBytes ([#447](https://github.com/alloy-rs/core/issues/447))

## [0.5.3](https://github.com/alloy-rs/core/releases/tag/v0.5.3) - 2023-12-16

### Bug Fixes

- [sol-types] Un-break decode revert ([#457](https://github.com/alloy-rs/core/issues/457))
- Ingest domain when instantiating TypedData ([#453](https://github.com/alloy-rs/core/issues/453))
- Don't decode ZSTs ([#454](https://github.com/alloy-rs/core/issues/454))
- [primitives] Return correct fixed length in ssz::Encode ([#451](https://github.com/alloy-rs/core/issues/451))

### Features

- Address from pubkey ([#455](https://github.com/alloy-rs/core/issues/455))
- Add `RevertReason` enum ([#450](https://github.com/alloy-rs/core/issues/450))
- [primitives] Update Bytes formatting, add UpperHex ([#446](https://github.com/alloy-rs/core/issues/446))

### Miscellaneous Tasks

- Release 0.5.3
- Bless tests ([#456](https://github.com/alloy-rs/core/issues/456))

## [0.5.2](https://github.com/alloy-rs/core/releases/tag/v0.5.2) - 2023-12-01

### Bug Fixes

- [dyn-abi] Fixed arrays coerce_str ([#442](https://github.com/alloy-rs/core/issues/442))

### Miscellaneous Tasks

- Release 0.5.2

### Testing

- Add some regression tests ([#443](https://github.com/alloy-rs/core/issues/443))

## [0.5.1](https://github.com/alloy-rs/core/releases/tag/v0.5.1) - 2023-11-30

### Bug Fixes

- Fix public leak ([#437](https://github.com/alloy-rs/core/issues/437))

### Miscellaneous Tasks

- Release 0.5.1

### Other

- Re-enable MSRV all features check ([#439](https://github.com/alloy-rs/core/issues/439))

## [0.5.0](https://github.com/alloy-rs/core/releases/tag/v0.5.0) - 2023-11-23

### Bug Fixes

- [sol-types] Many ABI coder fixes ([#434](https://github.com/alloy-rs/core/issues/434))
- [sol-types] ContractError decoding ([#430](https://github.com/alloy-rs/core/issues/430))
- [sol-macro] Handle outer attrs in abigen input ([#429](https://github.com/alloy-rs/core/issues/429))
- [sol-macro] Correctly print Custom types in parameters ([#425](https://github.com/alloy-rs/core/issues/425))
- [sol-types] Remove `SolType::ENCODED_SIZE` default ([#418](https://github.com/alloy-rs/core/issues/418))
- [syn-solidity] Raw keyword identifiers ([#415](https://github.com/alloy-rs/core/issues/415))
- Rust keyword conflict ([#405](https://github.com/alloy-rs/core/issues/405))
- Wrong as_u8 generated for enum ([#413](https://github.com/alloy-rs/core/issues/413))
- [dyn-abi] Correctly parse strings in `coerce_str` ([#410](https://github.com/alloy-rs/core/issues/410))
- [dyn-abi] Handle empty hex strings ([#400](https://github.com/alloy-rs/core/issues/400))
- [syn-solidity] Allow some duplicate attributes ([#399](https://github.com/alloy-rs/core/issues/399))
- [sol-type-parser] Normalize `u?int` to `u?int256` ([#397](https://github.com/alloy-rs/core/issues/397))
- Avoid symlinks ([#396](https://github.com/alloy-rs/core/issues/396))
- [primitives] Signed cleanup ([#395](https://github.com/alloy-rs/core/issues/395))
- Don't use directory symlinks ([#394](https://github.com/alloy-rs/core/issues/394))
- [sol-macro] Keep more attributes on contract module ([#391](https://github.com/alloy-rs/core/issues/391))
- [json-abi] `Param.ty` is not always a valid `TypeSpecifier` ([#386](https://github.com/alloy-rs/core/issues/386))
- [dyn-abi] Generate Int, Uint, FixedBytes adjusted to their size ([#384](https://github.com/alloy-rs/core/issues/384))
- [sol-types] `SolInterface::MIN_DATA_LENGTH` overflow ([#383](https://github.com/alloy-rs/core/issues/383))
- [docs] Switch incorrect function docs ([#374](https://github.com/alloy-rs/core/issues/374))
- [sol-macro] Bug fixes ([#372](https://github.com/alloy-rs/core/issues/372))
- [sol-macro] Correct `SolCall::abi_decode_returns` ([#367](https://github.com/alloy-rs/core/issues/367))
- [syn-solidity] Struct fields formatting ([#364](https://github.com/alloy-rs/core/issues/364))

### Features

- [primitives] Left and right padding conversions ([#424](https://github.com/alloy-rs/core/issues/424))
- [primitives] Improve utils ([#432](https://github.com/alloy-rs/core/issues/432))
- [sol-types] Add empty `bytes` and `string` specialization ([#435](https://github.com/alloy-rs/core/issues/435))
- [sol-macro] `SolEventInterface`: `SolInterface` for contract events enum ([#426](https://github.com/alloy-rs/core/issues/426))
- [sol-macro] Add `json-abi` item generation ([#422](https://github.com/alloy-rs/core/issues/422))
- Enable ruint ssz when primitives ssz ([#419](https://github.com/alloy-rs/core/issues/419))
- [json-abi] Permit keyword prefixes in HR parser ([#420](https://github.com/alloy-rs/core/issues/420))
- Added Hash to DynSolType and StructProp ([#411](https://github.com/alloy-rs/core/issues/411))
- [json-abi] Improve `JsonAbi::to_sol` ([#408](https://github.com/alloy-rs/core/issues/408))
- [sol-types] Add some more methods to `abi::Decoder` ([#404](https://github.com/alloy-rs/core/issues/404))
- [sol-macro] Add definition doc to structs and enums ([#393](https://github.com/alloy-rs/core/issues/393))
- [dyn-abi] `DynSolType::coerce_str` ([#380](https://github.com/alloy-rs/core/issues/380))

### Miscellaneous Tasks

- Release 0.5.0
- Update git-cliff config
- Restructure tests ([#421](https://github.com/alloy-rs/core/issues/421))
- Rename `TokenType` GAT and trait to `Token` ([#417](https://github.com/alloy-rs/core/issues/417))
- Remove dead code ([#416](https://github.com/alloy-rs/core/issues/416))
- Update .git-blame-ignore-revs
- Use winnow `separated` instead of `separated0` ([#403](https://github.com/alloy-rs/core/issues/403))
- Clean up ABI, EIP-712, docs ([#373](https://github.com/alloy-rs/core/issues/373))
- [sol-macro] Move generated docs below input attrs ([#363](https://github.com/alloy-rs/core/issues/363))
- [sol-types] Remove impls for isize/usize ([#362](https://github.com/alloy-rs/core/issues/362))

### Other

- SSZ implementation for alloy primitives ([#407](https://github.com/alloy-rs/core/issues/407))
- Enable rand feature for re-exported ruint crate ([#385](https://github.com/alloy-rs/core/issues/385))
- Cargo build instead of check ([#368](https://github.com/alloy-rs/core/issues/368))

### Styling

- Update rustfmt config ([#406](https://github.com/alloy-rs/core/issues/406))

### Testing

- Check version before running Solc ([#428](https://github.com/alloy-rs/core/issues/428))
- Add errors abi test ([#390](https://github.com/alloy-rs/core/issues/390))

## [0.4.2](https://github.com/alloy-rs/core/releases/tag/v0.4.2) - 2023-10-09

### Bug Fixes

- [primitives] Set serde derive feature ([#359](https://github.com/alloy-rs/core/issues/359))

### Miscellaneous Tasks

- Release 0.4.2

## [0.4.1](https://github.com/alloy-rs/core/releases/tag/v0.4.1) - 2023-10-09

### Bug Fixes

- [sol-macro] Flatten doc strings correctly ([#357](https://github.com/alloy-rs/core/issues/357))
- [json-abi] Fallback to tuple types for nested params in `to_sol` ([#354](https://github.com/alloy-rs/core/issues/354))
- [sol-macro] Correct `TypeArray::is_abi_dynamic` ([#353](https://github.com/alloy-rs/core/issues/353))
- [sol-macro] Dedup json abi items ([#346](https://github.com/alloy-rs/core/issues/346))
- Json-abi not using anonymous when converting to interface ([#342](https://github.com/alloy-rs/core/issues/342))
- [sol-macro] Remove extra 0x in function docs ([#341](https://github.com/alloy-rs/core/issues/341))
- [sol-macro] Pass attributes to all generated items ([#340](https://github.com/alloy-rs/core/issues/340))
- [syn-solidity] Set spans on generated struct names ([#336](https://github.com/alloy-rs/core/issues/336))
- Serde rename resolver to types ([#335](https://github.com/alloy-rs/core/issues/335))

### Documentation

- Add scope to changelog commits ([#328](https://github.com/alloy-rs/core/issues/328))
- Fix changelog link ([#323](https://github.com/alloy-rs/core/issues/323))

### Features

- [sol-macro] Add docs to generated contract modules ([#356](https://github.com/alloy-rs/core/issues/356))
- [json-abi] Deserialize more ContractObjects ([#348](https://github.com/alloy-rs/core/issues/348))
- [sol-macro] Improve error messages ([#345](https://github.com/alloy-rs/core/issues/345))
- [sol-types] Introduce `SolValue`, make `Encodable` an impl detail ([#333](https://github.com/alloy-rs/core/issues/333))
- [syn-solidity] Add even more Display impls ([#339](https://github.com/alloy-rs/core/issues/339))
- [sol-macro] Improve generated docs ([#338](https://github.com/alloy-rs/core/issues/338))
- [syn-solidity] Add some more Display impls ([#337](https://github.com/alloy-rs/core/issues/337))
- Add parsing support for JSON items ([#329](https://github.com/alloy-rs/core/issues/329))
- Add logs, add log dynamic decoding ([#271](https://github.com/alloy-rs/core/issues/271))

### Miscellaneous Tasks

- Release 0.4.1
- [sol-types] Rewrite encodable impl generics ([#332](https://github.com/alloy-rs/core/issues/332))
- Add count to all_the_tuples! macro ([#331](https://github.com/alloy-rs/core/issues/331))
- Enable ruint std feature ([#326](https://github.com/alloy-rs/core/issues/326))
- Fix typos ([#325](https://github.com/alloy-rs/core/issues/325))
- [dyn-abi] Make `resolve` module private ([#324](https://github.com/alloy-rs/core/issues/324))

### Other

- Run miri in ci ([#327](https://github.com/alloy-rs/core/issues/327))

### Testing

- Add regression test for [#351](https://github.com/alloy-rs/core/issues/351) ([#355](https://github.com/alloy-rs/core/issues/355))

## [0.4.0](https://github.com/alloy-rs/core/releases/tag/v0.4.0) - 2023-09-29

### Bug Fixes

- [syn-solidity] Test
- [sol-macro] Implement EventTopic for generated enums ([#320](https://github.com/alloy-rs/core/issues/320))
- Add super import on generated modules ([#307](https://github.com/alloy-rs/core/issues/307))
- Respect `all_derives = false`, fix custom type printing ([#272](https://github.com/alloy-rs/core/issues/272))
- Rand default-features typo ([#286](https://github.com/alloy-rs/core/issues/286))
- [syn-solidity] Parse modifiers without parens ([#284](https://github.com/alloy-rs/core/issues/284))
- Struct `eip712_data_word` ([#258](https://github.com/alloy-rs/core/issues/258))
- [syn-solidity] Imports ([#252](https://github.com/alloy-rs/core/issues/252))
- MSRV tests ([#246](https://github.com/alloy-rs/core/issues/246))
- Hex compatibility ([#244](https://github.com/alloy-rs/core/issues/244))

### Dependencies

- Bump all deps ([#273](https://github.com/alloy-rs/core/issues/273))
- Fix MSRV CI and dev deps ([#267](https://github.com/alloy-rs/core/issues/267))

### Documentation

- Add automated CHANGELOG.md ([#322](https://github.com/alloy-rs/core/issues/322))
- Improve `ResolveSolType` documentation ([#296](https://github.com/alloy-rs/core/issues/296))
- Document dollar sign in idents ([#288](https://github.com/alloy-rs/core/issues/288))
- Add note regarding ruint::uint macro ([#265](https://github.com/alloy-rs/core/issues/265))
- Update fixed bytes docs ([#255](https://github.com/alloy-rs/core/issues/255))
- Data types typo ([#248](https://github.com/alloy-rs/core/issues/248))

### Features

- [sol-macro] Add docs to generated items ([#321](https://github.com/alloy-rs/core/issues/321))
- [sol-macro] Add support for overloaded events ([#318](https://github.com/alloy-rs/core/issues/318))
- [syn-solidity] Added visitor hooks for all statements and expressions ([#314](https://github.com/alloy-rs/core/issues/314))
- [sol-macro] Improve type expansion ([#302](https://github.com/alloy-rs/core/issues/302))
- [syn-solidity] Add more `Spanned` impls ([#301](https://github.com/alloy-rs/core/issues/301))
- Unsupported message for $idents ([#293](https://github.com/alloy-rs/core/issues/293))
- [json-abi] Add `Function::signature_full` ([#289](https://github.com/alloy-rs/core/issues/289))
- [primitives] Add more methods to `Function` ([#290](https://github.com/alloy-rs/core/issues/290))
- Improve `SolError`, `SolInterface` structs and implementations ([#285](https://github.com/alloy-rs/core/issues/285))
- Add more `FixedBytes` to int conversion impls ([#281](https://github.com/alloy-rs/core/issues/281))
- Add support for `rand` ([#282](https://github.com/alloy-rs/core/issues/282))
- Use `FixedBytes` for `sol_data::FixedBytes` ([#276](https://github.com/alloy-rs/core/issues/276))
- Impl `bytes::Buf` for our own `Bytes` ([#279](https://github.com/alloy-rs/core/issues/279))
- Add more `Bytes` conversion impls ([#280](https://github.com/alloy-rs/core/issues/280))
- [primitives] Improve Bytes ([#269](https://github.com/alloy-rs/core/issues/269))
- [sol-macro] Expand getter functions' return types ([#262](https://github.com/alloy-rs/core/issues/262))
- Add attributes to enum variants ([#264](https://github.com/alloy-rs/core/issues/264))
- [sol-macro] Expand fields with attrs ([#263](https://github.com/alloy-rs/core/issues/263))
- [syn-solidity] Improve variable getters generation ([#260](https://github.com/alloy-rs/core/issues/260))
- [dyn-abi] Implement more ext traits for json-abi ([#243](https://github.com/alloy-rs/core/issues/243))
- [sol-macro] Add opt-in attributes for extra methods and derives ([#250](https://github.com/alloy-rs/core/issues/250))
- [primitives] Allow empty input in hex macros ([#245](https://github.com/alloy-rs/core/issues/245))

### Miscellaneous Tasks

- Release 0.4.0
- Prefix ABI encode and decode functions with `abi_` ([#311](https://github.com/alloy-rs/core/issues/311))
- Don't pass debug feature to winnow ([#317](https://github.com/alloy-rs/core/issues/317))
- Touch up [#314](https://github.com/alloy-rs/core/issues/314) ([#315](https://github.com/alloy-rs/core/issues/315))
- Simpler ENCODED_SIZE for SolType tuples ([#312](https://github.com/alloy-rs/core/issues/312))
- Unhide clippy config file ([#305](https://github.com/alloy-rs/core/issues/305))
- Sync crate level attributes ([#303](https://github.com/alloy-rs/core/issues/303))
- Assert_eq! on Ok instead of unwrapping where possible ([#297](https://github.com/alloy-rs/core/issues/297))
- Use `hex!` macro from `primitives` re-export ([#299](https://github.com/alloy-rs/core/issues/299))
- Add missing `#[automatically_derived]` ([#294](https://github.com/alloy-rs/core/issues/294))
- Do not implement SolType for SolStruct generically ([#275](https://github.com/alloy-rs/core/issues/275))
- Rename coding functions ([#274](https://github.com/alloy-rs/core/issues/274))
- Re-export ::bytes ([#278](https://github.com/alloy-rs/core/issues/278))
- Update CODEOWNERS ([#270](https://github.com/alloy-rs/core/issues/270))

### Other

- Cache on failure ([#306](https://github.com/alloy-rs/core/issues/306))
- Hash_message ([#304](https://github.com/alloy-rs/core/issues/304))
- Pin anstyle to 1.65 compat ([#266](https://github.com/alloy-rs/core/issues/266))
- Typo ([#249](https://github.com/alloy-rs/core/issues/249))

### Performance

- Optimize identifier parsing ([#295](https://github.com/alloy-rs/core/issues/295))
- Use `slice::Iter` where possible ([#256](https://github.com/alloy-rs/core/issues/256))

### Refactor

- Rewrite type parser with `winnow` ([#292](https://github.com/alloy-rs/core/issues/292))
- Simplify `Eip712Domain::encode_data` ([#277](https://github.com/alloy-rs/core/issues/277))

### Styling

- Format code snippets in docs ([#313](https://github.com/alloy-rs/core/issues/313))
- Move `decode_revert_reason` to alloy and add tests ([#308](https://github.com/alloy-rs/core/issues/308))
- Support yul ast  ([#268](https://github.com/alloy-rs/core/issues/268))
- Some clippy lints ([#251](https://github.com/alloy-rs/core/issues/251))

### Testing

- [syn-solidity] Improve contract tests ([#316](https://github.com/alloy-rs/core/issues/316))

## [0.3.2](https://github.com/alloy-rs/core/releases/tag/v0.3.2) - 2023-08-23

### Bug Fixes

- [json-abi] Properly handle Param `type` field ([#233](https://github.com/alloy-rs/core/issues/233))
- [sol-macro] Snake_case'd function names ([#226](https://github.com/alloy-rs/core/issues/226))
- Fix bincode serialization ([#223](https://github.com/alloy-rs/core/issues/223))
- [sol-macro] Encode UDVTs as their underlying type in EIP-712 ([#220](https://github.com/alloy-rs/core/issues/220))
- [sol-macro] Don't panic when encountering functions without names ([#217](https://github.com/alloy-rs/core/issues/217))

### Features

- Implement abi2sol ([#228](https://github.com/alloy-rs/core/issues/228))
- [primitives] More `FixedBytes<N>` <-> `[u8; N]` conversions ([#239](https://github.com/alloy-rs/core/issues/239))
- Add support for function input/output encoding/decoding ([#227](https://github.com/alloy-rs/core/issues/227))
- [syn-solidity] Add statements and expressions ([#199](https://github.com/alloy-rs/core/issues/199))
- [dyn-abi] Add match functions to value and doc aliases ([#234](https://github.com/alloy-rs/core/issues/234))
- Function type ([#224](https://github.com/alloy-rs/core/issues/224))
- [dyn-abi] Allow `T: Into<Cow<str>>` in `eip712_domain!` ([#222](https://github.com/alloy-rs/core/issues/222))
- [sol-macro] Expand getter functions for public state variables ([#218](https://github.com/alloy-rs/core/issues/218))

### Miscellaneous Tasks

- Release 0.3.2 ([#242](https://github.com/alloy-rs/core/issues/242))
- [primitives] Discourage use of `B160` ([#235](https://github.com/alloy-rs/core/issues/235))
- [json-abi] Avoid unsafe, remove unused generics ([#229](https://github.com/alloy-rs/core/issues/229))
- Clippy ([#225](https://github.com/alloy-rs/core/issues/225))

### Performance

- Optimize some stuff ([#231](https://github.com/alloy-rs/core/issues/231))
- Refactor TypeSpecifier parsing ([#230](https://github.com/alloy-rs/core/issues/230))

### Styling

- Port ethabi json tests ([#232](https://github.com/alloy-rs/core/issues/232))

## [0.3.1](https://github.com/alloy-rs/core/releases/tag/v0.3.1) - 2023-07-30

### Dependencies

- Bump ruint to 1.10.1 + alloc ([#213](https://github.com/alloy-rs/core/issues/213))

### Documentation

- Update no-std not in readme ([#215](https://github.com/alloy-rs/core/issues/215))
- Add ambiguity details to Encodable rustdoc ([#211](https://github.com/alloy-rs/core/issues/211))
- [json-abi] Add README.md ([#209](https://github.com/alloy-rs/core/issues/209))
- Update README.md ([#208](https://github.com/alloy-rs/core/issues/208))

### Features

- Support `ethabi` Contract methods ([#195](https://github.com/alloy-rs/core/issues/195))

### Miscellaneous Tasks

- Release 0.3.1 ([#216](https://github.com/alloy-rs/core/issues/216))

## [0.3.0](https://github.com/alloy-rs/core/releases/tag/v0.3.0) - 2023-07-26

### Bug Fixes

- Correct encodeType expansion for nested structs ([#203](https://github.com/alloy-rs/core/issues/203))
- Remove unused method body on solstruct ([#200](https://github.com/alloy-rs/core/issues/200))
- Remove unwrap in decode_populate ([#172](https://github.com/alloy-rs/core/issues/172))
- [sol-types] Empty data decode ([#159](https://github.com/alloy-rs/core/issues/159))
- Doc in dyn-abi ([#155](https://github.com/alloy-rs/core/issues/155))
- [alloy-primitives] Fix broken documentation link ([#152](https://github.com/alloy-rs/core/issues/152))

### Documentation

- Add licensing note to README.md ([#186](https://github.com/alloy-rs/core/issues/186))
- Add parser to readme ([#183](https://github.com/alloy-rs/core/issues/183))
- [rlp] Move example to README.md ([#177](https://github.com/alloy-rs/core/issues/177))
- Request that PR contributors allow maintainer edits ([#148](https://github.com/alloy-rs/core/issues/148))

### Features

- Bytes handles numeric arrays and bytearrays in deser ([#202](https://github.com/alloy-rs/core/issues/202))
- [dyb-abi] Impl ResolveSolType for Rc ([#189](https://github.com/alloy-rs/core/issues/189))
- Native keccak feature flag ([#185](https://github.com/alloy-rs/core/issues/185))
- [sol-macro] `#[sol]` attributes and JSON ABI support ([#173](https://github.com/alloy-rs/core/issues/173))
- Solidity type parser ([#181](https://github.com/alloy-rs/core/issues/181))
- [rlp] Improve implementations ([#182](https://github.com/alloy-rs/core/issues/182))
- [dyn-abi] Add arbitrary impls and proptests ([#175](https://github.com/alloy-rs/core/issues/175))
- [dyn-abi] Cfg CustomStruct for eip712, rm CustomValue ([#178](https://github.com/alloy-rs/core/issues/178))
- [dyn-abi] Clean up and improve performance ([#174](https://github.com/alloy-rs/core/issues/174))
- DynSolType::decode_params ([#166](https://github.com/alloy-rs/core/issues/166))
- [json-abi] Add more impls ([#164](https://github.com/alloy-rs/core/issues/164))
- [primitives] Add some impls ([#162](https://github.com/alloy-rs/core/issues/162))
- `SolEnum` and `SolInterface` ([#153](https://github.com/alloy-rs/core/issues/153))
- [primitives] Fixed bytes macros ([#156](https://github.com/alloy-rs/core/issues/156))

### Miscellaneous Tasks

- Release 0.3.0 ([#207](https://github.com/alloy-rs/core/issues/207))
- Wrap Bytes methods which return Self ([#206](https://github.com/alloy-rs/core/issues/206))
- Add release.toml ([#205](https://github.com/alloy-rs/core/issues/205))
- Replace `ruint2` with `ruint` ([#192](https://github.com/alloy-rs/core/issues/192))
- Clippy ([#196](https://github.com/alloy-rs/core/issues/196))
- Remove remaining refs to rlp ([#190](https://github.com/alloy-rs/core/issues/190))
- Move rlp crates to a separate repo ([#187](https://github.com/alloy-rs/core/issues/187))
- [dyn-abi] Gate eip712 behind a feature ([#176](https://github.com/alloy-rs/core/issues/176))
- Warn on all rustdoc lints ([#154](https://github.com/alloy-rs/core/issues/154))
- Clean ups ([#150](https://github.com/alloy-rs/core/issues/150))
- Add smaller image for favicon ([#142](https://github.com/alloy-rs/core/issues/142))
- Move macro doctests to separate folder ([#140](https://github.com/alloy-rs/core/issues/140))

### Other

- Cache wasm job ([#197](https://github.com/alloy-rs/core/issues/197))
- Significant dyn-abi fixes :) ([#168](https://github.com/alloy-rs/core/issues/168))
- Kuly14/cleanup ([#151](https://github.com/alloy-rs/core/issues/151))
- Explain alloy vs ethers-rs intention ([#146](https://github.com/alloy-rs/core/issues/146))

### Refactor

- Refactoring `dyn-abi` to performance parity with ethabi ([#144](https://github.com/alloy-rs/core/issues/144))
- Rename domain macro and add docs ([#147](https://github.com/alloy-rs/core/issues/147))
- Rename Sol*::Tuple to Parameters/Arguments  ([#145](https://github.com/alloy-rs/core/issues/145))
- Do not generate SolCall for return values ([#134](https://github.com/alloy-rs/core/issues/134))

### Testing

- Run UI tests only on nightly ([#194](https://github.com/alloy-rs/core/issues/194))

## [0.2.0](https://github.com/alloy-rs/core/releases/tag/v0.2.0) - 2023-06-23

### Bug Fixes

- Remove to_rust from most traits ([#133](https://github.com/alloy-rs/core/issues/133))
- Fmt ([#130](https://github.com/alloy-rs/core/issues/130))
- Links in readme ([#128](https://github.com/alloy-rs/core/issues/128))
- (u)int tokenization ([#123](https://github.com/alloy-rs/core/issues/123))
- Add `repr(C)` to json-abi items ([#100](https://github.com/alloy-rs/core/issues/100))
- Make detokenize infallible ([#86](https://github.com/alloy-rs/core/issues/86))
- Extra-traits in syn-solidity ([#65](https://github.com/alloy-rs/core/issues/65))
- Rlp impls ([#56](https://github.com/alloy-rs/core/issues/56))
- Hex breaking change ([#50](https://github.com/alloy-rs/core/issues/50))
- Type check int for dirty high bytes ([#47](https://github.com/alloy-rs/core/issues/47))
- Sol macro parsing and expansion ([#21](https://github.com/alloy-rs/core/issues/21))
- Add alloc features in no_std ([#18](https://github.com/alloy-rs/core/issues/18))
- Bump resolver to 2 to disable proptest in wasm
- Doc warnings and clippy
- Cargo t
- Desc in primitives cargo.toml
- Handle nested arrays
- Correct signed int handling in encodePacked
- Correct int handling in encodePacked
- Add missing type_check to decoding
- Std in abi lol
- No_std in abi

### Dependencies

- Bump ruint to have alloy-rlp
- Add missing deny.toml ([#23](https://github.com/alloy-rs/core/issues/23))
- Add Address w/ checksum support to `primitives` ([#19](https://github.com/alloy-rs/core/issues/19))
- Use workspace.{package,dependencies} ([#17](https://github.com/alloy-rs/core/issues/17))
- Bump uint
- Bump uint to support wasm
- Bump uint main
- Generic signed int implementation ([#3](https://github.com/alloy-rs/core/issues/3))

### Documentation

- Rlp-derive README.md ([#70](https://github.com/alloy-rs/core/issues/70))
- Contributing doc ([#49](https://github.com/alloy-rs/core/issues/49))
- Note on no_std support ([#44](https://github.com/alloy-rs/core/issues/44))
- Note that encode_list is preferred
- Main lib README ([#34](https://github.com/alloy-rs/core/issues/34))
- Brief doc on the type system ([#26](https://github.com/alloy-rs/core/issues/26))
- Improve abi encoding doc examples
- Encode_packed_to in doctest
- Remove extra tab in docstring
- Add implementer's guide to SolType
- Big lib front page :)

### Features

- Unify json-abi params impls ([#136](https://github.com/alloy-rs/core/issues/136))
- Add `Encodable` trait ([#121](https://github.com/alloy-rs/core/issues/121))
- Finish high-level Solidity parser ([#119](https://github.com/alloy-rs/core/issues/119))
- Improve SolType tuples ([#115](https://github.com/alloy-rs/core/issues/115))
- Make `TokenType::is_dynamic` a constant ([#114](https://github.com/alloy-rs/core/issues/114))
- More FixedBytes impls ([#111](https://github.com/alloy-rs/core/issues/111))
- Compute encoded size statically where possible ([#105](https://github.com/alloy-rs/core/issues/105))
- Json-abi event selector ([#104](https://github.com/alloy-rs/core/issues/104))
- Solidity events support ([#83](https://github.com/alloy-rs/core/issues/83))
- Issue and PR templates [#33](https://github.com/alloy-rs/core/issues/33) ([#93](https://github.com/alloy-rs/core/issues/93))
- `sol!` contracts ([#77](https://github.com/alloy-rs/core/issues/77))
- Abi-json crate ([#78](https://github.com/alloy-rs/core/issues/78))
- Syn-solidity visitors ([#68](https://github.com/alloy-rs/core/issues/68))
- Abi benchmarks ([#57](https://github.com/alloy-rs/core/issues/57))
- Move Solidity syn AST to `syn-solidity` ([#63](https://github.com/alloy-rs/core/issues/63))
- Support function overloading in `sol!` ([#53](https://github.com/alloy-rs/core/issues/53))
- Primitive utils and improvements ([#52](https://github.com/alloy-rs/core/issues/52))
- Add PanicKind enum ([#54](https://github.com/alloy-rs/core/issues/54))
- [sol-type-parser] Parse and expand custom errors and functions ([#24](https://github.com/alloy-rs/core/issues/24))
- Standard solidity revert & panic ([#28](https://github.com/alloy-rs/core/issues/28))
- Use `const-hex` instead of `hex` ([#25](https://github.com/alloy-rs/core/issues/25))
- Improve macros ([#7](https://github.com/alloy-rs/core/issues/7))
- Encode_eip712
- Sol proc_macro for UDTs
- User-defined solidity type
- Domain macro
- Update sol type parser to simplify access
- Borrow for primitive bits types
- Eip712 scaffolding
- Borrow abstraction for tokenization and encoding
- Serde for signed integer
- Dyn sol type for all I sizes
- Add uint as submodule ([#1](https://github.com/alloy-rs/core/issues/1))
- Add uint crate to repository
- Feature structs in sol parser
- Untested encodePacked
- Sol proc macro
- More expressive errors in ABI
- Encoder rewrite
- Abi, primitives, rlp

### Miscellaneous Tasks

- Add logo to all crates, add @gakonst to CODEOWNERS ([#138](https://github.com/alloy-rs/core/issues/138))
- Add .gitattributes ([#135](https://github.com/alloy-rs/core/issues/135))
- Typos ([#132](https://github.com/alloy-rs/core/issues/132))
- Typo fix ([#131](https://github.com/alloy-rs/core/issues/131))
- Typo fix ([#129](https://github.com/alloy-rs/core/issues/129))
- Clean up features ([#116](https://github.com/alloy-rs/core/issues/116))
- Add CODEOWNERS, update deny.toml and ci.yml ([#117](https://github.com/alloy-rs/core/issues/117))
- S/ruint/ruint2 until remco is back
- Feature-gate `getrandom`, document in README.md ([#71](https://github.com/alloy-rs/core/issues/71))
- Rename to Alloy ([#69](https://github.com/alloy-rs/core/issues/69))
- Enable `feature(doc_cfg, doc_auto_cfg)` ([#67](https://github.com/alloy-rs/core/issues/67))
- Remove syn "full" feature ([#66](https://github.com/alloy-rs/core/issues/66))
- Rename crates ([#45](https://github.com/alloy-rs/core/issues/45))
- Pre-release mega cleanup ([#35](https://github.com/alloy-rs/core/issues/35))
- Use crates.io uint, move crates to `crates/*` ([#31](https://github.com/alloy-rs/core/issues/31))
- Update error type ([#22](https://github.com/alloy-rs/core/issues/22))
- Readme build commands
- Add another todo for @gakonst
- More todo in readme, remove cargo.toml comment
- Update readme todos
- Remove dbgs
- Unused imports in test
- Docstrings and tests
- Missing RLP docs

### Other

- Release 0.2.0 ([#139](https://github.com/alloy-rs/core/issues/139))
- Revert "test: bless tests after updating to syn 2.0.19 ([#79](https://github.com/alloy-rs/core/issues/79))" ([#80](https://github.com/alloy-rs/core/issues/80))
- Add WASM job ([#76](https://github.com/alloy-rs/core/issues/76))
- Fix dep job, add feature-checks job ([#64](https://github.com/alloy-rs/core/issues/64))
- Fix rustdoc job, docs ([#46](https://github.com/alloy-rs/core/issues/46))
- Prestwich/crate readmes ([#41](https://github.com/alloy-rs/core/issues/41))
- Prestwich/ingest encode type ([#15](https://github.com/alloy-rs/core/issues/15))
- Add initial Continuous Integration Workflows using GitHub® Actions™   ([#8](https://github.com/alloy-rs/core/issues/8))
- Dynamic EIP-712 ([#6](https://github.com/alloy-rs/core/issues/6))
- Prestwich/int-edge-cases ([#4](https://github.com/alloy-rs/core/issues/4))
- Implement ABI for I256 ([#5](https://github.com/alloy-rs/core/issues/5))
- Prestwich/dyn enc ([#2](https://github.com/alloy-rs/core/issues/2))
- Simplify encode_params docs
- Delete type aliases
- Remove primitive-types dep, add from for B512
- Naming in readme
- Standardize on mod std_support
- のこりなくちるぞめでたき桜花ありて世の中はてのうければ

### Performance

- Improve rlp, update Address methods ([#118](https://github.com/alloy-rs/core/issues/118))

### Refactor

- Lifetimes for token types ([#120](https://github.com/alloy-rs/core/issues/120))
- Sol-macro expansion ([#113](https://github.com/alloy-rs/core/issues/113))
- Change is_dynamic to a const DYNAMIC ([#99](https://github.com/alloy-rs/core/issues/99))
- Implement `SolType` for `{Ui,I}nt<N>` and `FixedBytes<N>` with const-generics ([#92](https://github.com/alloy-rs/core/issues/92))
- `sol!` AST and macro expansion ([#61](https://github.com/alloy-rs/core/issues/61))
- Remerge SolType and SolDataType ([#30](https://github.com/alloy-rs/core/issues/30))
- Clean up abi crate structure
- Split dyn-abi into a separate crate
- Udt doesn't need phantomdata
- Simplify types somewhat
- Modularize signed int implementation
- Remove unused error variant
- Delete remainder of serde feature
- Trim error type
- Abi standardize on encode+single+params

### Styling

- Add fmt commit to .git-blame-ignore-revs ([#43](https://github.com/alloy-rs/core/issues/43))
- Add rustfmt.toml ([#42](https://github.com/alloy-rs/core/issues/42))
- Sol Type re-factoring ([#20](https://github.com/alloy-rs/core/issues/20))

### Testing

- Add more json abi tests ([#89](https://github.com/alloy-rs/core/issues/89))
- Bless tests after updating to syn 2.0.19 ([#79](https://github.com/alloy-rs/core/issues/79))
- Change should_panic to catch_unwind
- Test U1 sub
- Add custom sol structs to tests and docs
- Clean up some type check and add a couple new ones

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
