//! This is an implementation of serde for Log for
//! both human-readable and binary forms.
//!
//! Ethereum JSON RPC requires logs in a flattened form.
//! However `serde(flatten)` breaks binary implementations.
//!
//! This module uses a trick to select a proxy for serde:
//! 1. LogFlattenSerializer for a human-readable (JSON) serializer,
//! 2. LogFlattenVisitor for a human-readable (JSON) deserializer,
//! 3. LogUnflattenSerializer for a binary serializer,
//! 4. LogUnflattenDeserializer for a binary deserializer.

use crate::{Address, Log};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt, marker::PhantomData, mem};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{
        self, DeserializeSeed, IgnoredAny, MapAccess, Visitor,
        value::{
            BorrowedBytesDeserializer, BorrowedStrDeserializer, BytesDeserializer,
            MapAccessDeserializer, StrDeserializer, StringDeserializer,
        },
    },
};

#[derive(Serialize)]
#[serde(rename = "Log")]
struct LogFlattenSerializer<'a, T> {
    address: &'a Address,
    #[serde(flatten)]
    data: &'a T,
}

/// Deserializes `address` and flattens every other entry into `T`.
///
/// This avoids `#[serde(flatten)]`, which buffers all other entries into an owned tree first.
struct LogFlattenVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for LogFlattenVisitor<T> {
    type Value = Log<T>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("struct LogFlattenDeserializer")
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let mut filter = AddressFilter { map, address: None, next: Next::Unread };
        let data = T::deserialize(PayloadDeserializer(&mut filter))?;
        while filter.next_key::<IgnoredAny>()?.is_some() {
            filter.next_value::<IgnoredAny>()?;
        }
        let address = filter.address.ok_or_else(|| de::Error::missing_field("address"))?;
        Ok(Log { address, data })
    }
}

/// Presents the map to `T` like `serde(flatten)` does: a map, with newtypes, options and units
/// looking through it instead of failing on a map.
struct PayloadDeserializer<'a, 'de, A>(&'a mut AddressFilter<'de, A>);

impl<'de, A: MapAccess<'de>> Deserializer<'de> for PayloadDeserializer<'_, 'de, A> {
    type Error = A::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        MapAccessDeserializer::new(self.0).deserialize_any(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        MapAccessDeserializer::new(self.0).deserialize_enum(name, variants, visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    /// An option is `None` when no entry besides `address` is left, as for a serialized `None`.
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.0.is_end()? { visitor.visit_none() } else { visitor.visit_some(self) }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf seq
        tuple tuple_struct map struct identifier
    }
}

/// A [`MapAccess`] that takes the `address` entry and yields all others.
struct AddressFilter<'de, A> {
    map: A,
    address: Option<Address>,
    next: Next<'de>,
}

/// The next key of an [`AddressFilter`] once it has been looked at.
enum Next<'de> {
    Unread,
    Key(Key<'de>),
    End,
}

impl<'de, A: MapAccess<'de>> AddressFilter<'de, A> {
    /// Returns whether the map has no entry left besides `address`.
    fn is_end(&mut self) -> Result<bool, A::Error> {
        if matches!(self.next, Next::Unread) {
            self.next = match self.next_key_seed(PhantomData::<Key<'de>>)? {
                Some(key) => Next::Key(key),
                None => Next::End,
            };
        }
        Ok(matches!(self.next, Next::End))
    }
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for AddressFilter<'de, A> {
    type Error = A::Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        match mem::replace(&mut self.next, Next::Unread) {
            Next::Key(key) => return key.replay(seed).map(Some),
            Next::End => {
                self.next = Next::End;
                return Ok(None);
            }
            Next::Unread => {}
        }
        let mut seed = Some(seed);
        while let Some(key) = self.map.next_key_seed(KeySeed(&mut seed))? {
            if let Some(key) = key {
                return Ok(Some(key));
            }
            if self.address.is_some() {
                return Err(de::Error::duplicate_field("address"));
            }
            self.address = Some(self.map.next_value()?);
        }
        self.next = Next::End;
        Ok(None)
    }

    #[inline]
    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        self.map.next_value_seed(seed)
    }
}

/// Reads a key: `None` if it is `address`, otherwise passes it to the inner seed.
///
/// The seed sits in an `Option` so that it is only consumed by a key that is not `address`.
struct KeySeed<'a, K>(&'a mut Option<K>);

impl<'de, K: DeserializeSeed<'de>> DeserializeSeed<'de> for KeySeed<'_, K> {
    type Value = Option<K::Value>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de, K: DeserializeSeed<'de>> Visitor<'de> for KeySeed<'_, K> {
    type Value = Option<K::Value>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("field identifier")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        self.forward(v == "address", StrDeserializer::new(v))
    }

    fn visit_borrowed_str<E: de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
        self.forward(v == "address", BorrowedStrDeserializer::new(v))
    }

    fn visit_string<E: de::Error>(self, v: alloc::string::String) -> Result<Self::Value, E> {
        self.forward(v == "address", StringDeserializer::new(v))
    }

    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        self.forward(v == b"address", BytesDeserializer::new(v))
    }

    fn visit_borrowed_bytes<E: de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        self.forward(v == b"address", BorrowedBytesDeserializer::new(v))
    }
}

impl<'de, K: DeserializeSeed<'de>> KeySeed<'_, K> {
    fn forward<D: Deserializer<'de>>(
        self,
        is_address: bool,
        key: D,
    ) -> Result<Option<K::Value>, D::Error> {
        if is_address {
            return Ok(None);
        }
        let seed = self.0.take().expect("seed is consumed once per key");
        seed.deserialize(key).map(Some)
    }
}

/// A map key kept to be passed to a seed later.
enum Key<'de> {
    Str(&'de str),
    String(String),
    Bytes(&'de [u8]),
    ByteBuf(Vec<u8>),
}

impl<'de> Key<'de> {
    fn replay<K: DeserializeSeed<'de>, E: de::Error>(self, seed: K) -> Result<K::Value, E> {
        match self {
            Self::Str(v) => seed.deserialize(BorrowedStrDeserializer::new(v)),
            Self::String(v) => seed.deserialize(StringDeserializer::new(v)),
            Self::Bytes(v) => seed.deserialize(BorrowedBytesDeserializer::new(v)),
            Self::ByteBuf(v) => seed.deserialize(BytesDeserializer::new(&v)),
        }
    }
}

impl<'de> Deserialize<'de> for Key<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(KeyVisitor)
    }
}

struct KeyVisitor;

impl<'de> Visitor<'de> for KeyVisitor {
    type Value = Key<'de>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("field identifier")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Key::String(v.to_string()))
    }

    fn visit_borrowed_str<E: de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Ok(Key::Str(v))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Key::String(v))
    }

    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(Key::ByteBuf(v.to_vec()))
    }

    fn visit_borrowed_bytes<E: de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        Ok(Key::Bytes(v))
    }
}

#[derive(Serialize)]
#[serde(rename = "Log")]
struct LogUnflattenSerializer<'a, T> {
    address: &'a Address,
    data: &'a T,
}

#[derive(Deserialize)]
#[serde(rename = "Log")]
struct LogUnflattenDeserializer<T> {
    address: Address,
    data: T,
}

impl<T: Serialize> Serialize for Log<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Self { address, data } = self;
        if serializer.is_human_readable() {
            let replace = LogFlattenSerializer { address, data };
            replace.serialize(serializer)
        } else {
            let replace = LogUnflattenSerializer { address, data };
            replace.serialize(serializer)
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Log<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_map(LogFlattenVisitor(PhantomData))
        } else {
            let LogUnflattenDeserializer { address, data } =
                Deserialize::deserialize(deserializer)?;
            Ok(Self { address, data })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Bytes,
        log::{Log, LogData},
    };
    use alloc::{
        format,
        string::{String, ToString},
        vec::Vec,
    };
    use serde::{
        Deserialize,
        de::{
            IntoDeserializer,
            value::{BorrowedBytesDeserializer, MapDeserializer},
        },
    };
    use serde_json::{Value, json};

    const ADDRESS: &str = "0x3100000000000000000000000000000000000001";
    const TOPIC: &str = "0x32eff959e2e8d1609edc4b39ccf75900aa6c1da5719f8432752963fdf008234f";

    #[derive(Debug, PartialEq, Deserialize)]
    struct Rpc {
        #[serde(flatten)]
        log: Log,
        #[serde(rename = "blockNumber")]
        block_number: u64,
    }

    fn gen_log() -> Log {
        Log {
            address: ADDRESS.parse().unwrap(),
            data: LogData::new(vec![TOPIC.parse().unwrap()], Bytes::from_static(&[0x12, 0x34]))
                .unwrap(),
        }
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestStruct {
        logs: Vec<Log>,
    }

    fn gen_test_struct() -> TestStruct {
        // assume it's random:
        TestStruct {
            logs: vec![Log {
                address: address!("0x3100000000000000000000000000000000000001"),
                data: LogData::new(
                    vec![b256!("0x32eff959e2e8d1609edc4b39ccf75900aa6c1da5719f8432752963fdf008234f")],
                    Bytes::from_static(b"00000000000000000000000000000000000000000000000000000000000000021e9dbc1a11f8e046a72d1296cc2d8bb0d1544d56fd0b9bb8890a0f89b88036541e9dbc1a11f8e046a72d1296cc2d8bb0d1544d56fd0b9bb8890a0f89b8803654"),
                ).unwrap(),
            }],
        }
    }

    #[test]
    fn test_log_bincode_roundtrip() {
        let generated = gen_test_struct();

        let bytes = bincode::serialize(&generated).unwrap();
        let parsed: TestStruct = bincode::deserialize(&bytes).unwrap();
        assert_eq!(generated, parsed);
    }

    #[test]
    fn test_log_bcs_roundtrip() {
        let generated = gen_test_struct();

        let bytes = bcs::to_bytes(&generated).unwrap();
        let parsed: TestStruct = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(generated, parsed);
    }

    #[test]
    fn test_log_json_roundtrip() {
        let expected = "{\"logs\":[{\"address\":\"0x3100000000000000000000000000000000000001\",\"topics\":[\"0x32eff959e2e8d1609edc4b39ccf75900aa6c1da5719f8432752963fdf008234f\"],\"data\":\"0x303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030323165396462633161313166386530343661373264313239366363326438626230643135343464353666643062396262383839306130663839623838303336353431653964626331613131663865303436613732643132393663633264386262306431353434643536666430623962623838393061306638396238383033363534\"}]}";

        let parsed: TestStruct = serde_json::from_str(expected).unwrap();
        let dumped = serde_json::to_string(&parsed).unwrap();

        assert_eq!(expected, dumped);
    }

    #[test]
    fn test_log_json_flatten_any_key_order() {
        let log = gen_log();
        let (a, t) = (format!("\"address\":\"{ADDRESS}\""), format!("\"topics\":[\"{TOPIC}\"]"));
        for json in [
            format!("{{{a},{t},\"data\":\"0x1234\",\"blockNumber\":7}}"),
            format!("{{\"unknown\":[{{}}],{t},\"data\":\"0x1234\",\"blockNumber\":7,{a}}}"),
        ] {
            assert_eq!(serde_json::from_str::<Log>(&json).unwrap(), log);
            assert_eq!(
                serde_json::from_str::<Rpc>(&json).unwrap(),
                Rpc { log: log.clone(), block_number: 7 }
            );
            let value = serde_json::from_str::<Value>(&json).unwrap();
            assert_eq!(Log::deserialize(&value).unwrap(), log);
            assert_eq!(Log::deserialize(value).unwrap(), log);
        }
    }

    #[test]
    fn test_log_json_value_data() {
        let json = json!({"address": ADDRESS, "topics": [], "data": "0x", "extra": 1});
        let log = serde_json::from_value::<Log<Value>>(json).unwrap();
        assert_eq!(log.address, gen_log().address);
        assert_eq!(log.data, json!({"topics": [], "data": "0x", "extra": 1}));
    }

    #[test]
    fn test_log_json_payload_shapes() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Wrapped(LogData);

        let json = json!({"address": ADDRESS, "topics": [TOPIC], "data": "0x1234"});
        let expected = gen_log();

        let log = serde_json::from_value::<Log<Wrapped>>(json.clone()).unwrap();
        assert_eq!((log.address, log.data.0), (expected.address, expected.data.clone()));
        let log = serde_json::from_value::<Log<Option<LogData>>>(json.clone()).unwrap();
        assert_eq!(log.data, Some(expected.data));
        let log = serde_json::from_value::<Log<()>>(json).unwrap();
        assert_eq!(log.address, expected.address);

        let none = Log { address: expected.address, data: None::<LogData> };
        let dumped = serde_json::to_string(&none).unwrap();
        assert_eq!(serde_json::from_str::<Log<Option<LogData>>>(&dumped).unwrap(), none);
        let json = format!("{{\"extra\":1,\"address\":\"{ADDRESS}\"}}");
        assert_eq!(
            serde_json::from_str::<Log<Option<Value>>>(&json).unwrap().data,
            Some(json!({"extra": 1}))
        );
    }

    #[test]
    fn test_log_json_errors() {
        let (t, d) = ("\"topics\":[]", "\"data\":\"0x\"");
        let a = format!("\"address\":\"{ADDRESS}\"");
        let err = |json: String| serde_json::from_str::<Log>(&json).unwrap_err().to_string();
        assert!(err(format!("{{{a},{t},{a},{d}}}")).starts_with("duplicate field `address`"));
        assert!(err(format!("{{{t},{d}}}")).starts_with("missing field `address`"));
        assert!(err(format!("{{{a},{d}}}")).starts_with("missing field `topics`"));
    }

    #[test]
    fn test_log_json_bytes_keys() {
        let entries = [
            (b"topics".as_slice(), json!([])),
            (b"address".as_slice(), json!(ADDRESS)),
            (b"data".as_slice(), json!("0x")),
        ];
        let map = MapDeserializer::<_, serde_json::Error>::new(
            entries
                .into_iter()
                .map(|(k, v)| (BorrowedBytesDeserializer::new(k), v.into_deserializer())),
        );
        let log = Log::<LogData>::deserialize(map).unwrap();
        assert_eq!(log.address, gen_log().address);
        assert!(log.data.data.is_empty());
    }
}
