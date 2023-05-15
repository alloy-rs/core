# ethers-dyn-abi

Dynamic Solidity type encoder: Run-time representation of Ethereum's type
system with ABI encoding & decoding.

This library provides a runtime encoder/decoder for solidity types. It is
intended to be used when the solidity type is not known at compile time.
This is particularly useful for EIP-712 signing interfaces.

We **strongly** recommend using the [static encoder/decoder][abi] when possible.
The dyanmic encoder/decoder is significantly more expensive, especially for
complex types. It is also significantly more error prone, as the mapping
between solidity types and rust types is not enforced by the compiler.

[abi]: https://docs.rs/ethers-sol-types/latest/ethers_sol_types/

## Usage

```rust
use ethers_dyn_abi::{DynSolType, DynSolValue};

// parse a type from a string
// limitation: custom structs cannot currently be parsed this way.
let my_type: DynSolType = "uint8[2][]".parse().unwrap();

// set values
let uints = DynSolValue::FixedArray(vec![0u8.into(), 1u8.into()]);
let my_values = DynSolValue::Array(vec![uints]);

// encode
let encoded = my_type.encode_single(my_values.clone()).unwrap();

// decode
let decoded = my_type.decode_single(&encoded).unwrap();

assert_eq!(decoded, my_values);
```

## How it works

The dynamic encoder/decoder is implemented as a set of enums that represent
solidity types, solidity values (in rust representation form), and ABI
tokens. Unlike the static encoder, each of these must be instantiated at
runtime. The [`DynSolType`] enum represents a solidity type, and is
equivalent to an enum over types implementing the [`crate::SolType`] trait.
The [`DynSolValue`] enum represents a solidity value, and describes the
rust shapes of possible solidity values. It is similar to, but not
equivalent to an enum over types used as [`crate::SolType::RustType`]. The
[`DynToken`] enum represents an ABI token, and is equivalent to an enum over
the types implementing the [`ethers_abi_enc::TokenType`] trait.

Where the static encoding system encodes the expected type information into
the rust type system, the dynamic encoder/decoder encodes it as a concrete
instance of [`DynSolType`]. This type is used to tokenize and detokenize
[`DynSolValue`] instances. The [`std::str::FromStr`] impl is used to parse a
solidity type string into a [`DynSolType`] object.

- Tokenizing - `DynSolType + `DynSolValue`=`DynToken`
- Detokenizing -`DynSolType`+`DynToken`=`DynSolValue`

Users must manually handle the conversions between [`DynSolValue`] and their
own rust types. We provide several `From`implementations, but they fall
short when dealing with arrays and tuples. We also provide fallible casts
into the contents of each variant.

## `DynToken::decode_populate`

Because the shape of the data is known only at runtime, we cannot
compile-time allocate the memory needed to hold decoded data. Instead, we
pre-allocate a [`DynToken`] with the same shape as the expected type, and
empty values. We then populate the empty values with the decoded data.

This is a significant behavior departure from the static decoder. We do not
recommend using the [`DynToken`] type directly. Instead, we recommend using
the encoding and decoding methods on [`DynSolType`].

## Licensing

This crate is an extensive rewrite of the
[ethabi](https://github.com/rust-ethereum/ethabi) crate by the parity team.
That codebase is used under the terms of the **MIT** license. We have preserved
the original license notice in files incorporating `ethabi` code.
