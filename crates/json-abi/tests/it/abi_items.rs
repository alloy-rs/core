use alloy_json_abi::{
    AbiItem, Error, Event, EventParam, Function,
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

#[test]
fn deserialize_event_and_error_sequences() {
    let e = serde_json::from_str::<Event>(r#"["E",[{"type":"uint8","indexed":true}],true]"#);
    let e = e.unwrap();
    assert_eq!((e.name.as_str(), e.inputs[0].indexed, e.anonymous), ("E", true, true));
    let e = serde_json::from_value::<Error>(serde_json::json!(["E", [{"type":"uint8"}]])).unwrap();
    assert_eq!((e.name.as_str(), e.inputs.len()), ("E", 1));

    assert!(serde_json::from_str::<Event>(r#"["E",[]]"#).is_err());
    assert!(serde_json::from_str::<Error>(r#"["1",[]]"#).is_err());
    assert!(serde_json::from_str::<Error>(r#"["E",[{"type":"uint8","indexed":true}]]"#).is_err());
    assert!(serde_json::from_str::<Function>(r#"["f",[],[]]"#).is_err());
    assert!(item(r#"["E",[]]"#).is_err());
}

#[test]
fn full_signature_with_named_components_parses_back() {
    let order = r#"[{"name":"who","type":"address"},{"name":"amounts","type":"uint256[]"},
        {"name":"inner","type":"tuple","components":[{"name":"tag","type":"bytes32"}]}]"#;
    let event: Event = serde_json::from_str(&format!(
        r#"{{"type":"event","name":"Placed","anonymous":false,"inputs":[
            {{"name":"by","type":"address","indexed":true}},
            {{"name":"order","type":"tuple","indexed":false,"components":{order}}},
            {{"name":"orders","type":"tuple[]","indexed":false,"components":{order}}}]}}"#
    ))
    .unwrap();
    let sig = event.full_signature();
    assert_eq!(
        sig,
        "event Placed(address indexed by, tuple(address who, uint256[] amounts, tuple(bytes32 tag) inner) order, tuple(address who, uint256[] amounts, tuple(bytes32 tag) inner)[] orders)"
    );
    let parsed = Event::parse(&sig).unwrap();
    assert_eq!(parsed.selector(), event.selector());
    assert_eq!(
        parsed.inputs.iter().map(|input| input.indexed).collect::<Vec<_>>(),
        [true, false, false]
    );

    let function: Function = serde_json::from_str(&format!(
        r#"{{"type":"function","name":"place","stateMutability":"nonpayable","inputs":[
            {{"name":"order","type":"tuple","components":{order}}}],"outputs":[
            {{"name":"","type":"tuple","components":{order}}}]}}"#
    ))
    .unwrap();
    let parsed = Function::parse(&function.full_signature()).unwrap();
    assert_eq!(parsed.selector(), function.selector());
    assert_eq!(parsed.outputs[0].selector_type(), function.outputs[0].selector_type());
}
