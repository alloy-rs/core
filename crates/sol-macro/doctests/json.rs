use alloy_sol_types::{sol, SolCall};

sol!(
    MyJsonContract1,
    r#"[
        {
            "inputs": [
                { "name": "bar", "type": "uint256" },
                { 
                    "internalType": "struct MyJsonContract.MyStruct",
                    "name": "baz",
                    "type": "tuple",
                    "components": [
                        { "name": "a", "type": "bool[]" },
                        { "name": "b", "type": "bytes18[][]" }
                    ]
                }
            ],
            "outputs": [],
            "stateMutability": "view",
            "name": "foo",
            "type": "function"
        }
    ]"#
);

// This is the same as:
sol! {
    interface MyJsonContract2 {
        struct MyStruct {
            bool[] a;
            bytes18[][] b;
        }

        function foo(uint256 bar, MyStruct baz) external view;
    }
}

sol!(
    ContractWithDuplicates1,
    r#"[
        {
            "inputs": [
                {
                    "name": "aValue_",
                    "type": "tuple",
                    "internalType": "struct LibA.Struct",
                    "components": [
                        {
                            "name": "field64",
                            "type": "uint64",
                            "internalType": "uint64"
                        }
                    ]
                },
                {
                    "name": "bValue_",
                    "type": "tuple",
                    "internalType": "struct LibB.Struct",
                    "components": [
                        {
                            "name": "field128",
                            "type": "uint128",
                            "internalType": "uint128"
                        }
                    ]
                }
            ],
            "outputs": [],
            "stateMutability": "view",
            "name": "foo",
            "type": "function"
        }
    ]"#
);

// This is the same as:
sol! {
    interface ContractWithDuplicates2 {
        struct Struct0 {
            uint64 field64;
        }
        struct Struct1 {
            uint128 field128;
        }

        function foo(Struct0 aValue_, Struct1 bValue_) external view;
    }
}

#[test]
fn abigen() {
    assert_eq!(MyJsonContract1::fooCall::SIGNATURE, MyJsonContract2::fooCall::SIGNATURE,);
    assert_eq!(
        ContractWithDuplicates1::fooCall::SIGNATURE,
        ContractWithDuplicates2::fooCall::SIGNATURE,
    );
}
