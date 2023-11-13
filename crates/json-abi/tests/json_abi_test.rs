mod test_helpers;

use alloy_json_abi::{
    Constructor, Error, Event, EventParam, Fallback, Function, JsonAbi, Param, Receive,
    StateMutability,
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
