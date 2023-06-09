use alloy_json_abi::{AbiItem, AbiJson, Error, Param, SimpleParam};

#[test]
fn complex_error() {
    let json = r#"{
    "inputs": [
        {
            "internalType": "string",
            "name": "reason",
            "type": "string"
        }
    ],
    "name": "SomeName",
    "type": "error"
}"#;
    let decoded: Error = serde_json::from_str(json).unwrap();
    assert_eq!(
        decoded,
        Error {
            inputs: vec![Param::Simple(SimpleParam {
                internal_type: "string".into(),
                name: "reason".into(),
                ty: "string".into()
            })],
            name: "SomeName".into()
        }
    );
}

macro_rules! abi_parse_tests {
    ($($name:ident($path:literal, $len:literal))*) => {$(
        #[test]
        fn $name() {
            parse_test(include_str!($path), $len);
        }
    )*};
}

abi_parse_tests! {
    abiencoderv2("abi/Abiencoderv2Test.json", 1)
    console("abi/console.json", 379)
    event_with_struct("abi/EventWithStruct.json", 1)
    large_array("abi/LargeArray.json", 1)
    large_struct("abi/LargeStruct.json", 1)
    large_structs("abi/LargeStructs.json", 4)
    large_tuple("abi/LargeTuple.json", 1)
    seaport("abi/Seaport.json", 67)
}

#[track_caller]
fn parse_test(s: &str, len: usize) {
    let abi_items: Vec<AbiItem> = serde_json::from_str(s).unwrap();
    assert_eq!(abi_items.len(), len);

    let json = serde_json::to_string(&abi_items).unwrap();
    let abi1: AbiJson = serde_json::from_str(&json).unwrap();

    let abi2: AbiJson = serde_json::from_str(s).unwrap();

    assert_eq!(abi_items.len(), abi2.len());
    assert_eq!(abi1, abi2);

    let json = serde_json::to_string(&abi2).unwrap();
    let abi3: AbiJson = serde_json::from_str(&json).unwrap();
    assert_eq!(abi2, abi3);
}
