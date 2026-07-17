use alloy_json_abi::{
    Constructor, EnumDefinitions, Error, Event, EventParam, Fallback, Function, JsonAbi, Param,
    Receive, StateMutability, ToSolConfig,
};
use std::collections::BTreeMap;

#[test]
fn empty() {
    let json = "[]";

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn constructor() {
    let json = r#"
        [
            {
                "type": "constructor",
                "inputs": [
                    {
                        "name":"a",
                        "type":"address"
                    }
                ],
                "stateMutability": "nonpayable"
            }
        ]
    "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: Some(Constructor {
                inputs: vec![Param {
                    name: "a".to_string(),
                    internal_type: None,
                    ty: "address".into(),
                    components: vec![],
                }],
                state_mutability: StateMutability::NonPayable
            }),
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn functions() {
    let json = r#"
            [
                {
                    "type": "function",
                    "name": "foo",
                    "inputs": [
                        {
                            "name":"a",
                            "type":"address"
                        }
                    ],
                    "outputs": [
                        {
                            "name": "res",
                            "type":"address"
                        }
                    ],
                    "stateMutability": "nonpayable"
                },
                {
                    "type": "function",
                    "name": "bar",
                    "inputs": [],
                    "outputs": [],
                    "stateMutability": "nonpayable"
                }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::from_iter(vec![
                (
                    "foo".into(),
                    vec![Function {
                        name: "foo".into(),
                        inputs: vec![Param {
                            name: "a".into(),
                            internal_type: None,
                            ty: "address".into(),
                            components: vec![]
                        }],
                        outputs: vec![Param {
                            name: "res".into(),
                            internal_type: None,
                            ty: "address".into(),
                            components: vec![]
                        }],
                        state_mutability: StateMutability::NonPayable,
                    }]
                ),
                (
                    "bar".into(),
                    vec![Function {
                        name: "bar".into(),
                        inputs: vec![],
                        outputs: vec![],
                        state_mutability: StateMutability::NonPayable,
                    }]
                ),
            ]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn functions_overloads() {
    let json = r#"
            [
                {
                    "type": "function",
                    "name": "foo",
                    "inputs": [
                        {
                            "name":"a",
                            "type":"address"
                        }
                    ],
                    "outputs": [
                        {
                            "name": "res",
                            "type":"address"
                        }
                    ],
                    "stateMutability": "nonpayable"
                },
                {
                    "type": "function",
                    "name": "foo",
                    "inputs": [],
                    "outputs": [],
                    "stateMutability": "nonpayable"
                }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "foo".to_string(),
                vec![
                    Function {
                        name: "foo".into(),
                        inputs: vec![Param {
                            name: "a".into(),
                            internal_type: None,
                            ty: "address".into(),
                            components: vec![],
                        }],
                        outputs: vec![Param {
                            name: "res".into(),
                            internal_type: None,
                            ty: "address".into(),
                            components: vec![]
                        }],
                        state_mutability: StateMutability::NonPayable,
                    },
                    Function {
                        name: "foo".into(),
                        inputs: vec![],
                        outputs: vec![],
                        state_mutability: StateMutability::NonPayable,
                    },
                ]
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn events() {
    let json = r#"
            [
                {
                    "type": "event",
                    "name": "foo",
                    "inputs": [
                        {
                            "name":"a",
                            "type":"address",
                            "indexed": false
                        }
                    ],
                    "anonymous": false
                },
                {
                    "type": "event",
                    "name": "bar",
                    "inputs": [
                        {
                            "name":"a",
                            "type":"address",
                            "indexed": true
                        }
                    ],
                    "anonymous": false
                }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::new(),
            events: BTreeMap::from_iter(vec![
                (
                    "foo".into(),
                    vec![Event {
                        name: "foo".into(),
                        inputs: vec![EventParam {
                            name: "a".into(),
                            indexed: false,
                            ty: "address".into(),
                            components: vec![],
                            internal_type: None
                        }],
                        anonymous: false,
                    }]
                ),
                (
                    "bar".to_string(),
                    vec![Event {
                        name: "bar".into(),
                        inputs: vec![EventParam {
                            name: "a".into(),
                            indexed: true,
                            ty: "address".into(),
                            components: vec![],
                            internal_type: None
                        }],
                        anonymous: false,
                    }]
                ),
            ]),
            errors: BTreeMap::new(),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn events_overload() {
    let json = r#"
            [
                {
                    "type": "event",
                    "name": "foo",
                    "inputs": [
                        {
                            "name":"a",
                            "type":"address",
                            "indexed": false
                        }
                    ],
                    "anonymous": false
                },
                {
                    "type": "event",
                    "name": "foo",
                    "inputs": [
                        {
                            "name":"a",
                            "type":"address",
                            "indexed": true
                        }
                    ],
                    "anonymous": false
                }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::new(),
            events: BTreeMap::from_iter(vec![(
                "foo".to_string(),
                vec![
                    Event {
                        name: "foo".into(),
                        inputs: vec![EventParam {
                            name: "a".into(),
                            indexed: false,
                            ty: "address".into(),
                            components: vec![],
                            internal_type: None
                        }],
                        anonymous: false,
                    },
                    Event {
                        name: "foo".into(),
                        inputs: vec![EventParam {
                            name: "a".into(),
                            indexed: true,
                            ty: "address".into(),
                            components: vec![],
                            internal_type: None
                        }],
                        anonymous: false,
                    },
                ]
            )]),
            errors: BTreeMap::new(),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn errors() {
    let json = r#"
            [
              {
                "type": "error",
                "inputs": [
                  {
                    "name": "available",
                    "type": "uint256"
                  },
                  {
                    "name": "required",
                    "type": "address"
                  }
                ],
                "name": "foo"
              },
              {
                "type": "error",
                "inputs": [
                  {
                    "name": "a",
                    "type": "uint256"
                  },
                  {
                    "name": "b",
                    "type": "address"
                  }
                ],
                "name": "bar"
              }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            errors: BTreeMap::from_iter(vec![
                (
                    "foo".to_string(),
                    vec![Error {
                        name: "foo".into(),
                        inputs: vec![
                            Param {
                                name: "available".into(),
                                internal_type: None,
                                ty: "uint256".into(),
                                components: vec![]
                            },
                            Param {
                                name: "required".into(),
                                internal_type: None,
                                ty: "address".into(),
                                components: vec![],
                            }
                        ],
                    }]
                ),
                (
                    "bar".to_string(),
                    vec![Error {
                        name: "bar".into(),
                        inputs: vec![
                            Param {
                                name: "a".into(),
                                internal_type: None,
                                ty: "uint256".into(),
                                components: vec![]
                            },
                            Param {
                                name: "b".into(),
                                internal_type: None,
                                ty: "address".into(),
                                components: vec![]
                            }
                        ],
                    }]
                ),
            ]),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn errors_overload() {
    let json = r#"
            [
              {
                "type": "error",
                "inputs": [
                  {
                    "name": "a",
                    "type": "uint256"
                  }
                ],
                "name": "foo"
              },
              {
                "type": "error",
                "inputs": [
                  {
                    "name": "a",
                    "type": "uint256"
                  },
                  {
                    "name": "b",
                    "type": "address"
                  }
                ],
                "name": "foo"
              }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            errors: BTreeMap::from_iter(vec![(
                "foo".to_string(),
                vec![
                    Error {
                        name: "foo".into(),
                        inputs: vec![Param {
                            name: "a".into(),
                            internal_type: None,
                            ty: "uint256".into(),
                            components: vec![],
                        }],
                    },
                    Error {
                        name: "foo".into(),
                        inputs: vec![
                            Param {
                                name: "a".into(),
                                internal_type: None,
                                ty: "uint256".into(),
                                components: vec![],
                            },
                            Param {
                                name: "b".into(),
                                internal_type: None,
                                ty: "address".into(),
                                components: vec![],
                            }
                        ],
                    },
                ]
            ),]),
            receive: None,
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn receive() {
    let json = r#"
            [
                {
                    "type": "receive",
                    "stateMutability": "nonpayable"
                }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: Some(Receive { state_mutability: StateMutability::NonPayable }),
            fallback: None,
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn fallback() {
    let json = r#"
            [
                {
                    "type": "fallback",
                    "stateMutability": "nonpayable"
                }
            ]
        "#;

    let deserialized: JsonAbi = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        JsonAbi {
            constructor: None,
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: None,
            fallback: Some(Fallback { state_mutability: StateMutability::NonPayable }),
        }
    );

    assert_ser_de!(JsonAbi, deserialized);
}

#[test]
fn one_contract() {
    let test_contract_json = include_str!("../abi/TestContract.json");
    let deserialized: JsonAbi = serde_json::from_str(test_contract_json).unwrap();

    // Default config, generates the struct in a library
    let sol_interface = deserialized.to_sol("TestContract", Default::default());

    assert_eq!(
        sol_interface.trim(),
        r#"library ITestContract {
    type TestEnum is uint8;
    type Unsigned is uint256;
    struct TestStruct {
        address asset;
    }
}

interface TestContract {
    error TestError(ITestContract.Unsigned);

    event TestEvent(ITestContract.Unsigned);

    function test_struct(ITestContract.TestStruct memory) external;
}"#
        .trim()
    );

    let config = ToSolConfig::default().one_contract(true);
    let sol_interface = deserialized.to_sol("TestContract", Some(config));
    assert_eq!(
        sol_interface.trim(),
        r#"interface TestContract {
    // Types from `ITestContract`
    type TestEnum is uint8;
    type Unsigned is uint256;
    struct TestStruct {
        address asset;
    }

    error TestError(Unsigned);

    event TestEvent(Unsigned);

    function test_struct(TestStruct memory) external;
}"#
        .trim()
    );
}

#[test]
fn known_enum_definitions() {
    let abi = serde_json::from_str::<JsonAbi>(
        r#"[
            {
                "type": "function",
                "name": "useValues",
                "stateMutability": "pure",
                "inputs": [
                    {"name": "status", "type": "uint8", "internalType": "enum EnumUser.Status"},
                    {"name": "global", "type": "uint8", "internalType": "enum GlobalStatus"},
                    {"name": "external", "type": "uint8[]", "internalType": "enum Types.ExternalStatus[]"},
                    {
                        "name": "wrapper",
                        "type": "tuple",
                        "internalType": "struct EnumUser.Wrapper",
                        "components": [
                            {"name": "nested", "type": "uint8", "internalType": "enum EnumUser.Status"}
                        ]
                    },
                    {"name": "raw", "type": "uint8", "internalType": "type EnumUser.RawStatus"}
                ],
                "outputs": []
            }
        ]"#,
    )
    .unwrap();

    let default = abi.to_sol("EnumUser", None);
    assert!(default.contains("type Status is uint8;"));
    assert_eq!(default, abi.to_sol_with_enums("EnumUser", None, &EnumDefinitions::new()));

    let mut enums = EnumDefinitions::new();
    enums.insert("EnumUser.Status".into(), vec!["Pending".into(), "Active".into()]);
    enums.insert("GlobalStatus".into(), vec!["Off".into(), "On".into()]);
    enums.insert("Types.ExternalStatus".into(), vec!["Open".into(), "Closed".into()]);
    enums.insert("Unused".into(), vec!["Ignored".into()]);
    let enriched = abi.to_sol_with_enums("EnumUser", None, &enums);
    assert!(enriched.contains("enum Status { Pending, Active }"));
    assert!(enriched.contains("enum GlobalStatus { Off, On }"));
    assert!(enriched.contains("library Types"));
    assert!(enriched.contains("enum ExternalStatus { Open, Closed }"));
    assert!(enriched.contains("type RawStatus is uint8;"));
    assert!(enriched.contains("Status status"), "{enriched}");
    assert!(enriched.contains("GlobalStatus global"), "{enriched}");
    assert!(enriched.contains("Types.ExternalStatus[] memory external"), "{enriched}");
    assert!(enriched.contains("Status nested"), "{enriched}");
    assert!(enriched.contains("RawStatus raw"), "{enriched}");
    assert!(!enriched.contains("Unused"));

    let config = ToSolConfig::new().enums_as_udvt(false);
    let enums_disabled = abi.to_sol_with_enums("EnumUser", Some(config), &enums);
    assert!(!enums_disabled.contains("enum Status"));
    assert!(enums_disabled.contains("uint8 status"), "{enums_disabled}");
}

#[test]
fn conflicting_enum_definitions_fall_back_to_udvt() {
    let abi = serde_json::from_str::<JsonAbi>(
        r#"[{
            "type": "function",
            "name": "useValues",
            "stateMutability": "pure",
            "inputs": [
                {"name": "global", "type": "uint8", "internalType": "enum Status"},
                {"name": "local", "type": "uint8", "internalType": "enum C.Status"}
            ],
            "outputs": []
        }]"#,
    )
    .unwrap();
    let enums = EnumDefinitions::from([
        ("Status".into(), vec!["Global".into()]),
        ("C.Status".into(), vec!["Local".into()]),
    ]);

    let output = abi.to_sol_with_enums("C", None, &enums);
    assert_eq!(output.matches("type Status is uint8;").count(), 1, "{output}");
    assert!(!output.contains("enum Status"), "{output}");
}

#[test]
fn invalid_enum_definitions_fall_back_to_udvt() {
    let abi = serde_json::from_str::<JsonAbi>(
        r#"[{
            "type": "function",
            "name": "useValue",
            "stateMutability": "pure",
            "inputs": [
                {"name": "value", "type": "uint8", "internalType": "enum C.Status"}
            ],
            "outputs": []
        }]"#,
    )
    .unwrap();

    for variants in [Vec::new(), (0..257).map(|i| format!("V{i}")).collect()] {
        let enums = EnumDefinitions::from([("C.Status".into(), variants)]);
        let output = abi.to_sol_with_enums("C", None, &enums);
        assert_eq!(output.matches("type Status is uint8;").count(), 1, "{output}");
        assert!(!output.contains("enum Status"), "{output}");
    }
}

#[test]
fn flattened_conflicting_enum_definitions_fall_back_to_udvt() {
    let abi = serde_json::from_str::<JsonAbi>(
        r#"[{
            "type": "function",
            "name": "useValues",
            "stateMutability": "pure",
            "inputs": [
                {"name": "a", "type": "uint8", "internalType": "enum A.Status"},
                {"name": "b", "type": "uint8", "internalType": "enum B.Status"}
            ],
            "outputs": []
        }]"#,
    )
    .unwrap();
    let config = ToSolConfig::new().one_contract(true);
    let cases = [
        (
            EnumDefinitions::from([
                ("A.Status".into(), vec!["Same".into()]),
                ("B.Status".into(), vec!["Same".into()]),
            ]),
            true,
        ),
        (EnumDefinitions::new(), false),
        (EnumDefinitions::from([("A.Status".into(), vec!["Known".into()])]), false),
        (
            EnumDefinitions::from([
                ("A.Status".into(), vec!["A".into()]),
                ("B.Status".into(), vec!["B".into()]),
            ]),
            false,
        ),
    ];

    for (enums, expect_enum) in cases {
        let output = abi.to_sol_with_enums("C", Some(config.clone()), &enums);
        if expect_enum {
            assert_eq!(output.matches("enum Status { Same }").count(), 1, "{output}");
            assert!(!output.contains("type Status"), "{output}");
        } else {
            assert_eq!(output.matches("type Status is uint8;").count(), 1, "{output}");
            assert!(!output.contains("enum Status"), "{output}");
        }
    }
}
