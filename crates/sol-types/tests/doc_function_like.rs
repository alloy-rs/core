use alloy_primitives::{keccak256, U256};
use alloy_sol_types::{sol, SolCall, SolError};

// Unnamed arguments will be given a name based on their position,
// e.g. `_0`, `_1`...
//
// A current limitation for these types is that custom types, like structs,
// must be defined in the same macro scope, otherwise a signature cannot be
// generated at compile time.

sol! {
    /// Function definitions generate two types that implement [`SolCall`]:
    /// 1. `<name>Call`: struct with the function arguments;
    /// 2. `<name>Return`: struct with the return values;
    /// where `<name>` is the case-preserved name of the function.
    ///
    /// In the case of overloaded functions, an underscore and the index of the
    /// function will be appended to `<name>` (like `foo_0`, `foo_1`...) for
    /// disambiguation, but the signature will remain the same.
    ///
    /// Both of these types will have the attributes of the function, like this
    /// doc comment, but this might change in the future.
    ///
    /// Currently, return structs should only be used for decoding data using
    /// `decode_raw`, as the generated signature is not valid.
    function foo(uint256 a, uint256 b) external view returns (uint256);

    // These will be interpreted as `overloaded_0`, `overloaded_1`, and
    // `overloaded_2`, but the signatures will be the same.
    function overloaded();
    function overloaded(uint256) returns (uint256);
    function overloaded(string);

    /// Implements [`SolError`].
    #[derive(Debug, PartialEq)]
    error MyError(uint256 a, uint256 b);

    // TODO: events
    // event FooEvent(uint256 a, uint256 b);
}

#[test]
fn function_like() {
    // function
    assert_call_signature::<fooCall>("foo(uint256,uint256)");

    // not actually a valid signature, and it shouldn't be relied upon for
    // ABI encoding
    assert_call_signature::<fooReturn>("foo(uint256)");

    let call = fooCall {
        a: U256::from(1),
        b: U256::from(2),
    };
    let call_data = call.encode();

    // the signatures are unaffected
    let _ = overloaded_0Call {};
    assert_call_signature::<overloaded_0Call>("overloaded()");

    let _ = overloaded_1Call { _0: U256::from(1) };
    let _ = overloaded_1Return { _0: U256::from(2) };
    assert_call_signature::<overloaded_1Call>("overloaded(uint256)");

    let _ = overloaded_2Call { _0: "hello".into() };
    assert_call_signature::<overloaded_2Call>("overloaded(string)");

    // error
    assert_error_signature::<MyError>("MyError(uint256,uint256)");

    assert!(MyError::decode(&call_data, true).is_err());
    assert_eq!(
        MyError::decode_raw(&call_data[4..], true),
        Ok(MyError {
            a: U256::from(1),
            b: U256::from(2)
        })
    );
}

fn assert_call_signature<T: SolCall>(expected: &str) {
    assert_eq!(T::SIGNATURE, expected);
    assert_eq!(T::SELECTOR, keccak256(expected)[..4]);
}

fn assert_error_signature<T: SolError>(expected: &str) {
    assert_eq!(T::SIGNATURE, expected);
    assert_eq!(T::SELECTOR, keccak256(expected)[..4]);
}
