use alloy_primitives::Address;
use alloy_sol_types::{sol, SolType};

// Type definition: generates a new struct that implements `SolType`
sol! {
    type MyType is uint256;
}

// Type aliases
type B32 = sol! { bytes32 };
// This is equivalent to the following:
// type B32 = alloy_sol_types::sol_data::Bytes<32>;

type SolArrayOf<T> = sol! { T[] };
type SolTuple = sol! { tuple(address, bytes, string) };

#[test]
fn types() {
    let _ = <sol!(bool)>::encode(&true);
    let _ = B32::encode(&[0; 32]);
    let _ = SolArrayOf::<sol!(bool)>::encode(&vec![true, false]);
    let _ = SolTuple::encode(&(Address::ZERO, vec![0; 32], "hello".to_string()));
}
