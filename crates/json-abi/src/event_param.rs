use crate::param::Param;
use alloc::{borrow::Cow, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// A Solidity Event parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimpleEventParam {
    /// The name of the parameter.
    pub name: String,
    /// The Solidity type of the parameter.
    #[serde(rename = "type")]
    pub ty: String,
    /// Whether the parameter is indexed. Indexed parameters have their
    /// value, or the hash of their value, stored in the log topics.
    pub indexed: bool,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    #[serde(rename = "internalType")]
    pub internal_type: String,
}

impl SimpleEventParam {
    /// Type used to encode the preimage of the function or error selector, or
    /// event topic
    pub fn selector_type(&self) -> &str {
        &self.ty
    }
}

/// JSON representation of a complex event parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComplexEventParam {
    /// The name of the parameter.
    pub name: String,
    /// The Solidity type of the parameter.
    #[serde(rename = "type")]
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
    #[serde(rename = "internalType")]
    pub internal_type: String,
}

impl ComplexEventParam {
    /// Type used to encode the preimage of the event selector.
    pub fn selector_type(&self) -> String {
        crate::utils::signature("", &self.components)
    }
}

/// A Solidity Event parameter. Event parameters are distinct from function
/// parameters in that they have an `indexed` field.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventParam {
    /// [`ComplexEventParam`] variant
    Complex(ComplexEventParam),
    /// [`SimpleEventParam`] variant
    Simple(SimpleEventParam),
}

impl EventParam {
    /// Type used to encode the preimage of the event selector.
    pub fn selector_type(&self) -> Cow<'_, str> {
        match self {
            Self::Complex(c) => c.selector_type().into(),
            Self::Simple(s) => s.selector_type().into(),
        }
    }
}
