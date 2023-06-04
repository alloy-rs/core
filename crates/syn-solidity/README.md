# syn-solidity

[`syn`]-powered parser for Solidity-like [`TokenStream`]s.

**⚠️ Work in progress ⚠️**

The parsed root element is the [`File`], which contains a list of [`Item`]s.
[`Item`]s also support outer attributes, as shown below.

This parser is compatible with Ethereum Solidity versions v0.5.x and above, but
older versions may still parse successfully.

[`syn`]: https://github.com/dtolnay/syn
[`TokenStream`]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html

```rust,ignore
use quote::quote;
use syn_solidity::{File, item::{Item, Function}};

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
let ast: File = syn_solidity::parse2()?;

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
