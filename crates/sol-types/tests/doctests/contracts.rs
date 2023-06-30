use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolCall, SolInterface};
use hex_literal::hex;

// Contracts generate a module with the same name, which contains all the items.
// This module will also contain 3 container enums, one for each:
// - functions: `<contract_name>Calls`
// - errors: `<contract_name>Errors`
// - events: `<contract_name>Events`
sol! {
    /// Interface of the ERC20 standard as defined in [the EIP].
    ///
    /// [the EIP]: https://eips.ethereum.org/EIPS/eip-20
    #[derive(Debug, PartialEq)]
    interface IERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);

        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
    }
}

#[test]
fn contracts() {
    // random mainnet ERC20 transfer
    // https://etherscan.io/tx/0x947332ff624b5092fb92e8f02cdbb8a50314e861a4b39c29a286b3b75432165e
    let data = hex!(
        "a9059cbb"
        "0000000000000000000000008bc47be1e3abbaba182069c89d08a61fa6c2b292"
        "0000000000000000000000000000000000000000000000000000000253c51700"
    );
    let expected = IERC20::transferCall {
        to: Address::from(hex!("8bc47be1e3abbaba182069c89d08a61fa6c2b292")),
        amount: U256::from(9995360000_u64),
    };

    assert_eq!(data[..4], IERC20::transferCall::SELECTOR);
    let decoded = IERC20::IERC20Calls::decode(&data, true).unwrap();
    assert_eq!(decoded, IERC20::IERC20Calls::transfer(expected));
    assert_eq!(decoded.encode(), data);
}
