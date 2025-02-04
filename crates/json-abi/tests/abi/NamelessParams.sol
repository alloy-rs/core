interface NamelessParams {
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    event CancelLockupStream(uint256 streamId, address indexed sender, address indexed recipient, address indexed asset, uint128 senderAmount, uint128 recipientAmount);
    event CreateLockupDynamicStream(uint256 streamId, address funder, address indexed sender, address indexed recipient, (uint128, uint128, uint128) amounts, address indexed asset, bool cancelable, bool transferable, (uint128, uint64, uint40)[] segments, (uint40, uint40) range, address broker);
    event CreateLockupLinearStream(uint256 streamId, address funder, address indexed sender, address indexed recipient, (uint128, uint128, uint128) amounts, address indexed asset, bool cancelable, bool transferable, (uint40, uint40, uint40) range, address broker);
    event RenounceLockupStream(uint256 indexed streamId);
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event TransferAdmin(address indexed oldAdmin, address indexed newAdmin);
    event WithdrawFromLockupStream(uint256 indexed streamId, address indexed to, address indexed asset, uint128 amount);
}