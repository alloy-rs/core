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
