interface UniswapV3Position {
    struct Range {
        int24 lowerTick;
        int24 upperTick;
        uint24 feeTier;
    }

    function getLiquidityByRange(address pool_, address self_, int24 lowerTick_, int24 upperTick_) external view returns (uint128 liquidity);
    function getPositionId(address self_, int24 lowerTick_, int24 upperTick_) external pure returns (bytes32 positionId);
    function rangeExists(Range[] memory currentRanges_, Range memory range_) external pure returns (bool ok, uint256 index);
}