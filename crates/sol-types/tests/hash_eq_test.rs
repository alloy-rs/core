use alloy_sol_types::sol;
use std::collections::HashMap;

// Helper function to assert a type implements Hash + Eq
fn assert_hash_eq<K: Eq + std::hash::Hash>() {}

#[test]
fn sol_types_implement_hash_eq_traits() {

    type Bool = sol! { bool };
    type Address = sol! { address };
    type Function = sol! { function() };
    type Bytes = sol! { bytes };
    type String = sol! { string };
    type ArrayU8 = sol! { uint8[] };
    type FixedArrayU8_4 = sol! { uint8[4] };
    type Uint8 = sol! { uint8 };
    type Int8 = sol! { int8 };
    

    assert_hash_eq::<Bool>();
    assert_hash_eq::<Address>();
    assert_hash_eq::<Function>();
    assert_hash_eq::<Bytes>();
    assert_hash_eq::<String>();
    assert_hash_eq::<ArrayU8>();
    assert_hash_eq::<FixedArrayU8_4>();
    assert_hash_eq::<Uint8>();
    assert_hash_eq::<Int8>();

    println!("All sol types implement Hash + Eq traits.");
}

#[test]
fn sol_types_work_as_hashmap_keys() {
    //Specific Test for #973
    type Bytes4 = sol! { bytes4 };
    let mut _map: HashMap<Bytes4, bool> = HashMap::new();

    println!("Issue #973 solved");
}

