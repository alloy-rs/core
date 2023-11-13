mod test_helpers;

use alloy_json_abi::{InternalType::Struct, Param};

#[test]
fn param_simple() {
    let s = r#"{
            "name": "foo",
            "type": "address"
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: None,
            ty: "address".into(),
            components: vec![]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn param_simple_internal_type() {
    let s = r#"{
            "name": "foo",
            "type": "address",
            "internalType": "struct Verifier.Proof"
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: Some(Struct { contract: Some("Verifier".into()), ty: "Proof".into() }),
            ty: "address".into(),
            components: vec![]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn param_tuple() {
    let s = r#"{
            "name": "foo",
            "type": "tuple",
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
                            "type": "address",
                            "name": "c"
                        }
                    ]
                }
            ]
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: None,
            ty: "tuple".into(),
            components: vec![
                Param {
                    name: "a".to_owned(),
                    internal_type: None,
                    ty: "uint48".into(),
                    components: vec![],
                },
                Param {
                    name: "b".to_owned(),
                    internal_type: None,
                    ty: "tuple".into(),
                    components: vec![Param {
                        name: "c".to_owned(),
                        internal_type: None,
                        ty: "address".into(),
                        components: vec![],
                    },],
                },
            ]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn param_tuple_internal_type() {
    let s = r#"{
            "name": "foo",
            "type": "tuple",
            "internalType": "struct Pairing.G1Point[]",
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

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: Some(Struct {
                contract: Some("Pairing".into()),
                ty: "G1Point[]".into(),
            }),
            ty: "tuple".into(),
            components: vec![
                Param {
                    name: "a".to_owned(),
                    internal_type: None,
                    ty: "uint48".into(),
                    components: vec![]
                },
                Param {
                    name: "b".to_owned(),
                    internal_type: None,
                    ty: "tuple".into(),
                    components: vec![Param {
                        name: "c".to_owned(),
                        internal_type: None,
                        ty: "address".into(),
                        components: vec![]
                    }]
                }
            ]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn param_tuple_named() {
    let s = r#"{
            "name": "foo",
            "type": "tuple",
            "components": [
                {
                    "name": "amount",
                    "type": "uint48"
                },
                {
                    "name": "things",
                    "type": "tuple",
                    "components": [
                        {
                            "name": "baseTupleParam",
                            "type": "address"
                        }
                    ]
                }
            ]
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: None,
            ty: "tuple".into(),
            components: vec![
                Param {
                    name: "amount".to_owned(),
                    internal_type: None,
                    ty: "uint48".into(),
                    components: vec![]
                },
                Param {
                    name: "things".to_owned(),
                    internal_type: None,
                    ty: "tuple".into(),
                    components: vec![Param {
                        name: "baseTupleParam".to_owned(),
                        internal_type: None,
                        ty: "address".into(),
                        components: vec![]
                    },]
                },
            ]
        }
    );

    assert_ser_de!(Param, deserialized);
}

#[test]
fn param_tuple_array() {
    let s = r#"{
            "name": "foo",
            "type": "tuple[]",
            "components": [
                {
                    "name": "a",
                    "type": "uint48"
                },
                {
                    "name": "b",
                    "type": "address"
                },
                {
                    "name": "c",
                    "type": "address"
                }
            ]
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: None,
            ty: "tuple[]".into(),
            components: vec![
                Param {
                    name: "a".to_owned(),
                    internal_type: None,
                    ty: "uint48".into(),
                    components: vec![]
                },
                Param {
                    name: "b".to_owned(),
                    internal_type: None,
                    ty: "address".into(),
                    components: vec![]
                },
                Param {
                    name: "c".to_owned(),
                    internal_type: None,
                    ty: "address".into(),
                    components: vec![]
                }
            ]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn param_array_of_array_of_tuple() {
    let s = r#"{
            "name": "foo",
            "type": "tuple[][]",
            "components": [
                {
                    "name": "a",
                    "type": "uint8"
                },
                {
                    "name": "b",
                    "type": "uint16"
                }
            ]
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();
    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: None,
            ty: "tuple[][]".into(),
            components: vec![
                Param {
                    name: "a".to_owned(),
                    internal_type: None,
                    ty: "uint8".into(),
                    components: vec![]
                },
                Param {
                    name: "b".to_owned(),
                    internal_type: None,
                    ty: "uint16".into(),
                    components: vec![]
                }
            ]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn param_tuple_fixed_array() {
    let s = r#"{
            "name": "foo",
            "type": "tuple[2]",
            "components": [
                {
                    "name": "a",
                    "type": "uint48"
                },
                {
                    "name": "b",
                    "type": "address"
                },
                {
                    "name": "c",
                    "type": "address"
                }
            ]
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: None,
            ty: "tuple[2]".into(),
            components: vec![
                Param {
                    name: "a".to_owned(),
                    internal_type: None,
                    ty: "uint48".into(),
                    components: vec![]
                },
                Param {
                    name: "b".to_owned(),
                    internal_type: None,
                    ty: "address".into(),
                    components: vec![]
                },
                Param {
                    name: "c".to_owned(),
                    internal_type: None,
                    ty: "address".into(),
                    components: vec![]
                }
            ]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}

#[test]
fn param_tuple_with_nested_tuple_arrays() {
    let s = r#"{
            "name": "foo",
            "type": "tuple",
            "components": [
                {
                    "name": "a",
                    "type": "tuple[]",
                    "components": [
                        {
                            "name": "b",
                            "type": "address"
                        }
                    ]
                },
                {
                    "name": "c",
                    "type": "tuple[42]",
                    "components": [
                        {
                            "name": "d",
                            "type": "address"
                        }
                    ]
                }
            ]
        }"#;

    let deserialized: Param = serde_json::from_str(s).unwrap();

    assert_eq!(
        deserialized,
        Param {
            name: "foo".to_owned(),
            internal_type: None,
            ty: "tuple".into(),
            components: vec![
                Param {
                    name: "a".to_owned(),
                    internal_type: None,
                    ty: "tuple[]".into(),
                    components: vec![Param {
                        name: "b".to_owned(),
                        internal_type: None,
                        ty: "address".into(),
                        components: vec![]
                    },]
                },
                Param {
                    name: "c".to_owned(),
                    internal_type: None,
                    ty: "tuple[42]".into(),
                    components: vec![Param {
                        name: "d".to_owned(),
                        internal_type: None,
                        ty: "address".into(),
                        components: vec![]
                    }]
                }
            ]
        }
    );

    assert_json_eq!(s, serde_json::to_string(&deserialized).unwrap().as_str());
}
