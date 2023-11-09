use crate::{
    internal_type::BorrowedInternalType,
    utils::{mk_eparam, mk_param, validate_identifier},
    InternalType,
};
use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};
use alloy_sol_type_parser::{
    Error as ParserError, ParameterSpecifier, Result as ParserResult, TypeSpecifier,
};
use core::{fmt, str::FromStr};
use serde::{de::Unexpected, Deserialize, Deserializer, Serialize, Serializer};

/// JSON specification of a parameter.
///
/// Parameters are the inputs and outputs of [Function]s, and the fields of
/// [Error]s.
///
/// [Function]: crate::Function
/// [Error]: crate::Error
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Param {
    /// The canonical Solidity type of the parameter, using the word "tuple" to
    /// represent complex types. E.g. `uint256` or `bytes[2]` or `tuple` or
    /// `tuple[2]`.
    ///
    /// Generally, this is a valid [`TypeSpecifier`], but in very rare
    /// circumstances, such as when a function in a library contains an enum
    /// in its parameters or return types, this will be `Contract.EnumName`
    /// instead of the actual type (`uint8`).
    pub ty: String,
    /// The name of the parameter. This field always contains either the empty
    /// string, or a valid Solidity identifier.
    pub name: String,
    /// If the paramaeter is a compound type (a struct or tuple), a list of the
    /// parameter's components, in order. Empty otherwise
    pub components: Vec<Param>,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the Solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    pub internal_type: Option<InternalType>,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(it) = &self.internal_type { it.fmt(f) } else { f.write_str(&self.ty) }?;
        f.write_str(" ")?;
        f.write_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for Param {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        BorrowedParam::deserialize(deserializer).and_then(|inner| {
            if inner.indexed.is_none() {
                inner.validate_fields()?;
                Ok(Self {
                    name: inner.name.to_owned(),
                    ty: inner.ty.to_owned(),
                    internal_type: inner.internal_type.map(Into::into),
                    components: inner.components.into_owned(),
                })
            } else {
                Err(serde::de::Error::custom("indexed is not supported in params"))
            }
        })
    }
}

impl Serialize for Param {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_inner().serialize(serializer)
    }
}

impl FromStr for Param {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Param {
    /// Parse a parameter from a Solidity parameter string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_json_abi::Param;
    /// assert_eq!(
    ///     Param::parse("uint256[] foo"),
    ///     Ok(Param {
    ///         name: "foo".into(),
    ///         ty: "uint256[]".into(),
    ///         components: vec![],
    ///         internal_type: None,
    ///     })
    /// );
    /// ```
    pub fn parse(input: &str) -> ParserResult<Self> {
        ParameterSpecifier::parse(input).map(|p| mk_param(p.name, p.ty))
    }

    /// The internal type of the parameter.
    #[inline]
    pub const fn internal_type(&self) -> Option<&InternalType> {
        self.internal_type.as_ref()
    }

    /// True if the parameter is a UDT (user-defined type).
    ///
    /// A UDT will have
    /// - an internal type that does not match its canonical type
    /// - no space in its internal type (as it does not have a keyword body)
    ///
    /// Any `Other` specifier will definitely be a UDT if it contains a
    /// contract.
    #[inline]
    pub fn is_udt(&self) -> bool {
        match self.internal_type().and_then(|it| it.as_other()) {
            Some((contract, ty)) => contract.is_some() || (self.is_simple_type() && ty != self.ty),
            _ => false,
        }
    }

    /// True if the parameter is a struct.
    #[inline]
    pub const fn is_struct(&self) -> bool {
        match self.internal_type() {
            Some(ty) => ty.is_struct(),
            None => false,
        }
    }

    /// True if the parameter is an enum.
    #[inline]
    pub const fn is_enum(&self) -> bool {
        match self.internal_type() {
            Some(ty) => ty.is_enum(),
            None => false,
        }
    }

    /// True if the parameter is a contract.
    #[inline]
    pub const fn is_contract(&self) -> bool {
        match self.internal_type() {
            Some(ty) => ty.is_contract(),
            None => false,
        }
    }

    /// The UDT specifier is a [`TypeSpecifier`] containing the UDT name and any
    /// array sizes. It is computed from the `internal_type`. If this param is
    /// not a UDT, this function will return `None`.
    #[inline]
    pub fn udt_specifier(&self) -> Option<TypeSpecifier<'_>> {
        // UDTs are more annoying to check for, so we reuse logic here.
        if !self.is_udt() {
            return None;
        }
        self.internal_type().and_then(|ty| ty.other_specifier())
    }

    /// The struct specifier is a [`TypeSpecifier`] containing the struct name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    #[inline]
    pub fn struct_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.internal_type().and_then(|ty| ty.struct_specifier())
    }

    /// The enum specifier is a [`TypeSpecifier`] containing the enum name and
    /// any array sizes. It is computed from the `internal_type`. If this param
    /// is not a enum, this function will return `None`.
    #[inline]
    pub fn enum_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.internal_type().and_then(|ty| ty.enum_specifier())
    }

    /// The struct specifier is a [`TypeSpecifier`] containing the contract name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    #[inline]
    pub fn contract_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.internal_type().and_then(|ty| ty.contract_specifier())
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
    #[inline]
    pub fn selector_type_raw(&self, s: &mut String) {
        if self.components.is_empty() {
            s.push_str(&self.ty);
        } else {
            crate::utils::signature_raw(&self.components, s);
            // checked during deserialization, but might be invalid from a user
            if let Some(suffix) = self.ty.strip_prefix("tuple") {
                s.push_str(suffix);
            }
        }
    }

    /// Returns the canonical type of this parameter.
    ///
    /// This is used to encode the preimage of a function or error selector.
    #[inline]
    pub fn selector_type(&self) -> Cow<'_, str> {
        if self.components.is_empty() {
            Cow::Borrowed(&self.ty)
        } else {
            let mut s = String::with_capacity(self.components.len() * 32);
            self.selector_type_raw(&mut s);
            Cow::Owned(s)
        }
    }

    #[inline]
    fn borrowed_internal_type(&self) -> Option<BorrowedInternalType<'_>> {
        self.internal_type().as_ref().map(|it| it.as_borrowed())
    }

    #[inline]
    fn as_inner(&self) -> BorrowedParam<'_> {
        BorrowedParam {
            name: &self.name,
            ty: &self.ty,
            indexed: None,
            internal_type: self.borrowed_internal_type(),
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
    /// The canonical Solidity type of the parameter, using the word "tuple" to
    /// represent complex types. E.g. `uint256` or `bytes[2]` or `tuple` or
    /// `tuple[2]`.
    ///
    /// Generally, this is a valid [`TypeSpecifier`], but in very rare
    /// circumstances, such as when a function in a library contains an enum
    /// in its parameters or return types, this will be `Contract.EnumName`
    /// instead of the actual type (`uint8`).
    pub ty: String,
    /// The name of the parameter. This field always contains either the empty
    /// string, or a valid Solidity identifier.
    pub name: String,
    /// Whether the parameter is indexed. Indexed parameters have their
    /// value, or the hash of their value, stored in the log topics.
    pub indexed: bool,
    /// If the paramaeter is a compound type (a struct or tuple), a list of the
    /// parameter's components, in order. Empty otherwise. Because the
    /// components are not top-level event params, they will not have an
    /// `indexed` field.
    pub components: Vec<Param>,
    /// The internal type of the parameter. This type represents the type that
    /// the author of the Solidity contract specified. E.g. for a contract, this
    /// will be `contract MyContract` while the `type` field will be `address`.
    pub internal_type: Option<InternalType>,
}

impl fmt::Display for EventParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(it) = &self.internal_type { it.fmt(f) } else { f.write_str(&self.ty) }?;
        f.write_str(" ")?;
        f.write_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for EventParam {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        BorrowedParam::deserialize(deserializer).and_then(|inner| {
            inner.validate_fields()?;
            Ok(Self {
                name: inner.name.to_owned(),
                ty: inner.ty.to_owned(),
                indexed: inner.indexed.unwrap_or(false),
                internal_type: inner.internal_type.map(Into::into),
                components: inner.components.into_owned(),
            })
        })
    }
}

impl Serialize for EventParam {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_inner().serialize(serializer)
    }
}

impl FromStr for EventParam {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl EventParam {
    /// Parse an event parameter from a Solidity parameter string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use alloy_json_abi::EventParam;
    /// assert_eq!(
    ///     EventParam::parse("uint256[] indexed foo"),
    ///     Ok(EventParam {
    ///         name: "foo".into(),
    ///         ty: "uint256[]".into(),
    ///         indexed: true,
    ///         components: vec![],
    ///         internal_type: None,
    ///     })
    /// );
    /// ```
    #[inline]
    pub fn parse(input: &str) -> ParserResult<Self> {
        ParameterSpecifier::parse(input).map(mk_eparam)
    }

    /// The internal type of the parameter.
    #[inline]
    pub const fn internal_type(&self) -> Option<&InternalType> {
        self.internal_type.as_ref()
    }

    /// True if the parameter is a UDT (user-defined type).
    ///
    /// A UDT will have
    /// - an internal type that does not match its canonical type
    /// - no space in its internal type (as it does not have a keyword body)
    ///
    /// Any `Other` specifier will definitely be a UDT if it contains a
    /// contract.
    #[inline]
    pub fn is_udt(&self) -> bool {
        match self.internal_type().and_then(|it| it.as_other()) {
            Some((contract, ty)) => contract.is_some() || (self.is_simple_type() && ty != self.ty),
            _ => false,
        }
    }

    /// True if the parameter is a struct.
    #[inline]
    pub const fn is_struct(&self) -> bool {
        match self.internal_type() {
            Some(ty) => ty.is_struct(),
            None => false,
        }
    }

    /// True if the parameter is an enum.
    #[inline]
    pub const fn is_enum(&self) -> bool {
        match self.internal_type() {
            Some(ty) => ty.is_enum(),
            None => false,
        }
    }

    /// True if the parameter is a contract.
    #[inline]
    pub const fn is_contract(&self) -> bool {
        match self.internal_type() {
            Some(ty) => ty.is_contract(),
            None => false,
        }
    }

    /// The UDT specifier is a [`TypeSpecifier`] containing the UDT name and any
    /// array sizes. It is computed from the `internal_type`. If this param is
    /// not a UDT, this function will return `None`.
    #[inline]
    pub fn udt_specifier(&self) -> Option<TypeSpecifier<'_>> {
        // UDTs are more annoying to check for, so we reuse logic here.
        if !self.is_udt() {
            return None;
        }
        self.internal_type().and_then(|ty| ty.other_specifier())
    }

    /// The struct specifier is a [`TypeSpecifier`] containing the struct name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    #[inline]
    pub fn struct_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.internal_type().and_then(|ty| ty.struct_specifier())
    }

    /// The enum specifier is a [`TypeSpecifier`] containing the enum name and
    /// any array sizes. It is computed from the `internal_type`. If this param
    /// is not a enum, this function will return `None`.
    #[inline]
    pub fn enum_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.internal_type().and_then(|ty| ty.enum_specifier())
    }

    /// The struct specifier is a [`TypeSpecifier`] containing the contract name
    /// and any array sizes. It is computed from the `internal_type` If this
    /// param is not a struct, this function will return `None`.
    #[inline]
    pub fn contract_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.internal_type().and_then(|ty| ty.contract_specifier())
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
    #[inline]
    pub fn selector_type_raw(&self, s: &mut String) {
        if self.components.is_empty() {
            s.push_str(&self.ty);
        } else {
            crate::utils::signature_raw(&self.components, s);
            // checked during deserialization, but might be invalid from a user
            if let Some(suffix) = self.ty.strip_prefix("tuple") {
                s.push_str(suffix);
            }
        }
    }

    /// Returns the canonical type of this parameter.
    ///
    /// This is used to encode the preimage of the event selector.
    #[inline]
    pub fn selector_type(&self) -> Cow<'_, str> {
        if self.components.is_empty() {
            Cow::Borrowed(&self.ty)
        } else {
            let mut s = String::with_capacity(self.components.len() * 32);
            self.selector_type_raw(&mut s);
            Cow::Owned(s)
        }
    }

    #[inline]
    fn borrowed_internal_type(&self) -> Option<BorrowedInternalType<'_>> {
        self.internal_type().as_ref().map(|it| it.as_borrowed())
    }

    #[inline]
    fn as_inner(&self) -> BorrowedParam<'_> {
        BorrowedParam {
            name: &self.name,
            ty: &self.ty,
            indexed: Some(self.indexed),
            internal_type: self.borrowed_internal_type(),
            components: Cow::Borrowed(&self.components),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct BorrowedParam<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    ty: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    indexed: Option<bool>,
    #[serde(rename = "internalType", default, skip_serializing_if = "Option::is_none")]
    internal_type: Option<BorrowedInternalType<'a>>,
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    components: Cow<'a, [Param]>,
}

impl BorrowedParam<'_> {
    #[inline(always)]
    fn validate_fields<E: serde::de::Error>(&self) -> Result<(), E> {
        validate_identifier!(self.name);

        // any components means type is "tuple" + maybe brackets, so we can skip
        // parsing with TypeSpecifier
        if self.components.is_empty() {
            if alloy_sol_type_parser::TypeSpecifier::parse(self.ty).is_err() {
                return Err(E::invalid_value(
                    Unexpected::Str(self.ty),
                    &"a valid Solidity type specifier",
                ));
            }
        } else {
            // https://docs.soliditylang.org/en/latest/abi-spec.html#handling-tuple-types
            // checking for "tuple" prefix should be enough
            if !self.ty.starts_with("tuple") {
                return Err(E::invalid_value(
                    Unexpected::Str(self.ty),
                    &"a string prefixed with `tuple`, optionally followed by a sequence of `[]` or `[k]` with integers `k`",
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param() {
        let param = r#"{
            "internalType": "string",
            "name": "reason",
            "type": "string"
        }"#;
        let param = serde_json::from_str::<Param>(param).unwrap();
        assert_eq!(
            param,
            Param {
                name: "reason".into(),
                ty: "string".into(),
                internal_type: Some(InternalType::Other { contract: None, ty: "string".into() }),
                components: vec![],
            }
        );
    }
}
