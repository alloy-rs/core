// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface ITestContract {
    struct TestStruct {
        address asset;
    }

    type Unsigned is uint256;

    enum TestEnum {
        A,
        B,
        C
    }
}

contract TestContract is ITestContract {
    function test_struct(TestStruct memory) external {}

    error TestError(Unsigned);

    event TestEvent(Unsigned);

    constructor(TestStruct memory, TestEnum, Unsigned) {}
}
