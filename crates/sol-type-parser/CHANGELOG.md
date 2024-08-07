# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.7](https://github.com/alloy-rs/core/releases/tag/v0.7.7) - 2024-07-08

### Documentation

- Add per-crate changelogs ([#669](https://github.com/alloy-rs/core/issues/669))

### Features

- Add support for parsing visibility and state mutability ([#682](https://github.com/alloy-rs/core/issues/682))

### Miscellaneous Tasks

- Use workspace.lints ([#676](https://github.com/alloy-rs/core/issues/676))

### Styling

- Sort derives ([#662](https://github.com/alloy-rs/core/issues/662))

## [0.7.5](https://github.com/alloy-rs/core/releases/tag/v0.7.5) - 2024-06-04

### Features

- Create new method on Param and EventParam ([#634](https://github.com/alloy-rs/core/issues/634))

## [0.6.4](https://github.com/alloy-rs/core/releases/tag/v0.6.4) - 2024-02-29

### Bug Fixes

- [dyn-abi] Correctly parse empty lists of bytes ([#548](https://github.com/alloy-rs/core/issues/548))

### Miscellaneous Tasks

- Remove unused imports ([#534](https://github.com/alloy-rs/core/issues/534))

### Testing

- Add some more coerce error message tests ([#535](https://github.com/alloy-rs/core/issues/535))

## [0.6.3](https://github.com/alloy-rs/core/releases/tag/v0.6.3) - 2024-02-15

### Miscellaneous Tasks

- Fix winnow deprecation warnings ([#507](https://github.com/alloy-rs/core/issues/507))

## [0.6.0](https://github.com/alloy-rs/core/releases/tag/v0.6.0) - 2024-01-10

### Features

- [sol-type-parser] Improve error message for bad array size ([#470](https://github.com/alloy-rs/core/issues/470))

## [0.5.0](https://github.com/alloy-rs/core/releases/tag/v0.5.0) - 2023-11-23

### Bug Fixes

- [sol-type-parser] Normalize `u?int` to `u?int256` ([#397](https://github.com/alloy-rs/core/issues/397))
- [json-abi] `Param.ty` is not always a valid `TypeSpecifier` ([#386](https://github.com/alloy-rs/core/issues/386))

### Features

- [json-abi] Permit keyword prefixes in HR parser ([#420](https://github.com/alloy-rs/core/issues/420))
- [dyn-abi] `DynSolType::coerce_str` ([#380](https://github.com/alloy-rs/core/issues/380))

### Miscellaneous Tasks

- Use winnow `separated` instead of `separated0` ([#403](https://github.com/alloy-rs/core/issues/403))

### Styling

- Update rustfmt config ([#406](https://github.com/alloy-rs/core/issues/406))

## [0.4.1](https://github.com/alloy-rs/core/releases/tag/v0.4.1) - 2023-10-09

### Features

- Add parsing support for JSON items ([#329](https://github.com/alloy-rs/core/issues/329))

### Miscellaneous Tasks

- Fix typos ([#325](https://github.com/alloy-rs/core/issues/325))

## [0.4.0](https://github.com/alloy-rs/core/releases/tag/v0.4.0) - 2023-09-29

### Features

- [primitives] Add more methods to `Function` ([#290](https://github.com/alloy-rs/core/issues/290))

### Miscellaneous Tasks

- Don't pass debug feature to winnow ([#317](https://github.com/alloy-rs/core/issues/317))
- Sync crate level attributes ([#303](https://github.com/alloy-rs/core/issues/303))

### Performance

- Optimize identifier parsing ([#295](https://github.com/alloy-rs/core/issues/295))

### Refactor

- Rewrite type parser with `winnow` ([#292](https://github.com/alloy-rs/core/issues/292))

### Styling

- Format code snippets in docs ([#313](https://github.com/alloy-rs/core/issues/313))
- Some clippy lints ([#251](https://github.com/alloy-rs/core/issues/251))

## [0.3.2](https://github.com/alloy-rs/core/releases/tag/v0.3.2) - 2023-08-23

### Features

- Implement abi2sol ([#228](https://github.com/alloy-rs/core/issues/228))
- Function type ([#224](https://github.com/alloy-rs/core/issues/224))

### Performance

- Refactor TypeSpecifier parsing ([#230](https://github.com/alloy-rs/core/issues/230))

## [0.3.0](https://github.com/alloy-rs/core/releases/tag/v0.3.0) - 2023-07-26

### Features

- [sol-macro] `#[sol]` attributes and JSON ABI support ([#173](https://github.com/alloy-rs/core/issues/173))
- Solidity type parser ([#181](https://github.com/alloy-rs/core/issues/181))

### Miscellaneous Tasks

- Release 0.3.0 ([#207](https://github.com/alloy-rs/core/issues/207))

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
