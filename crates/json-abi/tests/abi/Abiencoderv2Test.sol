library Hello {
    struct Person {
        string name;
        uint256 age;
    }
}

interface Abiencoderv2Test {
    function defaultPerson() external pure returns (Hello.Person memory);
}