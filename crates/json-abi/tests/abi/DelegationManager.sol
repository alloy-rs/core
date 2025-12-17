library IDelegationManager {
    struct OperatorDetails {
        address __deprecated_earningsReceiver;
        address delegationApprover;
        uint32 stakerOptOutWindowBlocks;
    }
    struct QueuedWithdrawalParams {
        address[] strategies;
        uint256[] shares;
        address withdrawer;
    }
    struct Withdrawal {
        address staker;
        address delegatedTo;
        address withdrawer;
        uint256 nonce;
        uint32 startTimestamp;
        address[] strategies;
        DelegatedShares[] delegatedShares;
    }
}

library ISignatureUtils {
    struct SignatureWithExpiry {
        bytes signature;
        uint256 expiry;
    }
}

interface DelegationManager {
    type DelegatedShares is uint256;

    error ActivelyDelegated();
    error AllocationDelaySet();
    error CallerCannotUndelegate();
    error CurrentlyPaused();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputArrayLengthZero();
    error InvalidNewPausedStatus();
    error InvalidSignatureEIP1271();
    error InvalidSignatureSigner();
    error NotActivelyDelegated();
    error OnlyPauser();
    error OnlyStrategyManagerOrEigenPodManager();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error OperatorsCannotUndelegate();
    error SaltSpent();
    error SignatureExpired();
    error StakerOptOutWindowBlocksCannotDecrease();
    error StakerOptOutWindowBlocksExceedsMax();
    error WithdrawalDelayExeedsMax();
    error WithdrawalDelayNotElapsed();
    error WithdrawalExeedsMax();
    error WithdrawalNotQueued();
    error WithdrawerNotCaller();
    error WithdrawerNotStaker();

    event Initialized(uint8 version);
    event OperatorDetailsModified(address indexed operator, IDelegationManager.OperatorDetails newOperatorDetails);
    event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
    event OperatorRegistered(address indexed operator, IDelegationManager.OperatorDetails operatorDetails);
    event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event StakerDelegated(address indexed staker, address indexed operator);
    event StakerForceUndelegated(address indexed staker, address indexed operator);
    event StakerUndelegated(address indexed staker, address indexed operator);
    event Unpaused(address indexed account, uint256 newPausedStatus);
    event WithdrawalCompleted(bytes32 withdrawalRoot);
    event WithdrawalQueued(bytes32 withdrawalRoot, IDelegationManager.Withdrawal withdrawal);

    function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
    function DOMAIN_TYPEHASH() external view returns (bytes32);
    function LEGACY_MIN_WITHDRAWAL_DELAY_BLOCKS() external view returns (uint256);
    function LEGACY_WITHDRAWALS_TIMESTAMP() external view returns (uint32);
    function MIN_WITHDRAWAL_DELAY() external view returns (uint32);
    function STAKER_DELEGATION_TYPEHASH() external view returns (bytes32);
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function beaconChainETHStrategy() external view returns (address);
    function calculateCurrentStakerDelegationDigestHash(address staker, address operator, uint256 expiry) external view returns (bytes32);
    function calculateDelegationApprovalDigestHash(address staker, address operator, address _delegationApprover, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
    function calculateStakerDelegationDigestHash(address staker, uint256 _stakerNonce, address operator, uint256 expiry) external view returns (bytes32);
    function calculateWithdrawalRoot(IDelegationManager.Withdrawal memory withdrawal) external pure returns (bytes32);
    function completeQueuedWithdrawal(IDelegationManager.Withdrawal memory withdrawal, address[] memory tokens, bool receiveAsTokens) external;
    function completeQueuedWithdrawals(IDelegationManager.Withdrawal[] memory withdrawals, address[][] memory tokens, bool[] memory receiveAsTokens) external;
    function cumulativeWithdrawalsQueued(address) external view returns (uint256);
    function decreaseDelegatedShares(address staker, address strategy, uint256 removedShares) external;
    function delegateTo(address operator, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegateToBySignature(address staker, address operator, ISignatureUtils.SignatureWithExpiry memory stakerSignatureAndExpiry, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegatedTo(address) external view returns (address);
    function delegationApprover(address operator) external view returns (address);
    function delegationApproverSaltIsSpent(address, bytes32) external view returns (bool);
    function depositScalingFactors(address, address) external view returns (uint256);
    function domainSeparator() external view returns (bytes32);
    function eigenPodManager() external view returns (address);
    function getDelegatableShares(address staker) external view returns (address[] memory, uint256[] memory);
    function getDelegatableShares(address staker, address[] memory strategies) external view returns (uint256[] memory shares);
    function getOperatorDelegatedShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
    function increaseDelegatedShares(address staker, address strategy, uint256 existingPrincipalShares, uint256 addedShares) external;
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus) external;
    function isDelegated(address staker) external view returns (bool);
    function isOperator(address operator) external view returns (bool);
    function modifyOperatorDetails(IDelegationManager.OperatorDetails memory newOperatorDetails) external;
    function operatorDelegatedShares(address, address) external view returns (uint256);
    function operatorDetails(address operator) external view returns (IDelegationManager.OperatorDetails memory);
    function operatorShares(address operator, address strategy) external view returns (uint256);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pendingWithdrawals(bytes32) external view returns (bool);
    function queueWithdrawals(IDelegationManager.QueuedWithdrawalParams[] memory queuedWithdrawalParams) external returns (bytes32[] memory);
    function registerAsOperator(IDelegationManager.OperatorDetails memory registeringOperatorDetails, uint32 allocationDelay, string memory metadataURI) external;
    function renounceOwnership() external;
    function setPauserRegistry(address newPauserRegistry) external;
    function slasher() external view returns (address);
    function stakerNonce(address) external view returns (uint256);
    function stakerOptOutWindowBlocks(address operator) external view returns (uint256);
    function strategyManager() external view returns (address);
    function transferOwnership(address newOwner) external;
    function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
    function unpause(uint256 newPausedStatus) external;
    function updateOperatorMetadataURI(string memory metadataURI) external;
}