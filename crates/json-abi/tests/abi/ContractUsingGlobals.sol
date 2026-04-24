library Interface {
    struct InterfaceStruct {
        GlobalEnum kind;
        uint256 count;
    }
}

interface ContractUsingGlobals {
    type GlobalEnum is uint8;
    type GlobalUDT is uint256;
    struct GlobalStruct {
        uint256 value;
        GlobalEnum enum_;
    }

    error GlobalError(GlobalStruct payload);

    event GlobalEvent(GlobalStruct payload, GlobalUDT amount);

    function emitGlobalEvent(Interface.InterfaceStruct memory data) external;
    function triggerError() external pure;
}