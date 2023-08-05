# syn-solidity

[`syn`]-powered parser for Solidity-like [`TokenStream`]s.

The parsed root element is the [`File`], which contains a list of [`Item`]s.
[`Item`]s also support outer attributes, as shown below.

**⚠️ Work in progress ⚠️**

[`syn`]: https://github.com/dtolnay/syn
[`TokenStream`]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html

## Design

This parser is specifically designed for Rust procedural macros. It aims to
mimic the behavior of the official Solidity compiler (Solc) when it comes to
parsing valid Solidity code. This means that all valid Solidity code, as
recognized by Solc v0.5.*[^1] and above, will also be recognized and parsed
correctly by `syn-solidity`.

However, `syn-solidity` is more permissive and lenient compared to the official
Solidity compiler and grammar specifications. Some examples of code patterns
that are valid in `syn-solidity` but not in the official compiler include:
- identifiers are Rust identifiers (`syn::Ident`), and as such cannot contain
  the dollar sign (`$`), but can contain unicode characters
- trailing punctuation, like commas (`,`) in function arguments or enums
  definitions
- certain variable and function attributes in certain contexts, like `internal`
  functions or functions with implementations (`{ ... }`) in interfaces
- parameter storage locations in item definitions, like `uint256[] memory` in
  a struct or error definition
- the tuple type `(T, U, ..)` is allowed wherever a type is expected, and can
  optionally be preceded by the `tuple` keyword.
  This is the same as [`ethers.js`'s Human-Readable ABI][ethersjs-abi]

This lenient behavior is intentionally designed to facilitate usage within
procedural macros, and to reduce general code complexity in the parser and AST.

[ethersjs-abi]: https://docs.ethers.org/v5/api/utils/abi/formats/#abi-formats--human-readable-abi
[^1]: Older versions may still parse successfully, but this is not guaranteed.

## Known limitations

This parser is limited to only valid Rust tokens, meaning that certain Solidity
constructs are not supported. Some examples include, but are not limited to:
- single quote strings
- `hex` and `unicode` string literal prefixes.
  Literal prefixes are [reserved in Rust edition 2021 and above][reserved-2021].
- `"\uXXXX"` unicode escapes. Rust uses `"\u{XXXX}"` for unicode codepoints
- invalid nested block comments. For example, `/*/*/` does not parse.

For the most part, you can copy-paste Solidity code and expect it to parse
correctly most of the time. You can see a few examples of Solidity code that
parses correctly (after some very light patching) in the [tests] directory.

[reserved-2021]: https://doc.rust-lang.org/edition-guide/rust-2021/reserving-syntax.html
[tests]: https://github.com/alloy-rs/core/tree/main/crates/syn-solidity/tests/contracts

## Examples

Basic usage:

```rust
use quote::quote;
use syn_solidity::{Expr, File, Item, Lit, Stmt};

// Create a Solidity `TokenStream`
let tokens = quote! {
    /// @name HelloWorld
    /// @notice A hello world example in Solidity.
    contract HelloWorld {
        /// @notice Returns the string "Hello, World!".
        function helloWorld() external pure returns (string memory) {
            return "Hello, World!";
        }
    }
};

// Parse the tokens into a `File`
let ast: File = syn_solidity::parse2(tokens)?;

let items: &[Item] = &ast.items;
let Some(Item::Contract(contract)) = items.first() else {
    unreachable!()
};
assert_eq!(contract.name, "HelloWorld");
assert_eq!(contract.attrs.len(), 2); // doc comments

let body: &[Item] = &contract.body;
let Some(Item::Function(function)) = body.first() else {
    unreachable!()
};
assert_eq!(function.attrs.len(), 1); // doc comment
assert_eq!(function.name.as_ref().unwrap(), "helloWorld");
assert!(function.arguments.is_empty()); // ()
assert_eq!(function.attributes.len(), 2); // external pure
assert!(function.returns.is_some());

let Some([Stmt::Return(ret)]) = function.body() else {
    unreachable!()
};
let Some(Expr::Lit(Lit::Str(s))) = &ret.expr else {
    unreachable!()
};
assert_eq!(s.value(), "Hello, World!");
# syn::Result::Ok(())
```
