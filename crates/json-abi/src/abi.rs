use crate::{AbiItem, Constructor, Error, Event, Fallback, Function, Receive};
use alloc::{
    collections::{btree_map, btree_map::Values},
    string::String,
    vec::Vec,
};
use alloy_primitives::Bytes;
use btree_map::BTreeMap;
use core::{fmt, iter, iter::Flatten};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize,
};

/// The JSON contract ABI, as specified in the [Solidity ABI spec][ref].
///
/// [ref]: https://docs.soliditylang.org/en/latest/abi-spec.html#json
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct JsonAbi {
    /// The constructor function.
    pub constructor: Option<Constructor>,
    /// The fallback function.
    pub fallback: Option<Fallback>,
    /// The receive function.
    pub receive: Option<Receive>,
    /// The functions, indexed by the function name.
    pub functions: BTreeMap<String, Vec<Function>>,
    /// The events, indexed by the event name.
    pub events: BTreeMap<String, Vec<Event>>,
    /// The errors, indexed by the error name.
    pub errors: BTreeMap<String, Vec<Error>>,
}

impl JsonAbi {
    /// Returns the total number of items (of any type).
    pub fn len(&self) -> usize {
        self.constructor.is_some() as usize
            + self.fallback.is_some() as usize
            + self.receive.is_some() as usize
            + self.functions.values().map(Vec::len).sum::<usize>()
            + self.events.values().map(Vec::len).sum::<usize>()
            + self.errors.values().map(Vec::len).sum::<usize>()
    }

    /// Returns true if the ABI contains no items.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over all of the items in the ABI.
    #[inline]
    pub fn items(&self) -> Items<'_> {
        self.items_with_len(self.len())
    }

    // `len` must be `self.len()`
    #[inline]
    fn items_with_len(&self, len: usize) -> Items<'_> {
        Items {
            len,
            constructor: self.constructor.as_ref(),
            fallback: self.fallback.as_ref(),
            receive: self.receive.as_ref(),
            functions: self.functions(),
            events: self.events(),
            errors: self.errors(),
        }
    }

    /// Returns an iterator over all of the items in the ABI.
    #[inline]
    pub fn into_items(self) -> IntoItems {
        IntoItems {
            len: self.len(),
            constructor: self.constructor,
            fallback: self.fallback,
            receive: self.receive,
            functions: self.functions.into_values().flatten(),
            events: self.events.into_values().flatten(),
            errors: self.errors.into_values().flatten(),
        }
    }

    /// Creates constructor call builder.
    #[inline]
    pub const fn constructor(&self) -> Option<&Constructor> {
        self.constructor.as_ref()
    }

    /// Parse the ABI json from a `str`. This is a convenience wrapper around
    /// [`serde_json::from_str`].
    #[cfg(feature = "serde_json")]
    #[inline]
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Loads contract from json
    #[cfg(all(feature = "std", feature = "serde_json"))]
    pub fn load<T: std::io::Read>(mut reader: T) -> Result<Self, serde_json::Error> {
        // https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html
        // serde_json docs recommend buffering the whole reader to a string
        // This also prevents a borrowing issue when deserializing from a reader
        let mut json = String::with_capacity(1024);
        reader
            .read_to_string(&mut json)
            .map_err(serde_json::Error::io)?;

        Self::from_json_str(&json)
    }

    /// Gets all the functions with the given name.
    #[inline]
    pub fn function(&self, name: &str) -> Option<&[Function]> {
        self.functions.get(name).map(Vec::as_slice)
    }

    /// Gets all the events with the given name.
    #[inline]
    pub fn event(&self, name: &str) -> Option<&[Event]> {
        self.events.get(name).map(Vec::as_slice)
    }

    /// Gets all the errors with the given name.
    #[inline]
    pub fn error(&self, name: &str) -> Option<&[Error]> {
        self.errors.get(name).map(Vec::as_slice)
    }

    /// Iterates over all the functions of the contract in arbitrary order.
    #[inline]
    pub fn functions(&self) -> Flatten<Values<'_, String, Vec<Function>>> {
        self.functions.values().flatten()
    }

    /// Iterates over all the events of the contract in arbitrary order.
    #[inline]
    pub fn events(&self) -> Flatten<Values<'_, String, Vec<Event>>> {
        self.events.values().flatten()
    }

    /// Iterates over all the errors of the contract in arbitrary order.
    #[inline]
    pub fn errors(&self) -> Flatten<Values<'_, String, Vec<Error>>> {
        self.errors.values().flatten()
    }
}

macro_rules! next_item {
    ($self:ident; $($ident:ident.$f:ident()),* $(,)?) => {$(
        if let Some(next) = $self.$ident.$f() {
            $self.len -= 1;
            return Some(next.into())
        }
    )*};
}

macro_rules! iter_impl {
    (front) => {
        fn next(&mut self) -> Option<Self::Item> {
            next_item!(self;
                constructor.take(),
                fallback.take(),
                receive.take(),
                functions.next(),
                events.next(),
                errors.next(),
            );
            debug_assert_eq!(self.len, 0);
            None
        }

        #[inline]
        fn count(self) -> usize {
            self.len
        }

        #[inline]
        fn last(mut self) -> Option<Self::Item> {
            self.next_back()
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.len, Some(self.len))
        }
    };
    (back) => {
        fn next_back(&mut self) -> Option<Self::Item> {
            next_item!(self;
                errors.next_back(),
                events.next_back(),
                functions.next_back(),
                receive.take(),
                fallback.take(),
                constructor.take(),
            );
            debug_assert_eq!(self.len, 0);
            None
        }
    };
    (traits $ty:ty) => {
        impl DoubleEndedIterator for $ty {
            iter_impl!(back);
        }

        impl ExactSizeIterator for $ty {
            #[inline]
            fn len(&self) -> usize {
                self.len
            }
        }

        impl iter::FusedIterator for $ty {}
    };
}

type FlattenValues<'a, V> = Flatten<btree_map::Values<'a, String, Vec<V>>>;

/// An iterator over all of the items in the ABI.
///
/// This `struct` is created by [`JsonAbi::items`]. See its documentation for
/// more.
#[derive(Clone, Debug)] // TODO(MSRV-1.70): derive Default
pub struct Items<'a> {
    len: usize,
    constructor: Option<&'a Constructor>,
    fallback: Option<&'a Fallback>,
    receive: Option<&'a Receive>,
    functions: FlattenValues<'a, Function>,
    events: FlattenValues<'a, Event>,
    errors: FlattenValues<'a, Error>,
}

impl<'a> Iterator for Items<'a> {
    type Item = AbiItem<'a>;

    iter_impl!(front);
}

iter_impl!(traits Items<'_>);

type FlattenIntoValues<V> = Flatten<btree_map::IntoValues<String, Vec<V>>>;

/// An iterator over all of the items in the ABI.
///
/// This `struct` is created by [`JsonAbi::into_items`]. See its documentation
/// for more.
#[derive(Debug)] // TODO(MSRV-1.70): derive Default
pub struct IntoItems {
    len: usize,
    constructor: Option<Constructor>,
    fallback: Option<Fallback>,
    receive: Option<Receive>,
    functions: FlattenIntoValues<Function>,
    events: FlattenIntoValues<Event>,
    errors: FlattenIntoValues<Error>,
}

impl Iterator for IntoItems {
    type Item = AbiItem<'static>;

    iter_impl!(front);
}

iter_impl!(traits IntoItems);

impl<'de> Deserialize<'de> for JsonAbi {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<JsonAbi, D::Error> {
        deserializer.deserialize_seq(JsonAbiVisitor)
    }
}

impl Serialize for JsonAbi {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = self.len();
        let mut seq = serializer.serialize_seq(Some(len))?;
        for item in self.items_with_len(len) {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

macro_rules! set_if_none {
    ($opt:expr, $val:expr) => { set_if_none!(stringify!($opt) => $opt, $val) };
    ($name:expr => $opt:expr, $val:expr) => {{
        if $opt.is_some() {
            return Err(serde::de::Error::duplicate_field($name))
        }
        $opt = Some($val);
    }};
}

macro_rules! entry_and_push {
    ($map:expr, $v:expr) => {
        $map.entry($v.name.clone())
            .or_default()
            .push($v.into_owned())
    };
}

struct JsonAbiVisitor;

impl<'de> Visitor<'de> for JsonAbiVisitor {
    type Value = JsonAbi;

    #[inline]
    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("a valid JSON ABI sequence")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut abi = JsonAbi::default();
        while let Some(item) = seq.next_element()? {
            match item {
                AbiItem::Constructor(c) => set_if_none!(abi.constructor, c.into_owned()),
                AbiItem::Fallback(f) => set_if_none!(abi.fallback, f.into_owned()),
                AbiItem::Receive(r) => set_if_none!(abi.receive, r.into_owned()),
                AbiItem::Function(f) => entry_and_push!(abi.functions, f),
                AbiItem::Event(e) => entry_and_push!(abi.events, e),
                AbiItem::Error(e) => entry_and_push!(abi.errors, e),
            }
        }
        Ok(abi)
    }
}

/// Represents a generic contract's ABI, bytecode and deployed bytecode.
///
/// Can be deserialized from both an ABI array, and a JSON object with the `abi`
/// field with optionally the bytecode fields.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractObject {
    /// The contract ABI.
    pub abi: JsonAbi,
    /// The contract bytecode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<Bytes>,
    /// The contract deployed bytecode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployed_bytecode: Option<Bytes>,
}

impl<'de> Deserialize<'de> for ContractObject {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ContractAbiObjectVisitor)
    }
}

// Modified from `ethers_core::abi::raw`:
// https://github.com/gakonst/ethers-rs/blob/311086466871204c3965065b8c81e47418261412/ethers-core/src/abi/raw.rs#L154
struct ContractAbiObjectVisitor;

impl<'de> Visitor<'de> for ContractAbiObjectVisitor {
    type Value = ContractObject;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a sequence or map with `abi` key")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Bytecode {
            Bytes(Bytes),
            Object { object: Bytes },
        }

        impl Bytecode {
            #[allow(clippy::missing_const_for_fn)]
            fn bytes(self) -> Bytes {
                let (Self::Object { object: bytes } | Self::Bytes(bytes)) = self;
                bytes
            }
        }

        /// Represents nested bytecode objects of the `evm` value.
        #[derive(Deserialize)]
        struct EvmObj {
            bytecode: Option<Bytecode>,
            #[serde(rename = "deployedBytecode")]
            deployed_bytecode: Option<Bytecode>,
        }

        let mut abi = None;
        let mut bytecode = None;
        let mut deployed_bytecode = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "abi" => set_if_none!(abi, map.next_value()?),
                "evm" => {
                    let evm = map.next_value::<EvmObj>()?;
                    if let Some(bytes) = evm.bytecode {
                        set_if_none!(bytecode, bytes.bytes());
                    }
                    if let Some(bytes) = evm.deployed_bytecode {
                        set_if_none!(deployed_bytecode, bytes.bytes());
                    }
                }
                "byteCode" | "bytecode" | "bin" => {
                    set_if_none!(bytecode, map.next_value::<Bytecode>()?.bytes());
                }
                "deployedBytecode" | "deployedbytecode" | "runtimeBin" | "runtimebin" => {
                    set_if_none!(deployed_bytecode, map.next_value::<Bytecode>()?.bytes());
                }
                _ => {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let abi = abi.ok_or_else(|| serde::de::Error::missing_field("abi"))?;
        Ok(ContractObject {
            abi,
            bytecode,
            deployed_bytecode,
        })
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        JsonAbiVisitor.visit_seq(seq).map(|abi| ContractObject {
            abi,
            bytecode: None,
            deployed_bytecode: None,
        })
    }
}
