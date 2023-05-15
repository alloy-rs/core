## ethers-rlp

This crate provides Ethereum RLP (de)serialization functionality. RLP is
commonly used for Ethereum EL datastructures, and its documentation can be
found [at ethereum.org][ref].

[ref]: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/

### Usage

We strongly recommend deriving RLP traits via the `RlpEncodable` and
`RlpDecodable` derive macros.

Trait methods can then be accessed via the `Encodable` and `Decodable`
traits.

### Provenance note

This crate was originally part of the
[reth](https://github.com/paradigmxyz/reth/) project. Maintenance has been
taken over by the ethers developers.

Forked from an earlier Apache licenced version of the `fastrlp` crate, before
it changed licence to GPL. NOTE: The Rust fastrlp implementation is itself a
port of the [Golang Apache licensed fastrlp][gofastrlp].

[gofastrlp]: https://github.com/umbracle/fastrlp
