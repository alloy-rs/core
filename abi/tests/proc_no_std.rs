#![no_std]

use ethers_abi_enc::{no_std_prelude::*, sol, SolType};

use ethers_primitives::{B160, U256};

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
fn no_std_proc_macro() {
    // this is possible but not recomended :)
    <sol! {
        bool
    }>::hex_encode_single(true);

    let a = MyStruct {
        a: U256::from(1),
        b: [0; 32],
        c: Vec::new(),
    };

    MyTuple::hex_encode((a.clone(), [0; 32]));

    MyStruct::hex_encode(a.clone());

    LateBinding::<MyStruct>::hex_encode((vec![a.clone(), a.clone()], B160::default()));

    MyStruct2::hex_encode(MyStruct2 {
        a,
        b: [0; 32],
        c: vec![],
    });

    NestedArray::hex_encode(vec![[true, false], [true, false], [true, false]]);

    let mvt = MyValueType::from(U256::from(1));
    assert_eq!(
        mvt.encode_single(),
        ethers_abi_enc::sol_type::Uint::<256>::encode_single(U256::from(1))
    );
}
