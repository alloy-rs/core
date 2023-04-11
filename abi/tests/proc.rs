use ethers_abi_enc::{sol, SolType};

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

// This works in typdefs, but not in structs (yet?)
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
fn proc_macro_expansion() {
    // this is possible but not recomended :)
    <sol! {
        bool
    }>::hex_encode_single(true);

    let a = MyStruct {
        a: U256::from(1),
        b: [0; 32],
        c: vec![],
    };

    dbg!(MyTuple::hex_encode((a.clone(), [0; 32])));

    dbg!(MyStruct::hex_encode(a.clone()));

    dbg!(LateBinding::<MyStruct>::hex_encode((
        vec![a.clone(), a.clone()],
        B160::default()
    )));

    dbg!(MyStruct2::hex_encode(MyStruct2 {
        a,
        b: [0; 32],
        c: vec![],
    }));

    dbg!(NestedArray::hex_encode(vec![
        [true, false],
        [true, false],
        [true, false]
    ]));

    let mvt = MyValueType::from(U256::from(1));
    assert_eq!(
        mvt.encode_single(),
        ethers_abi_enc::sol_type::Uint::<256>::encode_single(U256::from(1))
    );
}
