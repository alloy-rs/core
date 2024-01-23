interface BlurExchange {
    type Side is uint8;
    type SignatureVersion is uint8;
    struct Execution {
        Input sell;
        Input buy;
    }
    struct Fee {
        uint16 rate;
        address payable recipient;
    }
    struct Input {
        Order order;
        uint8 v;
        bytes32 r;
        bytes32 s;
        bytes extraSignature;
        SignatureVersion signatureVersion;
        uint256 blockNumber;
    }
    struct Order {
        address trader;
        Side side;
        address matchingPolicy;
        address collection;
        uint256 tokenId;
        uint256 amount;
        address paymentToken;
        uint256 price;
        uint256 listingTime;
        uint256 expirationTime;
        Fee[] fees;
        uint256 salt;
        bytes extraParams;
    }

    event AdminChanged(address previousAdmin, address newAdmin);
    event BeaconUpgraded(address indexed beacon);
    event Closed();
    event Initialized(uint8 version);
    event NewBlockRange(uint256 blockRange);
    event NewExecutionDelegate(address indexed executionDelegate);
    event NewFeeRate(uint256 feeRate);
    event NewFeeRecipient(address feeRecipient);
    event NewGovernor(address governor);
    event NewOracle(address indexed oracle);
    event NewPolicyManager(address indexed policyManager);
    event NonceIncremented(address indexed trader, uint256 newNonce);
    event Opened();
    event OrderCancelled(bytes32 hash);
    event OrdersMatched(address indexed maker, address indexed taker, Order sell, bytes32 sellHash, Order buy, bytes32 buyHash);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Upgraded(address indexed implementation);

    constructor();

    function FEE_TYPEHASH() external view returns (bytes32);
    function INVERSE_BASIS_POINT() external view returns (uint256);
    function NAME() external view returns (string memory);
    function ORACLE_ORDER_TYPEHASH() external view returns (bytes32);
    function ORDER_TYPEHASH() external view returns (bytes32);
    function POOL() external view returns (address);
    function ROOT_TYPEHASH() external view returns (bytes32);
    function VERSION() external view returns (string memory);
    function WETH() external view returns (address);
    function _execute(Input memory sell, Input memory buy) external payable;
    function blockRange() external view returns (uint256);
    function bulkExecute(Execution[] memory executions) external payable;
    function cancelOrder(Order memory order) external;
    function cancelOrders(Order[] memory orders) external;
    function cancelledOrFilled(bytes32) external view returns (bool);
    function close() external;
    function execute(Input memory sell, Input memory buy) external payable;
    function executionDelegate() external view returns (address);
    function feeRate() external view returns (uint256);
    function feeRecipient() external view returns (address);
    function governor() external view returns (address);
    function incrementNonce() external;
    function initialize(address _executionDelegate, address _policyManager, address _oracle, uint256 _blockRange) external;
    function isInternal() external view returns (bool);
    function isOpen() external view returns (uint256);
    function nonces(address) external view returns (uint256);
    function open() external;
    function oracle() external view returns (address);
    function owner() external view returns (address);
    function policyManager() external view returns (address);
    function proxiableUUID() external view returns (bytes32);
    function remainingETH() external view returns (uint256);
    function renounceOwnership() external;
    function setBlockRange(uint256 _blockRange) external;
    function setExecutionDelegate(address _executionDelegate) external;
    function setFeeRate(uint256 _feeRate) external;
    function setFeeRecipient(address _feeRecipient) external;
    function setGovernor(address _governor) external;
    function setOracle(address _oracle) external;
    function setPolicyManager(address _policyManager) external;
    function transferOwnership(address newOwner) external;
    function upgradeTo(address newImplementation) external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
}