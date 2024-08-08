library Many {
    struct Info {
        uint128 x;
        int24 y;
        int24 z;
        uint256 a;
        int256 b;
        int256 c;
        int256 d;
        uint256 e;
        uint256 f;
    }
}

interface LargeStruct {
    function getById(bytes32 id) external view returns (Many.Info memory);
}