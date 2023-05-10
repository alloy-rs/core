# Solidity Type Representation

This crate is built around a representation of the Solidity type system. This doc is a primer for how we chose to represent Solidity types in Rust.

## Why?

The ABI encoding scheme, is tailored to the EVM and to Solidity specifically.
Its internals are complex and may not be well-understood by solidity devs.
However, Solidity devs generally do understand Solidity types. As a result, we
decided the best way to represent ABI coding was as a method on Solidity types.

Rather than `Coder::encode(data, type)` we felt that `Type::encode(data)` would
be more intuitive and idiomatic in Rust. To achieve this, we give each Solidity
type a concrete Rust type that contains its data. E.g. `bytes32` is `[u8; 32]`.
`uint256` is `U256`, `string` is `String`. This allows programmers to work with
Solidity _types_, but Rust _data_.

Static ABI typing also allows the compiler to do significantly more
optimization on encoding and decoding. Benchmarks are pending, but we expect
this to be one of the fastest implementations for regular encoding/decoding. :)

## Downside

This crate works only with types known at compile-time. For types known only at
runtime (including the eip712 `eth_signTypedData` json-rpc request), see the
`ethers-dyn-abi` crate.

### To what extent?

We support types at the interface between Solidity and other systems. These are
types that are commonly encoded/decoded. We do not support types that are
internal-only (e.g. `mapping`) or solidity type modifications describing EVM
internals (e.g. `payable` and `memory`).

**Support overview:**

- First-class solidity types

  - EXCEPT
    - [`function` types](https://docs.soliditylang.org/en/v0.8.17/types.html#function-types)
    - [`fixed`](https://docs.soliditylang.org/en/v0.8.17/types.html#fixed-point-numbers)
    - [enums](https://docs.soliditylang.org/en/v0.8.17/types.html#enums) (supported soon)

- Compound solidity types

  - Arrays `T[N]`
  - Dynamic arrays `T[]`
  - tuples

- User-defined Types

  - [Structs](https://solidity-by-example.org/structs/)
  - Function arguments
  - [Errors](https://blog.soliditylang.org/2021/04/21/custom-errors/)
  - [User-defined Value Types](https://blog.soliditylang.org/2021/09/27/user-defined-value-types/)

- Externalized Types
  - Function calls
  - Events (TODO)
  -

## How?

Solidity is represented as a set of traits. The `SolType` trait is at the root
of the type system, and contains functionality common to all solidity types.
This includes type name, ABI (de)tokenization and coding, as well as Solidity
type checking rules.

From there we divide types into two categories: Data and non-Data. The data
types implement `SolDataType`, and represent types that are computed on in a
Solidity contract. These types exist on the stack or in memory or storage in
Solidity, can be bound to variables, and can be struct properties or function arguments.

The non-data types cannot be bound to variables or computed on. These are
represented by the traits, `SolCall`, `SolEvent`, and `SolError`. These
types enter or exit the EVM, and are not part of normal Solidity computation, but are still ABI coded/decoded.

### Rough Edge:

Currently, our representation supports using tuples as struct props. This is
not allowed in Solidity, and future versions of this crate may change the type
system to disallow it.

### Trait Layout

```
SolType
├── SolError
├── SolCall
├── SolEvent (TODO)
└── SolDataType
    ├── SolStruct
    ├── SolEnum (TODO)
    ├── UDTs
    ├── address
    ├── bytes
    ├── intx
    ├── uintx
    ├── bool
    ├── T[N] (Array)
    ├── T[] (Dynamic Array)
    ├── string
    ├── bytesx
    └── tuples
```

### Trait Quick Reference

- `SolType` - Provides type name and properties, and basic ABI coding
- `SolDataType` - Provides EIP-712 and `encodePacked`
- `SolError` - describes custom Error types with selector, and provides
  specialized coding methods
- `SolCall` - describes function **arguments** with selector, and provides
  specialized coding methods
- `SolEvent` - describes Event types with topic 0 and internal tuple, and
  provides specialized coding methods

## Implementing these traits

Well, don't.

Due to the weirdness of Solidity and the sensitivity of the ABI coder to minor
issues, we do not recommend manually implementing these traits. Instead, most
users will want to use the `sol!` macro to auto-generate types and structs from
Solidity snippets at compile time.

## Using these traits

Users will typically want to interact with `SolType`. When using errors,
events, or calls, users will want to import the relevant trait, and use the
specialized coding methods

Users will generally only need to import `SolDataType` for `encodePacked`.
