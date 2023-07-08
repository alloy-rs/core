use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};
use alloy_sol_type_str::TypeSpecifier;
use core::fmt;
use serde::{de::Unexpected, Deserialize, Deserializer, Serialize, Serializer};

use crate::utils::{validate_identifier, validate_ty};

/// JSON specification of a parameter.
///
/// Parameters are the inputs and outputs of [Function]s, and the fields of
/// [Error]s.
///
/// [Function]: crate::Function
/// [Error]: crate::Error
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Param {
    /// The name of the parameter. This field always contains either the empty
    /// string, or a valid Solidity identifier.
    pub name: String,
    /// The canonical Solidity type of the parameter, using the word "tuple" to
    /// represent complex types. E.g. `uint256` or `bytes[2]` or `tuple` or
    /// `tuple[2]`. This field always contains a valid
    /// [`alloy_sol_type_str::TypeSpecifier`].
    pub ty: String,
    /// If the paramaeter is a compound type (a struct or tuple), a list of the
    /// parameter's components, in order. Empty otherwise
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
                validate_identifier!(inner.name);
                validate_ty!(inner.ty);
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
    /// True if the parameter is a UDT (user-defined type).
    ///
    /// A UDT will have
    /// - an internal type that does not match its canonical type
    /// - no space in its internal type (as it does not have a keyword prefix)
    pub fn is_udt(&self) -> bool {
        self.internal_type()
            .map(|ty| self.is_simple_type() && !ty.contains(' ') && ty != self.ty)
            .unwrap_or_default()
    }

    /// True if the parameter is a struct.
    pub fn is_struct(&self) -> bool {
        self.internal_type
            .as_ref()
            .map(|t| t.contains("struct "))
            .unwrap_or_default()
    }

    /// True if the parameter is an enum.
    pub fn is_enum(&self) -> bool {
        self.internal_type
            .as_ref()
            .map(|t| t.contains("enum "))
            .unwrap_or_default()
    }

    /// True if the parameter is a contract.
    pub fn is_contract(&self) -> bool {
        self.internal_type
            .as_ref()
            .map(|t| t.contains("contract "))
            .unwrap_or_default()
    }

    /// Borrow the internal type, if any
    pub fn internal_type(&self) -> Option<&str> {
        self.internal_type.as_deref()
    }

    /// The UDT specifier is a [`TypeSpecifier`] containing the UDT name and any
    /// array sizes. It is computed from the `internal_type`. If this param is
    /// not a UDT, this function will return `None`.
    pub fn udt_specifier(&self) -> Option<TypeSpecifier<'_>> {
        // UDTs are more annoying to check for, so we reuse logic here.
        if !self.is_udt() {
            return None
        }
        TypeSpecifier::try_from(self.internal_type()?).ok()
    }

    /// The struct specifier is a [`TypeSpecifier`] containing the struct name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    pub fn struct_specifier(&self) -> Option<TypeSpecifier<'_>> {
        let spec = TypeSpecifier::try_from(self.internal_type()?).ok()?;
        if spec.keyword != Some("struct") {
            return None
        }
        Some(spec)
    }
    /// The enum specifier is a [`TypeSpecifier`] containing the enum name and
    /// any array sizes. It is computed from the `internal_type`. If this param
    /// is not a enum, this function will return `None`.
    pub fn enum_specifier(&self) -> Option<TypeSpecifier<'_>> {
        let spec = TypeSpecifier::try_from(self.internal_type()?).ok()?;
        if spec.keyword != Some("enum") {
            return None
        }
        Some(spec)
    }
    /// The struct specifier is a [`TypeSpecifier`] containing the contract name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    pub fn contract_specifier(&self) -> Option<TypeSpecifier<'_>> {
        let spec = TypeSpecifier::try_from(self.internal_type()?).ok()?;
        if spec.keyword != Some("contract") {
            return None
        }
        Some(spec)
    }

    /// True if the type is simple
    #[inline]
    pub fn is_simple_type(&self) -> bool {
        self.components.is_empty()
    }

    /// True if the type is complex (tuple or struct)
    #[inline]
    pub fn is_complex_type(&self) -> bool {
        !self.components.is_empty()
    }

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
    /// The name of the parameter. This field always contains either the empty
    /// string, or a valid Solidity identifier.
    pub name: String,
    /// The canonical Solidity type of the parameter, using the word "tuple" to
    /// represent complex types. E.g. `uint256` or `bytes[2]` or `tuple` or
    /// `tuple[2]`. This field always contains a valid
    /// [`alloy_sol_type_str::TypeSpecifier`].
    pub ty: String,
    /// Whether the parameter is indexed. Indexed parameters have their
    /// value, or the hash of their value, stored in the log topics.
    pub indexed: bool,
    /// If the paramaeter is a compound type (a struct or tuple), a list of the
    /// parameter's components, in order. Empty otherwise. Because the
    /// components are not top-level event params, they will not have an
    /// `indexed` field.
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
                validate_identifier!(gp.name);
                validate_ty!(gp.ty);
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
    /// True if the parameter is a UDT (user-defined type).
    ///
    /// A UDT will have
    /// - an internal type that does not match its canonical type
    /// - no space in its internal type (as it does not have a keyword prefix)
    pub fn is_udt(&self) -> bool {
        self.internal_type()
            .map(|ty| self.is_simple_type() && !ty.contains(' ') && ty != self.ty)
            .unwrap_or_default()
    }

    /// True if the parameter is a struct.
    pub fn is_struct(&self) -> bool {
        self.internal_type
            .as_ref()
            .map(|t| t.contains("struct "))
            .unwrap_or_default()
    }

    /// True if the parameter is an enum.
    pub fn is_enum(&self) -> bool {
        self.internal_type
            .as_ref()
            .map(|t| t.contains("enum "))
            .unwrap_or_default()
    }

    /// True if the parameter is a contract.
    pub fn is_contract(&self) -> bool {
        self.internal_type
            .as_ref()
            .map(|t| t.contains("contract "))
            .unwrap_or_default()
    }

    /// Borrow the internal type, if any
    pub fn internal_type(&self) -> Option<&str> {
        self.internal_type.as_deref()
    }

    /// The UDT specifier is a [`TypeSpecifier`] containing the UDT name and any
    /// array sizes. It is computed from the `internal_type`. If this param is
    /// not a UDT, this function will return `None`.
    pub fn udt_specifier(&self) -> Option<TypeSpecifier<'_>> {
        // UDTs are more annoying to check for, so we reuse logic here.
        if !self.is_udt() {
            return None
        }
        TypeSpecifier::try_from(self.internal_type()?).ok()
    }

    /// The struct specifier is a [`TypeSpecifier`] containing the struct name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    pub fn struct_specifier(&self) -> Option<TypeSpecifier<'_>> {
        let spec = TypeSpecifier::try_from(self.internal_type()?).ok()?;
        if spec.keyword != Some("struct") {
            return None
        }
        Some(spec)
    }

    /// The enum specifier is a [`TypeSpecifier`] containing the enum name and
    /// any array sizes. It is computed from the `internal_type`. If this param
    /// is not a enum, this function will return `None`.
    pub fn enum_specifier(&self) -> Option<TypeSpecifier<'_>> {
        let spec = TypeSpecifier::try_from(self.internal_type()?).ok()?;
        if spec.keyword != Some("enum") {
            return None
        }
        Some(spec)
    }

    /// The struct specifier is a [`TypeSpecifier`] containing the contract name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    pub fn contract_specifier(&self) -> Option<TypeSpecifier<'_>> {
        let spec = TypeSpecifier::try_from(self.internal_type()?).ok()?;
        if spec.keyword != Some("contract") {
            return None
        }
        Some(spec)
    }
    /// True if the type is simple
    #[inline]
    pub fn is_simple_type(&self) -> bool {
        self.components.is_empty()
    }

    /// True if the type is complex (tuple or struct)
    #[inline]
    pub fn is_complex_type(&self) -> bool {
        !self.components.is_empty()
    }

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

#[derive(Deserialize, Serialize, Debug)]
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
