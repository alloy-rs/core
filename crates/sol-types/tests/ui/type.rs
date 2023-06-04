use ethers_sol_types::sol;

sol! {
    struct BuiltinTypes {
        address a;
        address payable ap;
        string s;
        bool b;

        bytes b0;
        bytes1 b1;
        bytes2 b2;
        bytes3 b3;
        bytes4 b4;
        bytes5 b5;
        bytes6 b6;
        bytes7 b7;
        bytes8 b8;
        bytes9 b9;
        bytes10 b10;
        bytes11 b11;
        bytes12 b12;
        bytes13 b13;
        bytes14 b14;
        bytes15 b15;
        bytes16 b16;
        bytes17 b17;
        bytes18 b18;
        bytes19 b19;
        bytes20 b20;
        bytes21 b21;
        bytes22 b22;
        bytes23 b23;
        bytes24 b24;
        bytes25 b25;
        bytes26 b26;
        bytes27 b27;
        bytes28 b28;
        bytes29 b29;
        bytes30 b30;
        bytes31 b31;
        bytes32 b32;

        int i;
        int8 i8;
        int16 i16;
        int24 i24;
        int32 i32;
        int40 i40;
        int48 i48;
        int56 i56;
        int64 i64;
        int72 i72;
        int80 i80;
        int88 i88;
        int96 i96;
        int104 i104;
        int112 i112;
        int120 i120;
        int128 i128;
        int136 i136;
        int144 i144;
        int152 i152;
        int160 i160;
        int168 i168;
        int176 i176;
        int184 i184;
        int192 i192;
        int200 i200;
        int208 i208;
        int216 i216;
        int224 i224;
        int232 i232;
        int240 i240;
        int248 i248;
        int256 i256;

        uint u;
        uint8 u8;
        uint16 u16;
        uint24 u24;
        uint32 u32;
        uint40 u40;
        uint48 u48;
        uint56 u56;
        uint64 u64;
        uint72 u72;
        uint80 u80;
        uint88 u88;
        uint96 u96;
        uint104 u104;
        uint112 u112;
        uint120 u120;
        uint128 u128;
        uint136 u136;
        uint144 u144;
        uint152 u152;
        uint160 u160;
        uint168 u168;
        uint176 u176;
        uint184 u184;
        uint192 u192;
        uint200 u200;
        uint208 u208;
        uint216 u216;
        uint224 u224;
        uint232 u232;
        uint240 u240;
        uint248 u248;
        uint256 u256;
    }
}

sol! {
    struct ArrayTypes {
        string[] s;
        bool[] b;
        address[] a;
        bytes[] b0;
        uint[] u0;
        int[] i0;

        string[][] ss;
        string[][][] sss;

        (bool, uint)[] bu;
        (bool, uint)[][] bubu;
        (bool, uint)[][][] bububu;
    }
}

sol! {
    struct TupleTypes {
        (bool,) one;
        (string, bool) two;
        ((string, bool), uint) nested;
        (string, ((bool,), uint)) nested2;
        (string, ((bool,), uint))[] nestedArray;
        (string, ((bool,), uint))[][] nestedArrayArray;
    }
}

sol! {
    struct EmptyTuple {
        () t;
    }
}

sol! {
    struct SingleElementTupleWithNoTrailingComma {
        (bool) t;
    }
}

sol! {
    struct Bytes0 {
        bytes0 a;
    }
}

sol! {
    struct BytesTooHigh {
        bytes33 a;
    }
}

sol! {
    struct Uint0 {
        uint0 a;
    }
}

sol! {
    struct UintTooHigh {
        uint264 a;
    }
}

sol! {
    struct UintNotMultipleOf8 {
        uint7 a;
    }
}

sol! {
    struct Int0 {
        int0 a;
    }
}

sol! {
    struct IntTooHigh {
        int264 a;
    }
}

sol! {
    struct IntNotMultipleOf8 {
        int7 a;
    }
}

sol! {
    struct CustomTypes {
        bytes_ a;
        bytes_32 b;
        uint_ c;
        uint_256 d;
        int_ e;
        int_256 f;
    }
}

sol! {
    struct RecursiveA {
        RecursiveB b;
    }

    struct RecursiveB {
        RecursiveA a;
    }
}

fn main() {}
