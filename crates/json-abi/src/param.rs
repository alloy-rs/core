use alloc::{borrow::Cow, string::String, vec::Vec};
use core::fmt;
use serde::{Deserialize, Serialize};

macro_rules! as_param_string {
    ($self:expr) => {{
        let mut s = String::with_capacity($self.internal_type.len() + $self.name.len() + 1);
        s.push_str(&$self.internal_type);
        s.push(' ');
        s.push_str(&$self.name);
        s
    }};

    ($self:expr, $f:expr) => {{
        $f.write_str(&$self.internal_type)?;
        $f.write_str(" ")?;
        $f.write_str(&$self.name)
    }};
}

/// A simple parameter. Simple params are not compound types, and have no
/// sub-components.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimpleParam {
    /// The name of the parameter.
    pub name: String,
    /// The Solidity type of the parameter.
    #[serde(rename = "type")]
    pub ty: String,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    #[serde(rename = "internalType")]
    pub internal_type: String,
}

impl fmt::Display for SimpleParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        as_param_string!(self, f)
    }
}

impl SimpleParam {
    /// Type used to encode the preimage of the function or error selector, or
    /// event topic
    pub fn selector_type(&self) -> &str {
        &self.ty
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in function signatures
    pub fn as_function_param(&self) -> String {
        as_param_string!(self)
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in EIP-712 [encodeType] strings
    ///
    /// [encodeType]: https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
    pub fn as_eip712_param(&self) -> String {
        as_param_string!(self)
    }
}

/// JSON specification of a complex parameter. Complex params are compound
/// types, and their components are specified in the `components` field.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComplexParam {
    /// The name of the parameter.
    pub name: String,
    /// The Solidity type of the parameter.
    #[serde(rename = "type")]
    pub ty: String,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    #[serde(rename = "internalType")]
    pub internal_type: String,
    /// A list of the parameter's components, in order.
    pub components: Vec<Param>,
}

impl fmt::Display for ComplexParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        as_param_string!(self, f)
    }
}

impl ComplexParam {
    /// Type used to encode the preimage of the function or error selector, or
    /// event topic
    pub fn selector_type(&self) -> String {
        let mut s = String::with_capacity(2 + self.components.len() * 32);
        s.push('(');
        for component in &self.components {
            s.push_str(&component.selector_type());
        }
        s.push(')');
        s
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in function signatures
    pub fn as_function_param(&self) -> String {
        as_param_string!(self)
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in EIP-712 [encodeType] strings
    ///
    /// [encodeType]: https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
    pub fn as_eip712_param(&self) -> String {
        as_param_string!(self)
    }
}

/// JSON specification of a parameter. Used in functions, errors, structs, etc.
/// A parameter may be either simple (contains no sub-components) or complex
/// (contains sub-components).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Param {
    /// [`ComplexParam`] variant
    Complex(ComplexParam),
    /// [`SimpleParam`] variant
    Simple(SimpleParam),
}

impl Param {
    /// Type used to encode the preimage of the function or error selector, or
    /// event topic
    pub fn selector_type(&self) -> Cow<'_, str> {
        match self {
            Param::Complex(c) => c.selector_type().into(),
            Param::Simple(s) => s.selector_type().into(),
        }
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in function signatures
    pub fn as_function_param(&self) -> String {
        match self {
            Param::Complex(c) => c.as_function_param(),
            Param::Simple(s) => s.as_function_param(),
        }
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in EIP-712 [encodeType] strings
    ///
    /// [encodeType]: https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
    pub fn as_eip712_param(&self) -> String {
        match self {
            Param::Complex(c) => c.as_eip712_param(),
            Param::Simple(s) => s.as_eip712_param(),
        }
    }
}

#[cfg(test)]
mod test {
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
