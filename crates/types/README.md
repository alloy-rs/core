# ethers-types

Ethereum type definitions.

# TODO

1. Implement RLP for `ruint::Uint` (upstream)
    - remove all `TODO_UINT_RLP`
    - fix encoding tests
2. Fix remaining `TODO`s and inline `cfg(TODO.*)`
    - see also [Cargo.toml](./Cargo.toml#L46) deps
3. Dedup _all_ primitive and rpc types
    - Tracked by `ambiguous_glob_reexports` rustc lint, #107880 >=1.70 (currently beta or nightly)
4. Split rpc types into `ethers-rpc-types`. We may be able to make it completely independent of `ethers-types`
