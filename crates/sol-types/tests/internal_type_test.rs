//! Tests for internal type generation in ABI.

#![cfg(feature = "json")]

use alloy_json_abi::{Error, Event, EventParam, Function, InternalType, Param, StateMutability};
use alloy_sol_types::sol;

#[test]
fn test_internal_type_generation() {
    sol! {
        #[sol(abi)]
        contract TestContract {
            struct Point {
                uint256 x;
                uint256 y;
            }

            enum Status {
                Active,
                Inactive
            }

            function testFunction(
                Point memory p,
                Point[] memory points,
                Status s
            ) external returns (Point memory);

            event TestEvent(Point indexed p, Status s);

            error TestError(Point p, Status[] statuses);
        }
    }

    let contract_abi = TestContract::abi::contract();

    let point_struct = Param {
        ty: "tuple".into(),
        name: "p".into(),
        internal_type: Some(InternalType::Struct {
            contract: Some("TestContract".into()),
            ty: "Point".into(),
        }),
        components: vec![param("uint256 x"), param("uint256 y")],
    };

    // Test function
    assert_eq!(
        *contract_abi.function("testFunction").unwrap().first().unwrap(),
        Function {
            name: "testFunction".into(),
            inputs: vec![
                point_struct.clone(),
                Param {
                    ty: "tuple[]".into(),
                    name: "points".into(),
                    internal_type: Some(InternalType::Struct {
                        contract: Some("TestContract".into()),
                        ty: "Point[]".into(),
                    }),
                    components: vec![param("uint256 x"), param("uint256 y")],
                },
                Param {
                    ty: "uint8".into(),
                    name: "s".into(),
                    internal_type: Some(InternalType::Enum {
                        contract: Some("TestContract".into()),
                        ty: "Status".into(),
                    }),
                    components: vec![],
                },
            ],
            outputs: vec![Param {
                ty: "tuple".into(),
                name: String::new(),
                internal_type: Some(InternalType::Struct {
                    contract: Some("TestContract".into()),
                    ty: "Point".into(),
                }),
                components: vec![param("uint256 x"), param("uint256 y")],
            }],
            state_mutability: StateMutability::NonPayable,
        }
    );

    // Test event
    assert_eq!(
        *contract_abi.event("TestEvent").unwrap().first().unwrap(),
        Event {
            name: "TestEvent".into(),
            inputs: vec![
                EventParam {
                    ty: "tuple".into(),
                    name: "p".into(),
                    internal_type: Some(InternalType::Struct {
                        contract: Some("TestContract".into()),
                        ty: "Point".into(),
                    }),
                    components: vec![param("uint256 x"), param("uint256 y")],
                    indexed: true,
                },
                EventParam {
                    ty: "uint8".into(),
                    name: "s".into(),
                    internal_type: Some(InternalType::Enum {
                        contract: Some("TestContract".into()),
                        ty: "Status".into(),
                    }),
                    components: vec![],
                    indexed: false,
                },
            ],
            anonymous: false,
        }
    );

    // Test error
    assert_eq!(
        *contract_abi.error("TestError").unwrap().first().unwrap(),
        Error {
            name: "TestError".into(),
            inputs: vec![
                point_struct,
                Param {
                    ty: "uint8[]".into(),
                    name: "statuses".into(),
                    internal_type: Some(InternalType::Enum {
                        contract: Some("TestContract".into()),
                        ty: "Status[]".into(),
                    }),
                    components: vec![],
                },
            ],
        }
    );
}

#[test]
fn test_address_payable_internal_type() {
    sol! {
        #[sol(abi)]
        contract TestPayable {
            function testPayable(
                address payable recipient,
                address payable[] memory recipients
            ) external;
        }
    }

    let contract_abi = TestPayable::abi::contract();

    assert_eq!(
        *contract_abi.function("testPayable").unwrap().first().unwrap(),
        Function {
            name: "testPayable".into(),
            inputs: vec![
                Param {
                    ty: "address".into(),
                    name: "recipient".into(),
                    internal_type: Some(InternalType::AddressPayable("address payable".into())),
                    components: vec![],
                },
                Param {
                    ty: "address[]".into(),
                    name: "recipients".into(),
                    internal_type: Some(InternalType::AddressPayable("address payable[]".into())),
                    components: vec![],
                },
            ],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
}

#[test]
fn test_contract_type_internal_type() {
    sol! {
        #[sol(abi)]
        contract ContractA {
            function doSomething() external;
        }

        #[sol(abi)]
        contract ContractB {
            function useContract(
                ContractA contractInstance,
                ContractA[] memory instances
            ) external;
        }
    }

    let contract_abi = ContractB::abi::contract();

    assert_eq!(
        *contract_abi.function("useContract").unwrap().first().unwrap(),
        Function {
            name: "useContract".into(),
            inputs: vec![
                Param {
                    ty: "address".into(),
                    name: "contractInstance".into(),
                    internal_type: Some(InternalType::Contract("ContractA".into())),
                    components: vec![],
                },
                Param {
                    ty: "address[]".into(),
                    name: "instances".into(),
                    internal_type: Some(InternalType::Contract("ContractA[]".into())),
                    components: vec![],
                },
            ],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
}

#[test]
fn test_udvt_in_nested_struct() {
    sol! {
        #[sol(abi)]
        contract TestUDVT {
            type CustomUint is uint128;
            type CustomAddress is address;

            struct Inner {
                CustomUint amount;
                CustomAddress addr;
            }

            struct Outer {
                Inner inner;
                CustomUint[] amounts;
            }

            function processNested(Outer memory data) external;
        }
    }

    let contract_abi = TestUDVT::abi::contract();

    let inner_struct_components = vec![
        Param {
            ty: "uint128".into(),
            name: "amount".into(),
            internal_type: Some(InternalType::Other {
                contract: Some("TestUDVT".into()),
                ty: "CustomUint".into(),
            }),
            components: vec![],
        },
        Param {
            ty: "address".into(),
            name: "addr".into(),
            internal_type: Some(InternalType::Other {
                contract: Some("TestUDVT".into()),
                ty: "CustomAddress".into(),
            }),
            components: vec![],
        },
    ];

    assert_eq!(
        *contract_abi.function("processNested").unwrap().first().unwrap(),
        Function {
            name: "processNested".into(),
            inputs: vec![Param {
                ty: "tuple".into(),
                name: "data".into(),
                internal_type: Some(InternalType::Struct {
                    contract: Some("TestUDVT".into()),
                    ty: "Outer".into()
                }),
                components: vec![
                    Param {
                        ty: "tuple".into(),
                        name: "inner".into(),
                        internal_type: Some(InternalType::Struct {
                            contract: Some("TestUDVT".into()),
                            ty: "Inner".into()
                        }),
                        components: inner_struct_components,
                    },
                    Param {
                        ty: "uint128[]".into(),
                        name: "amounts".into(),
                        internal_type: Some(InternalType::Other {
                            contract: Some("TestUDVT".into()),
                            ty: "CustomUint[]".into()
                        }),
                        components: vec![],
                    },
                ],
            }],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
}

fn param(s: &str) -> Param {
    let (ty, name) = s.split_once(' ').unwrap();
    let internal_type = Some(InternalType::Other { contract: None, ty: ty.to_string() });
    Param { ty: ty.into(), name: name.into(), internal_type, components: vec![] }
}
