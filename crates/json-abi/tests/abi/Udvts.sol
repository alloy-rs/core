interface Udvts {
    type ItemType is bytes32;
    type OrderType is uint256;
    type Side is bool;
    struct AdvancedOrder {
        OrderParameters parameters;
        uint120 numerator;
        uint120 denominator;
        bytes signature;
        bytes extraData;
    }
    struct ConsiderationItem {
        ItemType itemType;
        address token;
        uint256 identifierOrCriteria;
        uint256 startAmount;
        uint256 endAmount;
        address recipient;
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
        address recipient;
    }

    function fulfillAvailableAdvancedOrders(AdvancedOrder[] memory a, CriteriaResolver[] memory b, FulfillmentComponent[][] memory c, FulfillmentComponent[][] memory d, bytes32 fulfillerConduitKey, address recipient, uint256 maximumFulfilled) external payable returns (bool[] memory e, Execution[] memory f);
}