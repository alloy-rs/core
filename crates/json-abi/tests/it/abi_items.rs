use alloy_json_abi::{
    AbiItem, Event, EventParam, Function,
    InternalType::{Other, Struct},
    Param, StateMutability,
};
use std::borrow::Cow;

#[test]
fn operation() {
    let s = r#"{
            "type":"function",
            "inputs": [{
                "name":"a",
                "type":"address"
            }],
            "name":"foo",
            "outputs": [],
            "stateMutability": "nonpayable"
        }"#;

    let deserialized: AbiItem<'static> = serde_json::from_str(s).unwrap();

    #[allow(deprecated)]
    let function = Function {
        name: "foo".to_owned(),
        inputs: vec![Param {
            name: "a".to_owned(),
            internal_type: None,
            ty: "address".into(),
            components: vec![],
        }],
        outputs: vec![],
        state_mutability: StateMutability::NonPayable,
    };

    assert_eq!(deserialized, AbiItem::Function(Cow::Owned(function)));
    let ser = serde_json::to_string(&deserialized).unwrap();
    let de: AbiItem<'_> = serde_json::from_str(&ser).unwrap();
    assert_eq!(&deserialized, &de);
    assert_ser_de!(AbiItem<'_>, deserialized);
}

#[test]
fn event_operation_with_tuple_array_input() {
    let s = r#"{
            "type":"event",
            "inputs": [
                {
                    "name":"a",
                    "type":"address",
                    "indexed":true
                },
                {
                  "components": [
                    {
                      "internalType": "address",
                      "name": "to",
                      "type": "address"
                    },
                    {
                      "internalType": "uint256",
                      "name": "value",
                      "type": "uint256"
                    },
                    {
                      "internalType": "bytes",
                      "name": "data",
                      "type": "bytes"
                    }
                  ],
                  "indexed": false,
                  "internalType": "struct Action[]",
                  "name": "b",
                  "type": "tuple[]"
                }
            ],
            "name":"E",
            "outputs": [],
            "anonymous": false
        }"#;

    let deserialized: AbiItem<'static> = serde_json::from_str(s).unwrap();

    let event = Event {
        name: "E".to_owned(),
        inputs: vec![
            EventParam {
                name: "a".to_owned(),
                indexed: true,
                ty: "address".into(),
                components: vec![],
                internal_type: None,
            },
            EventParam {
                name: "b".to_owned(),
                indexed: false,
                ty: "tuple[]".into(),
                components: vec![
                    Param {
                        name: "to".into(),
                        ty: "address".into(),
                        components: vec![],
                        internal_type: Some(Other { contract: None, ty: "address".into() }),
                    },
                    Param {
                        name: "value".into(),
                        ty: "uint256".into(),
                        components: vec![],
                        internal_type: Some(Other { contract: None, ty: "uint256".into() }),
                    },
                    Param {
                        name: "data".into(),
                        ty: "bytes".into(),
                        components: vec![],
                        internal_type: Some(Other { contract: None, ty: "bytes".into() }),
                    },
                ],
                internal_type: Some(Struct { contract: None, ty: "Action[]".into() }),
            },
        ],
        anonymous: false,
    };

    assert_eq!(deserialized, AbiItem::Event(Cow::Owned(event)));
    assert_ser_de!(AbiItem<'_>, deserialized);
}

#[test]
fn deserialize_any_field_order() {
    let item = |s: &str| serde_json::from_str::<AbiItem<'static>>(s).map_err(|e| e.to_string());

    // `type` may come after the fields that depend on it, and legacy state mutability applies.
    let f = r#"{"outputs":[],"inputs":[{"name":"a","type":"uint8"}],"payable":true,"name":"f","type":"function"}"#;
    let f = item(f).unwrap();
    assert_eq!(f.state_mutability(), Some(StateMutability::Payable));
    assert_eq!(f.inputs().unwrap()[0].ty, "uint8");
    assert_eq!(
        item(r#"{"constant":true,"type":"receive"}"#).unwrap().state_mutability(),
        Some(StateMutability::View)
    );
    assert_eq!(
        item(r#"{"stateMutability":"payable","constant":true,"type":"fallback"}"#)
            .unwrap()
            .state_mutability(),
        Some(StateMutability::Payable)
    );
    assert!(item(r#"{"payable":true,"constant":true,"type":"constructor","inputs":[]}"#).is_err());

    // Only events have indexed inputs.
    let e = r#"{"inputs":[{"name":"a","type":"uint8","indexed":true}],"anonymous":false,"name":"E","type":"event"}"#;
    assert!(item(e).unwrap().event_inputs().unwrap()[0].indexed);
    assert!(item(&e.replace("event", "error")).is_err());
    assert!(item(&e.replace("event", "constructor")).is_err());

    // Fields of other item kinds are ignored, even when repeated or invalid.
    let c = r#"{"name":"1","name":"2","outputs":[{"type":"?"}],"inputs":[],"type":"constructor"}"#;
    assert_eq!(item(c).unwrap().json_type(), "constructor");
    assert!(item(r#"{"name":"1","name":"2","inputs":[],"outputs":[],"type":"function"}"#).is_err());

    assert!(item(r#"{"inputs":[]}"#).unwrap_err().contains("missing field `type`"));
    assert!(
        item(r#"{"type":"event","type":"event"}"#).unwrap_err().contains("duplicate field `type`")
    );
    assert!(item(r#"{"type":"nope"}"#).unwrap_err().contains("unknown variant `nope`"));

    // The item structs ignore `type`.
    let f = r#"{"type":"event","name":"f","inputs":[],"outputs":[]}"#;
    assert_eq!(serde_json::from_str::<Function>(f).unwrap().name, "f");
}
