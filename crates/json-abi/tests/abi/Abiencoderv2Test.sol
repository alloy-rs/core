interface Abiencoderv2Test {
    struct Person {
        string name;
        uint256 age;
    }

    function defaultPerson() external pure returns (Person memory);
}