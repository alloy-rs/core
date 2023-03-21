# sol-type-parser

This crate provides a proc macro that parses solidity types and generates
SolType types from them. This allows developers to specify complex solidity
types efficiently in their code. These types may then be used for safe encoding
and decoding.

### Examples

```
type B32 = sol!{ bytes32 }
