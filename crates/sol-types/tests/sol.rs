use alloy_primitives::{keccak256, Address, U256};
use alloy_sol_types::{sol, SolCall, SolError, SolType};

sol! {
    struct MyStruct {
        uint256 a;
        bytes32 b;
        address[] c;
    }
}

// Works only outside of function scope due to rust import rules
sol! {
    struct MyStruct2 {
        MyStruct a;
        bytes32 b;
        address[] c;
    }
}

// This works
type MyTuple = sol! {
    (MyStruct, bytes32)
};

// This works
type LateBinding<A> = sol! {
    (A[], address)
};

// testcase for something i messed up earlier :)
type NestedArray = sol! {
    bool[2][]
};

sol! {
    type MyValueType is uint256;
}

#[test]
fn test_sol() {
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

sol! {
    interface WETH {
        function deposit() external payable;
    }
}

#[test]
fn empty_call() {
    WETH::depositCall::decode(&WETH::depositCall::SELECTOR, true).expect("it should work");
}
