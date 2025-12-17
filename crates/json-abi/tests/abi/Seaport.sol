interface Seaport {
    type BasicOrderType is uint8;
    type ItemType is uint8;
    type OrderType is uint8;
    type Side is uint8;
    struct AdditionalRecipient {
        uint256 amount;
        address payable recipient;
    }
    struct AdvancedOrder {
        OrderParameters parameters;
        uint120 numerator;
        uint120 denominator;
        bytes signature;
        bytes extraData;
    }
    struct BasicOrderParameters {
        address considerationToken;
        uint256 considerationIdentifier;
        uint256 considerationAmount;
        address payable offerer;
        address zone;
        address offerToken;
        uint256 offerIdentifier;
        uint256 offerAmount;
        BasicOrderType basicOrderType;
        uint256 startTime;
        uint256 endTime;
        bytes32 zoneHash;
        uint256 salt;
        bytes32 offererConduitKey;
        bytes32 fulfillerConduitKey;
        uint256 totalOriginalAdditionalRecipients;
        AdditionalRecipient[] additionalRecipients;
        bytes signature;
    }
    struct ConsiderationItem {
        ItemType itemType;
        address token;
        uint256 identifierOrCriteria;
        uint256 startAmount;
        uint256 endAmount;
        address payable recipient;
    }
    struct CriteriaResolver {
        uint256 orderIndex;
        Side side;
        uint256 index;
        uint256 identifier;
        bytes32[] criteriaProof;
    }
    struct Execution {
        ReceivedItem item;
        address offerer;
        bytes32 conduitKey;
    }
    struct Fulfillment {
        FulfillmentComponent[] offerComponents;
        FulfillmentComponent[] considerationComponents;
    }
    struct FulfillmentComponent {
        uint256 orderIndex;
        uint256 itemIndex;
    }
    struct OfferItem {
        ItemType itemType;
        address token;
        uint256 identifierOrCriteria;
        uint256 startAmount;
        uint256 endAmount;
    }
    struct Order {
        OrderParameters parameters;
        bytes signature;
    }
    struct OrderComponents {
        address offerer;
        address zone;
        OfferItem[] offer;
        ConsiderationItem[] consideration;
        OrderType orderType;
        uint256 startTime;
        uint256 endTime;
        bytes32 zoneHash;
        uint256 salt;
        bytes32 conduitKey;
        uint256 counter;
    }
    struct OrderParameters {
        address offerer;
        address zone;
        OfferItem[] offer;
        ConsiderationItem[] consideration;
        OrderType orderType;
        uint256 startTime;
        uint256 endTime;
        bytes32 zoneHash;
        uint256 salt;
        bytes32 conduitKey;
        uint256 totalOriginalConsiderationItems;
    }
    struct ReceivedItem {
        ItemType itemType;
        address token;
        uint256 identifier;
        uint256 amount;
        address payable recipient;
    }
    struct SpentItem {
        ItemType itemType;
        address token;
        uint256 identifier;
        uint256 amount;
    }

    error BadContractSignature();
    error BadFraction();
    error BadReturnValueFromERC20OnTransfer(address token, address from, address to, uint256 amount);
    error BadSignatureV(uint8 v);
    error CannotCancelOrder();
    error ConsiderationCriteriaResolverOutOfRange();
    error ConsiderationLengthNotEqualToTotalOriginal();
    error ConsiderationNotMet(uint256 orderIndex, uint256 considerationIndex, uint256 shortfallAmount);
    error CriteriaNotEnabledForItem();
    error ERC1155BatchTransferGenericFailure(address token, address from, address to, uint256[] identifiers, uint256[] amounts);
    error InexactFraction();
    error InsufficientNativeTokensSupplied();
    error Invalid1155BatchTransferEncoding();
    error InvalidBasicOrderParameterEncoding();
    error InvalidCallToConduit(address conduit);
    error InvalidConduit(bytes32 conduitKey, address conduit);
    error InvalidContractOrder(bytes32 orderHash);
    error InvalidERC721TransferAmount(uint256 amount);
    error InvalidFulfillmentComponentData();
    error InvalidMsgValue(uint256 value);
    error InvalidNativeOfferItem();
    error InvalidProof();
    error InvalidRestrictedOrder(bytes32 orderHash);
    error InvalidSignature();
    error InvalidSigner();
    error InvalidTime(uint256 startTime, uint256 endTime);
    error MismatchedFulfillmentOfferAndConsiderationComponents(uint256 fulfillmentIndex);
    error MissingFulfillmentComponentOnAggregation(Side side);
    error MissingItemAmount();
    error MissingOriginalConsiderationItems();
    error NativeTokenTransferGenericFailure(address account, uint256 amount);
    error NoContract(address account);
    error NoReentrantCalls();
    error NoSpecifiedOrdersAvailable();
    error OfferAndConsiderationRequiredOnFulfillment();
    error OfferCriteriaResolverOutOfRange();
    error OrderAlreadyFilled(bytes32 orderHash);
    error OrderCriteriaResolverOutOfRange(Side side);
    error OrderIsCancelled(bytes32 orderHash);
    error OrderPartiallyFilled(bytes32 orderHash);
    error PartialFillsNotEnabledForOrder();
    error TokenTransferGenericFailure(address token, address from, address to, uint256 identifier, uint256 amount);
    error UnresolvedConsiderationCriteria(uint256 orderIndex, uint256 considerationIndex);
    error UnresolvedOfferCriteria(uint256 orderIndex, uint256 offerIndex);
    error UnusedItemParameters();

    event CounterIncremented(uint256 newCounter, address indexed offerer);
    event OrderCancelled(bytes32 orderHash, address indexed offerer, address indexed zone);
    event OrderFulfilled(bytes32 orderHash, address indexed offerer, address indexed zone, address recipient, SpentItem[] offer, ReceivedItem[] consideration);
    event OrderValidated(bytes32 orderHash, OrderParameters orderParameters);
    event OrdersMatched(bytes32[] orderHashes);

    receive() external payable;

    function cancel(OrderComponents[] memory orders) external returns (bool cancelled);
    function fulfillAdvancedOrder(AdvancedOrder memory, CriteriaResolver[] memory, bytes32 fulfillerConduitKey, address recipient) external payable returns (bool fulfilled);
    function fulfillAvailableAdvancedOrders(AdvancedOrder[] memory, CriteriaResolver[] memory, FulfillmentComponent[][] memory, FulfillmentComponent[][] memory, bytes32 fulfillerConduitKey, address recipient, uint256 maximumFulfilled) external payable returns (bool[] memory, Execution[] memory);
    function fulfillAvailableOrders(Order[] memory, FulfillmentComponent[][] memory, FulfillmentComponent[][] memory, bytes32 fulfillerConduitKey, uint256 maximumFulfilled) external payable returns (bool[] memory, Execution[] memory);
    function fulfillBasicOrder(BasicOrderParameters memory parameters) external payable returns (bool fulfilled);
    function fulfillBasicOrder_efficient_6GL6yc(BasicOrderParameters memory parameters) external payable returns (bool fulfilled);
    function fulfillOrder(Order memory, bytes32 fulfillerConduitKey) external payable returns (bool fulfilled);
    function getContractOffererNonce(address contractOfferer) external view returns (uint256 nonce);
    function getCounter(address offerer) external view returns (uint256 counter);
    function getOrderHash(OrderComponents memory) external view returns (bytes32 orderHash);
    function getOrderStatus(bytes32 orderHash) external view returns (bool isValidated, bool isCancelled, uint256 totalFilled, uint256 totalSize);
    function incrementCounter() external returns (uint256 newCounter);
    function information() external view returns (string memory version, bytes32 domainSeparator, address conduitController);
    function matchAdvancedOrders(AdvancedOrder[] memory, CriteriaResolver[] memory, Fulfillment[] memory, address recipient) external payable returns (Execution[] memory);
    function matchOrders(Order[] memory, Fulfillment[] memory) external payable returns (Execution[] memory);
    function name() external pure returns (string memory);
    function validate(Order[] memory) external returns (bool);
}