# alloy-rlp

This crate provides Ethereum RLP (de)serialization functionality. RLP is
commonly used for Ethereum EL datastructures, and its documentation can be
found [at ethereum.org][ref].

[ref]: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/

## Usage

We strongly recommend deriving RLP traits via the `RlpEncodable` and
`RlpDecodable` derive macros.

Trait methods can then be accessed via the `Encodable` and `Decodable` traits.

## Provenance note

This crate was originally part of the [reth] project, as [`reth_rlp`].

This was forked from an earlier Apache-licensed version of the [`fastrlp`]
crate, before it changed licence to GPL. The Rust `fastrlp` implementation is
itself a port of the [Golang Apache-licensed fastrlp][gofastrlp].

[reth]: https://github.com/paradigmxyz/reth
[`reth_rlp`]: https://github.com/paradigmxyz/reth/tree/99a314c59bbd94a34a285369da95fb5604883c65/crates/rlp
[`fastrlp`]: https://github.com/vorot93/fastrlp
[gofastrlp]: https://github.com/umbracle/fastrlp
