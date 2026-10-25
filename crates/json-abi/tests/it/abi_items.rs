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

fn item(s: &str) -> Result<AbiItem<'static>, String> {
    serde_json::from_str(s).map_err(|e| e.to_string())
}

#[test]
fn deserialize_type_after_fields() {
    let f = r#"{"outputs":[],"inputs":[{"name":"a","type":"uint8"}],"payable":true,"name":"f","type":"function"}"#;
    let f = item(f).unwrap();
    assert_eq!(f.state_mutability(), Some(StateMutability::Payable));
    assert_eq!(f.inputs().unwrap()[0].ty, "uint8");

    let e = r#"{"inputs":[{"name":"a","type":"uint8","indexed":true}],"anonymous":false,"name":"E","type":"event"}"#;
    assert!(item(e).unwrap().event_inputs().unwrap()[0].indexed);
    assert!(item(&e.replace("event", "error")).is_err());
    assert!(item(&e.replace("event", "constructor")).is_err());

    // The item structs ignore `type`.
    let f = r#"{"type":"event","name":"f","inputs":[],"outputs":[]}"#;
    assert_eq!(serde_json::from_str::<Function>(f).unwrap().name, "f");
}

#[test]
fn deserialize_legacy_state_mutability() {
    let sm = |s: &str| item(s).unwrap().state_mutability();
    assert_eq!(sm(r#"{"constant":true,"type":"receive"}"#), Some(StateMutability::View));
    assert_eq!(
        sm(r#"{"stateMutability":"payable","constant":true,"type":"fallback"}"#),
        Some(StateMutability::Payable)
    );
    assert!(item(r#"{"payable":true,"constant":true,"type":"constructor","inputs":[]}"#).is_err());
}

#[test]
fn deserialize_ignores_other_kinds_fields() {
    // Fields of other item kinds are not validated, even when repeated.
    let c = r#"{"name":"1","name":"2","outputs":[{"type":"?"}],"inputs":[],"type":"constructor"}"#;
    assert_eq!(item(c).unwrap().json_type(), "constructor");

    // But repeated fields of the item's own kind are rejected wherever `type` is.
    let c = r#"{"name":"a","name":"b","inputs":[],"inputs":[{"type":"?"}],"type":"constructor"}"#;
    assert!(item(c).unwrap_err().contains("duplicate field `inputs`"));
    let c = r#"{"type":"constructor","name":"a","name":"b","inputs":[],"inputs":[]}"#;
    assert!(item(c).unwrap_err().contains("duplicate field `inputs`"));
}

#[test]
fn deserialize_item_errors() {
    assert!(item(r#"{"name":"1","name":"2","inputs":[],"outputs":[],"type":"function"}"#).is_err());
    assert!(item(r#"{"inputs":[]}"#).unwrap_err().contains("missing field `type`"));
    assert!(
        item(r#"{"type":"event","type":"event"}"#).unwrap_err().contains("duplicate field `type`")
    );
    assert!(item(r#"{"type":"nope"}"#).unwrap_err().contains("unknown variant `nope`"));
}
