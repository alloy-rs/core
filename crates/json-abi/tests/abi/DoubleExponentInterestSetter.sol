interface DoubleExponentInterestSetter {
    function getCoefficients() external view returns (uint256[] memory);
    function getInterestRate(address, uint256 borrowWei, uint256 supplyWei) external view returns ((uint256,) memory);
    function getMaxAPR() external view returns (uint256);
}