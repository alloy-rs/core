use crate::{DynSolType, DynSolValue, Error, Result};
use alloy_primitives::Selector;
use alloy_sol_types::abi::Decoder;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// A representation of a Solidity call
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynSolCall {
    /// The selector of the call.
    selector: Selector,
    /// The types of the call.
    parameters: Vec<DynSolType>,
    /// The method name of the call, if available.
    method: Option<String>,
    /// The types of the call's returns.
    returns: DynSolReturns,
}

impl DynSolCall {
    /// Create a new `DynSolCall` with the given selector and types.
    pub const fn new(
        selector: Selector,
        parameters: Vec<DynSolType>,
        method: Option<String>,
        returns: DynSolReturns,
    ) -> Self {
        Self { selector, parameters, method, returns }
    }

    /// Get the selector of the call.
    pub const fn selector(&self) -> Selector {
        self.selector
    }

    /// Get the types of the call.
    pub fn types(&self) -> &[DynSolType] {
        &self.parameters
    }

    /// Get the method name of the call (if available)
    pub fn method(&self) -> Option<&str> {
        self.method.as_deref()
    }

    /// Get the types of the call's returns.
    pub const fn returns(&self) -> &DynSolReturns {
        &self.returns
    }

    /// ABI encode the given values as function params.
    pub fn abi_encode_input(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck_with_prefix(&self.parameters, values, self.selector.as_slice())
    }

    /// ABI encode the given values as function params without prefixing the
    /// selector.
    pub fn abi_encode_input_raw(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck(&self.parameters, values)
    }

    /// ABI decode the given data as function returns.
    pub fn abi_decode_input(&self, data: &[u8]) -> Result<Vec<DynSolValue>> {
        abi_decode(data, &self.parameters)
    }

    /// ABI encode the given values as function return values.
    pub fn abi_encode_output(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        self.returns.abi_encode_output(values)
    }

    /// ABI decode the given data as function return values.
    pub fn abi_decode_output(&self, data: &[u8]) -> Result<Vec<DynSolValue>> {
        self.returns.abi_decode_output(data)
    }
}

/// A representation of a Solidity call's returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynSolReturns(Vec<DynSolType>);

impl From<Vec<DynSolType>> for DynSolReturns {
    fn from(types: Vec<DynSolType>) -> Self {
        Self(types)
    }
}

impl From<DynSolReturns> for Vec<DynSolType> {
    fn from(returns: DynSolReturns) -> Self {
        returns.0
    }
}

impl DynSolReturns {
    /// Create a new `DynSolReturns` with the given types.
    pub const fn new(types: Vec<DynSolType>) -> Self {
        Self(types)
    }

    /// Get the types of the returns.
    pub fn types(&self) -> &[DynSolType] {
        &self.0
    }

    /// ABI encode the given values as function return values.
    pub fn abi_encode_output(&self, values: &[DynSolValue]) -> Result<Vec<u8>> {
        encode_typeck(self.types(), values)
    }

    /// ABI decode the given data as function return values.
    pub fn abi_decode_output(&self, data: &[u8]) -> Result<Vec<DynSolValue>> {
        abi_decode(data, self.types())
    }
}

pub(crate) fn encode_typeck(tys: &[DynSolType], values: &[DynSolValue]) -> Result<Vec<u8>> {
    encode_typeck_with_prefix(tys, values, &[])
}

fn encode_typeck_with_prefix(
    tys: &[DynSolType],
    values: &[DynSolValue],
    prefix: &[u8],
) -> Result<Vec<u8>> {
    if values.len() != tys.len() {
        return Err(Error::EncodeLengthMismatch { expected: tys.len(), actual: values.len() });
    }

    for (value, ty) in core::iter::zip(values, tys) {
        if !ty.matches(value) {
            return Err(Error::TypeMismatch {
                expected: ty.sol_type_name().into_owned(),
                actual: value.sol_type_name().unwrap_or_else(|| "<none>".into()).into_owned(),
            });
        }
    }

    Ok(DynSolValue::encode_seq_with_prefix(values, prefix))
}

pub(crate) fn abi_decode(data: &[u8], tys: &[DynSolType]) -> Result<Vec<DynSolValue>> {
    let mut values = Vec::with_capacity(tys.len());
    let mut decoder = Decoder::new(data);
    for ty in tys {
        let value = ty.abi_decode_inner(&mut decoder, None)?;
        values.push(value);
    }
    Ok(values)
}
