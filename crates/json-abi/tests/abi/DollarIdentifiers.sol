interface DollarIdentifiers {
    #[sol(rename = "$dEnum")] type _dEnum is uint8;
    #[sol(rename = "$dUDVT")] type _dUDVT is uint256;
    #[sol(rename = "$dStruct")] struct _dStruct {
        #[sol(rename = "$dField")] uint256 _dField;
    }

    #[sol(rename = "$dFunction")] function _dFunction(#[sol(rename = "$dStructArg")] _dStruct memory _dStructArg, #[sol(rename = "$dEnumArg")] _dEnum _dEnumArg, #[sol(rename = "$dUDVTArg")] _dUDVT _dUDVTArg) external;
    #[sol(rename = "$dPublicVariable")] function _dPublicVariable() external view returns (#[sol(rename = "$dField")] uint256 _dField);
}