#[cfg(test)]
mod test {
    use alloy_json_abi::{
        AbiItem, Event, EventParam, Function,
        InternalType::{Other, Struct},
        Param, StateMutability,
    };
    use std::borrow::Cow;

    macro_rules! assert_ser_de {
        ($value:expr) => {{
            let ser = serde_json::to_string(&$value).unwrap();
            let de: AbiItem<'_> = serde_json::from_str(&ser).unwrap();
            assert_eq!(
                &$value, &de,
                "Original value and deserialized value do not match."
            );
        }};
    }

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
        assert_ser_de!(deserialized);
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
                            internal_type: Some(Other {
                                contract: None,
                                ty: "address".into(),
                            }),
                        },
                        Param {
                            name: "value".into(),
                            ty: "uint256".into(),
                            components: vec![],
                            internal_type: Some(Other {
                                contract: None,
                                ty: "uint256".into(),
                            }),
                        },
                        Param {
                            name: "data".into(),
                            ty: "bytes".into(),
                            components: vec![],
                            internal_type: Some(Other {
                                contract: None,
                                ty: "bytes".into(),
                            }),
                        },
                    ],
                    internal_type: Some(Struct {
                        contract: None,
                        ty: "Action[]".into(),
                    }),
                },
            ],
            anonymous: false,
        };

        assert_eq!(deserialized, AbiItem::Event(Cow::Owned(event)));
        assert_ser_de!(deserialized);
    }

    // #[test]
    // fn sanitize_function_name() {
    //     fn test_sanitize_function_name(name: &str, expected: &str) {
    //         let s = format!(
    //             r#"{{
    // 				"type":"function",
    // 				"inputs": [{{
    // 					"name":"a",
    // 					"type":"address"
    // 				}}],
    // 				"name":"{}",
    // 				"outputs": []
    // 			}}"#,
    //             name
    //         );

    //         let deserialized: AbiItem = serde_json::from_str(&s).unwrap();
    //         let function = match &deserialized {
    //             AbiItem::Function(f) => f,
    //             _ => panic!("expected function"),
    //         };

    //         assert_eq!(function.name, expected);

    //         assert_ser_de!(deserialized);
    //     }

    //     test_sanitize_function_name("foo", "foo");
    //     test_sanitize_function_name("foo()", "foo");
    //     test_sanitize_function_name("()", "");
    //     test_sanitize_function_name("", "");
    // }

    // #[test]
    // fn sanitize_event_name() {
    //     fn test_sanitize_event_name(name: &str, expected: &str) {
    //         let s = format!(
    //             r#"{{
    // 				"type":"event",
    // 					"inputs": [{{
    // 						"name":"a",
    // 						"type":"address",
    // 						"indexed":true
    // 					}}],
    // 					"name":"{}",
    // 					"outputs": [],
    // 					"anonymous": false
    // 			}}"#,
    //             name
    //         );

    //         let deserialized: AbiItem = serde_json::from_str(&s).unwrap();
    //         let event = match deserialized {
    //             AbiItem::Event(e) => e,
    //             _ => panic!("expected event!"),
    //         };

    //         assert_eq!(event.name, expected);

    //         assert_ser_de!(AbiItem::Event(event.clone()));
    //     }

    //     test_sanitize_event_name("foo", "foo");
    //     test_sanitize_event_name("foo()", "foo");
    //     test_sanitize_event_name("()", "");
    //     test_sanitize_event_name("", "");
    // }
}
