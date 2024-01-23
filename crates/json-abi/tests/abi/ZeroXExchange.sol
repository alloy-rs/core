interface ZeroXExchange {
    event AssetProxyRegistered(bytes4 id, address assetProxy);
    event Cancel(address indexed makerAddress, address indexed feeRecipientAddress, address senderAddress, bytes32 indexed orderHash, bytes makerAssetData, bytes takerAssetData);
    event CancelUpTo(address indexed makerAddress, address indexed senderAddress, uint256 orderEpoch);
    event Fill(address indexed makerAddress, address indexed feeRecipientAddress, address takerAddress, address senderAddress, uint256 makerAssetFilledAmount, uint256 takerAssetFilledAmount, uint256 makerFeePaid, uint256 takerFeePaid, bytes32 indexed orderHash, bytes makerAssetData, bytes takerAssetData);
    event SignatureValidatorApproval(address indexed signerAddress, address indexed validatorAddress, bool approved);

    constructor(bytes _zrxAssetData);

    function EIP712_DOMAIN_HASH() external view returns (bytes32);
    function VERSION() external view returns (string memory);
    function ZRX_ASSET_DATA() external view returns (bytes memory);
    function allowedValidators(address, address) external view returns (bool);
    function assetProxies(bytes4) external view returns (address);
    function batchCancelOrders((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders) external;
    function batchFillOrKillOrders((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders, uint256[] memory takerAssetFillAmounts, bytes[] memory signatures) external returns ((uint256, uint256, uint256, uint256) memory totalFillResults);
    function batchFillOrders((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders, uint256[] memory takerAssetFillAmounts, bytes[] memory signatures) external returns ((uint256, uint256, uint256, uint256) memory totalFillResults);
    function batchFillOrdersNoThrow((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders, uint256[] memory takerAssetFillAmounts, bytes[] memory signatures) external returns ((uint256, uint256, uint256, uint256) memory totalFillResults);
    function cancelOrder((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes) memory order) external;
    function cancelOrdersUpTo(uint256 targetOrderEpoch) external;
    function cancelled(bytes32) external view returns (bool);
    function currentContextAddress() external view returns (address);
    function executeTransaction(uint256 salt, address signerAddress, bytes memory data, bytes memory signature) external;
    function fillOrKillOrder((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes) memory order, uint256 takerAssetFillAmount, bytes memory signature) external returns ((uint256, uint256, uint256, uint256) memory fillResults);
    function fillOrder((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes) memory order, uint256 takerAssetFillAmount, bytes memory signature) external returns ((uint256, uint256, uint256, uint256) memory fillResults);
    function fillOrderNoThrow((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes) memory order, uint256 takerAssetFillAmount, bytes memory signature) external returns ((uint256, uint256, uint256, uint256) memory fillResults);
    function filled(bytes32) external view returns (uint256);
    function getAssetProxy(bytes4 assetProxyId) external view returns (address);
    function getOrderInfo((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes) memory order) external view returns ((uint8, bytes32, uint256) memory orderInfo);
    function getOrdersInfo((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders) external view returns ((uint8, bytes32, uint256)[] memory);
    function isValidSignature(bytes32 hash, address signerAddress, bytes memory signature) external view returns (bool isValid);
    function marketBuyOrders((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders, uint256 makerAssetFillAmount, bytes[] memory signatures) external returns ((uint256, uint256, uint256, uint256) memory totalFillResults);
    function marketBuyOrdersNoThrow((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders, uint256 makerAssetFillAmount, bytes[] memory signatures) external returns ((uint256, uint256, uint256, uint256) memory totalFillResults);
    function marketSellOrders((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders, uint256 takerAssetFillAmount, bytes[] memory signatures) external returns ((uint256, uint256, uint256, uint256) memory totalFillResults);
    function marketSellOrdersNoThrow((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes)[] memory orders, uint256 takerAssetFillAmount, bytes[] memory signatures) external returns ((uint256, uint256, uint256, uint256) memory totalFillResults);
    function matchOrders((address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes) memory leftOrder, (address, address, address, address, uint256, uint256, uint256, uint256, uint256, uint256, bytes, bytes) memory rightOrder, bytes memory leftSignature, bytes memory rightSignature) external returns (((uint256, uint256, uint256, uint256), (uint256, uint256, uint256, uint256), uint256) memory matchedFillResults);
    function orderEpoch(address, address) external view returns (uint256);
    function owner() external view returns (address);
    function preSign(bytes32 hash, address signerAddress, bytes memory signature) external;
    function preSigned(bytes32, address) external view returns (bool);
    function registerAssetProxy(address assetProxy) external;
    function setSignatureValidatorApproval(address validatorAddress, bool approval) external;
    function transactions(bytes32) external view returns (bool);
    function transferOwnership(address newOwner) external;
}