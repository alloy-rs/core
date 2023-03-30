# sol-type-parser

This crate provides a proc macro that parses solidity types and generates
SolType types from them. This allows developers to specify complex solidity
types efficiently in their code. These types may then be used for safe encoding
and decoding.

### Examples

```rust
/// Assign type aliases for
type B32 = sol!{ bytes32 }
type SolArrayOf<T> = sol! { T[] };
type SolTuple = sol!{ tuple(address, bytes, string) }

// Structs don't need type assignments
sol! {
    struct MyStruct {
        uint256 a;
        bytes32 b;
        address[] c;
    }
}

// Can be used in a type position, via < >, but this is not generally
// recommended
<sol! {
    bool
}>::hex_encode_single(true);

```
