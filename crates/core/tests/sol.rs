#![cfg(feature = "sol-types")]

use alloy_core::sol;

sol! {
    struct MyStruct {
        uint32 a;
        uint32 b;
    }

    function myFunction(uint32 a, uint32 b) returns(uint32);
    event MyEvent(uint32 a, uint32 b);
    error MyError(uint32 a, uint32 b);

    contract MyContract {
        struct MyOtherStruct {
            uint32 a;
            uint32 b;
        }

        function myOtherFunction(uint32 a, uint32 b) returns(uint32);
        event MyOtherEvent(uint32 a, uint32 b);
        error MyOtherError(uint32 a, uint32 b);
    }
}
