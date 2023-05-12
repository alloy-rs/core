use ethers_abi_enc::sol;

sol! {
    struct BadMemoryStruct {
        bool memory a;
    }
}

sol! {
    struct BadStorageStruct {
        bool storage a;
    }
}

sol! {
    struct BadCalldataStruct {
        bool calldata a;
    }
}

sol! {
    function badMemoryFunction(
        bool memory a,
    );
}

sol! {
    function badStorageFunction(
        bool storage a,
    );
}

sol! {
    function badCalldataFunction(
        bool calldata a,
    );
}

sol! {
    struct Custom {
        bool a;
    }

    function validStorage(
        bytes memory a,
        bytes storage b,
        bytes calldata c,

        string memory d,
        string storage e,
        string calldata f,

        bool[] memory g,
        bool[] storage h,
        bool[] calldata i,

        Custom memory j,
        Custom storage k,
        Custom calldata l,
    ) external;
}

fn main() {}
