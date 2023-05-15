## RLP crate

This crate provides Ethereum RLP (de)serialization functionality. RLP is
commonly used for Ethereum EL datastructures, and its documentation can be
found [at ethereum.org](ref).

[ref]: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/

### Usage

We strongly recommend deriving RLP traits via the `RlpEncodable` and
`RlpDecodable` derive macros.

Trait methods can then be accessed via the `Encodable` and `Decodable`
traits.

```rust
use ethers_rlp::{RlpEncodable, RlpDecodable, Decodable, Encodable};

#[derive(Debug, RlpEncodable, RlpDecodable, PartialEq)]
pub struct MyStruct {
    pub a: u64,
    pub b: Vec<u8>,
}

fn main() {
    let my_struct = MyStruct {
        a: 42,
        b: vec![1, 2, 3],
    };

    let mut buffer = Vec::<u8>::new();
    let encoded = my_struct.encode(&mut buffer);
    let decoded = MyStruct::decode(&mut buffer.as_slice()).unwrap();
    assert_eq!(my_struct, decoded);
}
```

### Provenance notes

This crate was originally part of the
[reth](https://github.com/paradigmxyz/reth/) project. Maintenance has been
taken over by the ethers developers.

Forked from an earlier Apache licenced version of the `fastrlp` crate, before
it changed licence to GPL. NOTE: The Rust fastrlp implementation is itself a
port of the [Golang Apache licensed fastrlp](gofastrlp).

[gofastrlp]: https://github.com/umbracle/fastrlp
