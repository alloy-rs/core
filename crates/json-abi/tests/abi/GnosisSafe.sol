interface GnosisSafe {
    type Operation is uint8;

    event AddedOwner(address owner);
    event ApproveHash(bytes32 indexed approvedHash, address indexed owner);
    event ChangedMasterCopy(address masterCopy);
    event ChangedThreshold(uint256 threshold);
    event DisabledModule(address module);
    event EnabledModule(address module);
    event ExecutionFailure(bytes32 txHash, uint256 payment);
    event ExecutionFromModuleFailure(address indexed module);
    event ExecutionFromModuleSuccess(address indexed module);
    event ExecutionSuccess(bytes32 txHash, uint256 payment);
    event RemovedOwner(address owner);
    event SignMsg(bytes32 indexed msgHash);

    constructor();

    fallback() external payable;

    function NAME() external view returns (string memory);
    function VERSION() external view returns (string memory);
    function addOwnerWithThreshold(address owner, uint256 _threshold) external;
    function approveHash(bytes32 hashToApprove) external;
    function approvedHashes(address, bytes32) external view returns (uint256);
    function changeMasterCopy(address _masterCopy) external;
    function changeThreshold(uint256 _threshold) external;
    function disableModule(address prevModule, address module) external;
    function domainSeparator() external view returns (bytes32);
    function enableModule(address module) external;
    function encodeTransactionData(address to, uint256 value, bytes memory data, Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, uint256 _nonce) external view returns (bytes memory);
    function execTransaction(address to, uint256 value, bytes memory data, Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address payable refundReceiver, bytes memory signatures) external returns (bool success);
    function execTransactionFromModule(address to, uint256 value, bytes memory data, Operation operation) external returns (bool success);
    function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Operation operation) external returns (bool success, bytes memory returnData);
    function getMessageHash(bytes memory message) external view returns (bytes32);
    function getModules() external view returns (address[] memory);
    function getModulesPaginated(address start, uint256 pageSize) external view returns (address[] memory array, address next);
    function getOwners() external view returns (address[] memory);
    function getThreshold() external view returns (uint256);
    function getTransactionHash(address to, uint256 value, bytes memory data, Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, uint256 _nonce) external view returns (bytes32);
    function isOwner(address owner) external view returns (bool);
    function isValidSignature(bytes memory _data, bytes memory _signature) external returns (bytes4);
    function nonce() external view returns (uint256);
    function removeOwner(address prevOwner, address owner, uint256 _threshold) external;
    function requiredTxGas(address to, uint256 value, bytes memory data, Operation operation) external returns (uint256);
    function setFallbackHandler(address handler) external;
    function setup(address[] memory _owners, uint256 _threshold, address to, bytes memory data, address fallbackHandler, address paymentToken, uint256 payment, address payable paymentReceiver) external;
    function signMessage(bytes memory _data) external;
    function signedMessages(bytes32) external view returns (uint256);
    function swapOwner(address prevOwner, address oldOwner, address newOwner) external;
}