//! Public decoder coverage for comparison-mutated arguments.

use alloy_dyn_abi::{DynSolValue, JsonAbiExt};
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
