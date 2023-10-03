use alloy_json_abi::{Error, JsonAbi, Param};
use std::{fs::File, io::BufReader};

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
            inputs: vec![Param {
                internal_type: Some(alloy_json_abi::InternalType::Other {
                    contract: None,
                    ty: "string".into()
                }),
                name: "reason".into(),
                ty: "string".into(),
                components: vec![],
            }],
            name: "SomeName".into()
        }
    );
}

#[test]
fn big_function() {
    let s = include_str!("abi/LargeFunction.json");
    let expected = "fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)";
    let f = serde_json::from_str::<alloy_json_abi::Function>(s).unwrap();
    assert_eq!(f.signature(), expected);
    assert_eq!(f.selector(), alloy_primitives::keccak256(expected)[..4]);
    assert_eq!(f.selector(), alloy_primitives::hex!("87201b41"));

    let ethabi = serde_json::from_str::<ethabi::Function>(s).unwrap();
    assert_eq!(f.selector(), ethabi.short_signature());
    assert_eq!(f.signature(), ethabi.signature().split_once(':').unwrap().0);
}

#[test]
#[cfg_attr(miri, ignore = "takes too long")]
fn test_constructor() {
    // Parse the ABI JSON file
    let abi_items_wo_constructor = include_str!("abi/Abiencoderv2Test.json");
    let abi_items_w_constructor = include_str!("abi/Seaport.json");

    let abi_wo_constructor: JsonAbi =
        serde_json::from_str(abi_items_wo_constructor).expect("Failed to parse ABI JSON string");
    let abi_w_constructor: JsonAbi =
        serde_json::from_str(abi_items_w_constructor).expect("Failed to parse ABI JSON string");

    // Check that the ABI JSON file has no constructor
    assert!(abi_wo_constructor.constructor().is_none());

    // Check that the ABI JSON file has a constructor
    assert!(abi_w_constructor.constructor().is_some());
}

#[test]
#[cfg_attr(miri, ignore = "no fs")]
fn no_from_reader() {
    let path = "abi/Abiencoderv2Test.json";
    let file_path: String = format!("tests/{path}");
    let file: File = File::open(file_path).unwrap();
    let buffer: BufReader<File> = BufReader::new(file);

    let res = serde_json::from_reader::<_, JsonAbi>(buffer);
    assert!(res.is_err());
    assert!(
        format!("{}", res.unwrap_err()).contains("Using serde_json::from_reader is not supported.")
    );
}
