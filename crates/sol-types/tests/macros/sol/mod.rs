use alloy_primitives::{hex, keccak256, Address, I256, U256};
use alloy_sol_types::{sol, SolCall, SolError, SolEvent, SolStruct, SolType};
use serde::Serialize;
use serde_json::Value;

#[cfg(feature = "json")]
mod abi;
#[cfg(feature = "json")]
mod json;

mod eip712;

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

#[test]
fn rust_keywords() {
    sol! {
        contract dyn {
            struct const {
                bool unsafe;
                bytes32 box;
            }

            function mod(address impl) returns (bool is, bool fn);
        }
    }
    use r#dyn::*;

    let _ = r#const { r#unsafe: true, r#box: Default::default() };
    let m = modCall { r#impl: Address::ZERO };
    let _ = dynCalls::r#mod(m);
    let _ = modReturn { is: true, r#fn: false };
    assert_eq!(r#const::NAME, "const");
    assert_eq!(modCall::SIGNATURE, "mod(address)");
}

#[test]
fn most_rust_keywords() {
    // $(kw r#kw)*
    macro_rules! kws {
        ($($kw:tt $raw:tt)*) => { paste::paste! {
            $({
                sol! {
                    struct $kw {
                        uint $kw;
                    }

                    function $kw(bytes1 $kw) returns (uint $kw);
                }

                mod error {
                    use super::*;

                    sol! {
                        error $kw(bytes2 $kw);
                    }
                }

                mod event {
                    use super::*;

                    sol! {
                        event $kw(bytes3 $kw);
                    }
                }

                assert_eq!($raw::NAME, stringify!($kw));
                assert_ne!($raw::NAME, stringify!($raw));
                assert_eq!(<[<$kw Call>]>::SIGNATURE, concat!(stringify!($kw), "(bytes1)"));
                let _ = [<$kw Call>] { $raw: [0u8; 1].into() };
                assert_eq!(error::$raw::SIGNATURE, concat!(stringify!($kw), "(bytes2)"));
                let _ = error::$raw { $raw: [0u8; 2].into() };
                assert_eq!(event::$raw::SIGNATURE, concat!(stringify!($kw), "(bytes3)"));
                let _ = event::$raw { $raw: [0u8; 3].into() };
            })*
        } };
    }

    kws! {
        const r#const
        extern r#extern
        fn r#fn
        impl r#impl
        loop r#loop
        mod r#mod
        move r#move
        mut r#mut
        pub r#pub
        ref r#ref
        trait r#trait
        unsafe r#unsafe
        use r#use
        where r#where
        async r#async
        await r#await
        dyn r#dyn
        become r#become
        box r#box
        priv r#priv
        unsized r#unsized
        yield r#yield
    }
}

#[test]
fn raw_identifiers() {
    sol! {
        struct r#mod {
            int r#type;
        }
        function r#try();
    }
    let _ = r#mod { r#type: Default::default() };
    let _ = tryCall {};
    assert_eq!(r#mod::NAME, "mod");
    assert_eq!(tryCall::SIGNATURE, "try()");
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
fn duplicate_events() {
    sol! {
    #[derive(derive_more::Display)]
    interface Console {
        #[display(fmt = "{val}")]
        event log(string val);

        #[display(fmt = "{}", "hex::encode_prefixed(val)")]
        event logs(bytes val);

        #[display(fmt = "{val}")]
        event log_address(address val);

        #[display(fmt = "{val}")]
        event log_bytes32(bytes32 val);

        #[display(fmt = "{val}")]
        event log_int(int val);

        #[display(fmt = "{val}")]
        event log_uint(uint val);

        #[display(fmt = "{}", "hex::encode_prefixed(val)")]
        event log_bytes(bytes val);

        #[display(fmt = "{val}")]
        event log_string(string val);

        #[display(fmt = "{val:?}")]
        event log_array(uint256[] val);

        #[display(fmt = "{val:?}")]
        event log_array(int256[] val);

        #[display(fmt = "{val:?}")]
        event log_array(address[] val);

        #[display(fmt = "{key}: {val}")]
        event log_named_address(string key, address val);

        #[display(fmt = "{key}: {val}")]
        event log_named_bytes32(string key, bytes32 val);

        #[display(fmt = "{key}: {val}")]
        event log_named_decimal_int(string key, int val, uint decimals);

        #[display(fmt = "{key}: {val}")]
        event log_named_decimal_uint(string key, uint val, uint decimals);

        #[display(fmt = "{key}: {val}")]
        event log_named_int(string key, int val);

        #[display(fmt = "{key}: {val}")]
        event log_named_uint(string key, uint val);

        #[display(fmt = "{key}: {val:?}")]
        event log_named_bytes(string key, bytes val);

        #[display(fmt = "{key}: {val}")]
        event log_named_string(string key, string val);

        #[display(fmt = "{key}: {val:?}")]
        event log_named_array(string key, uint256[] val);

        #[display(fmt = "{key}: {val:?}")]
        event log_named_array(string key, int256[] val);

        #[display(fmt = "{key}: {val:?}")]
        event log_named_array(string key, address[] val);
    }
    }
}
