use alloy_primitives::{keccak256, U256};
use alloy_sol_types::{sol, SolCall, SolError};
use hex_literal::hex;

// Unnamed arguments will be given a name based on their position,
// e.g. `_0`, `_1`...
//
// A current limitation for these types is that custom types, like structs,
// must be defined in the same macro scope, otherwise a signature cannot be
// generated at compile time.

sol! {
    /// Function definitions generate a type that implements [`SolCall`]
    /// named `<name>Call`. This struct will contain the  function arguments
    ///
    /// In the case of overloaded functions, an underscore and the index of the
    /// function will be appended to `<name>` (like `foo_0`, `foo_1`...) for
    /// disambiguation, but the signature will remain the same.
    ///
    /// E.g. if there are two functions named `foo`, the generated types will be
    /// `foo_0Call` and `foo_1Call`, each of which will implement [`SolCall`]
    /// with their respective signatures.
    ///
    /// Both of these types will have the attributes of the function, like this
    /// doc comment, but this might change in the future.
    ///
    /// The [`SolCall`] implementation may be used to decode the return values
    /// via [`SolCall::decode_returns`]. The return value is a struct containing
    /// the return values, in the same order as the function definition. Unnamed
    /// return values will be named based on their position, e.g. `_0`, `_1`,
    /// like the arguments.
    ///
    /// For example the following input:
    ///  `function foo(uint256 a, uint256 b) external view returns (uint256);`
    ///
    /// Will produce Rust code similar to this:
    ///
    /// ```ignore,pseudo-code
    /// struct fooCall {
    ///     a: U256,
    ///     a: U256,
    /// }
    ///
    /// struct fooReturn {
    ///     _0: U256,
    /// }
    ///
    /// impl SolCall for fooCall {
    ///     type Return = fooReturn;
    /// }
    /// ```
    function foo(uint256 a, uint256 b) external view returns (uint256);

    // These will be interpreted as `overloaded_0`, `overloaded_1`, and
    // `overloaded_2`, each with a different signature.
    function overloaded();
    function overloaded(uint256) returns (uint256);
    function overloaded(string);

    /// Implements [`SolError`].
    #[derive(Debug, PartialEq)]
    error MyError(uint256 a, uint256 b);
}

#[test]
fn function() {
    assert_call_signature::<fooCall>("foo(uint256,uint256)");

    let call = fooCall {
        a: U256::from(1),
        b: U256::from(2),
    };
    let _call_data = call.encode();

    // the signatures are unaffected
    let _ = overloaded_0Call {};
    assert_call_signature::<overloaded_0Call>("overloaded()");

    let _ = overloaded_1Call { _0: U256::from(1) };
    assert_call_signature::<overloaded_1Call>("overloaded(uint256)");

    let _ = overloaded_2Call { _0: "hello".into() };
    assert_call_signature::<overloaded_2Call>("overloaded(string)");
}

#[test]
fn error() {
    assert_error_signature::<MyError>("MyError(uint256,uint256)");
    let call_data = hex!(
        "0000000000000000000000000000000000000000000000000000000000000001"
        "0000000000000000000000000000000000000000000000000000000000000002"
    );
    assert_eq!(
        MyError::decode_raw(&call_data, true),
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
