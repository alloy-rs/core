use ethers_abi_enc::{keccak256, Eip712Domain, SolStruct};
use ethers_primitives::B256;
use serde::{Deserialize, Serialize};

use crate::{
    eip712::{PropertyDef, Resolver},
    no_std_prelude::*,
    parser::TypeSpecifier,
    DynAbiError, DynSolType, DynSolValue,
};

/// Custom types for `TypedData`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct Eip712Types(BTreeMap<String, Vec<PropertyDef>>);

impl<'de> Deserialize<'de> for Eip712Types {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map: BTreeMap<String, Vec<PropertyDef>> = BTreeMap::deserialize(deserializer)?;

        for key in map.keys() {
            // ensure that all types are valid specifiers
            let _rt: TypeSpecifier<'_> =
                key.as_str().try_into().map_err(serde::de::Error::custom)?;
        }

        Ok(Self(map))
    }
}

impl IntoIterator for Eip712Types {
    type Item = (String, Vec<PropertyDef>);
    type IntoIter = btree_map::IntoIter<String, Vec<PropertyDef>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Eip712Types {
    /// Iterate over the underlying map
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<PropertyDef>)> {
        self.0.iter()
    }

    /// Insert a new type
    pub fn insert(&mut self, key: String, value: Vec<PropertyDef>) {
        self.0.insert(key, value);
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
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct TypedData {
    /// Signing domain metadata. The signing domain is the intended context for
    /// the signature (e.g. the dapp, protocol, etc. that it's intended for).
    /// This data is used to construct the domain seperator of the message.
    pub domain: Eip712Domain,
    /// The custom types used by this message.
    pub resolver: Resolver,
    #[serde(rename = "primaryType")]
    /// The type of the message.
    pub primary_type: String,
    /// The message to be signed.
    pub message: serde_json::Value,
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
            #[serde(default)]
            domain: Eip712Domain,
            types: Eip712Types,
            #[serde(rename = "primaryType")]
            primary_type: String,
            #[serde(default)]
            message: serde_json::Value,
        }

        impl From<TypedDataHelper> for TypedData {
            fn from(value: TypedDataHelper) -> Self {
                Self {
                    domain: value.domain,
                    resolver: Resolver::from(&value.types),
                    primary_type: value.primary_type,
                    message: value.message,
                }
            }
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ValOrString {
            Val(TypedDataHelper),
            String(String),
        }

        match ValOrString::deserialize(deserializer)? {
            ValOrString::Val(v) => Ok(v.into()),
            ValOrString::String(s) => {
                let v = serde_json::from_str::<TypedDataHelper>(&s)
                    .map_err(serde::de::Error::custom)?;
                Ok(v.into())
            }
        }
    }
}

impl TypedData {
    /// Instantiate [`TypedData`] from a [`SolStruct`] that implements
    /// [`serde::Serialize`].
    pub fn from_struct<S: SolStruct + Serialize>(s: &S, domain: Option<Eip712Domain>) -> Self {
        Self {
            domain: domain.unwrap_or_default(),
            resolver: Resolver::from_struct::<S>(),
            primary_type: S::NAME.to_string(),
            message: serde_json::to_value(s).unwrap(),
        }
    }

    /// Returns the domain for this typed data
    pub const fn domain(&self) -> &Eip712Domain {
        &self.domain
    }

    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        self.resolver.resolve(&self.primary_type)
    }

    /// Coerce the message to the type specified by `primary_type`, using the
    /// types map as a resolver
    pub fn coerce(&self) -> Result<DynSolValue, DynAbiError> {
        let ty = self.resolve()?;
        ty.coerce(&self.message)
    }

    /// Calculate the `typeHash` for this value
    /// Fails if this type is not a struct
    pub fn type_hash(&self) -> Result<B256, DynAbiError> {
        Ok(keccak256(
            self.resolver.encode_type(&self.primary_type)?.as_bytes(),
        ))
    }

    /// Calculate the `hashStruct` for this value
    /// Fails if this type is not a struct
    pub fn hash_struct(&self) -> Result<B256, DynAbiError> {
        let mut type_hash = self.type_hash()?.to_vec();
        type_hash.extend(self.encode_data()?);
        Ok(keccak256(type_hash))
    }

    /// Calculate the `encodeType` for this value
    /// Fails if this type is not a struct
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    pub fn encode_type(&self) -> Result<String, DynAbiError> {
        self.resolver.encode_type(&self.primary_type)
    }

    /// Calculate the `encodeData` for this value
    /// Fails if this type is not a struct
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata>
    pub fn encode_data(&self) -> Result<Vec<u8>, DynAbiError> {
        let s = self.coerce()?;
        Ok(self.resolver.encode_data(&s)?.unwrap())
    }

    /// Calculate the `encodeType` for this value
    /// Fails if this type is not a struct
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    pub fn eip712_encode_type(&self) -> Result<String, DynAbiError> {
        self.encode_type()
    }

    /// Calculate the eip712 signing hash for this value. This is the hash of
    /// the magic bytes 0x1901 concatenated with the domain separator and the
    /// `hashStruct`.
    ///
    /// <https://eips.ethereum.org/EIPS/eip-712#specification>
    pub fn eip712_signing_hash(&self) -> Result<B256, DynAbiError> {
        let mut buf = [0u8; 66];
        buf[0] = 0x19;
        buf[1] = 0x01;
        buf[2..34].copy_from_slice(self.domain.separator().as_bytes());

        // compatibility with <https://github.com/MetaMask/eth-sig-util>
        let len = if self.primary_type != "EIP712Domain" {
            buf[34..].copy_from_slice(self.hash_struct()?.as_bytes());
            66
        } else {
            34
        };

        Ok(keccak256(&buf[..len]))
    }
}

// Adapted tests from <https://github.com/MetaMask/eth-sig-util/blob/main/src/sign-typed-data.test.ts>
#[cfg(test)]
mod tests {
    use ethers_abi_enc::sol;

    use super::*;

    #[test]
    fn test_full_domain() {
        let json = serde_json::json!({
          "types": {
            "EIP712Domain": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "version",
                "type": "string"
              },
              {
                "name": "chainId",
                "type": "uint256"
              },
              {
                "name": "verifyingContract",
                "type": "address"
              },
              {
                "name": "salt",
                "type": "bytes32"
              }
            ]
          },
          "primaryType": "EIP712Domain",
          "domain": {
            "name": "example.metamask.io",
            "version": "1",
            "chainId": 1,
            "verifyingContract": "0x0000000000000000000000000000000000000000"
          },
          "message": {}
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();

        let hash = typed_data.eip712_signing_hash().unwrap();
        assert_eq!(
            "122d1c8ef94b76dad44dcb03fa772361e20855c63311a15d5afe02d1b38f6077",
            hex::encode(&hash[..])
        );
    }

    #[test]
    fn test_minimal_message() {
        let json = serde_json::json!( {"types":{"EIP712Domain":[]},"primaryType":"EIP712Domain","domain":{},"message":{}});

        let typed_data: TypedData = serde_json::from_value(json).unwrap();

        let hash = typed_data.eip712_signing_hash().unwrap();
        assert_eq!(
            "8d4a3f4082945b7879e2b55f181c31a77c8c0a464b70669458abbaaf99de4c38",
            hex::encode(&hash[..])
        );
    }

    #[test]
    fn test_encode_custom_array_type() {
        let json = serde_json::json!({
            "domain":{},
            "types":{
                "EIP712Domain":[],
                "Person":[
                    {"name":"name","type":"string"},
                    {"name":"wallet","type":"address[]"}
                ],
                "Mail":[
                    {"name":"from","type":"Person"},
                    {"name":"to","type":"Person[]"},
                    {"name":"contents","type":"string"}
                ]
            },
            "primaryType":"Mail",
            "message":{
                "from":{
                    "name":"Cow",
                    "wallet":["0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826","0xDD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"]
                },
                "to":[
                    {"name":"Bob","wallet":["0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"]}
                ],
                "contents":"Hello, Bob!"
            }
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();

        let hash = typed_data.eip712_signing_hash().unwrap();
        assert_eq!(
            "80a3aeb51161cfc47884ddf8eac0d2343d6ae640efe78b6a69be65e3045c1321",
            hex::encode(&hash[..])
        );
    }

    #[test]
    fn test_hash_typed_message_with_data() {
        let json = serde_json::json!( {
          "types": {
            "EIP712Domain": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "version",
                "type": "string"
              },
              {
                "name": "chainId",
                "type": "uint256"
              },
              {
                "name": "verifyingContract",
                "type": "address"
              }
            ],
            "Message": [
              {
                "name": "data",
                "type": "string"
              }
            ]
          },
          "primaryType": "Message",
          "domain": {
            "name": "example.metamask.io",
            "version": "1",
            "chainId": "1",
            "verifyingContract": "0x0000000000000000000000000000000000000000"
          },
          "message": {
            "data": "Hello!"
          }
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();

        let hash = typed_data.eip712_signing_hash().unwrap();
        assert_eq!(
            "232cd3ec058eb935a709f093e3536ce26cc9e8e193584b0881992525f6236eef",
            hex::encode(&hash[..])
        );
    }

    #[test]
    fn test_hash_custom_data_type() {
        let json = serde_json::json!({
            "domain":{},
            "types":{
                "EIP712Domain":[],
                "Person":[
                    {"name":"name","type":"string"},{"name":"wallet","type":"address"}],
                "Mail":[
                    {"name":"from","type":"Person"},
                    {"name":"to","type":"Person"},
                    {"name":"contents","type":"string"}
                ]
            },
            "primaryType":"Mail",
            "message":{
                "from":{
                    "name":"Cow","wallet":"0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                },
                "to":{
                    "name":"Bob","wallet":"0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                },
                "contents":"Hello, Bob!"
            }
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();
        let hash = typed_data.eip712_signing_hash().unwrap();
        assert_eq!(
            "25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775",
            hex::encode(&hash[..])
        );
    }

    #[test]
    fn test_hash_recursive_types() {
        let json = serde_json::json!( {
          "domain": {},
          "types": {
            "EIP712Domain": [],
            "Person": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "wallet",
                "type": "address"
              }
            ],
            "Mail": [
              {
                "name": "from",
                "type": "Person"
              },
              {
                "name": "to",
                "type": "Person"
              },
              {
                "name": "contents",
                "type": "string"
              },
              {
                "name": "replyTo",
                "type": "Mail"
              }
            ]
          },
          "primaryType": "Mail",
          "message": {
            "from": {
              "name": "Cow",
              "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
            },
            "to": {
              "name": "Bob",
              "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
            },
            "contents": "Hello, Bob!",
            "replyTo": {
              "to": {
                "name": "Cow",
                "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
              },
              "from": {
                "name": "Bob",
                "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
              },
              "contents": "Hello!"
            }
          }
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();

        assert_eq!(
            typed_data.eip712_signing_hash(),
            Err(DynAbiError::CircularDependency("Mail".into()))
        );
    }

    #[test]
    fn test_hash_nested_struct_array() {
        let json = serde_json::json!({
          "types": {
            "EIP712Domain": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "version",
                "type": "string"
              },
              {
                "name": "chainId",
                "type": "uint256"
              },
              {
                "name": "verifyingContract",
                "type": "address"
              }
            ],
            "OrderComponents": [
              {
                "name": "offerer",
                "type": "address"
              },
              {
                "name": "zone",
                "type": "address"
              },
              {
                "name": "offer",
                "type": "OfferItem[]"
              },
              {
                "name": "startTime",
                "type": "uint256"
              },
              {
                "name": "endTime",
                "type": "uint256"
              },
              {
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "name": "salt",
                "type": "uint256"
              },
              {
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "name": "counter",
                "type": "uint256"
              }
            ],
            "OfferItem": [
              {
                "name": "token",
                "type": "address"
              }
            ],
            "ConsiderationItem": [
              {
                "name": "token",
                "type": "address"
              },
              {
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "name": "endAmount",
                "type": "uint256"
              },
              {
                "name": "recipient",
                "type": "address"
              }
            ]
          },
          "primaryType": "OrderComponents",
          "domain": {
            "name": "Seaport",
            "version": "1.1",
            "chainId": "1",
            "verifyingContract": "0x00000000006c3852cbEf3e08E8dF289169EdE581"
          },
          "message": {
            "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "offer": [
              {
                "token": "0xA604060890923Ff400e8c6f5290461A83AEDACec"
              }
            ],
            "startTime": "1658645591",
            "endTime": "1659250386",
            "zone": "0x004C00500000aD104D7DBd00e3ae0A5C00560C00",
            "zoneHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "salt": "16178208897136618",
            "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
            "totalOriginalConsiderationItems": "2",
            "counter": "0"
          }
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();

        let hash = typed_data.eip712_signing_hash().unwrap();
        assert_eq!(
            "0b8aa9f3712df0034bc29fe5b24dd88cfdba02c7f499856ab24632e2969709a8",
            hex::encode(&hash[..])
        );
    }

    sol!(
      /// Fancy struct
      #[derive(serde::Serialize, serde::Deserialize)]
      struct MyStruct {
        string name;
        string otherThing;
      }
    );

    #[test]
    fn from_sol_struct() {
        let s = MyStruct {
            name: "hello".to_string(),
            otherThing: "world".to_string(),
        };

        let typed_data = TypedData::from_struct(&s, None);
        assert_eq!(
            typed_data.encode_type().unwrap(),
            "MyStruct(string name,string otherThing)"
        );
    }

    sol! {
      #[derive(serde::Serialize, serde::Deserialize)]
      struct Person {
        string name;
        address wallet;
      }
    }

    sol! {
      #[derive(serde::Serialize, serde::Deserialize)]
      struct Mail {
        Person from;
        Person to;
        string contents;
      }
    }

    #[test]
    fn e2e_from_sol_struct() {
        let sender = Person {
            name: "Cow".to_string(),
            wallet: "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                .parse()
                .unwrap(),
        };
        let recipient = Person {
            name: "Bob".to_string(),
            wallet: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                .parse()
                .unwrap(),
        };
        let mail = Mail {
            from: sender,
            to: recipient,
            contents: "Hello, Bob!".to_string(),
        };

        let typed_data = TypedData::from_struct(&mail, None);

        let hash = typed_data.eip712_signing_hash().unwrap();
        assert_eq!(
            "25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775",
            hex::encode(&hash[..])
        );
    }
}
