use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};

use crate::param::Param;

/// JSON specification of an error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// The name of the error.
    pub(crate) name: String,
    /// A list of the error's components, in order.
    pub(crate) inputs: Vec<Param>,
}

impl Error {
    /// Generate the selector preimage for this error.
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

    /// Generate the selector for this error.
    pub fn selector(&self) -> alloy_primitives::Selector {
        let mut selector = [0u8; 4];
        let digest = alloy_primitives::keccak256(self.selector_preimage().as_bytes());
        selector.copy_from_slice(&digest[..4]);
        selector
    }
}

struct ErrorVisitor;

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = Error;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an object with the `type` key set to `error`, a `name` key, and a `inputs` key"
        )
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name = None;
        let mut inputs: Option<Vec<_>> = None;
        let mut ty = None;

        while let Some(ref key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    if ty.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"))
                    }
                    ty = Some(());
                    let ty_str = map.next_value::<String>()?;
                    if &ty_str != "error" {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&ty_str),
                            &"error",
                        ))
                    }
                }
                "name" => {
                    if name.is_some() {
                        return Err(serde::de::Error::duplicate_field("name"))
                    }
                    name = Some(map.next_value()?);
                }
                "inputs" => {
                    if inputs.is_some() {
                        return Err(serde::de::Error::duplicate_field("inputs"))
                    }
                    inputs = Some(map.next_value()?);
                }
                _ => {}
            }
        }
        if ty.is_none() {
            return Err(serde::de::Error::missing_field("type"))
        }
        Ok(Error {
            name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
            inputs: inputs.ok_or_else(|| serde::de::Error::missing_field("inputs"))?,
        })
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> Result<Error, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Error", &["type", "name", "inputs"], ErrorVisitor)
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_struct("Error", 3)?;
        map.serialize_field("type", "error")?;
        map.serialize_field("name", &self.name)?;
        map.serialize_field("inputs", &self.inputs)?;
        map.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn complex_error() {
        let err = r#"{
            "inputs": [
                {
                "internalType": "string",
                "name": "reason",
                "type": "string"
                }
            ],
            "name": "SomeName",
            "type": "error"
        }"#;
        let _error: Error = serde_json::from_str(err).unwrap();
    }
}
