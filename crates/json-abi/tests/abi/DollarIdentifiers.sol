interface DollarIdentifiers {
    type _dEnum is uint8;
    type _dUDVT is uint256;
    struct _dStruct {
        uint256 _dField;
    }

    function _dFunction(_dStruct memory _dStructArg, _dEnum _dEnumArg, _dUDVT _dUDVTArg) external;
    function _dPublicVariable() external view returns (uint256 _dField);
}