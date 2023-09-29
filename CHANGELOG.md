# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/alloy-rs/core/compare/v0.3.2...HEAD)

### Bug Fixes

- Implement EventTopic for generated enums ([#320](https://github.com/alloy-rs/core/issues/320))
- Add super import on generated modules ([#307](https://github.com/alloy-rs/core/issues/307))
- Respect `all_derives = false`, fix custom type printing ([#272](https://github.com/alloy-rs/core/issues/272))
- Rand default-features typo ([#286](https://github.com/alloy-rs/core/issues/286))
- Parse modifiers without parens ([#284](https://github.com/alloy-rs/core/issues/284))
- Struct `eip712_data_word` ([#258](https://github.com/alloy-rs/core/issues/258))
- Imports ([#252](https://github.com/alloy-rs/core/issues/252))
- MSRV tests ([#246](https://github.com/alloy-rs/core/issues/246))
- Hex compatibility ([#244](https://github.com/alloy-rs/core/issues/244))

### Dependencies

- Bump all deps ([#273](https://github.com/alloy-rs/core/issues/273))
- Fix MSRV CI and dev deps ([#267](https://github.com/alloy-rs/core/issues/267))

### Documentation

- Add automated CHANGELOG.md
- Improve `ResolveSolType` documentation ([#296](https://github.com/alloy-rs/core/issues/296))
- Document dollar sign in idents ([#288](https://github.com/alloy-rs/core/issues/288))
- Add note regarding ruint::uint macro ([#265](https://github.com/alloy-rs/core/issues/265))
- Update fixed bytes docs ([#255](https://github.com/alloy-rs/core/issues/255))
- Data types typo ([#248](https://github.com/alloy-rs/core/issues/248))

### Features

- Add docs to generated items ([#321](https://github.com/alloy-rs/core/issues/321))
- Add support for overloaded events ([#318](https://github.com/alloy-rs/core/issues/318))
- Added visitor hooks for all statements and expressions ([#314](https://github.com/alloy-rs/core/issues/314))
- Improve type expansion ([#302](https://github.com/alloy-rs/core/issues/302))
- Add more `Spanned` impls ([#301](https://github.com/alloy-rs/core/issues/301))
- Unsupported message for $idents ([#293](https://github.com/alloy-rs/core/issues/293))
- Add `Function::signature_full` ([#289](https://github.com/alloy-rs/core/issues/289))
- Add more methods to `Function` ([#290](https://github.com/alloy-rs/core/issues/290))
- Improve `SolError`, `SolInterface` structs and implementations ([#285](https://github.com/alloy-rs/core/issues/285))
- Add more `FixedBytes` to int conversion impls ([#281](https://github.com/alloy-rs/core/issues/281))
- Add support for `rand` ([#282](https://github.com/alloy-rs/core/issues/282))
- Use `FixedBytes` for `sol_data::FixedBytes` ([#276](https://github.com/alloy-rs/core/issues/276))
- Impl `bytes::Buf` for our own `Bytes` ([#279](https://github.com/alloy-rs/core/issues/279))
- Add more `Bytes` conversion impls ([#280](https://github.com/alloy-rs/core/issues/280))
- Improve Bytes ([#269](https://github.com/alloy-rs/core/issues/269))
- Expand getter functions' return types ([#262](https://github.com/alloy-rs/core/issues/262))
- Add attributes to enum variants ([#264](https://github.com/alloy-rs/core/issues/264))
- Expand fields with attrs ([#263](https://github.com/alloy-rs/core/issues/263))
- Improve variable getters generation ([#260](https://github.com/alloy-rs/core/issues/260))
- Implement more ext traits for json-abi ([#243](https://github.com/alloy-rs/core/issues/243))
- Add opt-in attributes for extra methods and derives ([#250](https://github.com/alloy-rs/core/issues/250))
- Allow empty input in hex macros ([#245](https://github.com/alloy-rs/core/issues/245))

### Miscellaneous Tasks

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

- Improve contract tests ([#316](https://github.com/alloy-rs/core/issues/316))

## [0.3.2](https://github.com/alloy-rs/core/releases/tag/v0.3.2) - 2023-08-23

### Bug Fixes

- Properly handle Param `type` field ([#233](https://github.com/alloy-rs/core/issues/233))
- Snake_case'd function names ([#226](https://github.com/alloy-rs/core/issues/226))
- Fix bincode serialization ([#223](https://github.com/alloy-rs/core/issues/223))
- Encode UDVTs as their underlying type in EIP-712 ([#220](https://github.com/alloy-rs/core/issues/220))
- Don't panic when encountering functions without names ([#217](https://github.com/alloy-rs/core/issues/217))

### Features

- Implement abi2sol ([#228](https://github.com/alloy-rs/core/issues/228))
- More `FixedBytes<N>` <-> `[u8; N]` conversions ([#239](https://github.com/alloy-rs/core/issues/239))
- Add support for function input/output encoding/decoding ([#227](https://github.com/alloy-rs/core/issues/227))
- Add statements and expressions ([#199](https://github.com/alloy-rs/core/issues/199))
- Add match functions to value and doc aliases ([#234](https://github.com/alloy-rs/core/issues/234))
- Function type ([#224](https://github.com/alloy-rs/core/issues/224))
- Allow `T: Into<Cow<str>>` in `eip712_domain!` ([#222](https://github.com/alloy-rs/core/issues/222))
- Expand getter functions for public state variables ([#218](https://github.com/alloy-rs/core/issues/218))

### Miscellaneous Tasks

- Release 0.3.2 ([#242](https://github.com/alloy-rs/core/issues/242))
- Discourage use of `B160` ([#235](https://github.com/alloy-rs/core/issues/235))
- Avoid unsafe, remove unused generics ([#229](https://github.com/alloy-rs/core/issues/229))
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
- Add README.md ([#209](https://github.com/alloy-rs/core/issues/209))
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
- Empty data decode ([#159](https://github.com/alloy-rs/core/issues/159))
- Doc in dyn-abi ([#155](https://github.com/alloy-rs/core/issues/155))
- Fix broken documentation link ([#152](https://github.com/alloy-rs/core/issues/152))

### Documentation

- Add licensing note to README.md ([#186](https://github.com/alloy-rs/core/issues/186))
- Add parser to readme ([#183](https://github.com/alloy-rs/core/issues/183))
- Move example to README.md ([#177](https://github.com/alloy-rs/core/issues/177))
- Request that PR contributors allow maintainer edits ([#148](https://github.com/alloy-rs/core/issues/148))

### Features

- Bytes handles numeric arrays and bytearrays in deser ([#202](https://github.com/alloy-rs/core/issues/202))
- Impl ResolveSolType for Rc ([#189](https://github.com/alloy-rs/core/issues/189))
- Native keccak feature flag ([#185](https://github.com/alloy-rs/core/issues/185))
- `#[sol]` attributes and JSON ABI support ([#173](https://github.com/alloy-rs/core/issues/173))
- Solidity type parser ([#181](https://github.com/alloy-rs/core/issues/181))
- Improve implementations ([#182](https://github.com/alloy-rs/core/issues/182))
- Add arbitrary impls and proptests ([#175](https://github.com/alloy-rs/core/issues/175))
- Cfg CustomStruct for eip712, rm CustomValue ([#178](https://github.com/alloy-rs/core/issues/178))
- Clean up and improve performance ([#174](https://github.com/alloy-rs/core/issues/174))
- DynSolType::decode_params ([#166](https://github.com/alloy-rs/core/issues/166))
- Add more impls ([#164](https://github.com/alloy-rs/core/issues/164))
- Add some impls ([#162](https://github.com/alloy-rs/core/issues/162))
- `SolEnum` and `SolInterface` ([#153](https://github.com/alloy-rs/core/issues/153))
- Fixed bytes macros ([#156](https://github.com/alloy-rs/core/issues/156))

### Miscellaneous Tasks

- Release 0.3.0 ([#207](https://github.com/alloy-rs/core/issues/207))
- Wrap Bytes methods which return Self ([#206](https://github.com/alloy-rs/core/issues/206))
- Add release.toml ([#205](https://github.com/alloy-rs/core/issues/205))
- Replace `ruint2` with `ruint` ([#192](https://github.com/alloy-rs/core/issues/192))
- Clippy ([#196](https://github.com/alloy-rs/core/issues/196))
- Remove remaining refs to rlp ([#190](https://github.com/alloy-rs/core/issues/190))
- Move rlp crates to a separate repo ([#187](https://github.com/alloy-rs/core/issues/187))
- Gate eip712 behind a feature ([#176](https://github.com/alloy-rs/core/issues/176))
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
- Parse and expand custom errors and functions ([#24](https://github.com/alloy-rs/core/issues/24))
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

<!-- generated by git-cliff -->
