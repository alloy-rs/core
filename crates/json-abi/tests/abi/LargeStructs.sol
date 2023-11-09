interface LargeStructs {
    struct AssetStorage {
        bytes32 symbol;
        address tokenAddress;
        address muxTokenAddress;
        uint8 id;
        uint8 decimals;
        uint56 flags;
        uint32 initialMarginRate;
        uint32 maintenanceMarginRate;
        uint32 positionFeeRate;
        uint32 liquidationFeeRate;
        uint32 minProfitRate;
        uint32 minProfitTime;
        uint96 maxLongPositionSize;
        uint96 maxShortPositionSize;
        uint32 spotWeight;
        uint32 longFundingBaseRate8H;
        uint32 longFundingLimitRate8H;
        uint8 referenceOracleType;
        address referenceOracle;
        uint32 referenceDeviation;
        uint32 halfSpread;
        uint128 longCumulativeFundingRate;
        uint128 shortCumulativeFunding;
        uint96 spotLiquidity;
        uint96 credit;
        uint96 totalLongPosition;
        uint96 totalShortPosition;
        uint96 averageLongPrice;
        uint96 averageShortPrice;
        uint128 collectedFee;
        uint256 deduct;
    }
    struct ChainStorage {
        PoolStorage pool;
        AssetStorage[] assets;
        DexStorage[] dexes;
        uint32 liquidityLockPeriod;
        uint32 marketOrderTimeout;
        uint32 maxLimitOrderTimeout;
        uint256 lpDeduct;
        uint256 stableDeduct;
        bool isPositionOrderPaused;
        bool isLiquidityOrderPaused;
    }
    struct DexStorage {
        uint8 dexId;
        uint8 dexType;
        uint8[] assetIds;
        uint32[] assetWeightInDEX;
        uint256[] totalSpotInDEX;
        uint32 dexWeight;
        uint256 dexLPBalance;
        uint256[] liquidityBalance;
    }
    struct PoolStorage {
        uint32 shortFundingBaseRate8H;
        uint32 shortFundingLimitRate8H;
        uint32 fundingInterval;
        uint32 liquidityBaseFeeRate;
        uint32 liquidityDynamicFeeRate;
        uint96 mlpPriceLowerBound;
        uint96 mlpPriceUpperBound;
        uint32 lastFundingTime;
        uint32 sequence;
        uint32 strictStableDeviation;
    }
    struct SubAccountState {
        uint96 collateral;
        uint96 size;
        uint32 lastIncreasedTime;
        uint96 entryPrice;
        uint128 entryFunding;
    }

    function getChainStorage() external returns (ChainStorage memory chain);
    function getOrders(uint64[] memory orderIds) external pure returns (bytes32[3][] memory orders, bool[] memory isExist);
    function getSubAccounts(bytes32[] memory subAccountIds) external pure returns (SubAccountState[] memory subAccounts);
    function getSubAccountsAndOrders(bytes32[] memory subAccountIds, uint64[] memory orderIds) external pure returns (SubAccountState[] memory subAccounts, bytes32[3][] memory orders, bool[] memory isOrderExist);
}