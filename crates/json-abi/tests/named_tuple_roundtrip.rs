use alloy_json_abi::{Event, EventParam, Function, Param, StateMutability};

fn tuple_param() -> Param {
    // These fields are all represented in a full signature; internalType is not.
    serde_json::from_value(serde_json::json!({
        "name": "orders",
        "type": "tuple[][2]",
        "components": [
            { "name": "who", "type": "address" },
            {
                "name": "transfers", "type": "tuple[]", "components": [
                    { "name": "amount", "type": "uint256" },
                    { "name": "", "type": "bool" },
                    {
                        "name": "metadata", "type": "tuple[3]", "components": [
                            { "name": "$tag", "type": "bytes32" },
                            { "name": "enabled", "type": "bool" }
                        ]
                    }
                ]
            },
            { "name": "data", "type": "bytes" }
        ]
    }))
    .unwrap()
}

#[test]
fn function_full_signature_preserves_tuple_names() {
    for state_mutability in [
        StateMutability::NonPayable,
        StateMutability::Payable,
        StateMutability::View,
        StateMutability::Pure,
    ] {
        let mut output = tuple_param();
        output.name = "result".into();
        let function = Function {
            name: "process".into(),
            inputs: vec![tuple_param()],
            outputs: vec![output, Param::parse("bool success").unwrap()],
            state_mutability,
        };
        let parsed = Function::parse(&function.full_signature()).unwrap();
        assert_eq!(parsed, function);
        assert_eq!(
            parsed.signature(),
            "process((address,(uint256,bool,(bytes32,bool)[3])[],bytes)[][2])"
        );
        assert_eq!(parsed.selector(), function.selector());
    }
}

#[test]
fn event_full_signature_preserves_tuple_names() {
    let param = tuple_param();
    for indexed in [false, true] {
        for anonymous in [false, true] {
            let event = Event {
                name: "Processed".into(),
                inputs: vec![EventParam {
                    name: param.name.clone(),
                    ty: param.ty.clone(),
                    components: param.components.clone(),
                    internal_type: None,
                    indexed,
                }],
                anonymous,
            };
            let parsed = Event::parse(&event.full_signature()).unwrap();
            assert_eq!(parsed, event);
            assert_eq!(
                parsed.signature(),
                "Processed((address,(uint256,bool,(bytes32,bool)[3])[],bytes)[][2])"
            );
            assert_eq!(parsed.selector(), event.selector());
        }
    }
}

#[test]
fn tuple_component_names_with_whitespace_and_trailing_commas() {
    let expected: Param = serde_json::from_value(serde_json::json!({
        "name": "orders", "type": "tuple[]", "components": [
            { "name": "first", "type": "uint256" },
            { "name": "", "type": "uint256" },
            { "name": "last", "type": "uint256" },
            {
                "name": "nested", "type": "tuple[2]", "components": [
                    { "name": "who", "type": "address" },
                    { "name": "", "type": "bytes" }
                ]
            },
            { "name": "empty", "type": "tuple", "components": [] }
        ]
    }))
    .unwrap();
    let components = "( uint256 first ,\tuint256, uint256 last, tuple( address who , bytes, )[2] nested, () empty, )[]";
    for prefix in ["", "tuple"] {
        let input = format!("{prefix}{components} memory orders");
        assert_eq!(Param::parse(&input).unwrap(), expected);
        let input = format!("{prefix}{components} indexed orders");
        let parsed = EventParam::parse(&input).unwrap();
        assert_eq!(parsed.name, expected.name);
        assert_eq!(parsed.ty, expected.ty);
        assert_eq!(parsed.components, expected.components);
        assert!(parsed.indexed);
    }
}
