use crate::{AbiItem, Constructor, Error, Event, Fallback, Function, Receive};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

/// The JSON contract ABI, as specified in the [Solidity ABI spec][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#json
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AbiJson {
    /// The constructor function.
    pub constructor: Option<Constructor>,
    /// The fallback function.
    pub fallback: Option<Fallback>,
    /// The receive function.
    pub receive: Option<Receive>,
    /// The functions, indexed by the function name
    pub functions: BTreeMap<String, Vec<Function>>,
    /// The events, indexed by the event name
    pub events: BTreeMap<String, Vec<Event>>,
    /// The errors, indexed by the error name
    pub errors: BTreeMap<String, Vec<Error>>,
}

impl AbiJson {
    /// The total number of items (of any type)
    pub fn len(&self) -> usize {
        self.constructor.is_some() as usize
            + self.fallback.is_some() as usize
            + self.receive.is_some() as usize
            + self.functions.values().map(Vec::len).sum::<usize>()
            + self.events.values().map(Vec::len).sum::<usize>()
            + self.errors.values().map(Vec::len).sum::<usize>()
    }

    /// True if the ABI contains no items
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'de> Deserialize<'de> for AbiJson {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<AbiJson, D::Error> {
        deserializer.deserialize_seq(AbiJsonVisitor)
    }
}

impl Serialize for AbiJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        if let Some(constructor) = &self.constructor {
            seq.serialize_element(constructor)?;
        }
        if let Some(fallback) = &self.fallback {
            seq.serialize_element(fallback)?;
        }
        if let Some(receive) = &self.receive {
            seq.serialize_element(receive)?;
        }

        self.functions
            .values()
            .flatten()
            .try_for_each(|f| seq.serialize_element(f))?;

        self.events
            .values()
            .flatten()
            .try_for_each(|e| seq.serialize_element(e))?;

        self.errors
            .values()
            .flatten()
            .try_for_each(|e| seq.serialize_element(e))?;

        seq.end()
    }
}

struct AbiJsonVisitor;

impl<'de> Visitor<'de> for AbiJsonVisitor {
    type Value = AbiJson;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "a valid ABI JSON file")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut json_file = AbiJson::default();

        while let Some(item) = seq.next_element()? {
            match item {
                AbiItem::Constructor(c) => {
                    if json_file.constructor.is_some() {
                        return Err(serde::de::Error::duplicate_field("constructor"))
                    }
                    json_file.constructor = Some(c.into_owned());
                }
                AbiItem::Fallback(f) => {
                    if json_file.fallback.is_some() {
                        return Err(serde::de::Error::duplicate_field("fallback"))
                    }
                    json_file.fallback = Some(f.into_owned());
                }
                AbiItem::Receive(r) => {
                    if json_file.receive.is_some() {
                        return Err(serde::de::Error::duplicate_field("receive"))
                    }
                    json_file.receive = Some(r.into_owned());
                }
                AbiItem::Function(f) => {
                    json_file
                        .functions
                        .entry(f.name.clone())
                        .or_default()
                        .push(f.into_owned());
                }
                AbiItem::Event(e) => {
                    json_file
                        .events
                        .entry(e.name.clone())
                        .or_default()
                        .push(e.into_owned());
                }
                AbiItem::Error(e) => {
                    json_file
                        .errors
                        .entry(e.name.clone())
                        .or_default()
                        .push(e.into_owned());
                }
            }
        }
        Ok(json_file)
    }
}
