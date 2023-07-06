//! Contains utilities for parsing Solidity types.
//!
//! This is a simple representation of Solidity type grammar.

use alloy_sol_type_str::{RootType, TupleSpecifier, TypeSpecifier, TypeStem};

use crate::{DynAbiError, DynSolType};

pub(crate) trait Resolve {
    /// Resolve the type string into a Solidity type.
    fn resolve(&self) -> Result<DynSolType, DynAbiError>;
}

impl Resolve for RootType<'_> {
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        self.try_basic_solidity()?;

        let s = self.as_str();
        if let Some(s) = s.strip_prefix("int") {
            let len = s.trim().parse::<usize>().unwrap_or(256);
            return Ok(DynSolType::Int(len))
        }

        if let Some(s) = s.strip_prefix("uint") {
            let len = s.trim().parse::<usize>().unwrap_or(256);
            return Ok(DynSolType::Uint(len))
        }

        match s {
            "address" => return Ok(DynSolType::Address),
            "bool" => return Ok(DynSolType::Bool),
            "string" => return Ok(DynSolType::String),
            "bytes" => return Ok(DynSolType::Bytes),
            _ => {}
        }

        // This block must come after the match statement, as the `bytes`
        // prefix is shared between two types
        if let Some(s) = s.strip_prefix("bytes") {
            return Ok(DynSolType::FixedBytes(
                s.trim()
                    .parse()
                    .map_err(|_| alloy_sol_type_str::Error::invalid_size(s))?,
            ))
        }
        Err(DynAbiError::missing_type(s))
    }
}

impl Resolve for TupleSpecifier<'_> {
    /// Resolve the type string into a basic Solidity type if possible.
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        let tuple = self
            .types
            .iter()
            .map(|ty| ty.resolve())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DynSolType::Tuple(tuple))
    }
}

impl Resolve for TypeStem<'_> {
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        match self {
            Self::Root(root) => root.resolve(),
            Self::Tuple(tuple) => tuple.resolve(),
        }
    }
}

impl Resolve for TypeSpecifier<'_> {
    /// Resolve the type string into a basic Solidity type if possible.
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        let ty = self.stem.resolve()?;
        Ok(self.sizes.iter().fold(ty, |acc, item| match item {
            Some(size) => DynSolType::FixedArray(Box::new(acc), *size),
            _ => DynSolType::Array(Box::new(acc)),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Result<DynSolType, DynAbiError> {
        let s = s.trim();
        let ty = TypeSpecifier::try_from(s)?;

        ty.resolve()
    }

    #[test]
    fn it_resolves_tuples() {
        assert_eq!(
            parse("(bool,)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Bool])
        );
        assert_eq!(
            parse("(uint256,uint256)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])
        );
        assert_eq!(
            parse("(uint256,uint256)[2]").unwrap(),
            DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![
                    DynSolType::Uint(256),
                    DynSolType::Uint(256)
                ])),
                2
            )
        );
    }

    #[test]
    fn nested_tuples() {
        assert_eq!(
            parse("(bool,(uint256,uint256))").unwrap(),
            DynSolType::Tuple(vec![
                DynSolType::Bool,
                DynSolType::Tuple(vec![DynSolType::Uint(256), DynSolType::Uint(256)])
            ])
        );
        assert_eq!(
            parse("(((bool),),)").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Bool
            ])])])
        );
    }

    #[test]
    fn empty_tuples() {
        assert_eq!(parse("()").unwrap(), DynSolType::Tuple(vec![]));
        assert_eq!(
            parse("((),())").unwrap(),
            DynSolType::Tuple(vec![DynSolType::Tuple(vec![]), DynSolType::Tuple(vec![])])
        );
        assert_eq!(
            parse("((()))"),
            Ok(DynSolType::Tuple(vec![DynSolType::Tuple(vec![
                DynSolType::Tuple(vec![])
            ])]))
        );
    }

    #[test]
    fn it_parses_simple_types() {
        assert_eq!(parse("uint256").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("uint8").unwrap(), DynSolType::Uint(8));
        assert_eq!(parse("uint").unwrap(), DynSolType::Uint(256));
        assert_eq!(parse("address").unwrap(), DynSolType::Address);
        assert_eq!(parse("bool").unwrap(), DynSolType::Bool);
        assert_eq!(parse("string").unwrap(), DynSolType::String);
        assert_eq!(parse("bytes").unwrap(), DynSolType::Bytes);
        assert_eq!(parse("bytes32").unwrap(), DynSolType::FixedBytes(32));
    }

    #[test]
    fn it_parses_complex_solidity_types() {
        assert_eq!(
            parse("uint256[]").unwrap(),
            DynSolType::Array(Box::new(DynSolType::Uint(256)))
        );
        assert_eq!(
            parse("uint256[2]").unwrap(),
            DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2)
        );
        assert_eq!(
            parse("uint256[2][3]").unwrap(),
            DynSolType::FixedArray(
                Box::new(DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 2)),
                3
            )
        );
        assert_eq!(
            parse("uint256[][][]").unwrap(),
            DynSolType::Array(Box::new(DynSolType::Array(Box::new(DynSolType::Array(
                Box::new(DynSolType::Uint(256))
            )))))
        );

        assert_eq!(
            parse(r#"tuple(address,bytes, (bool, (string, uint256)[][3]))[2]"#),
            Ok(DynSolType::FixedArray(
                Box::new(DynSolType::Tuple(vec![
                    DynSolType::Address,
                    DynSolType::Bytes,
                    DynSolType::Tuple(vec![
                        DynSolType::Bool,
                        DynSolType::FixedArray(
                            Box::new(DynSolType::Array(Box::new(DynSolType::Tuple(vec![
                                DynSolType::String,
                                DynSolType::Uint(256)
                            ])))),
                            3
                        ),
                    ]),
                ])),
                2
            ))
        );
    }

    #[test]
    fn try_basic_solidity() {
        assert_eq!(
            TypeSpecifier::try_from("uint256")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("uint256[]")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("(uint256,uint256)")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("(uint256,uint256)[2]")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from("tuple(uint256,uint256)")
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
        assert_eq!(
            TypeSpecifier::try_from(r#"tuple(address,bytes, (bool, (string, uint256)[][3]))[2]"#)
                .unwrap()
                .try_basic_solidity(),
            Ok(())
        );
    }

    #[test]
    fn not_basic_solidity() {
        assert_eq!(
            TypeSpecifier::try_from("MyStruct")
                .unwrap()
                .try_basic_solidity(),
            Err(alloy_sol_type_str::Error::invalid_type_string("MyStruct").into())
        );
    }
}
