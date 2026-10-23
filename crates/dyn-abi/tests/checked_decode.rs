//! Public decoder coverage for comparison-mutated arguments.

use alloy_dyn_abi::{DynSolType, DynSolValue};
use alloy_sol_types::abi::{AbiDecoderConfig, Decoder};

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
        ty.abi_decode_sequence_with_decoder(&mut Decoder::with_config(&data, config)).unwrap(),
        DynSolValue::Tuple(vec![
            DynSolValue::Array(vec![DynSolValue::Bool(false)]),
            DynSolValue::Bytes(vec![0xaa; 5]),
            DynSolValue::String(String::from_utf8_lossy(b"\xffello").into_owned()),
        ])
    );

    // A comparison mutation must not turn a nested bool into an invalid Solidity argument.
    data[4 * 32 + 31] = 4;
    assert!(ty.abi_decode_sequence(&data).is_ok());
    assert!(ty.abi_decode_sequence_with_decoder(&mut Decoder::with_config(&data, config)).is_err());
}
