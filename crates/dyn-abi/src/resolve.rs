//! Contains utilities for parsing Solidity types.
//!
//! This is a simple representation of Solidity type grammar.

use crate::{DynAbiError, DynAbiResult, DynSolType, DynSolValue};
use alloc::vec::Vec;
use alloy_json_abi::{EventParam, Function, Param};
use alloy_sol_type_parser::{
    Error as TypeStrError, RootType, TupleSpecifier, TypeSpecifier, TypeStem,
};
use alloy_sol_types::{Error, Result};

/// The ResolveSolType trait is implemented by types that can be resolved into
/// a [`DynSolType`]. ABI and related systems have many different ways of
/// encoding solidity types. This trait provides a single pattern for resolving
/// those encodings into solidity types.
///
/// This trait is implemented for [`RootType`], [`TupleSpecifier`],
/// [`TypeStem`], and [`TypeSpecifier`], as well as the [`EventParam`] and
/// [`Param`] structs. The impl on `&str` parses a [`TypeSpecifier`] from the
/// string and resolves it.
///
/// ## Example
///
/// ```
/// # use alloy_dyn_abi::{DynSolType, ResolveSolType, DynAbiResult};
/// # use alloy_sol_type_parser::{RootType, TypeSpecifier};
/// # fn main() -> DynAbiResult<()> {
/// let my_ty = TypeSpecifier::try_from("bool")?.resolve()?;
/// assert_eq!(my_ty, DynSolType::Bool);
///
/// let my_ty = RootType::try_from("uint256")?.resolve()?;
/// assert_eq!(my_ty, DynSolType::Uint(256));
///
/// assert_eq!("bytes32".resolve()?, DynSolType::FixedBytes(32));
/// # Ok(())
/// # }
/// ```
pub trait ResolveSolType {
    /// Resolve this object into a [`DynSolType`].
    fn resolve(&self) -> DynAbiResult<DynSolType>;
}

impl ResolveSolType for str {
    #[inline]
    fn resolve(&self) -> DynAbiResult<DynSolType> {
        TypeSpecifier::parse(self)?.resolve()
    }
}

impl ResolveSolType for RootType<'_> {
    fn resolve(&self) -> DynAbiResult<DynSolType> {
        match self.span() {
            "address" => Ok(DynSolType::Address),
            "function" => Ok(DynSolType::Function),
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

impl ResolveSolType for TupleSpecifier<'_> {
    #[inline]
    fn resolve(&self) -> DynAbiResult<DynSolType> {
        self.types
            .iter()
            .map(TypeSpecifier::resolve)
            .collect::<Result<Vec<_>, _>>()
            .map(DynSolType::Tuple)
    }
}

impl ResolveSolType for TypeStem<'_> {
    #[inline]
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        match self {
            Self::Root(root) => root.resolve(),
            Self::Tuple(tuple) => tuple.resolve(),
        }
    }
}

impl ResolveSolType for TypeSpecifier<'_> {
    #[inline]
    fn resolve(&self) -> Result<DynSolType, DynAbiError> {
        self.stem
            .resolve()
            .map(|ty| ty.array_wrap_from_iter(self.sizes.iter().copied()))
    }
}

impl ResolveSolType for Param {
    fn resolve(&self) -> DynAbiResult<DynSolType> {
        let ty = TypeSpecifier::try_from(self.ty.as_str()).expect("always valid");

        // type is simple, and we can resolve it via the specifier
        if self.is_simple_type() {
            return ty.resolve()
        }

        // type is complex
        let tuple = self
            .components
            .iter()
            .map(|c| c.resolve())
            .collect::<Result<Vec<_>, _>>()?;

        #[cfg(feature = "eip712")]
        {
            let prop_names = self.components.iter().map(|c| c.name.clone()).collect();
            if let Some(spec) = self.struct_specifier() {
                return Ok(DynSolType::CustomStruct {
                    name: spec.stem.span().into(),
                    prop_names,
                    tuple,
                }
                .array_wrap_from_iter(spec.sizes.iter().copied()))
            }
        }

        Ok(DynSolType::Tuple(tuple).array_wrap_from_iter(ty.sizes.iter().copied()))
    }
}

impl ResolveSolType for EventParam {
    fn resolve(&self) -> DynAbiResult<DynSolType> {
        let ty = TypeSpecifier::try_from(self.ty.as_str()).expect("always valid");

        // type is simple, and we can resolve it via the specifier
        if self.is_simple_type() {
            return ty.resolve()
        }

        // type is complex. First extract the tuple of inner types
        let tuple = self
            .components
            .iter()
            .map(|c| c.resolve())
            .collect::<Result<Vec<_>, _>>()?;

        // if we have a struct specifier, we can use it to get the name of the
        // struct
        #[cfg(feature = "eip712")]
        {
            let prop_names = self.components.iter().map(|c| c.name.clone()).collect();
            if let Some(spec) = self.struct_specifier() {
                return Ok(DynSolType::CustomStruct {
                    name: spec.stem.span().into(),
                    prop_names,
                    tuple,
                }
                .array_wrap_from_iter(spec.sizes.iter().copied()))
            }
        }

        Ok(DynSolType::Tuple(tuple).array_wrap_from_iter(ty.sizes.iter().copied()))
    }
}

/// Implement to provide encoding and decoding for an ABI Function
pub trait FunctionExt {
    /// Create the ABI call with the given input arguments.
    fn encode_input(&self, args: DynSolValue) -> Result<Vec<u8>>;

    /// Parse the ABI output into DynSolValues
    fn decode_output(&self, data: &[u8]) -> Result<DynSolValue>;
}

impl FunctionExt for Function {
    fn encode_input(&self, args: DynSolValue) -> Result<Vec<u8>> {
        // if the function has no input params, it should take and args
        if self.inputs.is_empty() {
            return Err(Error::Other("no inputs expected for this function".into()))
        }

        // resolve params into their respective DynSolTypes
        let resolved_params = self
            .inputs
            .iter()
            .map(|i| i.resolve().expect("resolve to DynSolType"))
            .collect::<Vec<_>>();

        // since the above may result in a vec of 1 type, we check here
        // to prepare for the check below
        let param_type = match resolved_params.len() {
            1 => resolved_params[0].clone(),
            _ => DynSolType::Tuple(resolved_params),
        };

        // check the expected type(s) match input args
        if !param_type.matches(&args) {
            return Err(Error::Other(
                "input arguments do not match the expected input types".into(),
            ))
        }

        // ABI encode the call
        let encoded = self
            .selector()
            .iter()
            .copied()
            .chain(args.encode_params())
            .collect::<Vec<_>>();

        Ok(encoded)
    }

    fn decode_output(&self, data: &[u8]) -> Result<DynSolValue> {
        let resolved_params = self
            .outputs
            .iter()
            .map(|p| p.resolve().expect("resolve to DynSolType"))
            .collect::<Vec<_>>();

        // since the above may result in a vec of 1 type, we check here
        // to prepare for the check below
        let param_type = match resolved_params.len() {
            1 => resolved_params[0].clone(),
            _ => DynSolType::Tuple(resolved_params),
        };

        let result = param_type.decode_params(data)?;

        // check the expected type(s) match output params
        if !param_type.matches(&result) {
            return Err(Error::Other(
                "decoded data does not match the expected output types".into(),
            ))
        }

        Ok(result)
    }
}

macro_rules! deref_impl {
    ($($(#[$attr:meta])* [$($gen:tt)*] $t:ty),+ $(,)?) => {$(
        $(#[$attr])*
        impl<$($gen)*> ResolveSolType for $t {
            #[inline]
            fn resolve(&self) -> DynAbiResult<DynSolType> {
                (**self).resolve()
            }
        }
    )+};
}

deref_impl! {
    [] alloc::string::String,
    [T: ?Sized + ResolveSolType] &T,
    [T: ?Sized + ResolveSolType] &mut T,
    [T: ?Sized + ResolveSolType] alloc::boxed::Box<T>,
    [T: ?Sized + alloc::borrow::ToOwned + ResolveSolType] alloc::borrow::Cow<'_, T>,
    [T: ?Sized + ResolveSolType] alloc::rc::Rc<T>,
    [T: ?Sized + ResolveSolType] alloc::sync::Arc<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloy_primitives::{Address, U256};

    fn parse(s: &str) -> Result<DynSolType, DynAbiError> {
        s.parse()
    }

    #[test]
    fn extra_close_parens() {
        let test_str = "bool,uint256))";
        assert_eq!(
            parse(test_str),
            Err(alloy_sol_type_parser::Error::invalid_type_string(test_str).into())
        );
    }

    #[test]
    fn extra_open_parents() {
        let test_str = "(bool,uint256";
        assert_eq!(
            parse(test_str),
            Err(alloy_sol_type_parser::Error::invalid_type_string(test_str).into())
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
            parse("tuple(address,bytes, (bool, (string, uint256)[][3]))[2]"),
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
            TypeSpecifier::try_from("tuple(address,bytes, (bool, (string, uint256)[][3]))[2]")
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

    #[test]
    fn can_encode_decode_functions() {
        let json = r#"{
            "inputs": [
                {
                    "internalType": "address",
                    "name": "",
                    "type": "address"
                },
                {
                    "internalType": "address",
                    "name": "",
                    "type": "address"
                }
            ],
            "name": "allowance",
            "outputs": [
                {
                    "internalType": "uint256",
                    "name": "",
                    "type": "uint256"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        }"#;

        let func: Function = serde_json::from_str(json).unwrap();
        assert_eq!(2, func.inputs.len());
        assert_eq!(1, func.outputs.len());

        // encode
        let expected = vec![
            221, 98, 237, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ];
        let input = DynSolValue::Tuple(vec![
            DynSolValue::Address(Address::repeat_byte(1u8)),
            DynSolValue::Address(Address::repeat_byte(2u8)),
        ]);
        let result = func.encode_input(input).unwrap();
        assert_eq!(expected, result);

        // Fail on unexpected input
        let wrong_input = DynSolValue::Tuple(vec![
            DynSolValue::Uint(U256::from(10u8), 256),
            DynSolValue::Address(Address::repeat_byte(2u8)),
        ]);
        assert!(func.encode_input(wrong_input).is_err());

        // decode
        let response = U256::from(1u8).to_be_bytes_vec();
        let decoded = func.decode_output(&response).unwrap();
        assert_eq!(DynSolValue::Uint(U256::from(1u8), 256), decoded);

        // Fail on wrong response type
        let bad_response = Address::repeat_byte(3u8).to_vec();
        assert!(func.decode_output(&bad_response).is_err());
    }
}
