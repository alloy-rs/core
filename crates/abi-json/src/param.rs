/// A simple parameter. Simple params are not compound types, and have no
/// sub-components.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl SimpleParam {
    /// Type used to encode the preimage of the function or error selector, or
    /// event topic
    pub fn selector_type(&self) -> String {
        self.ty.clone()
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in function signatures
    pub fn as_function_param(&self) -> String {
        format!("{} {}", self.internal_type, self.name)
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in EIP-712 [encodeType] strings
    ///
    /// [encodeType]: https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
    pub fn as_eip712_param(&self) -> String {
        format!("{} {}", self.internal_type, self.name)
    }
}

/// JSON specification of a complex parameter. Complex params are compound
/// types, and their components are specified in the `components` field.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl ComplexParam {
    /// Type used to encode the preimage of the function or error selector, or
    /// event topic
    pub fn selector_type(&self) -> String {
        format!(
            "({})",
            self.components
                .iter()
                .map(|p| p.selector_type())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in function signatures
    pub fn as_function_param(&self) -> String {
        format!("{} {}", self.internal_type, self.name)
    }

    /// Returns a string representation of the parameter that can be used as a
    /// parameter in EIP-712 [encodeType] strings
    ///
    /// [encodeType]: https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
    pub fn as_eip712_param(&self) -> String {
        format!("{} {}", self.internal_type, self.name)
    }
}

/// JSON specification of a parameter. Used in functions, errors, structs, etc.
/// A parameter may be either simple (contains no sub-components) or complex
/// (contains sub-components).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub fn selector_type(&self) -> String {
        match self {
            Param::Complex(c) => c.selector_type(),
            Param::Simple(s) => s.selector_type(),
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
