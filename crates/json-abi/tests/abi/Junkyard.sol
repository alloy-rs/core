interface Junkyard {
    error InvalidAddress();
    error InvalidAddressString();
    error NotApprovedByGateway();

    event ContractValueUpdate(string, string);
    event ERC20PaymentReleased(address indexed token, address to, uint256 amount);
    event NewClaim(uint256 indexed, address, uint256);
    event NewFishingEntry(address indexed, uint256, bytes32);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address account);
    event PayeeAdded(address account, uint256 shares);
    event PaymentReceived(address from, uint256 amount);
    event PaymentReleased(address to, uint256 amount);
    event PricesChange(uint256, uint256);
    event Unpaused(address account);

    receive() external payable;

    function GAS_RECEIVER() external view returns (address);
    function batchTransferNFTs(address[] memory tokenAddresses, uint256[] memory tokenIds) external;
    function claim(uint256 requestId, uint256 tokenUID) external payable;
    function execute(bytes32 commandId, string memory sourceChain, string memory sourceAddress, bytes memory payload) external;
    function executeWithToken(bytes32 commandId, string memory sourceChain, string memory sourceAddress, bytes memory payload, string memory tokenSymbol, uint256 amount) external;
    function fishing(uint256 _qt) external payable;
    function gateway() external view returns (address);
    function managerAddress() external view returns (string memory);
    function managerChain() external view returns (string memory);
    function owner() external view returns (address);
    function pause() external;
    function paused() external view returns (bool);
    function payee(uint256 index) external view returns (address);
    function prices(uint256) external view returns (uint256);
    function releasable(address account) external view returns (uint256);
    function releasable(address token, address account) external view returns (uint256);
    function release(address payable account) external;
    function release(address token, address account) external;
    function released(address token, address account) external view returns (uint256);
    function released(address account) external view returns (uint256);
    function renounceOwnership() external;
    function setManagerAddress(string memory newManagerAddr) external;
    function setManagerChain(string memory newManagerChain) external;
    function setPrice(uint256 _qt, uint256 _newPrice) external;
    function setStorageAddress(string memory newStorageAddr) external;
    function setStorageChain(string memory newStorageChain) external;
    function shares(address account) external view returns (uint256);
    function storageAddress() external view returns (string memory);
    function storageChain() external view returns (string memory);
    function totalReleased(address token) external view returns (uint256);
    function totalReleased() external view returns (uint256);
    function totalShares() external view returns (uint256);
    function transferOwnership(address newOwner) external;
    function unpause() external;
}