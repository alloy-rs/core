// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

library EnumsInLibraryFunctions {
    enum TheEnum {
        DoesntMatter
    }

    function enum_(TheEnum x) external pure returns (TheEnum) {
        return x;
    }

    function enumDynArray(TheEnum[] memory x) external pure returns (TheEnum[] memory) {
        return x;
    }

    function enumArray(TheEnum[2] memory x) external pure returns (TheEnum[2] memory) {
        return x;
    }

    function enumArrays(TheEnum[][69][] memory x) external pure returns (TheEnum[][69][] memory) {
        return x;
    }
}
