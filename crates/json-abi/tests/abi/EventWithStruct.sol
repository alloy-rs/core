interface EventWithStruct {
    struct MyStruct {
        uint256 a;
        uint256 b;
    }

    event MyEvent(MyStruct, uint256 c);
}