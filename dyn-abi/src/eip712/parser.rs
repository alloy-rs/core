use crate::{no_std_prelude::*, DynSolType};

use std::collections::HashMap;

/// Error when parsing EIP-712 `encodeType` strings
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parse712Error {
    /// Invalid type string, extra chars, or invalid structure
    InvalidTypeString,
    /// Unknown type referenced from another type
    MissingType(Cow<'static, str>),
    /// Detected circular dep during typegraph resolution
    CircularDependency { type_1: Cow<'static, str> },
}

impl Parse712Error {
    pub(crate) fn missing_type(name: &str) -> Parse712Error {
        Parse712Error::MissingType(name.to_owned().into())
    }

    pub(crate) fn circular_dependency(type_1: &str) -> Parse712Error {
        Parse712Error::CircularDependency {
            type_1: type_1.to_owned().into(),
        }
    }
}

/// A type is a name and then a parenthesized list of types. This function pops
/// the first type off a concatenated string of types.
fn get_type(input: &str) -> Option<&str> {
    let mut depth = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&input[..i + 1]);
                }
            }
            _ => (),
        }
    }
    return None;
}

/// Break a eip712 type string into a list of type strings.
fn get_types(input: &str) -> Result<Vec<&str>, Parse712Error> {
    let mut types = vec![];
    let mut remaining = input;
    while let Some(t) = get_type(remaining) {
        types.push(t);
        remaining = &remaining[t.len()..];
    }
    if types.iter().map(|s| s.len()).sum::<usize>() != input.len() {
        dbg!(input);

        return Err(Parse712Error::InvalidTypeString);
    }
    return Ok(types);
}

/// Parse an EIP-712 `encodeType` type string into a `DynSolType::CustomStruct`.
///
/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
pub(crate) fn parse_structs(input: &str) -> Result<HashMap<String, DynSolType>, Parse712Error> {
    let mut types: HashMap<String, DynSolType> = HashMap::new();
    get_types(input)?.iter().rev().try_for_each(|t| {
        let (name, remaining) = t.split_once('(').unwrap();
        let remaining = remaining.strip_suffix(')').unwrap();
        let mut tuple = vec![];
        let mut prop_names = vec![];

        remaining.split(',').try_for_each(|t| {
            let (ty, name) = t.split_once(' ').unwrap();
            if let Ok(ty) = ty.parse() {
                prop_names.push(name.to_owned());
                tuple.push(ty);
            } else if let Some(ty) = types.get(ty) {
                prop_names.push(name.to_owned());
                tuple.push(ty.clone());
            } else {
                return Err(Parse712Error::missing_type(ty));
            }
            Ok(())
        })?;

        let ty = DynSolType::CustomStruct {
            name: name.to_owned(),
            prop_names,
            tuple,
        };

        types.insert(name.to_owned(), ty);
        Ok(())
    })?;
    Ok(types)
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE: &str = "Transaction(Person from,Person to,Asset tx)Asset(address token,uint256 amount)Person(address wallet,string name)";

    #[test]
    fn test_get_type() {
        assert_eq!(
            get_type("Transaction(Person from,Person to,Asset tx)"),
            Some("Transaction(Person from,Person to,Asset tx)")
        );
        assert_eq!(
            get_type("Asset(address token,uint256 amount)"),
            Some("Asset(address token,uint256 amount)")
        );
        assert_eq!(
            get_type("Person(address wallet,string name)"),
            Some("Person(address wallet,string name)")
        );
        assert_eq!(
            get_type("Person(address wallet,string name)Asset(address token,uint256 amount)"),
            Some("Person(address wallet,string name)")
        );
        assert_eq!(get_type("Person(address wallet,string name)Asset(address token,uint256 amount)Transaction(Person from,Person to,Asset tx)"), Some("Person(address wallet,string name)"));
    }

    #[test]
    fn test_get_types() {
        assert_eq!(
            get_types(EXAMPLE),
            Ok(vec![
                "Transaction(Person from,Person to,Asset tx)",
                "Asset(address token,uint256 amount)",
                "Person(address wallet,string name)"
            ])
        );
    }

    #[test]
    fn test_parse_structs() {
        let types = parse_structs(EXAMPLE).unwrap();

        let person = DynSolType::CustomStruct {
            name: "Person".to_owned(),
            prop_names: vec!["wallet".to_owned(), "name".to_owned()],
            tuple: vec![DynSolType::Address, DynSolType::String],
        };

        let asset = DynSolType::CustomStruct {
            name: "Asset".to_owned(),
            prop_names: vec!["token".to_owned(), "amount".to_owned()],
            tuple: vec![DynSolType::Address, DynSolType::Uint(256)],
        };

        let transaction = DynSolType::CustomStruct {
            name: "Transaction".to_owned(),
            prop_names: vec!["from".to_owned(), "to".to_owned(), "tx".to_owned()],
            tuple: vec![person.clone(), person.clone(), asset.clone()],
        };

        assert_eq!(types.get("Transaction"), Some(&transaction));
        assert_eq!(types.get("Asset"), Some(&asset));
        assert_eq!(types.get("Person"), Some(&person));
    }
}
