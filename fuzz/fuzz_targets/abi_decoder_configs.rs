#![no_main]

mod abi_types;

use abi_types::*;
use alloy_sol_types::{Error, SolType, abi::AbiDecoderConfig, abi::TokenSeq};
use core::fmt::Debug;
use libfuzzer_sys::fuzz_target;

#[derive(Clone, Copy)]
enum Oracle {
    /// Strict decoding without trailing bytes must preserve the entire encoding.
    Exact,
    /// Strict decoding with trailing bytes must preserve the consumed prefix.
    Prefix,
    /// Non-strict decoding may canonicalize offsets, padding, values, or trailing data.
    Normalize,
}

#[derive(Clone, Copy)]
struct ConfigCase {
    name: &'static str,
    config: AbiDecoderConfig,
    oracle: Oracle,
}

fn configs() -> [ConfigCase; 4] {
    [
        ConfigCase {
            name: "default",
            config: AbiDecoderConfig::new(),
            oracle: Oracle::Normalize,
        },
        ConfigCase {
            name: "validate",
            config: AbiDecoderConfig::new().validate(true),
            oracle: Oracle::Normalize,
        },
        ConfigCase {
            name: "strict",
            config: AbiDecoderConfig::new().strict(true),
            oracle: Oracle::Exact,
        },
        ConfigCase {
            name: "strict_allow_trailing",
            config: AbiDecoderConfig::new().strict(true).validate_allow_trailing_bytes(true),
            oracle: Oracle::Prefix,
        },
    ]
}

fn assert_recursion_limit<T>(
    result: Result<T, Error>,
    expected: &T,
    limit: usize,
    entrypoint: &str,
    sol_name: &str,
) where
    T: Debug + PartialEq,
{
    match result {
        Ok(value) => assert_eq!(&value, expected, "recursion {limit}: {entrypoint}: {sol_name}"),
        Err(Error::RecursionLimitExceeded(actual)) => assert_eq!(actual, limit),
        Err(err) => panic!("recursion {limit}: {entrypoint}: {sol_name}: {err}"),
    }
}

fn assert_memory_limit<T>(
    result: Result<T, Error>,
    expected: &T,
    limit: usize,
    entrypoint: &str,
    sol_name: &str,
) where
    T: Debug + PartialEq,
{
    match result {
        Ok(value) => assert_eq!(&value, expected, "memory {limit}: {entrypoint}: {sol_name}"),
        Err(Error::MemoryLimitExceeded(actual)) => assert_eq!(actual, limit),
        Err(err) => panic!("memory {limit}: {entrypoint}: {sol_name}: {err}"),
    }
}

fn assert_encoding(bytes: &[u8], canonical: &[u8], case: ConfigCase, sol_name: &str) {
    match case.oracle {
        Oracle::Exact => assert_eq!(canonical, bytes, "{}: {sol_name}", case.name),
        Oracle::Prefix => assert!(bytes.starts_with(canonical), "{}: {sol_name}", case.name),
        Oracle::Normalize => {}
    }
}

fn check_value<T>(bytes: &[u8], case: ConfigCase)
where
    T: SolType,
    T::RustType: Debug + PartialEq,
{
    let Ok(value) = T::abi_decode_with_config(bytes, case.config) else {
        return;
    };
    let canonical = T::abi_encode(&value);
    assert_encoding(bytes, &canonical, case, T::SOL_NAME);
    let strict = T::abi_decode_with_config(&canonical, AbiDecoderConfig::new().strict(true))
        .expect("canonical value must strictly decode");
    assert_eq!(strict, value, "{}", T::SOL_NAME);
}

fn check_params<T>(bytes: &[u8], case: ConfigCase)
where
    T: SolType,
    T::RustType: Debug + PartialEq,
    for<'a> T::Token<'a>: TokenSeq<'a>,
{
    let Ok(value) = T::abi_decode_params_with_config(bytes, case.config) else {
        return;
    };
    let canonical = T::abi_encode_params(&value);
    assert_encoding(bytes, &canonical, case, T::SOL_NAME);
    let strict = T::abi_decode_params_with_config(&canonical, AbiDecoderConfig::new().strict(true))
        .expect("canonical params must strictly decode");
    assert_eq!(strict, value, "{}", T::SOL_NAME);
}

fn check_sequence<T>(bytes: &[u8], case: ConfigCase)
where
    T: SolType,
    T::RustType: Debug + PartialEq,
    for<'a> T::Token<'a>: TokenSeq<'a>,
{
    let Ok(value) = T::abi_decode_sequence_with_config(bytes, case.config) else {
        return;
    };
    let canonical = T::abi_encode_sequence(&value);
    assert_encoding(bytes, &canonical, case, T::SOL_NAME);
    let strict =
        T::abi_decode_sequence_with_config(&canonical, AbiDecoderConfig::new().strict(true))
            .expect("canonical sequence must strictly decode");
    assert_eq!(strict, value, "{}", T::SOL_NAME);
}

fn check<T>(bytes: &[u8], case: ConfigCase)
where
    T: SolType,
    T::RustType: Debug + PartialEq,
    for<'a> T::Token<'a>: TokenSeq<'a>,
{
    check_value::<T>(bytes, case);
    check_params::<T>(bytes, case);
    check_sequence::<T>(bytes, case);
}

fn check_limits<T>(bytes: &[u8])
where
    T: SolType,
    T::RustType: Debug + PartialEq,
    for<'a> T::Token<'a>: TokenSeq<'a>,
{
    // Use permissive decoding for the limit differential so more mutated layouts reach the
    // recursion and allocation accounting. Each comparison changes only the relevant limit.
    let baseline = AbiDecoderConfig::new();

    if let Ok(expected) = T::abi_decode_with_config(bytes, baseline) {
        for limit in [0, 1, 4] {
            assert_recursion_limit(
                T::abi_decode_with_config(bytes, baseline.recursion_limit(limit)),
                &expected,
                limit,
                "value",
                T::SOL_NAME,
            );
        }
        for limit in [0, 32, 4096] {
            assert_memory_limit(
                T::abi_decode_with_config(bytes, baseline.memory_limit(limit)),
                &expected,
                limit,
                "value",
                T::SOL_NAME,
            );
        }
    }

    if let Ok(expected) = T::abi_decode_params_with_config(bytes, baseline) {
        for limit in [0, 1, 4] {
            assert_recursion_limit(
                T::abi_decode_params_with_config(bytes, baseline.recursion_limit(limit)),
                &expected,
                limit,
                "params",
                T::SOL_NAME,
            );
        }
        for limit in [0, 32, 4096] {
            assert_memory_limit(
                T::abi_decode_params_with_config(bytes, baseline.memory_limit(limit)),
                &expected,
                limit,
                "params",
                T::SOL_NAME,
            );
        }
    }

    if let Ok(expected) = T::abi_decode_sequence_with_config(bytes, baseline) {
        for limit in [0, 1, 4] {
            assert_recursion_limit(
                T::abi_decode_sequence_with_config(bytes, baseline.recursion_limit(limit)),
                &expected,
                limit,
                "sequence",
                T::SOL_NAME,
            );
        }
        for limit in [0, 32, 4096] {
            assert_memory_limit(
                T::abi_decode_sequence_with_config(bytes, baseline.memory_limit(limit)),
                &expected,
                limit,
                "sequence",
                T::SOL_NAME,
            );
        }
    }
}

fuzz_target!(|input: &[u8]| {
    for case in configs() {
        check::<ZonesTempoShape>(input, case);
        check::<EncoderShape>(input, case);
        check::<PrimitiveShape>(input, case);
        check::<StaticArrayShape>(input, case);
        check::<MixedShape>(input, case);
        check::<NestedArrayShape>(input, case);
        check::<EmptyFixedThenBytes>(input, case);
    }
    check_limits::<ZonesTempoShape>(input);
    check_limits::<EncoderShape>(input);
    check_limits::<PrimitiveShape>(input);
    check_limits::<StaticArrayShape>(input);
    check_limits::<MixedShape>(input);
    check_limits::<NestedArrayShape>(input);
    check_limits::<EmptyFixedThenBytes>(input);
});
