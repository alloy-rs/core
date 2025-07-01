use alloy_sol_types::sol;
use std::collections::HashMap;

type Bytes4 = sol! { bytes4 };

#[test]
fn hash_eq_works_for_sol_bytes4() {
    let mut map: HashMap<Bytes4, bool> = HashMap::new();

    // Use unsafe zeroed value as dummy
    let value: Bytes4 = unsafe { core::mem::zeroed() };

    map.insert(value, true);

    if map.contains_key(&value) {
        println!("Contains key");
    } else {
        println!("Does not contain key");
    }
}

