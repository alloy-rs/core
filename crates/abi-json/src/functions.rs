use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};

use crate::param::Param;

/// JSON specification for function state mutability
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StateMutability {
    /// Pure functions promise not to read from or modify the state.
    #[serde(rename = "pure")]
    Pure,
    /// View functions promise not to modify the state.
    #[serde(rename = "view")]
    View,
    /// Nonpayable functions promise not to receive Ether.
    #[serde(rename = "nonpayable")]
    NonPayable,
    /// Payable functions make no promises
    #[serde(rename = "payable")]
    Payable,
}

/// JSON specification for a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// The name of the function.
    pub name: String,
    /// The input types of the function. May be empty.
    pub inputs: Vec<Param>,
    /// The output types of the function. May be empty.
    pub outputs: Vec<Param>,
    /// The state mutability of the function.
    pub state_mutability: StateMutability,
}

impl Function {
    /// Generate the selector preimage for this function.
    pub fn selector_preimage(&self) -> String {
        format!(
            "{}({})",
            self.name,
            self.inputs
                .iter()
                .map(Param::selector_type)
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    /// Generate the selector for this function.
    pub fn selector(&self) -> alloy_primitives::Selector {
        let mut selector = [0u8; 4];
        let digest = alloy_primitives::keccak256(self.selector_preimage().as_bytes());
        selector.copy_from_slice(&digest[..4]);
        selector
    }
}

struct FunctionVisitor;

impl<'de> Visitor<'de> for FunctionVisitor {
    type Value = Function;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an object with the `type` key set to `function`, a `stateMutability` field, an optional list of inputs and an optional list of outputs"
        )
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name = None;
        let mut state_mutability = None;
        let mut ty = None;
        let mut inputs: Option<Vec<_>> = None;
        let mut outputs: Option<Vec<_>> = None;

        while let Some(ref key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    if ty.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"))
                    }
                    ty = Some(());
                    let ty_str = map.next_value::<String>()?;
                    if &ty_str != "function" {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&ty_str),
                            &"function",
                        ))
                    }
                }
                "name" => {
                    if name.is_some() {
                        return Err(serde::de::Error::duplicate_field("name"))
                    }
                    name = Some(map.next_value::<String>()?);
                }
                "stateMutability" => {
                    if state_mutability.is_some() {
                        return Err(serde::de::Error::duplicate_field("stateMutability"))
                    }
                    state_mutability = Some(map.next_value::<StateMutability>()?);
                }
                "inputs" => {
                    if inputs.is_some() {
                        return Err(serde::de::Error::duplicate_field("inputs"))
                    }
                    inputs = Some(map.next_value::<Vec<Param>>()?);
                }
                "outputs" => {
                    if outputs.is_some() {
                        return Err(serde::de::Error::duplicate_field("outputs"))
                    }
                    outputs = Some(map.next_value::<Vec<Param>>()?);
                }
                _ => {}
            }
        }
        if ty.is_none() {
            return Err(serde::de::Error::missing_field("type"))
        }
        Ok(Function {
            name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
            inputs: inputs.unwrap_or_default(),
            outputs: outputs.unwrap_or_default(),
            state_mutability: state_mutability
                .ok_or_else(|| serde::de::Error::missing_field("stateMutability"))?,
        })
    }
}

impl<'de> Deserialize<'de> for Function {
    fn deserialize<D>(deserializer: D) -> Result<Function, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Function",
            &["type", "name", "inputs", "outputs", "stateMutability"],
            FunctionVisitor,
        )
    }
}

impl Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Function", 5)?;
        s.serialize_field("type", "function")?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("inputs", &self.inputs)?;
        s.serialize_field("outputs", &self.outputs)?;
        s.serialize_field("stateMutability", &self.state_mutability)?;
        s.end()
    }
}

/// JSON specification for a constructor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    /// The input types of the constructor. May be empty.
    pub inputs: Vec<Param>,
    /// The state mutability of the constructor.
    pub state_mutability: StateMutability,
}

struct ConstructorVisitor;

impl<'de> Visitor<'de> for ConstructorVisitor {
    type Value = Constructor;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an object with a `stateMutability` field, and the `type` key set to `constructor`, and an optional list of inputs"
        )
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let mut state_mutability = None;
        let mut ty = None;
        let mut inputs: Option<Vec<_>> = None;

        while let Some(ref key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    if ty.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"))
                    }
                    ty = Some(());
                    let ty_str = map.next_value::<String>()?;
                    if &ty_str != "constructor" {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&ty_str),
                            &"constructor",
                        ))
                    }
                }
                "stateMutability" => {
                    if state_mutability.is_some() {
                        return Err(serde::de::Error::duplicate_field("stateMutability"))
                    }
                    state_mutability = Some(map.next_value()?);
                }
                "inputs" => {
                    if inputs.is_some() {
                        return Err(serde::de::Error::duplicate_field("inputs"))
                    }
                    inputs = map.next_value()?;
                }
                _ => {}
            }
        }
        if ty.is_none() {
            return Err(serde::de::Error::missing_field("type"))
        }
        Ok(Constructor {
            inputs: inputs.unwrap_or_default(),
            state_mutability: state_mutability
                .ok_or_else(|| serde::de::Error::missing_field("stateMutability"))?,
        })
    }
}

impl<'de> Deserialize<'de> for Constructor {
    fn deserialize<D>(deserializer: D) -> Result<Constructor, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Constructor",
            &["type", "inputs", "stateMutability"],
            ConstructorVisitor,
        )
    }
}

impl Serialize for Constructor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Constructor", 3)?;
        s.serialize_field("type", "constructor")?;
        s.serialize_field("inputs", &self.inputs)?;
        s.serialize_field("stateMutability", &self.state_mutability)?;
        s.end()
    }
}

/// JSON specification for a fallback function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fallback {
    /// The state mutability of the fallback function.
    pub state_mutability: StateMutability,
}

struct FallbackVisitor;

impl<'de> Visitor<'de> for FallbackVisitor {
    type Value = Fallback;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an object with a `stateMutability` field, and the `type` key set to `fallback`"
        )
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let mut state_mutability = None;
        let mut ty = None;

        while let Some(ref key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    if ty.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"))
                    }
                    ty = Some(());
                    let ty_str = map.next_value::<String>()?;
                    if &ty_str != "fallback" {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&ty_str),
                            &"fallback",
                        ))
                    }
                }
                "stateMutability" => {
                    if state_mutability.is_some() {
                        return Err(serde::de::Error::duplicate_field("stateMutability"))
                    }
                    state_mutability = Some(map.next_value()?);
                }
                _ => {}
            }
        }
        if ty.is_none() {
            return Err(serde::de::Error::missing_field("type"))
        }
        Ok(Fallback {
            state_mutability: state_mutability
                .ok_or_else(|| serde::de::Error::missing_field("stateMutability"))?,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Fallback {
    fn deserialize<D>(deserializer: D) -> Result<Fallback, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Fallback", &["type", "stateMutability"], FallbackVisitor)
    }
}

impl Serialize for Fallback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Fallback", 2)?;
        s.serialize_field("type", "fallback")?;
        s.serialize_field("stateMutability", &self.state_mutability)?;
        s.end()
    }
}

/// JSON specification for a receive function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Receive {
    /// The state mutability of the receive function.
    pub state_mutability: StateMutability,
}

struct ReceiveVisitor;

impl<'de> Visitor<'de> for ReceiveVisitor {
    type Value = Receive;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an object with a `stateMutability` field, and the `type` key set to `fallback`"
        )
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let mut state_mutability = None;
        let mut ty = None;

        while let Some(ref key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    if ty.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"))
                    }
                    ty = Some(());
                    let ty_str = map.next_value::<String>()?;
                    if &ty_str != "receive" {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&ty_str),
                            &"fallback",
                        ))
                    }
                }
                "stateMutability" => {
                    if state_mutability.is_some() {
                        return Err(serde::de::Error::duplicate_field("stateMutability"))
                    }
                    state_mutability = Some(map.next_value()?);
                }
                _ => {}
            }
        }
        if ty.is_none() {
            return Err(serde::de::Error::missing_field("type"))
        }
        Ok(Receive {
            state_mutability: state_mutability
                .ok_or_else(|| serde::de::Error::missing_field("stateMutability"))?,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Receive {
    fn deserialize<D>(deserializer: D) -> Result<Receive, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Receive", &["type", "stateMutability"], ReceiveVisitor)
    }
}

impl Serialize for Receive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Receive", 2)?;
        s.serialize_field("type", "receive")?;
        s.serialize_field("stateMutability", &self.state_mutability)?;
        s.end()
    }
}
