//! Contains utilities for parsing Solidity types.
//!
//! This is a simple representation of Solidity type grammar.

use crate::{DynAbiError, DynAbiResult, DynSolType};
use alloc::vec::Vec;

use alloy_sol_type_str::{
    Error as TypeStrError, RootType, TupleSpecifier, TypeSpecifier, TypeStem,
};

pub(crate) trait Resolve {
    fn resolve(&self) -> DynAbiResult<DynSolType>;
}

impl Resolve for RootType<'_> {
    /// Resolve the type string into a basic Solidity type.
    fn resolve(&self) -> DynAbiResult<DynSolType> {
        match self.span() {
            "address" => Ok(DynSolType::Address),
            "bool" => Ok(DynSolType::Bool),
            "string" => Ok(DynSolType::String),
            "bytes" => Ok(DynSolType::Bytes),
            "uint" => Ok(DynSolType::Uint(256)),
            "int" => Ok(DynSolType::Int(256)),
            name => {
                if let Some(sz) = name.strip_prefix("bytes") {
                    if let Ok(sz) = sz.parse() {
                        if sz != 0 && sz <= 32 {
                            return Ok(DynSolType::FixedBytes(sz))
                        }
                    }
                    return Err(TypeStrError::invalid_size(name).into())
                }

                // fast path both integer types
                let (s, is_uint) = if let Some(s) = name.strip_prefix('u') {
                    (s, true)
                } else {
                    (name, false)
                };

                if let Some(sz) = s.strip_prefix("int") {
                    if let Ok(sz) = sz.parse() {
                        if sz != 0 && sz <= 256 && sz % 8 == 0 {
                            return if is_uint {
                                Ok(DynSolType::Uint(sz))
                            } else {
                                Ok(DynSolType::Int(sz))
                            }
                        }
                    }
                    Err(TypeStrError::invalid_size(name).into())
                } else {
                    Err(TypeStrError::invalid_type_string(name).into())
                }
            }
        }
    }
}

impl Resolve for TupleSpecifier<'_> {
    /// Resolve the type string into a basic Solidity type if possible.
    #[inline]
    fn resolve(&self) -> DynAbiResult<DynSolType> {
        self.types
            .iter()
            .map(TypeSpecifier::resolve)
            .collect::<Result<Vec<_>, _>>()
            .map(DynSolType::Tuple)
    }
}

impl Resolve for TypeStem<'_> {
    /// Resolve the type string into a basic Solidity type if possible.
    #[inline]
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        match self {
            Self::Root(root) => root.resolve(),
            Self::Tuple(tuple) => tuple.resolve(),
        }
    }
}

impl Resolve for TypeSpecifier<'_> {
    /// Resolve the type string into a basic Solidity type if possible.
    #[inline]
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        let ty = self.stem.resolve()?;
        Ok(ty.array_wrap_from_iter(self.sizes.iter().copied()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Result<DynSolType, DynAbiError> {
        s.parse()
    }

    #[test]
    fn extra_close_parens() {
        let test_str = "bool,uint256))";
        assert_eq!(
            parse(test_str),
            Err(alloy_sol_type_str::Error::invalid_type_string(test_str).into())
        );
    }

    #[test]
    fn extra_open_parents() {
        let test_str = "(bool,uint256";
        assert_eq!(
            parse(test_str),
            Err(alloy_sol_type_str::Error::invalid_type_string(test_str).into())
        );
    }

    #[test]
    fn it_parses_tuples() {
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
            Err(TypeStrError::invalid_type_string("MyStruct"))
        );
    }
}
