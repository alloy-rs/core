library ModuleManager {
    struct FallbackHandler {
        address handler;
        CallType calltype;
    }
}

interface Bootstrap {
    type CallType is bytes1;
    struct BootstrapConfig {
        address module;
        bytes data;
    }

    error AccountAccessUnauthorized();
    error CannotRemoveLastValidator();
    error HookAlreadyInstalled(address currentHook);
    error HookPostCheckFailed();
    error InvalidModule(address module);
    error LinkedList_EntryAlreadyInList(address entry);
    error LinkedList_InvalidEntry(address entry);
    error LinkedList_InvalidPage();
    error NoFallbackHandler(bytes4 selector);

    fallback() external payable;

    receive() external payable;

    function _getInitMSACalldata(BootstrapConfig[] memory _valdiators, BootstrapConfig[] memory _executors, BootstrapConfig memory _hook, BootstrapConfig[] memory _fallbacks) external view returns (bytes memory init);
    function entryPoint() external view returns (address);
    function getActiveFallbackHandler(bytes4 functionSig) external view returns (ModuleManager.FallbackHandler memory);
    function getActiveHook() external view returns (address hook);
    function getExecutorsPaginated(address cursor, uint256 size) external view returns (address[] memory array, address next);
    function getValidatorsPaginated(address cursor, uint256 size) external view returns (address[] memory array, address next);
    function initMSA(BootstrapConfig[] memory _valdiators, BootstrapConfig[] memory _executors, BootstrapConfig memory _hook, BootstrapConfig[] memory _fallbacks) external;
    function singleInitMSA(address validator, bytes memory data) external;
}