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

## Example

```rust,ignore TODO
use quote::quote;
use syn_solidity::{File, Item};

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
let Some(Item::Contract(contract)) = items.first() else { unreachable!() };
assert_eq!(contract.name, "HelloWorld");
assert_eq!(contract.attrs.len(), 2);

let body: &[Item] = &contract.body;
let Some(Item::Function(function)) = items.first() else { unreachable!() };
assert_eq!(function.attrs.len(), 1);
assert_eq!(function.name, "helloWorld");
assert!(function.arguments.is_empty());
assert_eq!(function.attributes.len(), 3);

# syn::Result::Ok(())
```
