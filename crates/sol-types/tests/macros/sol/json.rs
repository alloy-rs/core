use alloy_json_abi::{Function, JsonAbi, Param, StateMutability};
use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolCall, SolError, SolStruct};
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn large_array() {
    sol!(
        #[sol(abi)]
        LargeArray,
        "../json-abi/tests/abi/LargeArray.json"
    );
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
    sol!(Seaport, "../json-abi/tests/abi/Seaport.json");
    use Seaport::*;

    // BasicOrderType is a uint8 UDVT
    let _ = BasicOrderType::from(0u8);

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

// Handle multiple identical error objects in the JSON ABI
// https://github.com/alloy-rs/core/issues/344
#[test]
fn aggregation_router_v5() {
    sol!(
        #[sol(docs = false)]
        AggregationRouterV5,
        "../json-abi/tests/abi/AggregationRouterV5.json"
    );

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
    sol!(UniswapV3Position, "../json-abi/tests/abi/UniswapV3Position.json");

    let _ = UniswapV3Position::getLiquidityByRangeCall {
        pool_: Address::ZERO,
        self_: Address::ZERO,
        lowerTick_: 0,
        upperTick_: 0,
    };
    assert_eq!(
        UniswapV3Position::getLiquidityByRangeCall::SIGNATURE,
        "getLiquidityByRange(address,address,int24,int24)"
    );

    let _ =
        UniswapV3Position::getPositionIdCall { self_: Address::ZERO, lowerTick_: 0, upperTick_: 0 };
    assert_eq!(
        UniswapV3Position::getPositionIdCall::SIGNATURE,
        "getPositionId(address,int24,int24)"
    );
}

// Ensure a trailing comma for single-element tuples in old JSON ABI
// https://github.com/alloy-rs/core/issues/360
#[test]
fn double_exponent_interest_setter() {
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

// Fully qualify `SolInterface::NAME` which conflicted with the `NAME` call
// https://github.com/alloy-rs/core/issues/361
#[test]
fn gnosis_safe() {
    sol!(GnosisSafe, "../json-abi/tests/abi/GnosisSafe.json");
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
fn zerox_proxy() {
    sol!(ZeroXExchangeProxy, "../json-abi/tests/abi/ZeroxExchangeProxy.json");
}
