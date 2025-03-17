library ITestContract {
    type Unsigned is uint256;
    struct TestStruct {
        address asset;
    }
}

interface TestContract {
    error TestError(ITestContract.Unsigned);

    event TestEvent(ITestContract.Unsigned);

    function test_struct(ITestContract.TestStruct memory) external;
}