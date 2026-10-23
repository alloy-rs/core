//! Public decoder coverage for comparison-mutated arguments.

use alloy_dyn_abi::{DynSolType, DynSolValue};
use alloy_sol_types::abi::AbiDecoderConfig;

#[test]
fn decode_mutated_arguments() {
    let config = AbiDecoderConfig::new().validate(true);
    let ty = "(bool[],bytes,string)".parse::<DynSolType>().unwrap();
    let mut data = DynSolValue::Tuple(vec![
        DynSolValue::Array(vec![DynSolValue::Bool(false)]),
        DynSolValue::Bytes(vec![0xaa; 40]),
        DynSolValue::String("hello".into()),
    ])
    .abi_encode_params();

    // Shorten bytes without clearing its old payload, and introduce non-UTF-8 string data.
    data[5 * 32 + 31] = 5;
    data[9 * 32] = 255;
    assert_eq!(
        ty.abi_decode_sequence_with_config(&data, config).unwrap(),
        DynSolValue::Tuple(vec![
            DynSolValue::Array(vec![DynSolValue::Bool(false)]),
            DynSolValue::Bytes(vec![0xaa; 5]),
            DynSolValue::String(String::from_utf8_lossy(b"\xffello").into_owned()),
        ])
    );

    // A comparison mutation must not turn a nested bool into an invalid Solidity argument.
    data[4 * 32 + 31] = 4;
    assert!(ty.abi_decode_sequence(&data).is_ok());
    assert!(ty.abi_decode_sequence_with_config(&data, config).is_err());
}

#[test]
fn configured_scalar_validation() {
    let config = AbiDecoderConfig::new().validate(true);
    for (ty, dirty_byte, dirty_value) in [
        ("bool", 31, 2),
        ("address", 0, 1),
        ("function", 31, 1),
        ("uint8", 30, 1),
        ("int8", 31, 0x80),
        ("bytes4", 4, 1),
    ] {
        let ty = ty.parse::<DynSolType>().unwrap();
        let mut data = [0; 32];
        assert!(ty.abi_decode_with_config(&data, config).is_ok(), "{ty}");
        data[dirty_byte] = dirty_value;
        assert!(ty.abi_decode(&data).is_ok(), "{ty}");
        assert!(ty.abi_decode_with_config(&data, config).is_err(), "{ty}");
        assert!(ty.abi_decode_params_with_config(&data, config).is_err(), "{ty}");
        assert!(
            ty.abi_decode_with_config(&data, config.strict(true).validate(false)).is_err(),
            "{ty}"
        );
        assert!(ty.abi_decode_with_config(&data, config.validate(false)).is_ok(), "{ty}");

        let tuple = DynSolType::Tuple(vec![ty]);
        assert!(tuple.abi_decode_sequence_with_config(&data, config).is_err());
        assert!(tuple.abi_decode_params_with_config(&data, config).is_err());
        assert!(tuple.abi_decode_sequence_checked(&data).is_err());
    }
}

#[test]
fn configured_nested_decoding() {
    let ty = "((bool,bytes)[],int8[2])".parse::<DynSolType>().unwrap();
    let value = DynSolValue::Tuple(vec![
        DynSolValue::Array(vec![DynSolValue::Tuple(vec![
            DynSolValue::Bool(true),
            DynSolValue::Bytes(vec![0xab]),
        ])]),
        DynSolValue::FixedArray(vec![
            DynSolValue::Int((-1).try_into().unwrap(), 8),
            DynSolValue::Int(127.try_into().unwrap(), 8),
        ]),
    ]);
    let config = AbiDecoderConfig::new().validate(true);
    let strict = config.strict(true);
    let mut data = value.abi_encode_params();
    assert_eq!(ty.abi_decode_params_with_config(&data, config).unwrap(), value);
    assert_eq!(ty.abi_decode_params_with_config(&data, strict).unwrap(), value);
    assert_eq!(ty.abi_decode_with_config(&value.abi_encode(), strict).unwrap(), value);
    data.push(0xff);
    assert_eq!(ty.abi_decode_params_with_config(&data, config).unwrap(), value);
    assert!(ty.abi_decode_params_with_config(&data, strict).is_err());
    assert_eq!(
        ty.abi_decode_params_with_config(&data, strict.validate_allow_trailing_bytes(true))
            .unwrap(),
        value,
    );
    assert!(ty.abi_decode_params_with_config(&data, config.recursion_limit(0)).is_err());
}

#[test]
fn configured_memory_limit() {
    let ty = DynSolType::Array(Box::new(DynSolType::Bool));
    let data = DynSolValue::Array(vec![DynSolValue::Bool(false); 2]).abi_encode();
    let config = AbiDecoderConfig::new().validate(true).memory_limit(0);
    assert!(ty.abi_decode_with_config(&data, config).is_err());
}
