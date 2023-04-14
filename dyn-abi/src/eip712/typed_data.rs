use ethers_abi_enc::Eip712Domain;
use serde::{Deserialize, Serialize};

use crate::{eip712::DepGraph, no_std_prelude::BTreeMap};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NameTypePair {
    /// Typename
    #[serde(rename = "type")]
    type_name: String,
    /// Property Name
    name: String,
}

impl NameTypePair {
    pub fn new(type_name: impl AsRef<str>, name: impl AsRef<str>) -> Self {
        Self {
            type_name: type_name.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
        }
    }

    /// Returns the type name of the property
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns the root type of the name/type pair, stripping any array
    pub fn root_type_name(&self) -> &str {
        &self
            .type_name
            .split_once('[')
            .map(|t| t.0)
            .unwrap_or(&self.type_name)
    }

    /// Returns the name of the property
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Custom types for `TypedData`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Types(BTreeMap<String, Vec<NameTypePair>>);

impl std::iter::IntoIterator for Types {
    type Item = (String, Vec<NameTypePair>);
    type IntoIter = std::collections::btree_map::IntoIter<String, Vec<NameTypePair>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Types {
    /// Returns the `EIP712Domain` type
    fn domain(&self) -> Option<&Vec<NameTypePair>> {
        self.0.get("EIP712Domain")
    }

    fn get(&self, type_name: &str) -> Option<&Vec<NameTypePair>> {
        self.0.get(type_name)
    }
}

impl From<Types> for DepGraph {
    fn from(types: Types) -> Self {
        let mut graph = DepGraph::default();
        graph.ingest_types(types);
        graph
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
    pub types: Types,
    #[serde(rename = "primaryType")]
    /// The type of the message.
    pub primary_type: String,
    /// The message to be signed.
    pub message: BTreeMap<String, serde_json::Value>,
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
            types: Types,
            #[serde(rename = "primaryType")]
            primary_type: String,
            message: BTreeMap<String, serde_json::Value>,
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
    fn domain(&self) -> &Eip712Domain {
        &self.domain
    }
}
