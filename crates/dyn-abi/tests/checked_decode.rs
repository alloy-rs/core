//! Public decoder coverage for comparison-mutated arguments.

use alloy_dyn_abi::{
    DynSolCall, DynSolType, DynSolValue, ErrorExt, EventExt, FunctionExt, JsonAbiExt,
};
use alloy_json_abi::{Error, Event, EventParam, Function};
use alloy_primitives::{B256, LogData, U256};
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

/// A `bool` word that is not 0 or 1.
const fn dirty_bool() -> [u8; 32] {
    let mut word = [0u8; 32];
    word[31] = 2;
    word
}

#[test]
fn type_decoders_validate_words() {
    let config = AbiDecoderConfig::new().validate_words(true);
    let data = dirty_bool();

    let ty = DynSolType::Bool;
    assert_eq!(ty.abi_decode(&data), Ok(DynSolValue::Bool(true)));
    assert_eq!(
        ty.abi_decode_with_config(&data, config).unwrap_err(),
        alloy_sol_types::Error::type_check_fail(&data, "bool").into()
    );
    assert_eq!(ty.abi_decode_params(&data), Ok(DynSolValue::Bool(true)));
    assert!(ty.abi_decode_params_with_config(&data, config).is_err());

    let ty = DynSolType::Tuple(vec![DynSolType::Bool]);
    assert_eq!(
        ty.abi_decode_sequence(&data),
        Ok(DynSolValue::Tuple(vec![DynSolValue::Bool(true)]))
    );
    assert!(ty.abi_decode_sequence_with_config(&data, config).is_err());
    assert!(ty.abi_decode_params_with_config(&data, config).is_err());

    // Nested words, and padding of other word types.
    let ty = DynSolType::Array(Box::new(DynSolType::Uint(8)));
    let mut data = DynSolValue::Array(vec![DynSolValue::Uint(U256::from(1), 8)]).abi_encode();
    assert!(ty.abi_decode_with_config(&data, config).is_ok());
    data[2 * 32] = 1;
    assert!(ty.abi_decode(&data).is_ok());
    assert!(ty.abi_decode_with_config(&data, config).is_err());

    // Canonical data is unaffected, and strict decoding is unsupported.
    let mut data = dirty_bool();
    data[31] = 1;
    let ty = DynSolType::Bool;
    assert_eq!(ty.abi_decode_with_config(&data, config), Ok(DynSolValue::Bool(true)));
    assert!(ty.abi_decode_with_config(&data, AbiDecoderConfig::new().strict(true)).is_err());
}

#[test]
fn call_decoders_validate_words() {
    let config = AbiDecoderConfig::new().validate_words(true);
    let data = dirty_bool();

    let function = Function::parse("f() returns (bool)").unwrap();
    assert_eq!(function.abi_decode_output(&data).unwrap(), vec![DynSolValue::Bool(true)]);
    assert!(function.abi_decode_output_with_config(&data, config).is_err());

    let call = DynSolCall::new(
        Default::default(),
        vec![DynSolType::Bool],
        None,
        vec![DynSolType::Bool].into(),
    );
    assert!(call.abi_decode_input(&data).is_ok());
    assert!(call.abi_decode_input_with_config(&data, config).is_err());
    assert!(call.abi_decode_output(&data).is_ok());
    assert!(call.abi_decode_output_with_config(&data, config).is_err());
}

#[test]
fn error_decoder_validates_words() {
    let config = AbiDecoderConfig::new().validate_words(true);
    let error = Error::parse("E(bool)").unwrap();
    let mut data = error.selector().to_vec();
    data.extend_from_slice(&dirty_bool());

    assert_eq!(error.decode_error(&data).unwrap().body, vec![DynSolValue::Bool(true)]);
    assert!(error.decode_error_with_config(&data, config).is_err());
}

#[test]
fn event_decoder_validates_words() {
    let config = AbiDecoderConfig::new().validate_words(true);
    let event = Event {
        name: "E".into(),
        inputs: vec![
            EventParam { ty: "bool".into(), indexed: true, ..Default::default() },
            EventParam { ty: "bool".into(), indexed: false, ..Default::default() },
        ],
        anonymous: false,
    };
    let clean = B256::with_last_byte(1);
    let dirty = B256::with_last_byte(2);

    // Dirty body.
    let topics = [event.selector(), clean];
    let decoded = event.decode_log_parts(topics, &dirty.0).unwrap();
    assert_eq!(decoded.body, vec![DynSolValue::Bool(true)]);
    assert!(event.decode_log_parts_with_config(topics, &dirty.0, config).is_err());

    // Dirty indexed topic.
    let topics = [event.selector(), dirty];
    let decoded = event.decode_log_parts(topics, &clean.0).unwrap();
    assert_eq!(decoded.indexed, vec![DynSolValue::Bool(true)]);
    assert!(event.decode_log_parts_with_config(topics, &clean.0, config).is_err());

    let log = LogData::new_unchecked(topics.to_vec(), clean.0.into());
    assert!(event.decode_log(&log).is_ok());
    assert!(event.decode_log_with_config(&log, config).is_err());
}
