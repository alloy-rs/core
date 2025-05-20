//! EIP-712 specific parsing structures.

// TODO: move to `sol-type-parser`

use crate::{
    eip712::resolver::{PropertyDef, TypeDef},
    Error,
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use parser::{Error as TypeParserError, TypeSpecifier};

use super::Resolver;

/// A property is a type and a name. Of the form `type name`. E.g.
/// `uint256 foo` or `(MyStruct[23],bool) bar`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropDef<'a> {
    /// The prop type specifier.
    pub ty: TypeSpecifier<'a>,
    /// The prop name.
    pub name: &'a str,
}

impl PropDef<'_> {
    /// Convert to an owned `PropertyDef`
    pub fn to_owned(&self) -> PropertyDef {
        PropertyDef::new(self.ty.span, self.name).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for PropDef<'a> {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl<'a> PropDef<'a> {
    /// Parse a string into property definition.
    pub fn parse(input: &'a str) -> Result<Self, Error> {
        let (ty, name) =
            input.rsplit_once(' ').ok_or_else(|| Error::invalid_property_def(input))?;
        Ok(PropDef { ty: ty.trim().try_into()?, name: name.trim() })
    }
}

/// Represents a single component type in an EIP-712 `encodeType` type string.
///
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentType<'a> {
    /// The span.
    pub span: &'a str,
    /// The name of the component type.
    pub type_name: &'a str,
    /// Properties of the component type.
    pub props: Vec<PropDef<'a>>,
}

impl<'a> TryFrom<&'a str> for ComponentType<'a> {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl<'a> ComponentType<'a> {
    /// Parse a string into a component type.
    pub fn parse(input: &'a str) -> Result<Self, Error> {
        let (name, props_str) = input
            .split_once('(')
            .ok_or_else(|| Error::TypeParser(TypeParserError::invalid_type_string(input)))?;

        let mut props = vec![];
        let mut depth = 1; // 1 to account for the ( in the split above
        let mut last = 0;

        for (i, c) in props_str.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        let candidate = &props_str[last..i];
                        if !candidate.is_empty() {
                            props.push(candidate.try_into()?);
                        }
                        last = i + c.len_utf8();
                        break;
                    }
                }
                ',' => {
                    if depth == 1 {
                        props.push(props_str[last..i].try_into()?);
                        last = i + c.len_utf8();
                    }
                }
                _ => {}
            }
        }

        Ok(Self { span: &input[..last + name.len() + 1], type_name: name.trim(), props })
    }

    /// Convert to an owned TypeDef.
    pub fn to_owned(&self) -> TypeDef {
        TypeDef::new(self.type_name, self.props.iter().map(|p| p.to_owned()).collect()).unwrap()
    }
}

/// Represents a list of component types in an EIP-712 `encodeType` type string.
#[derive(Debug, PartialEq, Eq)]
pub struct EncodeType<'a> {
    /// The list of component types.
    pub types: Vec<ComponentType<'a>>,
}

impl<'a> TryFrom<&'a str> for EncodeType<'a> {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl<'a> EncodeType<'a> {
    /// Parse a string into a list of component types.
    pub fn parse(input: &'a str) -> Result<Self, Error> {
        let mut types = vec![];
        let mut remaining = input;

        while let Ok(t) = ComponentType::parse(remaining) {
            remaining = &remaining[t.span.len()..];
            types.push(t);
        }

        Ok(Self { types })
    }

    /// Computes the canonical string representation of the type.
    ///
    /// Orders the `ComponentTypes` based on the EIP-712 rules, removes unsupported whitespaces, and
    /// validates them.
    pub fn canonicalize(&self) -> Result<String, Error> {
        // Ensure no unintended whitespaces
        let mut resolver = Resolver::default();
        for component_type in &self.types {
            resolver.ingest(component_type.to_owned());
        }

        // Resolve and validate non-dependent types
        let mut non_dependent = resolver.non_dependent_types();

        let first = non_dependent
            .next()
            .ok_or_else(|| Error::MissingType("primary component".to_string()))?;
        if let Some(second) = non_dependent.next() {
            let all_types = vec![first.type_name(), second.type_name()]
                .into_iter()
                .chain(non_dependent.map(|t| t.type_name()))
                .collect::<Vec<_>>()
                .join(", ");

            return Err(Error::MissingType(format!("primary component: {all_types}")));
        };

        let primary = first.type_name();
        _ = resolver.resolve(primary)?;

        // Encode primary type
        resolver.encode_type(primary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CANONICAL: &str = "Transaction(Person from,Person to,Asset tx)Asset(address token,uint256 amount)Person(address wallet,string name)";
    const MISSING_COMPONENT: &str =
        r#"Transaction(Person from, Person to, Asset tx) Person(address wallet, string name)"#;
    const MISSING_PRIMARY: &str =
        r#"Person(address wallet, string name) Asset(address token, uint256 amount)"#;
    const CIRCULAR: &str = r#"
        Transaction(Person from, Person to, Asset tx)
        Asset(Person token, uint256 amount)
        Person(Asset wallet, string name)
        "#;
    const MESSY: &str = r#"
        Person(address wallet, string name)
        Asset(address token, uint256 amount)
        Transaction(Person from, Person to, Asset tx)
        "#;

    #[test]
    fn empty_type() {
        let empty_domain_type =
            ComponentType { span: "EIP712Domain()", type_name: "EIP712Domain", props: vec![] };
        assert_eq!(ComponentType::parse("EIP712Domain()"), Ok(empty_domain_type.clone()));

        assert_eq!(
            EncodeType::try_from("EIP712Domain()"),
            Ok(EncodeType { types: vec![empty_domain_type] })
        );
    }

    #[test]
    fn test_component_type() {
        assert_eq!(
            ComponentType::parse("Transaction(Person from,Person to,Asset tx)"),
            Ok(ComponentType {
                span: "Transaction(Person from,Person to,Asset tx)",
                type_name: "Transaction",
                props: vec![
                    "Person from".try_into().unwrap(),
                    "Person to".try_into().unwrap(),
                    "Asset tx".try_into().unwrap(),
                ],
            })
        );
    }

    #[test]
    fn test_encode_type() {
        assert_eq!(
            EncodeType::parse(CANONICAL),
            Ok(EncodeType {
                types: vec![
                    "Transaction(Person from,Person to,Asset tx)".try_into().unwrap(),
                    "Asset(address token,uint256 amount)".try_into().unwrap(),
                    "Person(address wallet,string name)".try_into().unwrap(),
                ]
            })
        );
        assert_eq!(EncodeType::parse(CANONICAL).unwrap().canonicalize(), Ok(CANONICAL.to_string()));
    }

    #[test]
    fn test_encode_type_messy() {
        assert_eq!(EncodeType::parse(MESSY).unwrap().canonicalize(), Ok(CANONICAL.to_string()));
    }

    #[test]
    fn test_fails_encode_type_missing_type() {
        assert_eq!(
            EncodeType::parse(MISSING_COMPONENT).unwrap().canonicalize(),
            Err(Error::MissingType("Asset".into()))
        );
    }

    #[test]
    fn test_fails_encode_type_multi_primary() {
        assert_eq!(
            EncodeType::parse(MISSING_PRIMARY).unwrap().canonicalize(),
            Err(Error::MissingType("primary component: Asset, Person".into()))
        );
    }

    #[test]
    fn test_fails_encode_type_circular() {
        assert_eq!(
            EncodeType::parse(CIRCULAR).unwrap().canonicalize(),
            Err(Error::CircularDependency("Transaction".into()))
        );
    }
}
