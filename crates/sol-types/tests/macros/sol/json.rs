use alloy_json_abi::{Function, JsonAbi, Param, StateMutability};
use alloy_primitives::{Address, B256, I256, Signed, U256};
use alloy_sol_types::{SolCall, SolError, SolEvent, SolStruct, sol};
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn large_array() {
    sol!(
        #[sol(abi)]
        #[derive(Debug)]
        LargeArray,
        concat!(env!("CARGO_MANIFEST_DIR"), "/../json-abi/tests/abi/LargeArray.json")
    );

    let call = LargeArray::callWithLongArrayCall { longArray: [0; 128] };
    let _ = format!("{call:#?}");

    assert_eq!(LargeArray::callWithLongArrayCall::SIGNATURE, "callWithLongArray(uint64[128])");
    let contract = LargeArray::abi::contract();
    assert_eq!(
        contract,
        JsonAbi {
            constructor: None,
            fallback: None,
            receive: None,
            errors: Default::default(),
            events: Default::default(),
            functions: [(
                "callWithLongArray".into(),
                vec![Function {
                    name: "callWithLongArray".into(),
                    inputs: vec![Param {
                        ty: "uint64[128]".into(),
                        name: "longArray".into(),
                        internal_type: None,
                        components: vec![],
                    }],
                    outputs: vec![],
                    state_mutability: StateMutability::View,
                }],
            )]
            .into(),
        }
    );
}

#[test]
fn seaport() {
    sol!(
        #[derive(Debug, PartialEq, Eq)]
        Seaport,
        "../json-abi/tests/abi/Seaport.json"
    );
    use Seaport::*;

    // Constructor with a single argument
    let _ = constructorCall { conduitController: Address::ZERO };

    // BasicOrderType is a uint8 UDVT
    let o1 = BasicOrderType::from(0u8);
    let o2: BasicOrderType = 0.into();

    assert_eq!(o1, o2);

    let o1_inner: u8 = o1.into();
    let o2_inner: u8 = o2.into();

    assert_eq!(o1_inner, o2_inner);

    // BasicOrderParameters is a struct that contains UDVTs (basicOrderType) and a
    // struct array. The only component should be the struct of the struct array.
    let root_type = "BasicOrderParameters(address considerationToken,uint256 considerationIdentifier,uint256 considerationAmount,address offerer,address zone,address offerToken,uint256 offerIdentifier,uint256 offerAmount,uint8 basicOrderType,uint256 startTime,uint256 endTime,bytes32 zoneHash,uint256 salt,bytes32 offererConduitKey,bytes32 fulfillerConduitKey,uint256 totalOriginalAdditionalRecipients,AdditionalRecipient[] additionalRecipients,bytes signature)";
    let component = "AdditionalRecipient(uint256 amount,address recipient)";

    assert_eq!(BasicOrderParameters::eip712_root_type(), root_type);
    assert_eq!(BasicOrderParameters::eip712_components(), [Cow::Borrowed(component)]);
    assert_eq!(
        <BasicOrderParameters as SolStruct>::eip712_encode_type(),
        root_type.to_string() + component
    );
}

// https://etherscan.io/address/0x1111111254eeb25477b68fb85ed929f73a960582#code
sol!(
    #[sol(docs = false)]
    #[derive(Debug)]
    AggregationRouterV5,
    "../json-abi/tests/abi/AggregationRouterV5.json"
);

// Handle multiple identical error objects in the JSON ABI
// https://github.com/alloy-rs/core/issues/344
#[test]
fn aggregation_router_v5() {
    assert_eq!(
        <AggregationRouterV5::ETHTransferFailed as SolError>::SIGNATURE,
        "ETHTransferFailed()"
    );
    assert_eq!(<AggregationRouterV5::InvalidMsgValue as SolError>::SIGNATURE, "InvalidMsgValue()");
}

// Handle contract types in JSON ABI
// https://github.com/alloy-rs/core/issues/351
#[test]
fn uniswap_v3_position() {
    // https://etherscan.io/address/0x8638fbd429b19249bb3bcf3ec72d07a657e49642#code
    sol!(UniswapV3Position, "../json-abi/tests/abi/UniswapV3Position.json");

    let _ = UniswapV3Position::getLiquidityByRangeCall {
        pool_: Address::ZERO,
        self_: Address::ZERO,
        lowerTick_: Signed::ZERO,
        upperTick_: Signed::ZERO,
    };
    assert_eq!(
        UniswapV3Position::getLiquidityByRangeCall::SIGNATURE,
        "getLiquidityByRange(address,address,int24,int24)"
    );

    let _ = UniswapV3Position::getPositionIdCall {
        self_: Address::ZERO,
        lowerTick_: Signed::ZERO,
        upperTick_: Signed::ZERO,
    };
    assert_eq!(
        UniswapV3Position::getPositionIdCall::SIGNATURE,
        "getPositionId(address,int24,int24)"
    );
}

// Ensure a trailing comma for single-element tuples in old JSON ABI
// https://github.com/alloy-rs/core/issues/360
#[test]
fn double_exponent_interest_setter() {
    // https://etherscan.io/address/0xef2ed07cc7a0825ced8ac1a67f88a0e17414fa6c#code
    sol!(DoubleExponentInterestSetter, "../json-abi/tests/abi/DoubleExponentInterestSetter.json");
    let _ = DoubleExponentInterestSetter::getInterestRateCall {
        _0: Address::ZERO,
        borrowWei: U256::ZERO,
        supplyWei: U256::ZERO,
    };
}

// Same as `event_tokenize_fields`
// https://github.com/alloy-rs/core/issues/361
#[test]
fn uniswap_v2_factory() {
    sol!(UniswapV2Factory, "../json-abi/tests/abi/UniswapV2Factory.json");
    let _ = UniswapV2Factory::PairCreated {
        token0: Address::ZERO,
        token1: Address::ZERO,
        pair: Address::ZERO,
        _3: U256::ZERO,
    };
}

sol!(GnosisSafe, "../json-abi/tests/abi/GnosisSafe.json");

// Fully qualify `SolInterface::NAME` which conflicted with the `NAME` call
// https://github.com/alloy-rs/core/issues/361
#[test]
fn gnosis_safe() {
    let GnosisSafe::NAMECall {} = GnosisSafe::NAMECall {};
    let GnosisSafe::NAMEReturn { _0: _ } = GnosisSafe::NAMEReturn { _0: String::new() };
}

// Have enough recursion depth to handle `BlurExchange` types
// https://github.com/alloy-rs/core/issues/371
#[test]
fn blur_exchange() {
    sol!(BlurExchange, "../json-abi/tests/abi/BlurExchange.json");
    let BlurExchange::NAMECall {} = BlurExchange::NAMECall {};
    let BlurExchange::NAMEReturn { _0: _ } = BlurExchange::NAMEReturn { _0: String::new() };
}

#[test]
fn zerox_exchange_proxy() {
    sol!(ZeroXExchangeProxy, "../json-abi/tests/abi/ZeroxExchangeProxy.json");
}

// TODO: Error and event with the same name
// https://github.com/alloy-rs/core/issues/376
#[test]
#[cfg(any())]
fn auction() {
    // https://etherscan.io/address/0xbb37a88508d464a1bb54cf627d05e39883ae0ef9
    sol!(Auction, "../json-abi/tests/abi/Auction.json");
}

// https://github.com/alloy-rs/core/issues/378
#[test]
fn uniswap_v2_factory_with_migrator() {
    // This contract has the same ABI as `UniswapV2Factory`
    // https://etherscan.io/address/0x1ffbe925f22fca796adf2a63313b8b70b5b1a7f4

    // https://etherscan.io/address/0xc1a2706ceb8c21967d5c930c00c8ed16480f7255
    sol!(UniswapV2FactoryWithMigrator, "../json-abi/tests/abi/UniswapV2FactoryWithMigrator.json");
}

// https://github.com/alloy-rs/core/issues/379
#[test]
fn junkyard() {
    // https://etherscan.io/address/0x2e4b0f20bdb1caa0886c531256efdaab925dbe72
    sol!(Junkyard, "../json-abi/tests/abi/Junkyard.json");
}

// Handle missing state mutability in JSON ABI
// https://github.com/alloy-rs/core/issues/485
#[test]
fn zrx_token() {
    // https://etherscan.io/address/0xe41d2489571d322189246dafa5ebde1f4699f498#code
    sol!(ZRXToken, "../json-abi/tests/abi/ZRXToken.json");

    let _ = ZRXToken::approveCall { _spender: Address::ZERO, _value: U256::ZERO };
    assert_eq!(ZRXToken::approveCall::SIGNATURE, "approve(address,uint256)");
}

// https://etherscan.io/address/0xBA12222222228d8Ba445958a75a0704d566BF2C8#code
sol!(
    #![sol(all_derives)]
    BalancerV2Vault,
    "../json-abi/tests/abi/BalancerV2Vault.json"
);

// Handle contract **array** types in JSON ABI
// https://github.com/alloy-rs/core/issues/585
#[test]
fn balancer_v2_vault() {
    let _ = BalancerV2Vault::PoolBalanceChanged {
        poolId: B256::ZERO,
        liquidityProvider: Address::ZERO,
        tokens: vec![Address::ZERO],
        deltas: vec![I256::ZERO],
        protocolFeeAmounts: vec![U256::ZERO],
    };
    assert_eq!(
        BalancerV2Vault::PoolBalanceChanged::SIGNATURE,
        "PoolBalanceChanged(bytes32,address,address[],int256[],uint256[])"
    );
}

// TODO: https://github.com/alloy-rs/core/issues/744
// #[test]
// fn eigenlayer_delegation_manager() {
//     sol!(DelegationManager, "../json-abi/tests/abi/DelegationManager.json");
// }

// TODO: https://github.com/alloy-rs/core/issues/746
// #[test]
// fn smartsession_bootstrap() {
//     sol!(Bootstrap, "../json-abi/tests/abi/Bootstrap.json");
// }

#[test]
fn inner_macros() {
    sol!(
        #[sol(all_derives)]
        Name,
        concat!("[", "]"),
    );
    #[allow(unused_imports)]
    use Name::*;
}

#[test]
fn ignore_unlinked_bytecode_attr() {
    sol! (
        // Inner
        #![sol(ignore_unlinked)]
        Unlinked,
        "../json-abi/tests/abi/SomeLibUser.json"
    );

    let _ = Unlinked::addCall { a: U256::ZERO, b: U256::ZERO };

    sol!(
        // Outer
        #[sol(ignore_unlinked)]
        AnotherUnlinked,
        "../json-abi/tests/abi/SomeLibUser.json"
    );

    let _ = AnotherUnlinked::addCall { a: U256::ZERO, b: U256::ZERO };
}
