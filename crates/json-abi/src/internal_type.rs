use core::fmt;

use alloc::string::{String, ToString};

use alloy_sol_type_parser::TypeSpecifier;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

/// The contract internal type. This could be a regular solidity type, a
/// user-defined type, an enum, a struct, a contract, or an address payable.
///
/// The internal type represents the Solidity definition of the type, stripped
/// of the memory or storage keywords. It is used to convey the application dev
/// and user-facing type, while the json param "type" field is used to convey
/// the underlying ABI type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InternalType {
    /// Address payable.
    AddressPayable(String),
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
    /// Other. Possible of the form `contract.other`.
    Other {
        /// Contract qualifier, if any
        contract: Option<String>,
        /// Struct name
        ty: String,
    },
}

impl From<BorrowedInternalType<'_>> for InternalType {
    #[inline]
    fn from(borrowed: BorrowedInternalType<'_>) -> InternalType {
        match borrowed {
            BorrowedInternalType::AddressPayable(s) => InternalType::AddressPayable(s.to_string()),
            BorrowedInternalType::Contract(s) => InternalType::Contract(s.to_string()),
            BorrowedInternalType::Enum { contract, ty } => InternalType::Enum {
                contract: contract.map(String::from),
                ty: ty.to_string(),
            },
            BorrowedInternalType::Struct { contract, ty } => InternalType::Struct {
                contract: contract.map(String::from),
                ty: ty.to_string(),
            },
            BorrowedInternalType::Other { contract, ty } => InternalType::Other {
                contract: contract.map(String::from),
                ty: ty.to_string(),
            },
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
    /// Parse a string into an instance, taking ownership of data
    #[inline]
    pub fn parse(s: &str) -> Option<Self> {
        BorrowedInternalType::parse(s).map(Into::into)
    }

    /// True if the instance is a `struct` variant.
    #[inline]
    pub const fn is_struct(&self) -> bool {
        matches!(self, InternalType::Struct { .. })
    }

    /// True if the instance is a `enum` variant.
    #[inline]
    pub const fn is_enum(&self) -> bool {
        matches!(self, InternalType::Enum { .. })
    }

    /// True if the instance is a `contract` variant.
    #[inline]
    pub const fn is_contract(&self) -> bool {
        matches!(self, InternalType::Contract(_))
    }

    /// True if the instance is a `address payable` variant.
    #[inline]
    pub const fn is_address_payable(&self) -> bool {
        matches!(self, InternalType::AddressPayable(_))
    }

    /// True if the instance is a `other` variant.
    #[inline]
    pub const fn is_other(&self) -> bool {
        matches!(self, InternalType::Other { .. })
    }

    /// Fallible conversion to a variant.
    #[inline]
    pub fn as_struct(&self) -> Option<(Option<&str>, &str)> {
        match self {
            InternalType::Struct { contract, ty } => Some((contract.as_deref(), ty)),
            _ => None,
        }
    }

    /// Fallible conversion to a variant.
    #[inline]
    pub fn as_enum(&self) -> Option<(Option<&str>, &str)> {
        match self {
            InternalType::Enum { contract, ty } => Some((contract.as_deref(), ty)),
            _ => None,
        }
    }

    /// Fallible conversion to a variant.
    #[inline]
    pub fn as_contract(&self) -> Option<&str> {
        match self {
            InternalType::Contract(s) => Some(s),
            _ => None,
        }
    }

    /// Fallible conversion to a variant.
    #[inline]
    pub fn as_other(&self) -> Option<(Option<&str>, &str)> {
        match self {
            InternalType::Other { contract, ty } => Some((contract.as_deref(), ty)),
            _ => None,
        }
    }

    /// Return a [`TypeSpecifier`] describing the struct if this type is a
    /// struct.
    #[inline]
    pub fn struct_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_struct()
            .and_then(|s| TypeSpecifier::try_from(s.1).ok())
    }

    /// Return a [`TypeSpecifier`] describing the enum if this type is an enum.
    #[inline]
    pub fn enum_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_enum()
            .and_then(|s| TypeSpecifier::try_from(s.1).ok())
    }

    /// Return a [`TypeSpecifier`] describing the contract if this type is a
    /// contract.
    #[inline]
    pub fn contract_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_contract()
            .and_then(|s| TypeSpecifier::try_from(s).ok())
    }

    /// Return a [`TypeSpecifier`] describing the other if this type is an
    /// other. An "other" specifier indicates EITHER a regular solidity type OR
    /// a user-defined type. It is not possible to distinguish between the two
    /// without additional context.
    #[inline]
    pub fn other_specifier(&self) -> Option<TypeSpecifier<'_>> {
        self.as_other()
            .and_then(|s| TypeSpecifier::try_from(s.1).ok())
    }

    #[inline]
    pub(crate) fn as_borrowed(&self) -> BorrowedInternalType<'_> {
        match self {
            InternalType::AddressPayable(s) => BorrowedInternalType::AddressPayable(s),
            InternalType::Contract(s) => BorrowedInternalType::Contract(s),
            InternalType::Enum { contract, ty } => BorrowedInternalType::Enum {
                contract: contract.as_deref(),
                ty,
            },
            InternalType::Struct { contract, ty } => BorrowedInternalType::Struct {
                contract: contract.as_deref(),
                ty,
            },
            InternalType::Other { contract, ty } => BorrowedInternalType::Other {
                contract: contract.as_deref(),
                ty,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BorrowedInternalType<'a> {
    AddressPayable(&'a str),
    Contract(&'a str),
    Enum {
        contract: Option<&'a str>,
        ty: &'a str,
    },
    Struct {
        contract: Option<&'a str>,
        ty: &'a str,
    },
    Other {
        contract: Option<&'a str>,
        ty: &'a str,
    },
}

impl fmt::Display for BorrowedInternalType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorrowedInternalType::AddressPayable(s) => f.write_str(s),
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
            BorrowedInternalType::Other { contract, ty } => {
                if let Some(c) = contract {
                    write!(f, "{}.{}", c, ty)
                } else {
                    write!(f, "{}", ty)
                }
            }
        }
    }
}

impl Serialize for BorrowedInternalType<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for BorrowedInternalType<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ItVisitor)
    }
}

impl<'a> BorrowedInternalType<'a> {
    /// Instantiate a borrowed internal type by parsing a string.
    fn parse(v: &'a str) -> Option<Self> {
        if v.starts_with("address payable") {
            return Some(BorrowedInternalType::AddressPayable(v))
        }
        if let Some(body) = v.strip_prefix("enum ") {
            if let Some((contract, ty)) = body.split_once('.') {
                Some(BorrowedInternalType::Enum {
                    contract: Some(contract),
                    ty,
                })
            } else {
                Some(BorrowedInternalType::Enum {
                    contract: None,
                    ty: body,
                })
            }
        } else if let Some(body) = v.strip_prefix("struct ") {
            if let Some((contract, ty)) = body.split_once('.') {
                Some(BorrowedInternalType::Struct {
                    contract: Some(contract),
                    ty,
                })
            } else {
                Some(BorrowedInternalType::Struct {
                    contract: None,
                    ty: body,
                })
            }
        } else if let Some(body) = v.strip_prefix("contract ") {
            Some(BorrowedInternalType::Contract(body))
        } else if let Some((contract, ty)) = v.split_once('.') {
            Some(BorrowedInternalType::Other {
                contract: Some(contract),
                ty,
            })
        } else {
            Some(BorrowedInternalType::Other {
                contract: None,
                ty: v,
            })
        }
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
        BorrowedInternalType::parse(v).ok_or_else(|| {
            E::invalid_value(serde::de::Unexpected::Str(v), &"a valid internal type")
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parser_test {
        ($test_str:expr, $expected:expr) => {
            assert_eq!(InternalType::parse($test_str).unwrap(), $expected);
        };
    }

    #[test]
    fn parse_simple_internal_types() {
        parser_test!(
            "struct SpentItem[]",
            InternalType::Struct {
                contract: None,
                ty: "SpentItem[]".into()
            }
        );
        parser_test!(
            "struct Contract.Item",
            InternalType::Struct {
                contract: Some("Contract".into()),
                ty: "Item".into()
            }
        );
        parser_test!(
            "enum ItemType[32]",
            InternalType::Enum {
                contract: None,
                ty: "ItemType[32]".into()
            }
        );
        parser_test!(
            "enum Contract.Item",
            InternalType::Enum {
                contract: Some("Contract".into()),
                ty: "Item".into()
            }
        );

        parser_test!("contract Item", InternalType::Contract("Item".into()));
        parser_test!(
            "address payable",
            InternalType::AddressPayable("address payable".to_string())
        );
        parser_test!(
            "address payable[][][][][]",
            InternalType::AddressPayable("address payable[][][][][]".into())
        );
        parser_test!(
            "Item",
            InternalType::Other {
                contract: None,
                ty: "Item".into()
            }
        );
        parser_test!(
            "Contract.Item[][33]",
            InternalType::Other {
                contract: Some("Contract".into()),
                ty: "Item[][33]".into()
            }
        );
    }
}
