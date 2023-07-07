# alloy-sol-types

Compile-time representation of Ethereum's type system with ABI and EIP-712
support.

This crate provides a developer-friendly interface to Ethereum's type system,
by representing Solidity types. See [type_system.md](./type_system.md) for a rundown, and the
[crate docs] for more information

[crate docs]: https://docs.rs/alloy-sol-types/latest/alloy_sol_types/

### Features

- static representation of Solidity types
- ABI encoding and decoding
- EIP-712 encoding and decoding
- EIP-712 Domain object w/ `serde` support

### Usage

See the [crate docs] for more details.

```rust
// Declare a solidity type in standard solidity
sol! {
    struct Foo {
        bar: u256;
        baz: bool;
    }
}

// A corresponding Rust struct is generated!
let foo = Foo {
    bar: 42.into(),
    baz: true,
};

// Works for UDTs
sol! { type MyType is uint8; }
let my_type = MyType::from(42u8);

// For errors
sol! {
    error MyError(
        string message,
    );
}

// And for functions!
sol! { function myFunc() external returns (uint256); }
```

## Licensing

This crate is an extensive rewrite of the
[ethabi](https://github.com/rust-ethereum/ethabi) crate by the parity team.
That codebase is used under the terms of the **MIT** license. We have preserved
the original license notice in files incorporating `ethabi` code.
