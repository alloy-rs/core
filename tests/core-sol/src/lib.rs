//! Tests `#[sol(alloy_sol_types = ...)]`.
//!
//! This has to be in a separate crate where `alloy_sol_types` is not provided as a dependency.

#![no_std]

use alloy_core::sol;

type _MyUint = sol!(uint32);
type _MyTuple = sol!((_MyUint, bytes, bool, string, bytes32, (address, uint64)));

sol! {
    #![sol(abi)]

    enum MyEnum {
        A, B
    }

    #[derive(Default, PartialEq, Eq, Hash)]
    struct MyStruct {
        uint32 a;
        uint64 b;
    }

    event MyEvent(MyEnum a, MyStruct indexed b, bytes c, string indexed d, bytes32 indexed e);

    error MyError(uint32 a, MyStruct b);

    constructor myConstructor(address);

    function myFunction(MyStruct a, bytes b) returns(uint32);

    modifier myModifier(bool a, string b);

    mapping(bytes32 a => bool b) myMapping;

    type MyType is uint32;
}

sol! {
    contract MyContract {
        enum MyOtherEnum {
            A, B
        }

        struct MyOtherStruct {
            uint32 a;
            uint32 b;
        }

        event MyOtherEvent(MyOtherEnum indexed a, MyOtherStruct b, (bool, string) indexed c);

        error MyOtherError(uint32 a, MyOtherStruct b);

        constructor myOtherConstructor(address);

        function myOtherFunction(MyOtherStruct a, bytes b) returns(uint32);

        modifier myOtherModifier(bool a, string b);

        mapping(bytes32 a => bool b) myOtherMapping;

        type MyOtherType is uint32;
    }
}

/// Docs
#[deny(missing_docs)]
pub mod no_missing_docs {
    alloy_core::sol! {
        #[allow(missing_docs)]
        contract Allowed {
            uint256 public number;

            struct MyStruct {
                uint256 a;
                bool b;
            }

            function setNumber(uint256 newNumber) public {
                number = newNumber;
            }

            function increment() public {
                number++;
            }

            event Transfer(address indexed from, address indexed to, uint256 value);
            event Approval(address indexed owner, address indexed spender, uint256 value);

            error Transfer2(address from, address to, uint256 value);
            error Approval2(address owner, address spender, uint256 value);
        }

        /// Docs
        contract NotAllowed {
            /// Docs
            uint256 public number;

            /// Docs
            struct MyStruct {
                /// Docs
                uint256 a;
                /// Docs
                bool b;
            }

            /// Docs
            function setNumber(uint256 newNumber) public {
                number = newNumber;
            }

            /// Docs
            function increment() public {
                number++;
            }

            /// Docs
            event Transfer(address indexed from, address indexed to, uint256 value);
            /// Docs
            event Approval(address indexed owner, address indexed spender, uint256 value);

            /// Docs
            error Transfer2(address from, address to, uint256 value);
            /// Docs
            error Approval2(address owner, address spender, uint256 value);
        }
    }
}

#[test]
fn do_stuff() {
    let mut set = alloy_core::primitives::map::B256Map::<MyStruct>::default();
    set.insert(
        alloy_core::primitives::hex_literal::hex!(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        )
        .into(),
        Default::default(),
    );
    assert_eq!(set.len(), 1);
}
