//! Comprehensive test for contract-level derives applying to all generated types.
//! Tests all_derives and extra_derives on contracts with events and errors.

use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use std::{collections::HashSet, hash::Hash};

#[test]
fn test_all_derives() {
    sol! {
        #[sol(all_derives)]
        contract AllDerivesContract {
            function transfer(address to, uint256 amount) external returns (bool);
            event Transfer(address indexed from, address indexed to, uint256 value);
            error InsufficientBalance(uint256 requested, uint256 available);
        }
    }

    use AllDerivesContract::*;

    let event1 = Transfer { from: Address::ZERO, to: Address::ZERO, value: U256::from(50) };
    let event2 = Transfer { from: Address::ZERO, to: Address::ZERO, value: U256::from(50) };
    let events_enum1 = AllDerivesContractEvents::Transfer(event1);
    let events_enum2 = AllDerivesContractEvents::Transfer(event2);

    let error1 = InsufficientBalance { requested: U256::from(100), available: U256::from(50) };
    let error2 = InsufficientBalance { requested: U256::from(100), available: U256::from(50) };
    let errors_enum1 = AllDerivesContractErrors::InsufficientBalance(error1);
    let errors_enum2 = AllDerivesContractErrors::InsufficientBalance(error2);

    // Test PartialEq and Debug
    assert_eq!(errors_enum1, errors_enum2);
    assert_eq!(events_enum1, events_enum2);

    // Test Hash and Eq derives
    let mut events_set = HashSet::new();
    events_set.insert(events_enum1);
    events_set.insert(events_enum2);
    // Should not increase size since they're equal
    assert_eq!(events_set.len(), 1);

    let mut errors_set = HashSet::new();
    errors_set.insert(errors_enum1);
    errors_set.insert(errors_enum2);
    // Should not increase size since they're equal
    assert_eq!(errors_set.len(), 1);
}

#[test]
fn test_extra_derives() {
    sol! {
        #[sol(extra_derives(PartialEq, Eq, Hash, Debug))]
        contract ExtraDerivesContract {
            function transfer(address to, uint256 amount) external returns (bool);
            event Transfer(address indexed from, address indexed to, uint256 value);
            error InsufficientBalance(uint256 requested, uint256 available);
        }
    }

    use ExtraDerivesContract::*;

    let event1 = Transfer { from: Address::ZERO, to: Address::ZERO, value: U256::from(50) };
    let event2 = Transfer { from: Address::ZERO, to: Address::ZERO, value: U256::from(50) };
    let events_enum1 = ExtraDerivesContractEvents::Transfer(event1);
    let events_enum2 = ExtraDerivesContractEvents::Transfer(event2);

    let error1 = InsufficientBalance { requested: U256::from(100), available: U256::from(50) };
    let error2 = InsufficientBalance { requested: U256::from(100), available: U256::from(50) };
    let errors_enum1 = ExtraDerivesContractErrors::InsufficientBalance(error1);
    let errors_enum2 = ExtraDerivesContractErrors::InsufficientBalance(error2);

    // Test PartialEq and Debug
    assert_eq!(errors_enum1, errors_enum2);
    assert_eq!(events_enum1, events_enum2);

    // Test Hash and Eq derives
    let mut events_set = HashSet::new();
    events_set.insert(events_enum1);
    events_set.insert(events_enum2);
    // Should not increase size since they're equal
    assert_eq!(events_set.len(), 1);

    let mut errors_set = HashSet::new();
    errors_set.insert(errors_enum1);
    errors_set.insert(errors_enum2);
    // Should not increase size since they're equal
    assert_eq!(errors_set.len(), 1);
}

// Overloaded events (same name, different params) get suffixed variant names
// (e.g. `Swap_0`, `Swap_1`). The generated `*Events` enum must still receive
// the `#[sol(all_derives)]` traits. See alloy-rs/alloy#3856.
#[test]
#[allow(non_snake_case)]
fn test_all_derives_overloaded_events() {
    sol! {
        #[sol(all_derives)]
        contract OverloadedEvents {
            event Swap(address indexed sender, uint256 amount0);
            event Swap(address indexed sender, address indexed to, int256 amount0, int256 amount1);
        }
    }

    use OverloadedEvents::*;

    let a =
        OverloadedEventsEvents::Swap_0(Swap_0 { sender: Address::ZERO, amount0: U256::from(1) });
    let b =
        OverloadedEventsEvents::Swap_0(Swap_0 { sender: Address::ZERO, amount0: U256::from(1) });

    // Debug + PartialEq
    assert_eq!(a, b);
    let _ = format!("{a:?}");

    // Hash + Eq
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}
