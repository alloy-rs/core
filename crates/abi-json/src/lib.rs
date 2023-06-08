// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ABI JSON file format for Solidity contracts.
//!
//! Please consult the [specification] for full details.
//!
//! This crate is a reimplementation of [ethabi]. There's only one right way to
//! implement a JSON serialization scheme in rust. So while the internals are
//! nearly-identical, the API is our own.
//!
//! [specification]: https://docs.soliditylang.org/en/v0.8.20/abi-spec.html#json
//! [ethabi]: https://github.com/rust-ethereum/ethabi

#![warn(
    missing_docs,
    unreachable_pub,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_const_for_fn
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use std::{borrow::Cow, collections::BTreeMap};

use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Serialize,
};

/// Error JSON format
pub mod error;
/// Event JSON format
pub mod event;
/// Function JSON format
pub mod functions;
/// Param JSON format, used as components of functions, errors, structs, tuples.
pub mod param;

/// The ABI JSON file format.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AbiJson {
    /// The constructor function.
    pub constructor: Option<functions::Constructor>,
    /// The fallback function.
    pub fallback: Option<functions::Fallback>,
    /// The receive function.
    pub receive: Option<functions::Receive>,
    /// The functions, indexed by the function name
    pub functions: BTreeMap<String, Vec<functions::Function>>,
    /// The events, indexed by the event name
    pub events: BTreeMap<String, Vec<event::Event>>,
    /// The errors, indexed by the error name
    pub errors: BTreeMap<String, Vec<error::Error>>,
}

impl AbiJson {
    /// The total number of items (of any type)
    pub fn len(&self) -> usize {
        self.constructor.is_some() as usize
            + self.fallback.is_some() as usize
            + self.receive.is_some() as usize
            + self.functions.values().map(|v| v.len()).sum::<usize>()
            + self.events.values().map(|v| v.len()).sum::<usize>()
            + self.errors.values().map(|v| v.len()).sum::<usize>()
    }

    /// True if the ABI contains no items
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Abi Items
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum AbiItem<'a> {
    /// [`functions::Constructor`]
    Constructor(Cow<'a, functions::Constructor>),
    /// [`functions::Fallback`]
    Fallback(Cow<'a, functions::Fallback>),
    /// [`functions::Receive`]
    Receive(Cow<'a, functions::Receive>),
    /// [`functions::Function`]
    Function(Cow<'a, functions::Function>),
    /// [`event::Event`]
    Event(Cow<'a, event::Event>),
    /// [`error::Error`]
    Error(Cow<'a, error::Error>),
}

struct AbiJsonVisitor;

impl<'de> Visitor<'de> for AbiJsonVisitor {
    type Value = AbiJson;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<'de> serde::Deserialize<'de> for AbiJson {
    fn deserialize<D>(deserializer: D) -> Result<AbiJson, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
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
            seq.serialize_element(&AbiItem::Constructor(Cow::Borrowed(constructor)))?;
        }
        if let Some(fallback) = &self.fallback {
            seq.serialize_element(&AbiItem::Fallback(Cow::Borrowed(fallback)))?;
        }
        if let Some(receive) = &self.receive {
            seq.serialize_element(&AbiItem::Receive(Cow::Borrowed(receive)))?;
        }

        self.functions
            .values()
            .flatten()
            .try_for_each(|f| seq.serialize_element(&AbiItem::Function(Cow::Borrowed(f))))?;

        self.events
            .values()
            .flatten()
            .try_for_each(|e| seq.serialize_element(&AbiItem::Event(Cow::Borrowed(e))))?;

        self.errors
            .values()
            .flatten()
            .try_for_each(|e| seq.serialize_element(&AbiItem::Error(Cow::Borrowed(e))))?;

        seq.end()
    }
}

#[cfg(test)]
mod test {
    use super::AbiJson;

    const JSON: &str = include_str!("../test_data/seaport_1_3.abi.json");

    #[test]
    fn deserialize() {
        let abi: AbiJson = serde_json::from_str(JSON).unwrap();
        assert_eq!(abi.len(), 67);
    }

    #[test]
    fn round_trip() {
        let abi: AbiJson = serde_json::from_str(JSON).unwrap();

        let json = serde_json::to_string_pretty(&abi).unwrap();
        let abi2: AbiJson = serde_json::from_str(&json).unwrap();
        assert_eq!(abi, abi2);
    }
}
