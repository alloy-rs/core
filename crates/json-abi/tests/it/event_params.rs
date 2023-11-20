use alloy_json_abi::{EventParam, Param};

#[test]
fn event_param_deserialization() {
    let s = r#"{
            "name": "foo",
            "type": "address",
            "indexed": true
        }"#;

    let deserialized: EventParam = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        EventParam {
            name: "foo".to_owned(),
            indexed: true,
            ty: "address".into(),
            components: vec![],
            internal_type: None,
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn event_param_tuple_deserialization() {
    let s = r#"{
            "name": "foo",
            "type": "tuple",
            "indexed": true,
            "components": [
                {
                    "name": "a",
                    "type": "uint48"
                },
                {
                    "name": "b",
                    "type": "tuple",
                    "components": [
                        {
                            "name": "c",
                            "type": "address"
                        }
                    ]
                }
            ]
        }"#;

    let deserialized: EventParam = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        EventParam {
            name: "foo".to_owned(),
            indexed: true,
            ty: "tuple".into(),
            components: vec![
                Param {
                    name: "a".into(),
                    ty: "uint48".into(),
                    components: vec![],
                    internal_type: None,
                },
                Param {
                    name: "b".into(),
                    ty: "tuple".into(),
                    components: vec![Param {
                        name: "c".into(),
                        ty: "address".into(),
                        components: vec![],
                        internal_type: None,
                    }],
                    internal_type: None,
                }
            ],
            internal_type: None,
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn event_param_tuple_array_deserialization() {
    let s = r#"{
            "components": [
                {
                    "type": "uint256",
                    "name": "a"
                },
                {
                    "type": "address",
                    "name": "b"
                },
                {
                    "components": [
                        {
                            "type": "address",
                            "name": "c"
                        },
                        {
                            "type": "address",
                            "name": "d"
                        }
                    ],
                    "type": "tuple",
                    "name": "e"
                },
                {
                    "type": "uint256",
                    "name": "f"
                },
                {
                    "components": [
                        {
                            "components": [
                                {
                                    "type": "address",
                                    "name": "g"
                                },
                                {
                                    "type": "bytes",
                                    "name": "h"
                                }
                            ],
                            "type": "tuple[]",
                            "name": "i"
                        },
                        {
                            "components": [
                                {
                                    "type": "address",
                                    "name": "j"
                                },
                                {
                                    "type": "uint256",
                                    "name": "k"
                                }
                            ],
                            "type": "tuple[]",
                            "name": "l"
                        },
                        {
                            "type": "uint256",
                            "name": "m"
                        }
                    ],
                    "type": "tuple[]",
                    "name": "n"
                },
                {
                    "type": "uint256",
                    "name": "o"
                }
            ],
            "indexed": false,
            "name": "LogTaskSubmitted",
            "type": "tuple"
        }"#;

    let deserialized: EventParam = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        EventParam {
            name: "LogTaskSubmitted".to_owned(),
            indexed: false,
            ty: "tuple".into(),
            components: vec![
                Param {
                    name: "a".into(),
                    ty: "uint256".into(),
                    components: vec![],
                    internal_type: None,
                },
                Param {
                    name: "b".into(),
                    ty: "address".into(),
                    components: vec![],
                    internal_type: None,
                },
                Param {
                    name: "e".into(),
                    ty: "tuple".into(),
                    components: vec![
                        Param {
                            name: "c".into(),
                            ty: "address".into(),
                            components: vec![],
                            internal_type: None,
                        },
                        Param {
                            name: "d".into(),
                            ty: "address".into(),
                            components: vec![],
                            internal_type: None,
                        },
                    ],
                    internal_type: None,
                },
                Param {
                    name: "f".into(),
                    ty: "uint256".into(),
                    components: vec![],
                    internal_type: None,
                },
                Param {
                    name: "n".into(),
                    ty: "tuple[]".into(),
                    components: vec![
                        Param {
                            name: "i".into(),
                            ty: "tuple[]".into(),
                            components: vec![
                                Param {
                                    name: "g".into(),
                                    ty: "address".into(),
                                    components: vec![],
                                    internal_type: None,
                                },
                                Param {
                                    name: "h".into(),
                                    ty: "bytes".into(),
                                    components: vec![],
                                    internal_type: None,
                                },
                            ],
                            internal_type: None,
                        },
                        Param {
                            name: "l".into(),
                            ty: "tuple[]".into(),
                            components: vec![
                                Param {
                                    name: "j".into(),
                                    ty: "address".into(),
                                    components: vec![],
                                    internal_type: None,
                                },
                                Param {
                                    name: "k".into(),
                                    ty: "uint256".into(),
                                    components: vec![],
                                    internal_type: None,
                                },
                            ],
                            internal_type: None,
                        },
                        Param {
                            name: "m".into(),
                            ty: "uint256".into(),
                            components: vec![],
                            internal_type: None,
                        },
                    ],
                    internal_type: None,
                },
                Param {
                    name: "o".into(),
                    ty: "uint256".into(),
                    components: vec![],
                    internal_type: None,
                },
            ],
            internal_type: None
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}
