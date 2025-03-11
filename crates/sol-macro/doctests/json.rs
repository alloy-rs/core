use alloy_sol_types::{sol, SolCall};

sol!(
    MyJsonContract1,
    r#"[
        {
            "inputs": [
                { "name": "bar", "type": "uint256" },
                { 
                    "internalType": "struct MyStruct",
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

// And:
// sol!(MyJsonContract, concat!(env!("CARGO_MANIFEST_DIR"), "/path/to/MyJsonContract.json"));

#[test]
fn abigen() {
    assert_eq!(MyJsonContract1::fooCall::SIGNATURE, MyJsonContract2::fooCall::SIGNATURE);
}
