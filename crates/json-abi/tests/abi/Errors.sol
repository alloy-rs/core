interface Errors {
    error Err(string msg);
    error InvalidAmount(uint256 requested, uint256 maximum);
    error LiqEng_CannotModifySAFE();
    error LiqEng_DustySAFE();
    error LiqEng_InvalidAmounts();
    error LiqEng_InvalidSAFESaviourOperation();
    error LiqEng_LiquidationLimitHit();
    error LiqEng_NullAuction();
    error LiqEng_NullCollateralToSell();
    error LiqEng_OnlyLiqEng();
    error LiqEng_SAFENotUnsafe();
    error LiqEng_SaviourNotAuthorized();
    error LiqEng_SaviourNotOk();
    error MyError();
}