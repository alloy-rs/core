library Interface {
    struct InterfaceStruct {
        GlobalEnum kind;
        uint256 count;
    }
}

interface ContractUsingGlobals {
    error GlobalError(GlobalStruct payload);

    event GlobalEvent(GlobalStruct payload, GlobalUDT amount);

    function emitGlobalEvent(Interface.InterfaceStruct memory data) external;
    function triggerError() external pure;
}
type GlobalEnum is uint8;
type GlobalUDT is uint256;
struct GlobalStruct {
    uint256 value;
    GlobalEnum enum_;
}
