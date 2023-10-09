use crate::{AbiItem, Constructor, Error, Event, Fallback, Function, Receive};
use alloc::{collections::btree_map, string::String, vec::Vec};
use alloy_primitives::Bytes;
use alloy_sol_type_parser::{Error as ParserError, Result as ParserResult};
use btree_map::BTreeMap;
use core::{fmt, iter, iter::Flatten};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize,
};

macro_rules! set_if_none {
    ($opt:expr, $val:expr) => { set_if_none!(stringify!($opt) => $opt, $val) };
    (@serde $opt:expr, $val:expr) => { set_if_none!(serde::de::Error::duplicate_field(stringify!($opt)) => $opt, $val) };
    ($name:expr => $opt:expr, $val:expr) => {{
        if $opt.is_some() {
            return Err($name)
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

type FlattenValues<'a, V> = Flatten<btree_map::Values<'a, String, Vec<V>>>;
type FlattenIntoValues<V> = Flatten<btree_map::IntoValues<String, Vec<V>>>;

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

impl<'a> FromIterator<AbiItem<'a>> for JsonAbi {
    fn from_iter<T: IntoIterator<Item = AbiItem<'a>>>(iter: T) -> Self {
        let mut abi = Self::new();
        for item in iter {
            let _ = abi.insert_item(item);
        }
        abi
    }
}

impl JsonAbi {
    /// Creates an empty ABI object.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a [Human-Readable ABI] string into a JSON object.
    ///
    /// [Human-Readable ABI]: https://docs.ethers.org/v5/api/utils/abi/formats/#abi-formats--human-readable-abi
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_json_abi::JsonAbi;
    /// assert_eq!(JsonAbi::parse([])?, JsonAbi::new());
    ///
    /// let abi = JsonAbi::parse([
    ///     "constructor(string symbol, string name)",
    ///     "function transferFrom(address from, address to, uint value)",
    ///     "function balanceOf(address owner)(uint balance)",
    ///     "event Transfer(address indexed from, address indexed to, address value)",
    ///     "error InsufficientBalance(account owner, uint balance)",
    ///     "function addPerson(tuple(string, uint16) person)",
    ///     "function addPeople(tuple(string, uint16)[] person)",
    ///     "function getPerson(uint id)(tuple(string, uint16))",
    ///     "event PersonAdded(uint indexed id, tuple(string, uint16) person)",
    /// ])?;
    /// assert_eq!(abi.len(), 9);
    /// # Ok::<(), alloy_sol_type_parser::Error>(())
    /// ```
    pub fn parse<'a, I: IntoIterator<Item = &'a str>>(strings: I) -> ParserResult<Self> {
        let mut abi = Self::new();
        for string in strings {
            let item = AbiItem::parse(string)?;
            abi.insert_item(item)
                .map_err(|s| ParserError::_new("duplicate JSON ABI field: ", &s))?;
        }
        Ok(abi)
    }

    /// Parse a JSON string into an ABI object.
    ///
    /// This is a convenience wrapper around [`serde_json::from_str`].
    #[cfg(feature = "serde_json")]
    #[inline]
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Loads contract from a JSON [Reader](std::io::Read).
    ///
    /// This is a convenience wrapper around [`serde_json::from_str`].
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

    /// Formats this JSON ABI as a Solidity interface.
    ///
    /// The order of the definitions is not guaranteed.
    ///
    /// Generates:
    ///
    /// ```solidity
    /// interface <name> {
    ///     <enums>...
    ///     <UDVTs>...
    ///     <structs>...
    ///     <errors>...
    ///     <events>...
    ///     <fallback>
    ///     <receive>
    ///     <functions>...
    /// }
    /// ```
    ///
    /// Note that enums are going to be identical to `uint8` UDVTs, since no
    /// other information about enums is present in the ABI.
    #[inline]
    pub fn to_sol(&self, name: &str) -> String {
        let mut out = String::new();
        self.to_sol_raw(name, &mut out);
        out
    }

    /// Formats this JSON ABI as a Solidity interface into the given string.
    ///
    /// See [`to_sol`](JsonAbi::to_sol) for more information.
    pub fn to_sol_raw(&self, name: &str, out: &mut String) {
        let len = self.len();
        out.reserve((len + 1) * 128);

        out.push_str("interface ");
        if !name.is_empty() {
            out.push_str(name);
            out.push(' ');
        }
        out.push('{');
        if len > 0 {
            out.push('\n');
            crate::to_sol::ToSol::to_sol(self, out);
        }
        out.push('}');
    }

    /// Returns this contract's constructor.
    #[inline]
    pub const fn constructor(&self) -> Option<&Constructor> {
        self.constructor.as_ref()
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
    pub fn functions(&self) -> FlattenValues<'_, Function> {
        self.functions.values().flatten()
    }

    /// Iterates over all the events of the contract in arbitrary order.
    #[inline]
    pub fn events(&self) -> FlattenValues<'_, Event> {
        self.events.values().flatten()
    }

    /// Iterates over all the errors of the contract in arbitrary order.
    #[inline]
    pub fn errors(&self) -> FlattenValues<'_, Error> {
        self.errors.values().flatten()
    }

    /// Inserts an item into the ABI.
    fn insert_item(&mut self, item: AbiItem<'_>) -> Result<(), &'static str> {
        match item {
            AbiItem::Constructor(c) => set_if_none!(self.constructor, c.into_owned()),
            AbiItem::Fallback(f) => set_if_none!(self.fallback, f.into_owned()),
            AbiItem::Receive(r) => set_if_none!(self.receive, r.into_owned()),
            AbiItem::Function(f) => entry_and_push!(self.functions, f),
            AbiItem::Event(e) => entry_and_push!(self.events, e),
            AbiItem::Error(e) => entry_and_push!(self.errors, e),
        };
        Ok(())
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
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
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

struct JsonAbiVisitor;

impl<'de> Visitor<'de> for JsonAbiVisitor {
    type Value = JsonAbi;

    #[inline]
    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("a valid JSON ABI sequence")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut abi = JsonAbi::new();
        while let Some(item) = seq.next_element()? {
            abi.insert_item(item)
                .map_err(serde::de::Error::duplicate_field)?;
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abi: Option<JsonAbi>,
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
        deserializer.deserialize_any(ContractObjectVisitor)
    }
}

// Modified from `ethers_core::abi::raw`:
// https://github.com/gakonst/ethers-rs/blob/311086466871204c3965065b8c81e47418261412/ethers-core/src/abi/raw.rs#L154
struct ContractObjectVisitor;

impl<'de> Visitor<'de> for ContractObjectVisitor {
    type Value = ContractObject;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an ABI sequence or contract object")
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
            #[inline(always)]
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
                "abi" => set_if_none!(@serde abi, map.next_value()?),
                "evm" => {
                    let evm = map.next_value::<EvmObj>()?;
                    if let Some(bytes) = evm.bytecode {
                        set_if_none!(@serde bytecode, bytes.bytes());
                    }
                    if let Some(bytes) = evm.deployed_bytecode {
                        set_if_none!(@serde deployed_bytecode, bytes.bytes());
                    }
                }
                "bytecode" | "bin" => {
                    set_if_none!(@serde bytecode, map.next_value::<Bytecode>()?.bytes());
                }
                "deployedBytecode" | "deployedbytecode" | "deployed_bytecode" | "runtimeBin"
                | "runtimebin" | "runtime " => {
                    set_if_none!(@serde deployed_bytecode, map.next_value::<Bytecode>()?.bytes());
                }
                _ => {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        Ok(ContractObject {
            abi,
            bytecode,
            deployed_bytecode,
        })
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        JsonAbiVisitor.visit_seq(seq).map(|abi| ContractObject {
            abi: Some(abi),
            bytecode: None,
            deployed_bytecode: None,
        })
    }
}
