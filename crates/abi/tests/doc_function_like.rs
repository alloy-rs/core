use ethers_abi_enc::{sol, SolCall, SolError};
use ethers_primitives::{keccak256, U256};

// Unnamed arguments will be given a name based on their position,
// e.g. `_0`, `_1`...
//
// A current limitation for these types is that custom types, like structs,
// must be defined in the same macro scope, otherwise a signature cannot be
// generated at compile time.

sol! {
    // Function definitions generate two types that implement `SolCall`:
    // 1. `<name>Call`: struct with the function arguments;
    // 2. `<name>Return`: struct with the return values.
    // `<name>` is the case-preserved name of the function.
    //
    // Currently, return structs should only be used for decoding data using
    // `decode_raw`, as the generated signature is not valid.
    function foo(uint256 a, uint256 b) external view returns (uint256);

    #[derive(Debug, PartialEq)]
    error MyError(uint256 a, uint256 b);

    // TODO: events
    // event FooEvent(uint256 a, uint256 b);
}

#[test]
fn function_like() {
    // function
    let expected_signature = "foo(uint256,uint256)";
    assert_eq!(fooCall::SIGNATURE, expected_signature);
    assert_eq!(&fooCall::SELECTOR[..], &keccak256(expected_signature)[..4]);
    // not actually a valid signature, and it shouldn't be relied upon
    assert_eq!(fooReturn::SIGNATURE, "foo(uint256)");

    let call = fooCall {
        a: U256::from(1),
        b: U256::from(2),
    };
    let call_data = call.encode();

    // error
    let expected_signature: &str = "MyError(uint256,uint256)";
    assert_eq!(MyError::SIGNATURE, expected_signature);
    assert_eq!(&MyError::SELECTOR[..], &keccak256(expected_signature)[..4]);

    assert!(MyError::decode(&call_data, true).is_err());
    assert_eq!(
        MyError::decode_raw(&call_data[4..], true),
        Ok(MyError {
            a: U256::from(1),
            b: U256::from(2)
        })
    );
}
