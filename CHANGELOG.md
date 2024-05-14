# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.3](https://github.com/alloy-rs/core/releases/tag/v0.7.3) - 2024-05-14

### Miscellaneous Tasks

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
- Refactoring `dyn-abi` to performance parity with ethabi ([#144](https://github.com/alloy-rs/core/issues/144))
- Kuly14/cleanup ([#151](https://github.com/alloy-rs/core/issues/151))
- Explain alloy vs ethers-rs intention ([#146](https://github.com/alloy-rs/core/issues/146))

### Refactor

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
