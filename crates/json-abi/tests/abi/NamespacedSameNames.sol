library LibA {
    struct Struct {
        uint64 field64;
    }
}

library LibB {
    struct Struct {
        uint128 field128;
    }
}

interface NamespacedSameNames {
    function fn(LibA.Struct memory aValue_, LibB.Struct memory bValue_) external;
}