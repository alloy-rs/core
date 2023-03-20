use ethers_abi_enc::sol;

use ethers_abi_enc::SolType;

#[test]
fn expand_and_contract() {
    type B32 = sol! {(bytes32, address)};
    assert_eq!(B32::sol_type_name(), "tuple(bytes32,address)");

    type Complex = sol! {((address, address)[],address)};
    assert_eq!(
        Complex::sol_type_name(),
        "tuple(tuple(address,address)[],address)"
    );

    type Gamut = sol! {
        (
            address, bool[], bytes15[12], uint256, uint24, int8, int56, (function, string, bytes,)
        )
    };
    assert_eq!(
        Gamut::sol_type_name(),
        "tuple(address,bool[],bytes15[12],uint256,uint24,int8,int56,tuple(function,string,bytes))"
    );
}
