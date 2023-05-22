# ethers-types

Ethereum type definitions.

# TODO

1. [x] Implement RLP for `ruint::Uint` (upstream)
    - remove all `TODO_UINT_RLP`
    - fix encoding tests
2. [x] Fix remaining `TODO`s and inline `cfg(TODO.*)`
    - ~~see also Cargo.toml deps~~
3. [ ] Dedup _all_ primitive and rpc types
    - Tracked by `unreachable_pub` or `ambiguous_glob_reexports` rustc lints, #107880 >=1.70 (currently beta or nightly)
4. [ ] Split rpc types into `ethers-rpc-types`. We may be able to make it completely independent of `ethers-types`
