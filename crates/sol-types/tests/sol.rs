use alloy_primitives::{keccak256, Address, U256};
use alloy_sol_types::{sol, SolCall, SolError, SolType};

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

    <sol!(bool)>::hex_encode_single(&true);

    let a = MyStruct {
        a: U256::from(1),
        b: [0; 32],
        c: Vec::new(),
    };

    MyTuple::hex_encode(&(a.clone(), [0; 32]));
    MyStruct::hex_encode(&a);

    LateBinding::<MyStruct>::hex_encode(&(vec![a.clone(), a.clone()], Address::default()));

    MyStruct2::hex_encode(&MyStruct2 {
        a,
        b: [0; 32],
        c: vec![],
    });

    NestedArray::hex_encode(&vec![[true, false], [true, false], [true, false]]);

    let mvt = MyValueType::from(U256::from(1));
    assert_eq!(
        mvt.encode_single(),
        alloy_sol_types::sol_data::Uint::<256>::encode_single(&U256::from(1))
    );
}

#[test]
fn function() {
    sol! {
        struct customStruct {
            address a;
            uint64 b;
        }

        function someFunction(
            uint256 basic,
            string memory string_,
            bytes calldata longBytes,
            address[] memory array,
            bool[2] memory fixedArray,
            customStruct struct_,
            customStruct[] structArray,
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
        struct_: customStruct {
            a: Address::ZERO,
            b: 2,
        },
        structArray: vec![
            customStruct {
                a: Address::ZERO,
                b: 3,
            },
            customStruct {
                a: Address::ZERO,
                b: 4,
            },
            customStruct {
                a: Address::ZERO,
                b: 5,
            },
            customStruct {
                a: Address::ZERO,
                b: 6,
            },
        ],
    };
    let encoded = call.encode();
    assert_eq!(
        encoded.len(),
        someFunctionCall::SELECTOR.len() + call.encoded_size()
    );

    assert_eq!(
        call.encoded_size(),
        32 + (64 + 32) + (64 + 32 + 32) + (64 + 3 * 32) + 2 * 32 + (32 + 32) + (64 + 4 * (32 + 32))
    );
}

#[test]
fn error() {
    sol! {
        error SomeError(uint256 a);
    }

    let sig = "SomeError(uint256)";
    assert_eq!(SomeError::SIGNATURE, sig);
    assert_eq!(SomeError::SELECTOR, keccak256(sig)[..4]);

    let e = SomeError { a: U256::from(1) };
    assert_eq!(e.encoded_size(), 32);
}

// https://github.com/alloy-rs/core/issues/158
#[test]
fn empty_call() {
    sol! {
        interface WETH {
            function deposit() external payable;
        }
    }
    use WETH::depositCall;

    assert_eq!(depositCall {}.encode(), depositCall::SELECTOR);
    assert_eq!(depositCall {}.encoded_size(), 0);
    let mut out = vec![];
    depositCall {}.encode_raw(&mut out);
    assert!(out.is_empty());

    let depositCall {} = depositCall::decode(&depositCall::SELECTOR, true).unwrap();
    let depositCall {} = depositCall::decode_raw(&[], true).unwrap();
}

#[test]
fn abigen_sol() {
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
#[cfg(feature = "json")]
fn abigen_json() {
    sol!(Contract, "../json-abi/tests/abi/LargeArray.json");
    assert_eq!(
        Contract::callWithLongArrayCall::SIGNATURE,
        "callWithLongArray(uint64[128])"
    );
}
