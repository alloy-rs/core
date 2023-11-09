interface EnumsInLibraryFunctions {
    type TheEnum is uint8;

    function enumArray(TheEnum[2] x) external pure returns (TheEnum[2]);
    function enumArrays(TheEnum[][69][] x) external pure returns (TheEnum[][69][]);
    function enumDynArray(TheEnum[] x) external pure returns (TheEnum[]);
    function enum_(TheEnum x) external pure returns (TheEnum);
}