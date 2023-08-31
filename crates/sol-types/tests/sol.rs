use std::str::FromStr;

use alloy_primitives::{keccak256, Address, B256, U256};
use alloy_sol_types::{eip712_domain, sol, SolCall, SolError, SolType};

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
#[cfg(feature = "json")]
fn abigen_json_large_array() {
    sol!(LargeArray, "../json-abi/tests/abi/LargeArray.json");
    assert_eq!(
        LargeArray::callWithLongArrayCall::SIGNATURE,
        "callWithLongArray(uint64[128])"
    );
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
    assert_eq!(
        BasicOrderParameters::eip712_components(),
        [Cow::Borrowed(component)]
    );
    assert_eq!(
        <BasicOrderParameters as SolStruct>::eip712_encode_type(),
        root_type.to_string() + component
    );
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

    assert_eq!(A::eip712_encode_type().unwrap(), "A(uint256 a)");
    assert_eq!(B::eip712_encode_type().unwrap(), "B(bytes32 b)");
    assert_eq!(
        C::eip712_encode_type().unwrap(),
        "C(A a,B b)A(uint256 a)B(bytes32 b)"
    );
    assert_eq!(
        D::eip712_encode_type().unwrap(),
        "D(C c,A a,B b)A(uint256 a)B(bytes32 b)C(A a,B b)"
    );
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
            wallet: "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                .parse()
                .unwrap(),
        },
        to: Person {
            name: "Bob".to_owned(),
            wallet: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                .parse()
                .unwrap(),
        },
        contents: "Hello, Bob!".to_owned(),
    };

    assert_eq!(
        alloy_sol_types::SolStruct::eip712_signing_hash(&mail, &domain),
        B256::from_str("25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775").unwrap()
    )
}
