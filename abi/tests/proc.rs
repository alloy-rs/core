use ethers_abi_enc::{sol, SolStruct, SolType};

sol!(
    /// Hello this is extra docs
    #[derive(Hash)]
    struct MySingleProp {
        uint256 a;
    }
);

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
fn strings() {
    assert_eq!(
        MyStruct::encode_type(),
        "MyStruct(uint256 a,bytes32 b,address[] c)"
    );

    assert_eq!(
        MyStruct2::encode_type(),
        "MyStruct2(MyStruct a,bytes32 b,address[] c)MyStruct(uint256 a,bytes32 b,address[] c)"
    );
}

#[test]
fn proc_macro_expansion() {
    use ethers_primitives::{Address, U256};

    // this is possible but not recomended :)
    <sol!(bool)>::hex_encode_single(true);

    let a = MyStruct {
        a: U256::from(1),
        b: [0; 32],
        c: vec![],
    };

    MyTuple::hex_encode((a.clone(), [0; 32]));

    MyStruct::hex_encode(a.clone());

    LateBinding::<MyStruct>::hex_encode((vec![a.clone(), a.clone()], Address::default()));

    MyStruct2::hex_encode(MyStruct2 {
        a,
        b: [0; 32],
        c: vec![],
    });

    NestedArray::hex_encode(vec![[true, false], [true, false], [true, false]]);

    let mvt = MyValueType::from(U256::from(1));
    assert_eq!(
        mvt.encode_single(),
        ethers_abi_enc::sol_data::Uint::<256>::encode_single(U256::from(1))
    );
}
