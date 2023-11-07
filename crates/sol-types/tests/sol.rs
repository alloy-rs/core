use alloy_primitives::{hex, keccak256, Address, B256, I256, U256};
use alloy_sol_types::{eip712_domain, sol, SolCall, SolError, SolStruct, SolType};
use serde::Serialize;
use serde_json::Value;

#[test]
fn e2e() {
    sol! {
        struct MyStruct {
            uint256 a;
            bytes32 b;
            address[] c;
        }
    }

    sol! {
        struct MyStruct2 {
            MyStruct a;
            bytes32 b;
            address[] c;
        }
    }

    type MyTuple = sol! {
        (MyStruct, bytes32)
    };

    type LateBinding<A> = sol! {
        (A[], address)
    };

    type NestedArray = sol! {
        bool[2][]
    };

    sol! {
        type MyValueType is uint256;
    }

    <sol!(bool)>::abi_encode(&true);

    let a = MyStruct { a: U256::from(1), b: [0; 32].into(), c: Vec::new() };

    MyTuple::abi_encode(&(a.clone(), [0; 32]));
    MyStruct::abi_encode(&a);

    LateBinding::<MyStruct>::abi_encode(&(vec![a.clone(), a.clone()], Address::default()));

    MyStruct2::abi_encode(&MyStruct2 { a, b: [0; 32].into(), c: vec![] });

    NestedArray::abi_encode(&vec![[true, false], [true, false], [true, false]]);

    let mvt = MyValueType::from(U256::from(1));
    assert_eq!(
        mvt.abi_encode(),
        alloy_sol_types::sol_data::Uint::<256>::abi_encode(&U256::from(1))
    );
}

#[test]
fn function() {
    sol! {
        struct CustomStruct {
            address a;
            uint64 b;
        }

        function someFunction(
            uint256 basic,
            string memory string_,
            bytes calldata longBytes,
            address[] memory array,
            bool[2] memory fixedArray,
            CustomStruct struct_,
            CustomStruct[] structArray,
        ) returns (bool x);
    }

    let sig =
        "someFunction(uint256,string,bytes,address[],bool[2],(address,uint64),(address,uint64)[])";
    assert_eq!(someFunctionCall::SIGNATURE, sig);
    assert_eq!(someFunctionCall::SELECTOR, keccak256(sig)[..4]);

    let call = someFunctionCall {
        basic: U256::from(1),
        string_: "Hello World".to_owned(),
        longBytes: vec![0; 36],
        array: vec![Address::ZERO, Address::ZERO, Address::ZERO],
        fixedArray: [true, false],
        struct_: CustomStruct { a: Address::ZERO, b: 2 },
        structArray: vec![
            CustomStruct { a: Address::ZERO, b: 3 },
            CustomStruct { a: Address::ZERO, b: 4 },
            CustomStruct { a: Address::ZERO, b: 5 },
            CustomStruct { a: Address::ZERO, b: 6 },
        ],
    };
    let encoded = call.abi_encode();
    assert_eq!(encoded.len(), someFunctionCall::SELECTOR.len() + call.abi_encoded_size());

    assert_eq!(
        call.abi_encoded_size(),
        32 + (64 + 32) + (64 + 32 + 32) + (64 + 3 * 32) + 2 * 32 + (32 + 32) + (64 + 4 * (32 + 32))
    );
}

#[test]
fn function_returns() {
    sol! {
        #[derive(Debug, PartialEq)]
        function test() returns (uint256[]);
    }
    assert_eq!(
        testCall::abi_decode_returns(
            &hex!(
                "0000000000000000000000000000000000000000000000000000000000000020
                 0000000000000000000000000000000000000000000000000000000000000000"
            ),
            true,
        ),
        Ok(testReturn { _0: vec![] })
    );
    assert_eq!(
        testCall::abi_decode_returns(
            &hex!(
                "0000000000000000000000000000000000000000000000000000000000000020
                 0000000000000000000000000000000000000000000000000000000000000001
                 0000000000000000000000000000000000000000000000000000000000000002"
            ),
            true,
        ),
        Ok(testReturn { _0: vec![U256::from(2)] })
    );
    assert_eq!(
        testCall::abi_decode_returns(
            &hex!(
                "0000000000000000000000000000000000000000000000000000000000000020
                 0000000000000000000000000000000000000000000000000000000000000002
                 0000000000000000000000000000000000000000000000000000000000000042
                 0000000000000000000000000000000000000000000000000000000000000069"
            ),
            true,
        ),
        Ok(testReturn { _0: vec![U256::from(0x42), U256::from(0x69)] })
    );
}

#[test]
fn error() {
    sol! {
        error SomeError(int a, bool b);
    }

    let sig = "SomeError(int256,bool)";
    assert_eq!(SomeError::SIGNATURE, sig);
    assert_eq!(SomeError::SELECTOR, keccak256(sig)[..4]);

    let e = SomeError { a: I256::ZERO, b: false };
    assert_eq!(e.abi_encoded_size(), 64);
}

// Handle empty call encoding/decoding correctly
// https://github.com/alloy-rs/core/issues/158
#[test]
fn empty_call() {
    sol! {
        interface WETH {
            function deposit() external payable;
        }
    }
    use WETH::depositCall;

    assert_eq!(depositCall {}.abi_encode(), depositCall::SELECTOR);
    assert_eq!(depositCall {}.abi_encoded_size(), 0);
    let mut out = vec![];
    depositCall {}.abi_encode_raw(&mut out);
    assert!(out.is_empty());

    let depositCall {} = depositCall::abi_decode(&depositCall::SELECTOR, true).unwrap();
    let depositCall {} = depositCall::abi_decode_raw(&[], true).unwrap();
}

#[test]
fn function_names() {
    sol! {
        #[sol(extra_methods)]
        contract LeadingUnderscores {
            function f();
            function _f();
            function __f();
        }
    }
    use LeadingUnderscores::*;

    let call = LeadingUnderscoresCalls::f(fCall {});
    assert!(call.is_f());
    assert!(!call.is__f());
    assert!(!call.is___f());
}

#[test]
fn getters() {
    // modified from https://docs.soliditylang.org/en/latest/contracts.html#getter-functions
    sol! {
        struct Data {
            uint a;
            bytes3 b;
            uint[3] c;
            uint[] d;
            bytes e;
        }
        mapping(uint => mapping(bool => Data[])) public data1;
        mapping(uint => mapping(bool => Data)) public data2;

        mapping(bool => mapping(address => uint256[])[])[][] public nestedMapArray;
    }

    assert_eq!(data1Call::SIGNATURE, "data1(uint256,bool,uint256)");
    let _ = data1Return { _0: U256::ZERO, _1: [0, 0, 0].into(), _2: vec![] };

    assert_eq!(data2Call::SIGNATURE, "data2(uint256,bool)");
    let _ = data2Return { _0: U256::ZERO, _1: [0, 0, 0].into(), _2: vec![] };

    assert_eq!(
        nestedMapArrayCall::SIGNATURE,
        "nestedMapArray(uint256,uint256,bool,uint256,address,uint256)"
    );
    let _ = nestedMapArrayReturn { _0: U256::ZERO };
}

#[test]
fn abigen_sol_multicall() {
    sol!("../syn-solidity/tests/contracts/Multicall.sol");

    sol! {
        // SPDX-License-Identifier: MIT
        pragma solidity >=0.8.12 <0.9.0;

        interface IMulticall3_2 {
            struct Call {
                address target;
                bytes callData;
            }

            struct Call3 {
                address target;
                bool allowFailure;
                bytes callData;
            }

            struct Call3Value {
                address target;
                bool allowFailure;
                uint256 value;
                bytes callData;
            }

            struct Result {
                bool success;
                bytes returnData;
            }

            function aggregate(Call[] calldata calls) external payable returns (uint256 blockNumber, bytes[] memory returnData);

            function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData);

            function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData);

            function blockAndAggregate(
                Call[] calldata calls
            ) external payable returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);

            function getBasefee() external view returns (uint256 basefee);

            function getBlockHash(uint256 blockNumber) external view returns (bytes32 blockHash);

            function getBlockNumber() external view returns (uint256 blockNumber);

            function getChainId() external view returns (uint256 chainid);

            function getCurrentBlockCoinbase() external view returns (address coinbase);

            function getCurrentBlockDifficulty() external view returns (uint256 difficulty);

            function getCurrentBlockGasLimit() external view returns (uint256 gaslimit);

            function getCurrentBlockTimestamp() external view returns (uint256 timestamp);

            function getEthBalance(address addr) external view returns (uint256 balance);

            function getLastBlockHash() external view returns (bytes32 blockHash);

            function tryAggregate(
                bool requireSuccess,
                Call[] calldata calls
            ) external payable returns (Result[] memory returnData);

            function tryBlockAndAggregate(
                bool requireSuccess,
                Call[] calldata calls
            ) external payable returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);
        }
    }

    use IMulticall3 as M1;
    use IMulticall3_2 as M2;

    macro_rules! assert_signatures {
        ($($t:ident),+ $(,)?) => {$(
            assert_eq!(
                M1::$t::SIGNATURE,
                M2::$t::SIGNATURE,
                concat!("signature mismatch for ", stringify!($t))
            );
            assert_eq!(
                M1::$t::SELECTOR,
                M2::$t::SELECTOR,
                concat!("selector mismatch for ", stringify!($t))
            );
        )+};
    }

    assert_signatures!(
        aggregate3Call,
        aggregate3ValueCall,
        blockAndAggregateCall,
        getBasefeeCall,
        getBlockHashCall,
        getBlockNumberCall,
        getChainIdCall,
        getCurrentBlockCoinbaseCall,
        getCurrentBlockDifficultyCall,
        getCurrentBlockGasLimitCall,
        getCurrentBlockTimestampCall,
        getEthBalanceCall,
        getLastBlockHashCall,
        tryAggregateCall,
        tryBlockAndAggregateCall,
    );
}

#[test]
fn struct_field_attrs() {
    sol! {
        #[derive(Serialize, Default)]
        struct MyStruct {
            #[serde(skip)]
            uint256 a;
            bytes32 b;
            address[] c;
        }
    }

    assert_eq!(
        serde_json::from_str::<Value>(
            serde_json::to_string(&MyStruct::default()).unwrap().as_str()
        )
        .unwrap()["a"],
        Value::Null
    );
}

#[test]
fn enum_variant_attrs() {
    sol! {
        #[derive(Default, Debug, PartialEq, Eq, Serialize)]
        enum MyEnum {
            A,
            #[default]
            B,
            #[serde(skip)]
            C,
        }
    }

    assert_eq!(MyEnum::default(), MyEnum::B);
    assert!(serde_json::to_string(&MyEnum::C).is_err());
}

#[test]
fn nested_items() {
    // This has to be in a module (not a function) because of Rust import rules
    mod nested {
        alloy_sol_types::sol! {
            #[derive(Debug, PartialEq)]
            struct FilAddress {
                bytes data;
            }

            #[derive(Debug, PartialEq)]
            struct BigInt {
                bytes val;
                bool neg;
            }

            #[derive(Debug, PartialEq)]
            interface InterfaceTest {
                function f1(FilAddress memory fAddress, uint256 value) public payable;

                function f2(BigInt memory b) public returns (BigInt memory);
            }
        }
    }
    use nested::{InterfaceTest::*, *};

    let _ = FilAddress { data: vec![] };
    let _ = BigInt { val: vec![], neg: false };
    assert_eq!(f1Call::SIGNATURE, "f1((bytes),uint256)");
    assert_eq!(f2Call::SIGNATURE, "f2((bytes,bool))");
}

// Allow enums as fields of structs
// https://github.com/alloy-rs/core/issues/319
#[test]
fn enum_field_of_struct() {
    sol! {
        enum MyEnum {
            FIRST,
            SECOND
        }

        struct MyStruct {
            MyEnum myOption;
            uint value;
        }
    }

    let _ = MyStruct { myOption: MyEnum::FIRST, value: U256::ZERO };
}

#[test]
fn same_names_different_namespaces() {
    sol! {
        library RouterErrors {
            error ReturnAmountIsNotEnough();
            error InvalidMsgValue();
            error ERC20TransferFailed();
        }

        library Errors {
            error InvalidMsgValue();
            error ETHTransferFailed();
        }
    }
}

// TODO
// https://github.com/alloy-rs/core/issues/343
#[test]
#[cfg(TODO)]
fn rust_keywords() {
    sol! {
        function mod(address impl) returns (bool is, bool fn);
    }
}

// Translate contract types to `address`
// https://github.com/alloy-rs/core/issues/347
#[test]
#[cfg(TODO)]
fn contract_type() {
    sol! {
        interface IERC20 {}
        function func(IERC20 addr);
        struct Struct {
            IERC20 addr;
        }
    }
}

// Correctly identify whether a type is dynamic
// https://github.com/alloy-rs/core/issues/352
#[test]
fn word_dynarray_event() {
    sol! {
        event Dynamic1(string[] indexed);
        event Dynamic2(string[] indexed, bytes[] indexed);

        event Word1(address[] indexed);
        event Word2(address[] indexed, bytes32[] indexed);
        event Word3(address[] indexed, bytes32[] indexed, uint256[] indexed);
    }
}

// TODO: make commented out code work
#[test]
fn paths_resolution_1() {
    sol! {
        // library OrderRFQLib {
            struct OrderRFQ {
                uint256 info;
                address makerAsset;
                address takerAsset;
                address maker;
                address allowedSender;
                uint256 makingAmount;
                uint256 takingAmount;
            }
        // }

        function fillOrderRFQ(
            /*OrderRFQLib.*/OrderRFQ memory order,
            bytes calldata signature,
            uint256 flagsAndAmount
        ) external payable returns(uint256, uint256, bytes32) {
            return fillOrderRFQTo(order, signature, flagsAndAmount, msg.sender);
        }
    }
}

// Correctly expand the `tokenize` function statements for events
// https://github.com/alloy-rs/core/issues/361
#[test]
fn event_tokenize_fields() {
    sol! {
        event PairCreated(address indexed token0, address indexed token1, address pair, uint256);
    }
    let _ = PairCreated {
        token0: Address::ZERO,
        token1: Address::ZERO,
        pair: Address::ZERO,
        _3: U256::ZERO,
    };
}

// Allow multiple overrides of the same function
// https://github.com/alloy-rs/core/issues/398
#[test]
fn duplicate_attributes() {
    sol! {
        contract TaxableTeamToken is IERC20, Context, Ownable {
            constructor(
                string memory name,
                string memory symbol,
                uint8 decimals,
                uint256 supply,
                uint256 fees,
                address owner,
                address feeWallet
            ) public checkIsFeesValid(fees) checkIsFeesValid(fees2) checkIsAddressValid(owner) checkIsAddressValid(feeWallet) {
                require(decimals >=8 && decimals <= 18, "[Validation] Not valid decimals");
                require(supply > 0, "[Validation] inital supply should be greater than 0");
                require(owner != feeWallet, "[Validation] fee wallet and owner wallet cannot be same.");

                _name = name;
                _symbol = symbol;
                _decimals = decimals;
                _feesPercentage = fees;

                _tTotal = supply;
                _rTotal = (MAX - (MAX % _tTotal));

                _rOwned[owner] = _rTotal;

                emit Transfer(address(0), owner, _tTotal);

                emit TeamFinanceTokenMint(owner);
            }
        }
    }
}

#[test]
#[cfg(feature = "json")]
fn abigen_json_large_array() {
    sol!(LargeArray, "../json-abi/tests/abi/LargeArray.json");
    assert_eq!(LargeArray::callWithLongArrayCall::SIGNATURE, "callWithLongArray(uint64[128])");
}

#[test]
#[cfg(feature = "json")]
fn abigen_json_seaport() {
    use alloy_sol_types::SolStruct;
    use std::borrow::Cow;
    use Seaport::*;

    sol!(Seaport, "../json-abi/tests/abi/Seaport.json");

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
#[cfg(feature = "json")]
fn abigen_json_aggregation_router_v5() {
    sol!(AggregationRouterV5, "../json-abi/tests/abi/AggregationRouterV5.json");

    assert_eq!(
        <AggregationRouterV5::ETHTransferFailed as SolError>::SIGNATURE,
        "ETHTransferFailed()"
    );
    assert_eq!(<AggregationRouterV5::InvalidMsgValue as SolError>::SIGNATURE, "InvalidMsgValue()");
}

// Handle contract types in JSON ABI
// https://github.com/alloy-rs/core/issues/351
#[test]
#[cfg(feature = "json")]
fn abigen_json_uniswap_v3_position() {
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
#[cfg(feature = "json")]
fn abigen_json_double_exponent_interest_setter() {
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
#[cfg(feature = "json")]
fn abigen_json_uniswap_v2_factory() {
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
#[cfg(feature = "json")]
fn abigen_json_gnosis_safe() {
    sol!(GnosisSafe, "../json-abi/tests/abi/GnosisSafe.json");
    let GnosisSafe::NAMECall {} = GnosisSafe::NAMECall {};
    let GnosisSafe::NAMEReturn { _0: _ } = GnosisSafe::NAMEReturn { _0: String::new() };
}

// Have enough recursion depth to handle `BlurExchange` types
// https://github.com/alloy-rs/core/issues/371
#[test]
#[cfg(feature = "json")]
fn abigen_json_blur_exchange() {
    sol!(BlurExchange, "../json-abi/tests/abi/BlurExchange.json");
    let BlurExchange::NAMECall {} = BlurExchange::NAMECall {};
    let BlurExchange::NAMEReturn { _0: _ } = BlurExchange::NAMEReturn { _0: String::new() };
}

#[test]
fn eip712_encode_type_nesting() {
    sol! {
        struct A {
            uint256 a;
        }

        struct B {
            bytes32 b;
        }

        struct C {
            A a;
            B b;
        }

        struct D {
            C c;
            A a;
            B b;
        }
    }

    assert_eq!(A::eip712_encode_type(), "A(uint256 a)");
    assert_eq!(B::eip712_encode_type(), "B(bytes32 b)");
    assert_eq!(C::eip712_encode_type(), "C(A a,B b)A(uint256 a)B(bytes32 b)");
    assert_eq!(D::eip712_encode_type(), "D(C c,A a,B b)A(uint256 a)B(bytes32 b)C(A a,B b)");
}

#[test]
fn eip712_encode_data_nesting() {
    sol! {
        struct Person {
            string name;
            address wallet;
        }

        struct Mail {
            Person from;
            Person to;
            string contents;
        }
    }
    let domain = eip712_domain! {};

    let mail = Mail {
        from: Person {
            name: "Cow".to_owned(),
            wallet: "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826".parse().unwrap(),
        },
        to: Person {
            name: "Bob".to_owned(),
            wallet: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB".parse().unwrap(),
        },
        contents: "Hello, Bob!".to_owned(),
    };

    assert_eq!(
        alloy_sol_types::SolStruct::eip712_signing_hash(&mail, &domain),
        "25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775".parse::<B256>().unwrap()
    )
}
