use alloy_sol_types::sol;

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
    enum MaxEnum {
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
        _16,
        _17,
        _18,
        _19,
        _20,
        _21,
        _22,
        _23,
        _24,
        _25,
        _26,
        _27,
        _28,
        _29,
        _30,
        _31,
        _32,
        _33,
        _34,
        _35,
        _36,
        _37,
        _38,
        _39,
        _40,
        _41,
        _42,
        _43,
        _44,
        _45,
        _46,
        _47,
        _48,
        _49,
        _50,
        _51,
        _52,
        _53,
        _54,
        _55,
        _56,
        _57,
        _58,
        _59,
        _60,
        _61,
        _62,
        _63,
        _64,
        _65,
        _66,
        _67,
        _68,
        _69,
        _70,
        _71,
        _72,
        _73,
        _74,
        _75,
        _76,
        _77,
        _78,
        _79,
        _80,
        _81,
        _82,
        _83,
        _84,
        _85,
        _86,
        _87,
        _88,
        _89,
        _90,
        _91,
        _92,
        _93,
        _94,
        _95,
        _96,
        _97,
        _98,
        _99,
        _100,
        _101,
        _102,
        _103,
        _104,
        _105,
        _106,
        _107,
        _108,
        _109,
        _110,
        _111,
        _112,
        _113,
        _114,
        _115,
        _116,
        _117,
        _118,
        _119,
        _120,
        _121,
        _122,
        _123,
        _124,
        _125,
        _126,
        _127,
        _128,
        _129,
        _130,
        _131,
        _132,
        _133,
        _134,
        _135,
        _136,
        _137,
        _138,
        _139,
        _140,
        _141,
        _142,
        _143,
        _144,
        _145,
        _146,
        _147,
        _148,
        _149,
        _150,
        _151,
        _152,
        _153,
        _154,
        _155,
        _156,
        _157,
        _158,
        _159,
        _160,
        _161,
        _162,
        _163,
        _164,
        _165,
        _166,
        _167,
        _168,
        _169,
        _170,
        _171,
        _172,
        _173,
        _174,
        _175,
        _176,
        _177,
        _178,
        _179,
        _180,
        _181,
        _182,
        _183,
        _184,
        _185,
        _186,
        _187,
        _188,
        _189,
        _190,
        _191,
        _192,
        _193,
        _194,
        _195,
        _196,
        _197,
        _198,
        _199,
        _200,
        _201,
        _202,
        _203,
        _204,
        _205,
        _206,
        _207,
        _208,
        _209,
        _210,
        _211,
        _212,
        _213,
        _214,
        _215,
        _216,
        _217,
        _218,
        _219,
        _220,
        _221,
        _222,
        _223,
        _224,
        _225,
        _226,
        _227,
        _228,
        _229,
        _230,
        _231,
        _232,
        _233,
        _234,
        _235,
        _236,
        _237,
        _238,
        _239,
        _240,
        _241,
        _242,
        _243,
        _244,
        _245,
        _246,
        _247,
        _248,
        _249,
        _250,
        _251,
        _252,
        _253,
        _254,
        _255,
    }
}

sol! {
    enum TooBigEnum {
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
        _16,
        _17,
        _18,
        _19,
        _20,
        _21,
        _22,
        _23,
        _24,
        _25,
        _26,
        _27,
        _28,
        _29,
        _30,
        _31,
        _32,
        _33,
        _34,
        _35,
        _36,
        _37,
        _38,
        _39,
        _40,
        _41,
        _42,
        _43,
        _44,
        _45,
        _46,
        _47,
        _48,
        _49,
        _50,
        _51,
        _52,
        _53,
        _54,
        _55,
        _56,
        _57,
        _58,
        _59,
        _60,
        _61,
        _62,
        _63,
        _64,
        _65,
        _66,
        _67,
        _68,
        _69,
        _70,
        _71,
        _72,
        _73,
        _74,
        _75,
        _76,
        _77,
        _78,
        _79,
        _80,
        _81,
        _82,
        _83,
        _84,
        _85,
        _86,
        _87,
        _88,
        _89,
        _90,
        _91,
        _92,
        _93,
        _94,
        _95,
        _96,
        _97,
        _98,
        _99,
        _100,
        _101,
        _102,
        _103,
        _104,
        _105,
        _106,
        _107,
        _108,
        _109,
        _110,
        _111,
        _112,
        _113,
        _114,
        _115,
        _116,
        _117,
        _118,
        _119,
        _120,
        _121,
        _122,
        _123,
        _124,
        _125,
        _126,
        _127,
        _128,
        _129,
        _130,
        _131,
        _132,
        _133,
        _134,
        _135,
        _136,
        _137,
        _138,
        _139,
        _140,
        _141,
        _142,
        _143,
        _144,
        _145,
        _146,
        _147,
        _148,
        _149,
        _150,
        _151,
        _152,
        _153,
        _154,
        _155,
        _156,
        _157,
        _158,
        _159,
        _160,
        _161,
        _162,
        _163,
        _164,
        _165,
        _166,
        _167,
        _168,
        _169,
        _170,
        _171,
        _172,
        _173,
        _174,
        _175,
        _176,
        _177,
        _178,
        _179,
        _180,
        _181,
        _182,
        _183,
        _184,
        _185,
        _186,
        _187,
        _188,
        _189,
        _190,
        _191,
        _192,
        _193,
        _194,
        _195,
        _196,
        _197,
        _198,
        _199,
        _200,
        _201,
        _202,
        _203,
        _204,
        _205,
        _206,
        _207,
        _208,
        _209,
        _210,
        _211,
        _212,
        _213,
        _214,
        _215,
        _216,
        _217,
        _218,
        _219,
        _220,
        _221,
        _222,
        _223,
        _224,
        _225,
        _226,
        _227,
        _228,
        _229,
        _230,
        _231,
        _232,
        _233,
        _234,
        _235,
        _236,
        _237,
        _238,
        _239,
        _240,
        _241,
        _242,
        _243,
        _244,
        _245,
        _246,
        _247,
        _248,
        _249,
        _250,
        _251,
        _252,
        _253,
        _254,
        _255,
        _256,
    }
}

sol! {
    struct Mappings {
        mapping(mapping(a b => c d) e => mapping(f g => h i) j) map;
    }
}

sol! {
    mapping(bool => mapping(address => uint256[])[])[][] public nestedMapArray;
}

sol! {
    mapping(mapping(int => int) => int) public mapKeyOfMap;
}

sol! {
    function mappings(mapping(uint256 a => bool b), mapping(bool => bool) x);
}

sol! {
    struct FunctionTypes {
        function(function(bool) external pure returns (function(function())) f) external returns (function()) c;
    }

    function functionTypes(FunctionTypes f) returns (function(function(function(), function())), function(function(), function()));
}

fn main() {}
