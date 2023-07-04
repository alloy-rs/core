use crate::{DynAbiError, DynSolType, DynSolValue, Word};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use alloy_primitives::{Address, I256, U256};

/// Coerce a `serde_json::Value` to a `DynSolValue::Address`
pub(crate) fn address(value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
    let address = value
        .as_str()
        .map(|s| {
            s.parse::<Address>()
                .map_err(|_| DynAbiError::type_mismatch(DynSolType::Address, value))
        })
        .ok_or_else(|| DynAbiError::type_mismatch(DynSolType::Address, value))??;

    Ok(DynSolValue::Address(address))
}

pub(crate) fn bool(value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
    if let Some(bool) = value.as_bool() {
        return Ok(DynSolValue::Bool(bool))
    }

    let bool = value
        .as_str()
        .map(|s| {
            s.parse::<bool>()
                .map_err(|_| DynAbiError::type_mismatch(DynSolType::Address, value))
        })
        .ok_or_else(|| DynAbiError::type_mismatch(DynSolType::Address, value))??;
    Ok(DynSolValue::Bool(bool))
}

pub(crate) fn bytes(value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
    let bytes = value
        .as_str()
        .map(|s| hex::decode(s).map_err(|_| DynAbiError::type_mismatch(DynSolType::Bytes, value)))
        .ok_or_else(|| DynAbiError::type_mismatch(DynSolType::Bytes, value))??;
    Ok(DynSolValue::Bytes(bytes))
}

pub(crate) fn fixed_bytes(n: usize, value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
    if let Some(Ok(buf)) = value.as_str().map(hex::decode) {
        let mut word: Word = Default::default();
        let min = n.min(buf.len());
        word[..min].copy_from_slice(&buf[..min]);
        return Ok(DynSolValue::FixedBytes(word, n))
    }

    Err(DynAbiError::type_mismatch(DynSolType::FixedBytes(n), value))
}

pub(crate) fn int(n: usize, value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
    if let Some(num) = value.as_i64() {
        return Ok(DynSolValue::Int(I256::try_from(num).unwrap(), n))
    }

    if let Some(Ok(i)) = value.as_str().map(|s| s.parse()) {
        return Ok(DynSolValue::Int(i, n))
    }

    Err(DynAbiError::type_mismatch(DynSolType::Int(n), value))
}

pub(crate) fn uint(n: usize, value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
    if let Some(num) = value.as_u64() {
        return Ok(DynSolValue::Uint(U256::from(num), n))
    }

    if let Some(s) = value.as_str() {
        let s = s.strip_prefix("0x").unwrap_or(s);
        if let Ok(int) = U256::from_str_radix(s, 10) {
            return Ok(DynSolValue::Uint(int, n))
        }
        if let Ok(int) = U256::from_str_radix(s, 16) {
            return Ok(DynSolValue::Uint(int, n))
        }
    }

    Err(DynAbiError::type_mismatch(DynSolType::Uint(n), value))
}

pub(crate) fn string(value: &serde_json::Value) -> Result<DynSolValue, DynAbiError> {
    let string = value
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| DynAbiError::type_mismatch(DynSolType::String, value))?;
    Ok(DynSolValue::String(string))
}

pub(crate) fn tuple(
    inner: &[DynSolType],
    value: &serde_json::Value,
) -> Result<DynSolValue, DynAbiError> {
    if let Some(arr) = value.as_array() {
        if inner.len() != arr.len() {
            return Err(DynAbiError::type_mismatch(
                DynSolType::Tuple(inner.to_vec()),
                value,
            ))
        }

        let tuple = arr
            .iter()
            .zip(inner.iter())
            .map(|(v, t)| t.coerce(v))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(DynSolValue::Tuple(tuple))
    }

    Err(DynAbiError::type_mismatch(
        DynSolType::Tuple(inner.to_vec()),
        value,
    ))
}

pub(crate) fn array(
    inner: &DynSolType,
    value: &serde_json::Value,
) -> Result<DynSolValue, DynAbiError> {
    if let Some(arr) = value.as_array() {
        let array = arr
            .iter()
            .map(|v| inner.coerce(v))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(DynSolValue::Array(array))
    }

    Err(DynAbiError::type_mismatch(
        DynSolType::Array(Box::new(inner.clone())),
        value,
    ))
}

pub(crate) fn fixed_array(
    inner: &DynSolType,
    n: usize,
    value: &serde_json::Value,
) -> Result<DynSolValue, DynAbiError> {
    if let Some(arr) = value.as_array() {
        if arr.len() != n {
            return Err(DynAbiError::type_mismatch(
                DynSolType::FixedArray(Box::new(inner.clone()), n),
                value,
            ))
        }

        let array = arr
            .iter()
            .map(|v| inner.coerce(v))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(DynSolValue::FixedArray(array))
    }

    Err(DynAbiError::type_mismatch(
        DynSolType::FixedArray(Box::new(inner.clone()), n),
        value,
    ))
}

pub(crate) fn coerce_custom_struct(
    name: &str,
    prop_names: &[String],
    inner: &[DynSolType],
    value: &serde_json::Value,
) -> Result<DynSolValue, DynAbiError> {
    if let Some(map) = value.as_object() {
        let mut tuple = vec![];
        for (name, ty) in prop_names.iter().zip(inner.iter()) {
            if let Some(v) = map.get(name) {
                tuple.push(ty.coerce(v)?);
            } else {
                return Err(DynAbiError::type_mismatch(
                    DynSolType::CustomStruct {
                        name: name.to_string(),
                        prop_names: prop_names.to_vec(),
                        tuple: inner.to_vec(),
                    },
                    value,
                ))
            }
        }
        return Ok(DynSolValue::CustomStruct {
            name: name.to_string(),
            prop_names: prop_names.to_vec(),
            tuple,
        })
    }

    Err(DynAbiError::type_mismatch(
        DynSolType::CustomStruct {
            name: name.to_string(),
            prop_names: prop_names.to_vec(),
            tuple: inner.to_vec(),
        },
        value,
    ))
}

pub(crate) fn coerce_custom_value(
    name: &str,
    value: &serde_json::Value,
) -> Result<DynSolValue, DynAbiError> {
    if let Some(Ok(buf)) = value.as_str().map(hex::decode) {
        let mut word: Word = Default::default();
        let amnt = if buf.len() > 32 { 32 } else { buf.len() };
        word[..amnt].copy_from_slice(&buf[..amnt]);

        return Ok(DynSolValue::CustomValue {
            name: name.to_string(),
            inner: word,
        })
    }

    Err(DynAbiError::type_mismatch(
        DynSolType::CustomValue {
            name: name.to_owned(),
        },
        value,
    ))
}
