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

contract Contract {
    LibA.Struct internal aValue;
    LibB.Struct internal bValue;

    struct Self {
        uint c;
    }

    constructor(
        LibA.Struct memory aValue_,
        LibB.Struct memory bValue_,
        Self memory s
    )
    {
        aValue = aValue_;
        bValue = bValue_;
    }

    function fn(
        LibA.Struct memory aValue_,
        LibB.Struct memory bValue_,
        Self memory s
    ) public
    {
        aValue = aValue_;
        bValue = bValue_;
    }
}
