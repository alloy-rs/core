use serde::{de::Visitor, Deserialize, Serialize};

use crate::param::Param;

/// A Solidity Event parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleEventParam {
    /// The name of the parameter.
    pub name: String,
    /// The Solidity type of the parameter.
    #[serde(rename = "type")]
    pub ty: String,
    /// Whether the parameter is indexed. Indexed parameters have their
    ///value, or the hash of their value, stored in the log topics.
    pub indexed: bool,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    #[serde(rename = "internalType")]
    pub internal_type: String,
}

/// JSON representation of a complex event parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplexEventParam {
    /// The name of the parameter.
    pub name: String,
    /// The Solidity type of the parameter.
    #[serde(rename = "type")]
    pub ty: String,
    /// Whether the parameter is indexed. Indexed parameters have their
    ///value, or the hash of their value, stored in the log topics.
    pub indexed: bool,
    /// A list of the parameter's components, in order. This is a tuple
    /// definition, and sub-components will NOT have an `indexed` field.
    pub components: Vec<Param>,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    #[serde(rename = "internalType")]
    pub internal_type: String,
}

/// A Solidity Event parameter. Event parameters are distinct from function
/// parameters in that they have an `indexed` field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventParam {
    /// [`ComplexEventParam`] variant
    Complex(ComplexEventParam),
    /// [`SimpleEventParam`] variant
    Simple(SimpleEventParam),
}

/// JSON specification of a Solidity event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// The name of the event.
    pub name: String,
    /// A list of the event's inputs, in order.
    pub inputs: Vec<EventParam>,
    /// Whether the event is anonymous. Anonymous events do not have their
    pub anonymous: bool,
}

struct EventVisitor;

impl<'de> Visitor<'de> for EventVisitor {
    type Value = Event;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an object with the type key set to `event`, a name key, and an inputs key"
        )
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name = None;
        let mut inputs = None;
        let mut anonymous = false;
        let mut ty = None;

        while let Some(ref key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    if ty.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"))
                    }
                    ty = Some(());
                    let ty_str = map.next_value::<String>()?;
                    if &ty_str != "event" {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&ty_str),
                            &"event",
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
                "anonymous" => {
                    if anonymous {
                        return Err(serde::de::Error::duplicate_field("anonymous"))
                    }
                    anonymous = map.next_value()?;
                }

                _ => {}
            }
        }
        if ty.is_none() {
            return Err(serde::de::Error::missing_field("type"))
        }
        Ok(Event {
            name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
            inputs: inputs.ok_or_else(|| serde::de::Error::missing_field("inputs"))?,
            anonymous,
        })
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Event",
            &["type", "name", "inputs", "anonymous"],
            EventVisitor,
        )
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Event", 4)?;
        state.serialize_field("type", "event")?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("inputs", &self.inputs)?;
        state.serialize_field("anonymous", &self.anonymous)?;
        state.end()
    }
}
