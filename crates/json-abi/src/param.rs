use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};
use core::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// JSON specification of a parameter.
///
/// Parameters are the inputs and outputs of [Function]s, and the fields of
/// [Error]s.
///
/// [Function]: crate::Function
/// [Error]: crate::Error
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Param {
    /// The name of the parameter.
    pub name: String,
    /// The canonical Solidity type of the parameter.
    pub ty: String,
    /// A list of the parameter's components, in order.
    pub components: Vec<Param>,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    pub internal_type: Option<String>,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(internal_type) = &self.internal_type {
            f.write_str(internal_type)?;
            f.write_str(" ")?;
        }
        f.write_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for Param {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        BorrowedParam::deserialize(deserializer).and_then(|inner| {
            if inner.indexed.is_none() {
                Ok(Self {
                    name: inner.name.to_owned(),
                    ty: inner.ty.to_owned(),
                    internal_type: inner.internal_type.map(str::to_owned),
                    components: inner.components.into_owned(),
                })
            } else {
                Err(serde::de::Error::custom(
                    "indexed is not supported in params",
                ))
            }
        })
    }
}

impl Serialize for Param {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_inner().serialize(serializer)
    }
}

impl Param {
    /// Formats the canonical type of this parameter into the given string.
    ///
    /// This is used to encode the preimage of a function or error selector.
    pub fn selector_type_raw(&self, s: &mut String) {
        if self.components.is_empty() {
            s.push_str(&self.ty)
        } else {
            crate::utils::signature_raw("", &self.components, s);
        }
    }

    /// Returns the canonical type of this parameter.
    ///
    /// This is used to encode the preimage of a function or error selector.
    pub fn selector_type(&self) -> Cow<'_, str> {
        if self.components.is_empty() {
            Cow::Borrowed(&self.ty)
        } else {
            Cow::Owned(crate::utils::signature("", &self.components))
        }
    }

    #[inline]
    fn as_inner(&self) -> BorrowedParam<'_, Param> {
        BorrowedParam {
            name: &self.name,
            ty: &self.ty,
            indexed: None,
            internal_type: self.internal_type.as_deref(),
            components: Cow::Borrowed(&self.components),
        }
    }
}

/// A Solidity Event parameter.
///
/// Event parameters are distinct from function parameters in that they have an
/// `indexed` field.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EventParam {
    /// The name of the parameter.
    pub name: String,
    /// The canonical Solidity type of the parameter.
    pub ty: String,
    /// Whether the parameter is indexed. Indexed parameters have their
    /// value, or the hash of their value, stored in the log topics.
    pub indexed: bool,
    /// A list of the parameter's components, in order. This is a tuple
    /// definition, and sub-components will NOT have an `indexed` field.
    pub components: Vec<Param>,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    pub internal_type: Option<String>,
}

impl fmt::Display for EventParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(internal_type) = &self.internal_type {
            f.write_str(internal_type)?;
            f.write_str(" ")?;
        }
        f.write_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for EventParam {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        BorrowedParam::deserialize(deserializer).and_then(|gp| {
            if let Some(indexed) = gp.indexed {
                Ok(Self {
                    name: gp.name.to_owned(),
                    ty: gp.ty.to_owned(),
                    indexed,
                    internal_type: gp.internal_type.map(String::from),
                    components: gp.components.into_owned(),
                })
            } else {
                Err(serde::de::Error::custom(
                    "indexed is required in event params",
                ))
            }
        })
    }
}

impl Serialize for EventParam {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_inner().serialize(serializer)
    }
}

impl EventParam {
    /// Formats the canonical type of this parameter into the given string.
    ///
    /// This is used to encode the preimage of the event selector.
    pub fn selector_type_raw(&self, s: &mut String) {
        if self.components.is_empty() {
            s.push_str(&self.ty)
        } else {
            crate::utils::signature_raw("", &self.components, s)
        }
    }

    /// Returns the canonical type of this parameter.
    ///
    /// This is used to encode the preimage of the event selector.
    pub fn selector_type(&self) -> Cow<'_, str> {
        if self.components.is_empty() {
            Cow::Borrowed(&self.ty)
        } else {
            Cow::Owned(crate::utils::signature("", &self.components))
        }
    }

    #[inline]
    fn as_inner(&self) -> BorrowedParam<'_, Param> {
        BorrowedParam {
            name: &self.name,
            ty: &self.ty,
            indexed: Some(self.indexed),
            internal_type: self.internal_type.as_deref(),
            components: Cow::Borrowed(&self.components),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(bound(deserialize = "<[T] as ToOwned>::Owned: Default + Deserialize<'de>"))]
struct BorrowedParam<'a, T: Clone> {
    name: &'a str,
    #[serde(rename = "type")]
    ty: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    indexed: Option<bool>,
    #[serde(
        rename = "internalType",
        default,
        skip_serializing_if = "Option::is_none",
        borrow
    )]
    internal_type: Option<&'a str>,
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    components: Cow<'a, [T]>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_param() {
        let param = r#"{
            "internalType": "string",
            "name": "reason",
            "type": "string"
        }"#;
        let _param = serde_json::from_str::<Param>(param).unwrap();
    }
}
