use alloy_sol_types::sol;

compile_error!("No fail cases");

sol! {
    struct Custom {
        bool a;
    }

    struct ValidStorageStruct {
        bool memory a;
        bool storage b;
        bool calldata c;

        bytes memory d;
        bytes storage e;
        bytes calldata f;

        string memory g;
        string storage h;
        string calldata i;

        bool[] memory j;
        bool[] storage k;
        bool[] calldata l;

        Custom memory m;
        Custom storage n;
        Custom calldata o;

        Custom[3] memory p;
        Custom[3] storage q;
        Custom[3] calldata r;
    }

    function validStorageFunction(
        bool memory a,
        bool storage b,
        bool calldata c,

        bytes memory d,
        bytes storage e,
        bytes calldata f,

        string memory g,
        string storage h,
        string calldata i,

        bool[] memory j,
        bool[] storage k,
        bool[] calldata l,

        Custom memory m,
        Custom storage n,
        Custom calldata o,

        Custom[3] memory p,
        Custom[3] storage q,
        Custom[3] calldata r,
    );

    error validStorageError(
        bool memory a,
        bool storage b,
        bool calldata c,

        bytes memory d,
        bytes storage e,
        bytes calldata f,

        string memory g,
        string storage h,
        string calldata i,

        bool[] memory j,
        bool[] storage k,
        bool[] calldata l,

        Custom memory m,
        Custom storage n,
        Custom calldata o,

        Custom[3] memory p,
        Custom[3] storage q,
        Custom[3] calldata r,
    );

    error validStorageEvent(
        bool memory a,
        bool storage b,
        bool calldata c,

        bytes memory d,
        bytes storage e,
        bytes calldata f,

        string memory g,
        string storage h,
        string calldata i,

        bool[] memory j,
        bool[] storage k,
        bool[] calldata l,

        Custom memory m,
        Custom storage n,
        Custom calldata o,

        Custom[3] memory p,
        Custom[3] storage q,
        Custom[3] calldata r,
    );
}

fn main() {}
