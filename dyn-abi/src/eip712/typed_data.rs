use ethers_abi_enc::Eip712Domain;
use serde::{Deserialize, Serialize};

use crate::{eip712::PropertyDef, no_std_prelude::*, parser::RootType};

/// Thin wrapper around `serde_json::Value`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object(serde_json::Value);

impl From<Object> for serde_json::Value {
    fn from(obj: Object) -> Self {
        obj.0
    }
}

impl From<serde_json::Value> for Object {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl AsRef<serde_json::Value> for Object {
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}

/// Custom types for `TypedData`
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Eip712Types(BTreeMap<String, Vec<PropertyDef>>);

impl<'de> Deserialize<'de> for Eip712Types {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map: BTreeMap<String, Vec<PropertyDef>> = BTreeMap::deserialize(deserializer)?;

        for key in map.keys() {
            let _rt: RootType<'_> = key.as_str().try_into().map_err(serde::de::Error::custom)?;
        }

        Ok(Self(map))
    }
}

impl core::iter::IntoIterator for Eip712Types {
    type Item = (String, Vec<PropertyDef>);
    type IntoIter = btree_map::IntoIter<String, Vec<PropertyDef>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Represents the [EIP-712](https://eips.ethereum.org/EIPS/eip-712) typed data object.
///
/// Typed data is a JSON object containing type information, domain separator parameters and the
/// message object which has the following schema
///
/// ```json
/// {
///     "type": "object",
///     "properties": {
///         "types": {
///             "type": "object",
///             "properties": {
///                 "EIP712Domain": { "type": "array" }
///             },
///             "additionalProperties": {
///                 "type": "array",
///                 "items": {
///                     "type": "object",
///                     "properties": {
///                         "name": { "type": "string" },
///                         "type": { "type": "string" }
///                     },
///                     "required": ["name", "type"]
///                 }
///             },
///             "required": ["EIP712Domain"]
///         },
///         "primaryType": { "type": "string" },
///         "domain": { "type": "object" },
///         "message": { "type": "object" }
///     },
///     "required": ["types", "primaryType", "domain", "message"]
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedData {
    /// Signing domain metadata. The signing domain is the intended context for
    /// the signature (e.g. the dapp, protocol, etc. that it's intended for).
    /// This data is used to construct the domain seperator of the message.
    pub domain: Eip712Domain,
    /// The custom types used by this message.
    pub types: Eip712Types,
    #[serde(rename = "primaryType")]
    /// The type of the message.
    pub primary_type: String,
    /// The message to be signed.
    pub message: Object,
}

/// According to the MetaMask implementation,
/// the message parameter may be JSON stringified in versions later than V1
/// See <https://github.com/MetaMask/metamask-extension/blob/0dfdd44ae7728ed02cbf32c564c75b74f37acf77/app/scripts/metamask-controller.js#L1736>
/// In fact, ethers.js JSON stringifies the message at the time of writing.
impl<'de> Deserialize<'de> for TypedData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TypedDataHelper {
            domain: Eip712Domain,
            types: Eip712Types,
            #[serde(rename = "primaryType")]
            primary_type: String,
            message: Object,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Type {
            Val(TypedDataHelper),
            String(String),
        }

        match Type::deserialize(deserializer)? {
            Type::Val(v) => {
                let TypedDataHelper {
                    domain,
                    types,
                    primary_type,
                    message,
                } = v;
                Ok(TypedData {
                    domain,
                    types,
                    primary_type,
                    message,
                })
            }
            Type::String(s) => {
                let TypedDataHelper {
                    domain,
                    types,
                    primary_type,
                    message,
                } = serde_json::from_str(&s).map_err(serde::de::Error::custom)?;
                Ok(TypedData {
                    domain,
                    types,
                    primary_type,
                    message,
                })
            }
        }
    }
}

impl TypedData {
    /// Returns the domain for this typed data
    pub const fn domain(&self) -> &Eip712Domain {
        &self.domain
    }
}
