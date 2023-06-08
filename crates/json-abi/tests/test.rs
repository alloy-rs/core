use alloy_json_abi::{AbiJson, Error, Param, SimpleParam};

const JSON: &str = include_str!("./seaport_1_3.abi.json");

#[test]
fn deserialize() {
    let abi: AbiJson = serde_json::from_str(JSON).unwrap();
    assert_eq!(abi.len(), 67);
}

#[test]
fn round_trip() {
    let abi: AbiJson = serde_json::from_str(JSON).unwrap();
    let json = serde_json::to_string(&abi).unwrap();
    let abi2: AbiJson = serde_json::from_str(&json).unwrap();
    assert_eq!(abi, abi2);
}

#[test]
fn complex_error() {
    let err = r#"{
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
    let decoded: Error = serde_json::from_str(err).unwrap();
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
