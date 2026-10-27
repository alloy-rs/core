//! Public decoder coverage for comparison-mutated arguments.

use alloy_dyn_abi::{DynSolType, DynSolValue, FunctionExt, JsonAbiExt};
use alloy_json_abi::Function;
use alloy_sol_types::abi::AbiDecoderConfig;

#[test]
fn decode_mutated_arguments() {
    let config = AbiDecoderConfig::new().validate_words(true);
    let function = Function::parse("f(bool[],bytes,string)").unwrap();
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
        function.abi_decode_input_with_config(&data, config).unwrap(),
        vec![
            DynSolValue::Array(vec![DynSolValue::Bool(false)]),
            DynSolValue::Bytes(vec![0xaa; 5]),
            DynSolValue::String(String::from_utf8_lossy(b"\xffello").into_owned()),
        ]
    );
    // Full validation also rejects non-UTF-8 strings.
    assert!(
        function
            .abi_decode_input_with_config(&data, AbiDecoderConfig::new().validate(true))
            .is_err()
    );

    // A comparison mutation must not turn a nested bool into an invalid Solidity argument.
    data[4 * 32 + 31] = 4;
    assert!(function.abi_decode_input(&data).is_ok());
    assert!(function.abi_decode_input_with_config(&data, config).is_err());
    assert!(
        function
            .abi_decode_input_with_config(&data, AbiDecoderConfig::new().validate(true))
            .is_err()
    );

    // Dynamic decoding does not enforce strict ABI encoding, so it rejects strict configs.
    let function = Function::parse("f(bool)").unwrap();
    let mut data = function.abi_encode_input_raw(&[DynSolValue::Bool(true)]).unwrap();
    data.push(0xaa);
    assert!(
        function.abi_decode_input_with_config(&data, AbiDecoderConfig::new().strict(true)).is_err()
    );
}

#[test]
fn malformed_data_wins_over_invalid_words() {
    let function = Function::parse("f((bool,bytes))").unwrap();
    let mut data = vec![0u8; 4 * 32];
    // The tuple's offset, a non-canonical bool, then a `bytes` length that overruns the data.
    data[31] = 0x20;
    data[63] = 2;
    data[95] = 0x40;
    data[127] = 100;

    let config = AbiDecoderConfig::new().validate_words(true);
    assert_eq!(
        function.abi_decode_input_with_config(&data, config).unwrap_err(),
        alloy_sol_types::Error::Overrun.into()
    );
}

#[test]
fn decode_type_with_config() {
    let config = AbiDecoderConfig::new().validate_words(true);
    let ty = "(bool[],string)".parse::<DynSolType>().unwrap();
    let mut data = DynSolValue::Tuple(vec![
        DynSolValue::Array(vec![DynSolValue::Bool(false)]),
        DynSolValue::String("hello".into()),
    ])
    .abi_encode_params();

    // Non-UTF-8 strings are only rejected by full validation.
    data[5 * 32] = 255;
    assert!(ty.abi_decode_sequence_with_config(&data, config).is_ok());
    assert!(ty.abi_decode_params_with_config(&data, config).is_ok());
    assert!(
        ty.abi_decode_sequence_with_config(&data, AbiDecoderConfig::new().validate(true)).is_err()
    );

    // A dirty nested bool is rejected by both.
    data[5 * 32] = b'h';
    data[3 * 32 + 31] = 4;
    assert!(ty.abi_decode_sequence(&data).is_ok());
    assert!(ty.abi_decode_sequence_with_config(&data, config).is_err());
    assert!(ty.abi_decode_params_with_config(&data, config).is_err());
    assert!(
        ty.abi_decode_sequence_with_config(&data, AbiDecoderConfig::new().validate(true)).is_err()
    );

    let ty = DynSolType::Bool;
    let mut data = DynSolValue::Bool(true).abi_encode();
    data[31] = 2;
    assert!(ty.abi_decode(&data).is_ok());
    assert!(ty.abi_decode_with_config(&data, config).is_err());
    assert!(ty.abi_decode_params_with_config(&data, config).is_err());

    // Dynamic decoding does not enforce strict ABI encoding, so it rejects strict configs.
    let strict = AbiDecoderConfig::new().strict(true);
    data[31] = 1;
    data.push(0xaa);
    assert!(ty.abi_decode_with_config(&data, strict).is_err());
    let ty = DynSolType::Tuple(vec![DynSolType::Bool]);
    assert!(ty.abi_decode_sequence_with_config(&data, strict).is_err());
}

#[test]
fn decode_output_with_config() {
    let config = AbiDecoderConfig::new().validate_words(true);
    let function = Function::parse("f() returns (bool[],string)").unwrap();
    let mut data = function
        .abi_encode_output(&[
            DynSolValue::Array(vec![DynSolValue::Bool(false)]),
            DynSolValue::String("hello".into()),
        ])
        .unwrap();

    data[5 * 32] = 255;
    assert!(function.abi_decode_output_with_config(&data, config).is_ok());
    assert!(
        function
            .abi_decode_output_with_config(&data, AbiDecoderConfig::new().validate(true))
            .is_err()
    );

    data[3 * 32 + 31] = 4;
    assert!(function.abi_decode_output(&data).is_ok());
    assert!(function.abi_decode_output_with_config(&data, config).is_err());

    let function = Function::parse("f() returns (bool)").unwrap();
    let mut data = function.abi_encode_output(&[DynSolValue::Bool(true)]).unwrap();
    data.push(0xaa);
    assert!(
        function
            .abi_decode_output_with_config(&data, AbiDecoderConfig::new().strict(true))
            .is_err()
    );
}
