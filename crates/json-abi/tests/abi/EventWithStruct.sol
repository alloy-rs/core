library MyContract {
    struct MyStruct {
        uint256 a;
        uint256 b;
    }
}

interface EventWithStruct {
    event MyEvent(MyContract.MyStruct, uint256 c);
}