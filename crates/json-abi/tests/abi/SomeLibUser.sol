library SomeLib {
    function add(uint256 a, uint256 b) external pure returns (uint256) {
        return a + b;
    }
}

contract SomeLibUser {
    function add(uint256 a, uint256 b) public pure returns (uint256) {
        return SomeLib.add(a, b);
    }
}
