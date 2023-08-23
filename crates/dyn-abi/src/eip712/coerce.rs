use crate::{DynSolType, DynSolValue, Error, Result, Word};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use alloy_primitives::{Address, Function, I256, U256};

impl DynSolType {
    /// Coerce a [`serde_json::Value`] to a [`DynSolValue`] via this type.
    pub fn coerce(&self, value: &serde_json::Value) -> Result<DynSolValue> {
        match self {
            Self::Address => address(value),
            Self::Function => function(value),
            Self::Bool => bool(value),
            Self::Int(n) => int(*n, value),
            Self::Uint(n) => uint(*n, value),
            Self::FixedBytes(n) => fixed_bytes(*n, value),
            Self::String => string(value),
            Self::Bytes => bytes(value),
            Self::Array(inner) => array(inner, value),
            Self::FixedArray(inner, n) => fixed_array(inner, *n, value),
            Self::Tuple(inner) => tuple(inner, value),
            Self::CustomStruct {
                name,
                prop_names,
                tuple,
            } => coerce_custom_struct(name, prop_names, tuple, value),
        }
    }
}

fn address(value: &serde_json::Value) -> Result<DynSolValue> {
    let address = value
        .as_str()
        .map(|s| {
            s.parse::<Address>()
                .map_err(|_| Error::type_mismatch(&DynSolType::Address, value))
        })
        .ok_or_else(|| Error::type_mismatch(&DynSolType::Address, value))??;

    Ok(DynSolValue::Address(address))
}

fn function(value: &serde_json::Value) -> Result<DynSolValue> {
    let function = value
        .as_str()
        .map(|s| {
            s.parse::<Function>()
                .map_err(|_| Error::type_mismatch(&DynSolType::Function, value))
        })
        .ok_or_else(|| Error::type_mismatch(&DynSolType::Function, value))??;

    Ok(DynSolValue::Function(function))
}

fn bool(value: &serde_json::Value) -> Result<DynSolValue> {
    if let Some(bool) = value.as_bool() {
        return Ok(DynSolValue::Bool(bool))
    }

    let bool = value
        .as_str()
        .map(|s| {
            s.parse::<bool>()
                .map_err(|_| Error::type_mismatch(&DynSolType::Address, value))
        })
        .ok_or_else(|| Error::type_mismatch(&DynSolType::Address, value))??;
    Ok(DynSolValue::Bool(bool))
}

fn int(n: usize, value: &serde_json::Value) -> Result<DynSolValue> {
    if let Some(num) = value.as_i64() {
        return Ok(DynSolValue::Int(I256::try_from(num).unwrap(), n))
    }

    if let Some(Ok(i)) = value.as_str().map(|s| s.parse()) {
        return Ok(DynSolValue::Int(i, n))
    }

    Err(Error::type_mismatch(&DynSolType::Int(n), value))
}

fn uint(n: usize, value: &serde_json::Value) -> Result<DynSolValue> {
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

    Err(Error::type_mismatch(&DynSolType::Uint(n), value))
}

fn fixed_bytes(n: usize, value: &serde_json::Value) -> Result<DynSolValue> {
    if let Some(Ok(buf)) = value.as_str().map(hex::decode) {
        let mut word: Word = Default::default();
        let min = n.min(buf.len());
        word[..min].copy_from_slice(&buf[..min]);
        return Ok(DynSolValue::FixedBytes(word, n))
    }

    Err(Error::type_mismatch(&DynSolType::FixedBytes(n), value))
}

fn string(value: &serde_json::Value) -> Result<DynSolValue> {
    let string = value
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::type_mismatch(&DynSolType::String, value))?;
    Ok(DynSolValue::String(string))
}

fn bytes(value: &serde_json::Value) -> Result<DynSolValue> {
    let bytes = value
        .as_str()
        .map(|s| hex::decode(s).map_err(|_| Error::type_mismatch(&DynSolType::Bytes, value)))
        .ok_or_else(|| Error::type_mismatch(&DynSolType::Bytes, value))??;
    Ok(DynSolValue::Bytes(bytes))
}

fn tuple(inner: &[DynSolType], value: &serde_json::Value) -> Result<DynSolValue> {
    if let Some(arr) = value.as_array() {
        if inner.len() != arr.len() {
            return Err(Error::type_mismatch(
                &DynSolType::Tuple(inner.to_vec()),
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

    Err(Error::type_mismatch(
        &DynSolType::Tuple(inner.to_vec()),
        value,
    ))
}

fn array(inner: &DynSolType, value: &serde_json::Value) -> Result<DynSolValue> {
    if let Some(arr) = value.as_array() {
        let array = arr
            .iter()
            .map(|v| inner.coerce(v))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(DynSolValue::Array(array))
    }

    Err(Error::type_mismatch(
        &DynSolType::Array(Box::new(inner.clone())),
        value,
    ))
}

fn fixed_array(inner: &DynSolType, n: usize, value: &serde_json::Value) -> Result<DynSolValue> {
    if let Some(arr) = value.as_array() {
        if arr.len() != n {
            return Err(Error::type_mismatch(
                &DynSolType::FixedArray(Box::new(inner.clone()), n),
                value,
            ))
        }

        let array = arr
            .iter()
            .map(|v| inner.coerce(v))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(DynSolValue::FixedArray(array))
    }

    Err(Error::type_mismatch(
        &DynSolType::FixedArray(Box::new(inner.clone()), n),
        value,
    ))
}

pub(crate) fn coerce_custom_struct(
    name: &str,
    prop_names: &[String],
    inner: &[DynSolType],
    value: &serde_json::Value,
) -> Result<DynSolValue> {
    if let Some(map) = value.as_object() {
        let mut tuple = vec![];
        for (name, ty) in prop_names.iter().zip(inner.iter()) {
            if let Some(v) = map.get(name) {
                tuple.push(ty.coerce(v)?);
            } else {
                return Err(Error::type_mismatch(
                    &DynSolType::CustomStruct {
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

    Err(Error::type_mismatch(
        &DynSolType::CustomStruct {
            name: name.to_string(),
            prop_names: prop_names.to_vec(),
            tuple: inner.to_vec(),
        },
        value,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{borrow::ToOwned, string::ToString};
    use serde_json::json;

    #[test]
    fn it_coerces() {
        let j = json!({
            "message": {
                "contents": "Hello, Bob!",
                "attachedMoneyInEth": 4.2,
                "from": {
                    "name": "Cow",
                    "wallets": [
                        "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
                        "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF",
                    ]
                },
                "to": [{
                    "name": "Bob",
                    "wallets": [
                        "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
                        "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57",
                        "0xB0B0b0b0b0b0B000000000000000000000000000",
                    ]
                }]
            }
        });

        let ty = DynSolType::CustomStruct {
            name: "Message".to_owned(),
            prop_names: vec!["contents".to_string(), "from".to_string(), "to".to_string()],
            tuple: vec![
                DynSolType::String,
                DynSolType::CustomStruct {
                    name: "Person".to_owned(),
                    prop_names: vec!["name".to_string(), "wallets".to_string()],
                    tuple: vec![
                        DynSolType::String,
                        DynSolType::Array(Box::new(DynSolType::Address)),
                    ],
                },
                DynSolType::Array(Box::new(DynSolType::CustomStruct {
                    name: "Person".to_owned(),
                    prop_names: vec!["name".to_string(), "wallets".to_string()],
                    tuple: vec![
                        DynSolType::String,
                        DynSolType::Array(Box::new(DynSolType::Address)),
                    ],
                })),
            ],
        };
        let top = j.as_object().unwrap().get("message").unwrap();

        assert_eq!(
            ty.coerce(top).unwrap(),
            DynSolValue::CustomStruct {
                name: "Message".to_owned(),
                prop_names: vec!["contents".to_string(), "from".to_string(), "to".to_string()],
                tuple: vec![
                    DynSolValue::String("Hello, Bob!".to_string()),
                    DynSolValue::CustomStruct {
                        name: "Person".to_owned(),
                        prop_names: vec!["name".to_string(), "wallets".to_string()],
                        tuple: vec![
                            DynSolValue::String("Cow".to_string()),
                            vec![
                                DynSolValue::Address(
                                    "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF"
                                        .parse()
                                        .unwrap()
                                ),
                            ]
                            .into()
                        ]
                    },
                    vec![DynSolValue::CustomStruct {
                        name: "Person".to_owned(),
                        prop_names: vec!["name".to_string(), "wallets".to_string()],
                        tuple: vec![
                            DynSolValue::String("Bob".to_string()),
                            vec![
                                DynSolValue::Address(
                                    "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57"
                                        .parse()
                                        .unwrap()
                                ),
                                DynSolValue::Address(
                                    "0xB0B0b0b0b0b0B000000000000000000000000000"
                                        .parse()
                                        .unwrap()
                                ),
                            ]
                            .into()
                        ]
                    }]
                    .into()
                ]
            }
        );
    }
}
