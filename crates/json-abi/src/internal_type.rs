use core::fmt;

use alloc::string::String;

use alloy_sol_type_str::TypeSpecifier;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// The contract internal type. This could be a regular solidity type, a
/// user-defined type, an enum, a struct, a contract, or an address payable.
///
/// The internal type represents the Solidity definition of the type, stripped
/// of the memory or storage keywords. It is used to convey the application dev
/// and user-facing type, while the json param "type" field is used to convey
/// the underlying ABI type.
pub enum InternalType {
    /// Address payable.
    AddressPayable,
    /// Contract.
    Contract(String),
    /// Enum. Possibly of the form `contract.enum`.
    Enum {
        /// Contract qualifier, if any
        contract: Option<String>,
        /// Enum name
        ty: String,
    },
    /// Struct. Possibly of the form `contract.struct`.
    Struct {
        /// Contract qualifier, if any
        contract: Option<String>,
        /// Struct name
        ty: String,
    },
    /// Other.
    Other(String),
}

impl From<BorrowedInternalType<'_>> for InternalType {
    fn from(borrowed: BorrowedInternalType<'_>) -> InternalType {
        match borrowed {
            BorrowedInternalType::AddressPayable => InternalType::AddressPayable,
            BorrowedInternalType::Contract(s) => InternalType::Contract((*s).to_owned()),
            BorrowedInternalType::Enum { contract, ty } => InternalType::Enum {
                contract: contract.map(String::from),
                ty: (*ty).to_owned(),
            },
            BorrowedInternalType::Struct { contract, ty } => InternalType::Struct {
                contract: contract.map(String::from),
                ty: (*ty).to_owned(),
            },
            BorrowedInternalType::Other(s) => InternalType::Other((*s).to_owned()),
        }
    }
}

impl fmt::Display for InternalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_borrowed().fmt(f)
    }
}

impl Serialize for InternalType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_borrowed().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InternalType {
    fn deserialize<D>(deserializer: D) -> Result<InternalType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ItVisitor).map(Into::into)
    }
}

impl InternalType {
    /// True if the instance is a `struct` variant.
    pub fn is_struct(&self) -> bool {
        matches!(self, InternalType::Struct { .. })
    }

    /// True if the instance is a `enum` variant.
    pub fn is_enum(&self) -> bool {
        matches!(self, InternalType::Enum { .. })
    }

    /// True if the instance is a `contract` variant.
    pub fn is_contract(&self) -> bool {
        matches!(self, InternalType::Contract(_))
    }

    /// True if the instance is a `address payable` variant.
    pub fn is_address_payable(&self) -> bool {
        matches!(self, InternalType::AddressPayable)
    }

    /// True if the instance is a `other` variant.
    pub fn is_other(&self) -> bool {
        matches!(self, InternalType::Other(_))
    }

    /// Fallible conversion to a variant.
    pub fn as_struct(&self) -> Option<(Option<&str>, &str)> {
        match self {
            InternalType::Struct { contract, ty } => Some((contract.as_deref(), ty)),
            _ => None,
        }
    }

    /// Fallible conversion to a variant.
    pub fn as_enum(&self) -> Option<(Option<&str>, &str)> {
        match self {
            InternalType::Enum { contract, ty } => Some((contract.as_deref(), ty)),
            _ => None,
        }
    }

    /// Fallible conversion to a variant.
    pub fn as_contract(&self) -> Option<&str> {
        match self {
            InternalType::Contract(s) => Some(s),
            _ => None,
        }
    }

    /// Fallible conversion to a variant.
    pub fn as_other(&self) -> Option<&str> {
        match self {
            InternalType::Other(s) => Some(s),
            _ => None,
        }
    }

    /// Return a [`TypeSpecifier`] describing the struct if this type is a
    /// struct.
    pub fn struct_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_struct()
            .and_then(|s| TypeSpecifier::try_from(s.1).ok())
    }

    /// Return a [`TypeSpecifier`] describing the enum if this type is an enum.
    pub fn enum_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_enum()
            .and_then(|s| TypeSpecifier::try_from(s.1).ok())
    }

    /// Return a [`TypeSpecifier`] describing the contract if this type is a
    /// contract.
    pub fn contract_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_contract()
            .and_then(|s| TypeSpecifier::try_from(s).ok())
    }

    /// Return a [`TypeSpecifier`] describing the other if this type is an
    /// other. An "other" specifier indicates EITHER a regular solidity type OR
    /// a user-defined type. It is not possible to distinguish between the two
    /// without additional context.
    pub fn other_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_other()
            .and_then(|s| TypeSpecifier::try_from(s).ok())
    }

    pub(crate) fn as_borrowed(&self) -> BorrowedInternalType<'_> {
        match self {
            InternalType::AddressPayable => BorrowedInternalType::AddressPayable,
            InternalType::Contract(s) => BorrowedInternalType::Contract(s),
            InternalType::Enum { contract, ty } => BorrowedInternalType::Enum {
                contract: contract.as_deref(),
                ty,
            },
            InternalType::Struct { contract, ty } => BorrowedInternalType::Struct {
                contract: contract.as_deref(),
                ty,
            },
            InternalType::Other(s) => BorrowedInternalType::Other(s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BorrowedInternalType<'a> {
    AddressPayable,
    Contract(&'a str),
    Enum {
        contract: Option<&'a str>,
        ty: &'a str,
    },
    Struct {
        contract: Option<&'a str>,
        ty: &'a str,
    },
    Other(&'a str),
}

impl fmt::Display for BorrowedInternalType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorrowedInternalType::AddressPayable => write!(f, "address payable"),
            BorrowedInternalType::Contract(s) => write!(f, "contract {}", s),
            BorrowedInternalType::Enum { contract, ty } => {
                if let Some(c) = contract {
                    write!(f, "enum {}.{}", c, ty)
                } else {
                    write!(f, "enum {}", ty)
                }
            }
            BorrowedInternalType::Struct { contract, ty } => {
                if let Some(c) = contract {
                    write!(f, "struct {}.{}", c, ty)
                } else {
                    write!(f, "struct {}", ty)
                }
            }
            BorrowedInternalType::Other(s) => write!(f, "{}", s),
        }
    }
}

impl Serialize for BorrowedInternalType<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for BorrowedInternalType<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ItVisitor)
    }
}

pub(crate) struct ItVisitor;

impl<'de> Visitor<'de> for ItVisitor {
    type Value = BorrowedInternalType<'de>;

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(formatter, "a valid internal type")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "address payable" {
            return Ok(BorrowedInternalType::AddressPayable)
        }
        if let Some(body) = v.strip_prefix("enum ") {
            if let Some((contract, ty)) = body.split_once('.') {
                Ok(BorrowedInternalType::Enum {
                    contract: Some(contract),
                    ty,
                })
            } else {
                Ok(BorrowedInternalType::Enum {
                    contract: None,
                    ty: body,
                })
            }
        } else if let Some(body) = v.strip_prefix("struct ") {
            if let Some((contract, ty)) = body.split_once('.') {
                Ok(BorrowedInternalType::Struct {
                    contract: Some(contract),
                    ty,
                })
            } else {
                Ok(BorrowedInternalType::Struct {
                    contract: None,
                    ty: body,
                })
            }
        } else if let Some(body) = v.strip_prefix("contract ") {
            Ok(BorrowedInternalType::Contract(body))
        } else {
            Ok(BorrowedInternalType::Other(v))
        }
    }
}
