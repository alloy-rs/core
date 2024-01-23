interface UniswapV2Factory {
    event PairCreated(address indexed token0, address indexed token1, address pair, uint256);

    constructor(address _feeToSetter);

    function allPairs(uint256) external view returns (address);
    function allPairsLength() external view returns (uint256);
    function createPair(address tokenA, address tokenB) external returns (address pair);
    function feeTo() external view returns (address);
    function feeToSetter() external view returns (address);
    function getPair(address, address) external view returns (address);
    function setFeeTo(address _feeTo) external;
    function setFeeToSetter(address _feeToSetter) external;
}