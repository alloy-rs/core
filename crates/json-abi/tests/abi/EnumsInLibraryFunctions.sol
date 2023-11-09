interface EnumsInLibraryFunctions {
    type TheEnum is uint8;

    function enumArray(TheEnum[2] memory x) external pure returns (TheEnum[2] memory);
    function enumArrays(TheEnum[][69][] memory x) external pure returns (TheEnum[][69][] memory);
    function enumDynArray(TheEnum[] memory x) external pure returns (TheEnum[] memory);
    function enum_(TheEnum x) external pure returns (TheEnum);
}