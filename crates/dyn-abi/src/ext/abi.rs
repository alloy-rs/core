use crate::{DynSolValue, Error as CrateError, Result, Specifier};
use alloc::vec::Vec;
use alloy_json_abi::{Constructor, Error, Function, Param};
use alloy_primitives::utils::vec_try_with_capacity;
use alloy_sol_types::abi::{AbiDecoderConfig, Decoder};

#[allow(unknown_lints, unnameable_types)]
mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Constructor {}
    impl Sealed for super::Error {}
    impl Sealed for super::Function {}
}
use sealed::Sealed;

/// Provides ABI encoding and decoding functionality.
///
/// This trait is sealed and cannot be implemented for types outside of this
/// crate. It is implemented only for the following types:
/// - [`Constructor`]
/// - [`Error`]
/// - [`Function`]
pub trait JsonAbiExt: Sealed {
    /// ABI-encodes the given values, prefixed by this item's selector, if any.
    ///
    /// The selector is:
    /// - `None` for [`Constructor`],
    /// - `Some(self.selector())` for [`Error`] and [`Function`].
    ///
    /// This behaviour is to ensure consistency with `ethabi`.
    ///
    /// To encode the data without the selector, use
    /// [`abi_encode_input_raw`](JsonAbiExt::abi_encode_input_raw).
    ///
    /// # Errors
    ///
    /// This function will return an error if the given values do not match the
    /// expected input types.
    fn abi_encode_input(&self, values: &[DynSolValue]) -> Result<Vec<u8>>;

    /// ABI-encodes the given values, without prefixing the data with the item's
    /// selector.
    ///
    /// For [`Constructor`], this is the same as
    /// [`abi_encode_input`](JsonAbiExt::abi_encode_input).
    ///
    /// # Errors
    ///
    /// This function will return an error if the given values do not match the
    /// expected input types.
    fn abi_encode_input_raw(&self, values: &[DynSolValue]) -> Result<Vec<u8>>;

    /// ABI-decodes the given data according to this item's input types.
    ///
    /// # Errors
    ///
    /// This function will return an error if the decoded data does not match
    /// the expected input types.
    #[inline]
    fn abi_decode_input(&self, data: &[u8]) -> Result<Vec<DynSolValue>> {
        self.abi_decode_input_with_config(data, AbiDecoderConfig::new())
    }

    /// ABI-decodes the given data according to this item's input types, with the given decoder
    /// configuration.
    ///
    /// Strict decoding is not supported.
    fn abi_decode_input_with_config(
        &self,
        data: &[u8],
        config: AbiDecoderConfig,
    ) -> Result<Vec<DynSolValue>>;
}

/// Provide ABI encoding and decoding for the [`Function`] type.
///
/// This trait is sealed and cannot be implemented for types outside of this
/// crate. It is implemented only for [`Function`].
pub trait FunctionExt: JsonAbiExt + Sealed {
    /// ABI-encodes the given values.
    ///
    /// Note that, contrary to
    /// [`abi_encode_input`](JsonAbiExt::abi_encode_input), this method does
    /// not prefix the return data with the function selector.
    ///
    /// # Errors
    ///
    /// This function will return an error if the given values do not match the
    /// expected input types.
    fn abi_encode_output(&self, values: &[DynSolValue]) -> Result<Vec<u8>>;

    /// ABI-decodes the given data according to this functions's output types.
    ///
    /// This method does not check for any prefixes or selectors.
    fn abi_decode_output(&self, data: &[u8]) -> Result<Vec<DynSolValue>>;
}

impl JsonAbiExt for Constructor {
    #[inline]
    fn abi_encode_input(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck(&self.inputs, values)
    }

    #[inline]
    fn abi_encode_input_raw(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck(&self.inputs, values)
    }

    #[inline]
    fn abi_decode_input_with_config(
        &self,
        data: &[u8],
        config: AbiDecoderConfig,
    ) -> Result<Vec<DynSolValue>> {
        abi_decode(data, &self.inputs, config)
    }
}

impl JsonAbiExt for Error {
    #[inline]
    fn abi_encode_input(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck_with_prefix(&self.inputs, values, self.selector().as_slice())
    }

    #[inline]
    fn abi_encode_input_raw(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck(&self.inputs, values)
    }

    #[inline]
    fn abi_decode_input_with_config(
        &self,
        data: &[u8],
        config: AbiDecoderConfig,
    ) -> Result<Vec<DynSolValue>> {
        abi_decode(data, &self.inputs, config)
    }
}

impl JsonAbiExt for Function {
    #[inline]
    fn abi_encode_input(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck_with_prefix(&self.inputs, values, self.selector().as_slice())
    }

    #[inline]
    fn abi_encode_input_raw(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck(&self.inputs, values)
    }

    #[inline]
    fn abi_decode_input_with_config(
        &self,
        data: &[u8],
        config: AbiDecoderConfig,
    ) -> Result<Vec<DynSolValue>> {
        abi_decode(data, &self.inputs, config)
    }
}

impl FunctionExt for Function {
    #[inline]
    fn abi_encode_output(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck(&self.outputs, values)
    }

    #[inline]
    fn abi_decode_output(&self, data: &[u8]) -> Result<Vec<DynSolValue>> {
        abi_decode(data, &self.outputs, AbiDecoderConfig::new())
    }
}

fn encode_typeck(params: &[Param], values: &[DynSolValue]) -> Result<Vec<u8>> {
    encode_typeck_with_prefix(params, values, &[])
}

fn encode_typeck_with_prefix(
    params: &[Param],
    values: &[DynSolValue],
    prefix: &[u8],
) -> Result<Vec<u8>> {
    if values.len() != params.len() {
        return Err(CrateError::EncodeLengthMismatch {
            expected: params.len(),
            actual: values.len(),
        });
    }
    for (value, param) in core::iter::zip(values, params) {
        let ty = param.resolve()?;
        if !ty.matches(value) {
            return Err(CrateError::TypeMismatch {
                expected: ty.sol_type_name().into_owned(),
                actual: value.sol_type_name().unwrap_or_else(|| "<none>".into()).into_owned(),
            });
        }
    }

    Ok(DynSolValue::encode_seq_with_prefix(values, prefix))
}

fn abi_decode(data: &[u8], params: &[Param], config: AbiDecoderConfig) -> Result<Vec<DynSolValue>> {
    if config.get_strict() {
        return Err(CrateError::custom("strict decoding is not supported"));
    }
    let mut decoder = Decoder::with_config(data, config);
    decoder.reserve_elements::<DynSolValue>(params.len())?;
    let mut values = vec_try_with_capacity(params.len())?;
    for param in params {
        let ty = param.resolve()?;
        let value = ty.abi_decode_inner(
            &mut decoder,
            config.get_validate_words().then(|| config.get_validate()),
        )?;
        values.push(value);
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256, address, bytes, hex};

    #[test]
    fn selector_prefix_preserves_nested_offsets() {
        let function = Function::parse("submit((uint256,bytes)[],string,bytes[2])").unwrap();
        let values = vec![
            DynSolValue::Array(vec![
                DynSolValue::Tuple(vec![
                    DynSolValue::Uint(U256::from(1), 256),
                    DynSolValue::Bytes(vec![7; 33]),
                ]),
                DynSolValue::Tuple(vec![
                    DynSolValue::Uint(U256::from(2), 256),
                    DynSolValue::Bytes(vec![]),
                ]),
            ]),
            DynSolValue::String("hello".into()),
            DynSolValue::FixedArray(vec![
                DynSolValue::Bytes(vec![]),
                DynSolValue::Bytes(vec![9; 65]),
            ]),
        ];
        let mut expected = function.selector().to_vec();
        expected.extend(function.abi_encode_input_raw(&values).unwrap());
        assert_eq!(function.abi_encode_input(&values).unwrap(), expected);

        let call: crate::DynSolCall = function.resolve().unwrap();
        assert_eq!(call.abi_encode_input(&values).unwrap(), expected);
        let error = Error { name: function.name.clone(), inputs: function.inputs.clone() };
        assert_eq!(error.abi_encode_input(&values).unwrap(), expected);

        for bad_values in [&[][..], &[DynSolValue::Bool(false)][..]] {
            assert_eq!(
                function.abi_encode_input(bad_values).unwrap_err(),
                function.abi_encode_input_raw(bad_values).unwrap_err(),
            );
            assert_eq!(
                call.abi_encode_input(bad_values).unwrap_err(),
                call.abi_encode_input_raw(bad_values).unwrap_err(),
            );
        }
        let empty = Function::parse("noop()").unwrap();
        assert_eq!(empty.abi_encode_input(&[]).unwrap(), empty.selector().as_slice());
    }

    #[test]
    fn can_encode_decode_functions() {
        let json = r#"{
            "inputs": [
                {
                    "internalType": "address",
                    "name": "",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "",
                    "type": "address"
                }
            ],
            "name": "allowance",
            "outputs": [
                {
                    "internalType": "uint256",
                    "name": "",
                    "type": "uint256"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        }"#;

        let func: Function = serde_json::from_str(json).unwrap();
        assert_eq!(2, func.inputs.len());
        assert_eq!(1, func.outputs.len());
        assert_eq!(func.signature(), "allowance(address,address)");

        // encode
        let expected = alloy_primitives::hex!(
            "dd62ed3e"
            "0000000000000000000000001111111111111111111111111111111111111111"
            "0000000000000000000000002222222222222222222222222222222222222222"
        );
        let input = [
            DynSolValue::Address(Address::repeat_byte(0x11)),
            DynSolValue::Address(Address::repeat_byte(0x22)),
        ];
        let result = func.abi_encode_input(&input).unwrap();
        assert_eq!(expected[..], result);

        // Fail on unexpected input
        let wrong_input = [
            DynSolValue::Uint(U256::from(10u8), 256),
            DynSolValue::Address(Address::repeat_byte(2u8)),
        ];
        assert!(func.abi_encode_input(&wrong_input).is_err());

        // decode
        let response = U256::from(1u8).to_be_bytes_vec();
        let decoded = func.abi_decode_output(&response).unwrap();
        assert_eq!(decoded, [DynSolValue::Uint(U256::from(1u8), 256)]);

        // Fail on wrong response type
        let bad_response = Address::repeat_byte(3u8).to_vec();
        assert!(func.abi_decode_output(&bad_response).is_err());
    }

    // https://github.com/foundry-rs/foundry/issues/7280
    // Same as `encode_empty_bytes_array_in_tuple` in sol-types.
    #[test]
    fn empty_bytes_array() {
        let func = Function::parse("register(bytes,address,bytes[])").unwrap();
        let input = [
            DynSolValue::Bytes(bytes!("09736b79736b79736b79026f7300").into()),
            DynSolValue::Address(address!("0xB7b54cd129e6D8B24e6AE652a473449B273eE3E4")),
            DynSolValue::Array(vec![]),
        ];
        let result = func.abi_encode_input(&input).unwrap();

        let expected = hex!(
            "
            d123f99a
            0000000000000000000000000000000000000000000000000000000000000060
            000000000000000000000000B7b54cd129e6D8B24e6AE652a473449B273eE3E4
            00000000000000000000000000000000000000000000000000000000000000a0
            000000000000000000000000000000000000000000000000000000000000000e
            09736b79736b79736b79026f7300000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
    	"
        );
        assert_eq!(hex::encode(expected), hex::encode(result));
    }

    #[test]
    fn decode_input_charges_parameter_list() {
        let func = Function::parse("f(uint256,uint256)").unwrap();
        let data = [0u8; 64];
        let size = 2 * core::mem::size_of::<DynSolValue>();

        let config = AbiDecoderConfig::new().memory_limit(size);
        assert!(func.abi_decode_input_with_config(&data, config).is_ok());

        let config = AbiDecoderConfig::new().memory_limit(size - 1);
        assert_eq!(
            func.abi_decode_input_with_config(&data, config),
            Err(alloy_sol_types::Error::MemoryLimitExceeded(size - 1).into()),
        );
    }
}
